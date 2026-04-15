#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Output, Level, Speed};
// CORRECTION : On utilise timer::simple_pwm
use embassy_stm32::timer::simple_pwm::{SimplePwm, PwmPin};
use embassy_time::{Duration, Timer};

#[path = "../stepper.rs"]
mod stepper;
use stepper::{Stepper, Direction, MicrosteppingMode};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // 1. Configurer le PWM pour le pin Step (PA6 -> TIM3_CH1)
    let ch1 = PwmPin::new_ch1(p.PA6, embassy_stm32::gpio::OutputType::PushPull);
    let pwm = SimplePwm::new(p.TIM3, Some(ch1), None, None, None, embassy_stm32::time::Hertz(1000), Default::default());

    // 2. Configurer les autres pins GPIO
    let dir = Output::new(p.PA7, Level::Low, Speed::Low).degrade();
    let enable = Output::new(p.PA12, Level::High, Speed::Low).degrade();
    let ms1 = Output::new(p.PA11, Level::Low, Speed::Low).degrade();
    let ms2 = Output::new(p.PB12, Level::Low, Speed::Low).degrade();

    // 3. Créer le driver
    let mut motor = Stepper::new(pwm, dir, enable, ms1, ms2);

    defmt::info!("Moteur prêt ! Activation...");
    motor.enable();
    motor.set_microstepping(MicrosteppingMode::Half);

    loop {
        // Rotation sens horaire, 1000 pas par seconde
        defmt::info!("Avance...");
        motor.set_speed(1000, Direction::Forward);
        Timer::after(Duration::from_secs(2)).await;

        // Arrêt
        defmt::info!("Pause...");
        motor.set_speed(0, Direction::Forward);
        Timer::after(Duration::from_secs(1)).await;

        // Rotation sens inverse, plus lent
        defmt::info!("Retour...");
        motor.set_speed(500, Direction::Backward);
        Timer::after(Duration::from_secs(2)).await;
    }
}