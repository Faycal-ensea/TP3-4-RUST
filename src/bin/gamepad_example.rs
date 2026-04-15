#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};

// Cette ligne permet d'importer ton fichier gamepad.rs qui est dans le dossier parent
#[path = "../gamepad.rs"]
mod gamepad;
use gamepad::{Button, Gamepad};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // 1. Initialisation des 5 boutons en entrée (Pull-Up) et dégradation en AnyPin
    let btn_top = Input::new(p.PC8, Pull::Up).degrade();
    let btn_bottom = Input::new(p.PB11, Pull::Up).degrade();
    let btn_right = Input::new(p.PC9, Pull::Up).degrade();
    let btn_left = Input::new(p.PC6, Pull::Up).degrade();
    let btn_center = Input::new(p.PC5, Pull::Up).degrade();

    // 2. Création de l'instance du Gamepad
    let gamepad = Gamepad::new(btn_top, btn_bottom, btn_left, btn_right, btn_center);

    defmt::info!("Gamepad initialisé. Lecture en cours...");

    // 3. Boucle principale synchrone (comme demandé : pas d'interruptions)
    loop {
        // On récupère l'état global avec poll()
        let state = gamepad.poll();

        // defmt va automatiquement formater notre structure GamepadState
        defmt::info!("Etat global : {}", state);

        // Exemple d'utilisation de is_pressed() seul
        if gamepad.is_pressed(Button::Center) {
            defmt::info!("Le bouton central est pressé ");
        }

        // On patiente 500ms pour ne pas inonder le terminal de messages
        Timer::after(Duration::from_millis(500)).await;
    }
}