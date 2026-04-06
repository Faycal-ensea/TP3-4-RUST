#![no_std]
#![no_main]

use defmt_rtt as _; 
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer};

mod bargraph;
use bargraph::BarGraph;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // --- 1. INITIALISATION DU BARGRAPH (8 LEDs) ---
    // On utilise .degrade() pour les transformer en AnyPin
    let led0 = Output::new(p.PC7, Level::Low, Speed::Low).degrade();
    let led1 = Output::new(p.PB2, Level::Low, Speed::Low).degrade();
    let led2 = Output::new(p.PA8, Level::Low, Speed::Low).degrade();
    let led3 = Output::new(p.PB1, Level::Low, Speed::Low).degrade();
    let led4 = Output::new(p.PB15, Level::Low, Speed::Low).degrade();
    let led5 = Output::new(p.PB4, Level::Low, Speed::Low).degrade();
    let led6 = Output::new(p.PB14, Level::Low, Speed::Low).degrade();
    let led7 = Output::new(p.PB5, Level::Low, Speed::Low).degrade();

    // Création du bargraph avec les 8 LEDs
    let mut bargraph = BarGraph::new([led0, led1, led2, led3, led4, led5, led6, led7]);

    // --- 2. INITIALISATION DU GAMEPAD (5 Boutons) ---
    // On configure en entrée avec une résistance de tirage vers le haut (Pull::Up)
    let btn_top = Input::new(p.PC8, Pull::Up);
    let btn_bottom = Input::new(p.PB11, Pull::Up);
    let btn_right = Input::new(p.PC9, Pull::Up);
    let btn_left = Input::new(p.PC6, Pull::Up);
    let btn_center = Input::new(p.PC5, Pull::Up);

    // --- 3. BOUCLE PRINCIPALE ---
    loop {
        // Test interactif : Si on maintient le bouton central enfoncé
        if btn_center.is_low() {
            bargraph.set_level(8); // On allume tout à fond !
            Timer::after(Duration::from_millis(50)).await;
        } 
        else {
            // Sinon, on fait l'animation classique de 0 à 8
            for niveau in 0..=8 {
                // Si on appuie pendant l'animation, on sort de la boucle for
                if btn_center.is_low() { break; } 
                
                bargraph.set_level(niveau);
                Timer::after(Duration::from_millis(150)).await;
            }
        }
    }
}