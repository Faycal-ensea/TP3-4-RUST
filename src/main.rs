#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

// Importations générales et outils de synchronisation
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;

// Importations matérielles (GPIO, Timers, PWM, EXTI)
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::PA15; // Pour la signature de l'arrêt d'urgence
use embassy_stm32::timer::qei::{Qei, QeiPin};
use embassy_stm32::timer::simple_pwm::{SimplePwm, PwmPin};
use embassy_stm32::time::Hertz;

// Nos modules locaux
mod bargraph;
mod encoder;
mod stepper;
mod gamepad;

use bargraph::BarGraph;
use encoder::RotaryEncoder;
use stepper::{Direction, MicrosteppingMode, Stepper};
use gamepad::{Gamepad, GamepadState};

// =========================================================================
// VARIABLES PARTAGÉES (Boîtes aux lettres & Mutex)
// =========================================================================

// Drapeau d'arrêt d'urgence
static EMERGENCY_STOP: AtomicBool = AtomicBool::new(false);

// Variables pour le moteur (0 = Forward, 1 = Backward)
static STEPPER_DIRECTION: AtomicU8 = AtomicU8::new(0);
static STEPPER_SPEED: AtomicU32 = AtomicU32::new(0);
static STEPPER_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

// Mutex pour protéger l'accès matériel à l'encodeur
static ENCODER_MUTEX: Mutex<CriticalSectionRawMutex, ()> = Mutex::new(());

// Mutex pour stocker l'état de la manette
static GAMEPAD_STATE: Mutex<CriticalSectionRawMutex, GamepadState> = Mutex::new(GamepadState::new());

// =========================================================================
// TÂCHES ASYNCHRONES
// =========================================================================

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: BarGraph<'static, 8>) {
    defmt::info!("[TASK] Bargraph démarrée");
    loop {
        bargraph.wait_and_update().await;
    }
}

#[embassy_executor::task]
async fn stepper_update_task(mut stepper: Stepper<'static>) {
    defmt::info!("[TASK] Stepper démarrée");
    stepper.set_microstepping(MicrosteppingMode::Half);

    loop {
        STEPPER_SIGNAL.wait().await;
        
        if EMERGENCY_STOP.load(Ordering::Relaxed) {
            stepper.disable();
            continue;
        }
        
        stepper.enable();
        let speed = STEPPER_SPEED.load(Ordering::Relaxed);
        let dir_val = STEPPER_DIRECTION.load(Ordering::Relaxed);
        
        let direction = if dir_val == 0 { Direction::Forward } else { Direction::Backward };
        stepper.set_speed(speed, direction);
    }
}

#[embassy_executor::task]
async fn encoder_task(encoder: RotaryEncoder<'static>) {
    defmt::info!("[TASK] Encodeur démarrée");
    
    {
        let _lock = ENCODER_MUTEX.lock().await;
        encoder.reset();
    }

    loop {
        Timer::after(Duration::from_millis(200)).await;

        let position;
        {
            let _lock = ENCODER_MUTEX.lock().await;
            position = encoder.get_position();
        }

        let delta = position - 5000;
        let direction = if delta >= 0 { 0 } else { 1 };
        let speed = delta.abs() as u32 * 10; 
        
        let mut level = (delta.abs() / 10) as u32;
        if level > 8 { level = 8; }

        STEPPER_DIRECTION.store(direction, Ordering::Relaxed);
        STEPPER_SPEED.store(speed, Ordering::Relaxed);
        STEPPER_SIGNAL.signal(());

        bargraph::update_value(level);
    }
}

#[embassy_executor::task]
async fn emergency_stop_task(mut btn_emergency: ExtiInput<'static, PA15>) {
    defmt::info!("[TASK] Arrêt d'urgence prête (Bouton central)");
    
    loop {
        btn_emergency.wait_for_falling_edge().await;
        
        defmt::warn!("!!! ARRÊT D'URGENCE ACTIVÉ !!!");

        EMERGENCY_STOP.store(true, Ordering::Relaxed);
        STEPPER_SPEED.store(0, Ordering::Relaxed);
        STEPPER_SIGNAL.signal(()); 

        {
            let _lock = ENCODER_MUTEX.lock().await;
            let tim2 = embassy_stm32::pac::TIM2;
            tim2.cr1().modify(|w| w.set_cen(false));
            tim2.cnt().write_value(embassy_stm32::pac::timer::regs::Cnt32(5000)); 
            tim2.cr1().modify(|w| w.set_cen(true));
        }
        
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn gamepad_task(gamepad: Gamepad<'static>) {
    defmt::info!("[TASK] Gamepad démarrée");

    loop {
        let state = gamepad.poll();

        {
            let mut shared_state = GAMEPAD_STATE.lock().await;
            *shared_state = state; 
        }

        // Si on appuie sur le bouton central de la manette ET que le système est en urgence
        if state.center && EMERGENCY_STOP.load(Ordering::Relaxed) {
            defmt::info!("Système déverrouillé par le Gamepad ! Redémarrage...");
            EMERGENCY_STOP.store(false, Ordering::Relaxed);
            STEPPER_SIGNAL.signal(()); 
        }

        Timer::after(Duration::from_millis(50)).await;
    }
}

// =========================================================================
// PROGRAMME PRINCIPAL
// =========================================================================

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    defmt::info!("Initialisation du matériel...");

    // --- 1. INITIALISATION BARGRAPH ---
    let led0 = Output::new(p.PC7, Level::Low, Speed::Low).degrade();
    let led1 = Output::new(p.PB2, Level::Low, Speed::Low).degrade();
    let led2 = Output::new(p.PA8, Level::Low, Speed::Low).degrade();
    let led3 = Output::new(p.PB1, Level::Low, Speed::Low).degrade();
    let led4 = Output::new(p.PB15, Level::Low, Speed::Low).degrade();
    let led5 = Output::new(p.PB4, Level::Low, Speed::Low).degrade();
    let led6 = Output::new(p.PB14, Level::Low, Speed::Low).degrade();
    let led7 = Output::new(p.PB5, Level::Low, Speed::Low).degrade();
    let bargraph = BarGraph::new([led0, led1, led2, led3, led4, led5, led6, led7]);

    // --- 2. INITIALISATION ENCODEUR & ARRÊT D'URGENCE ---
    let ch1 = QeiPin::new_ch1(p.PA0);
    let ch2 = QeiPin::new_ch2(p.PA1);
    let qei = Qei::new(p.TIM2, ch1, ch2);
    let encoder = RotaryEncoder::new(qei);
    
    let pin_emergency = Input::new(p.PA15, Pull::Up);
    let btn_emergency = ExtiInput::new(pin_emergency, p.EXTI15);

    // --- 3. INITIALISATION MOTEUR ---
    let pwm_ch1 = PwmPin::new_ch1(p.PA6, embassy_stm32::gpio::OutputType::PushPull);
    let pwm = SimplePwm::new(p.TIM3, Some(pwm_ch1), None, None, None, Hertz(1000), Default::default());
    let dir = Output::new(p.PA7, Level::Low, Speed::Low).degrade();
    let enable = Output::new(p.PA12, Level::High, Speed::Low).degrade();
    let ms1 = Output::new(p.PA11, Level::Low, Speed::Low).degrade();
    let ms2 = Output::new(p.PB12, Level::Low, Speed::Low).degrade();
    let motor = Stepper::new(pwm, dir, enable, ms1, ms2);

    // --- 4. INITIALISATION GAMEPAD ---
    let btn_top = Input::new(p.PC8, Pull::Up).degrade();
    let btn_bottom = Input::new(p.PB11, Pull::Up).degrade();
    let btn_right = Input::new(p.PC9, Pull::Up).degrade();
    let btn_left = Input::new(p.PC6, Pull::Up).degrade();
    let btn_center = Input::new(p.PC5, Pull::Up).degrade();
    let gamepad_device = Gamepad::new(btn_top, btn_bottom, btn_left, btn_right, btn_center);

    // --- 5. LANCEMENT DES TÂCHES ---
    defmt::info!("Lancement des tâches...");
    spawner.spawn(bargraph_task(bargraph)).unwrap();
    spawner.spawn(stepper_update_task(motor)).unwrap();
    spawner.spawn(encoder_task(encoder)).unwrap();
    spawner.spawn(emergency_stop_task(btn_emergency)).unwrap();
    spawner.spawn(gamepad_task(gamepad_device)).unwrap();

    // --- 6. BOUCLE PRINCIPALE ---
    loop {
        Timer::after(Duration::from_secs(10)).await;
    }
}