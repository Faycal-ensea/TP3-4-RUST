#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
// On ajoute QeiPin à l'importation
use embassy_stm32::timer::qei::{Qei, QeiPin}; 
use embassy_time::{Duration, Timer};

#[path = "../encoder.rs"]
mod encoder;
use encoder::RotaryEncoder;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let timer = p.TIM2;
    
    // On transforme nos broches brutes en canaux QEI officiels
    let ch1 = QeiPin::new_ch1(p.PA0);
    let ch2 = QeiPin::new_ch2(p.PA1);
    
    let qei = Qei::new(timer, ch1, ch2);

    // Initialisation du bouton (toujours avec .degrade() pour avoir un AnyPin)
    let btn_encoder = Input::new(p.PA15, Pull::Up).degrade();

    // Création du driver
    let encoder = RotaryEncoder::new(qei, btn_encoder);

    // On force la position au milieu
    encoder.reset();

    defmt::info!("Encodeur pret ! Tournez la molette.");

    let mut last_position = encoder.get_position();

    loop {
        let current_position = encoder.get_position();

        if current_position != last_position {
            defmt::info!("Position de l'encodeur : {}", current_position);
            last_position = current_position;
        }

        if encoder.is_pressed() {
            defmt::info!("Bouton presse ! Retour au centre.");
            encoder.reset();
            last_position = encoder.get_position();
            
            Timer::after(Duration::from_millis(300)).await;
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}