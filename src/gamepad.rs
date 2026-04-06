use embassy_stm32::gpio::{AnyPin, Input};

// Énumération pour identifier chaque bouton
#[derive(Clone, Copy)]
pub enum Button {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

// Structure représentant l'état (appuyé ou non) de tous les boutons à un instant T.
// #[derive(defmt::Format)] permet à defmt d'afficher cette structure automatiquement !
#[derive(defmt::Format)]
pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

// La structure contient 5 entrées "génériques" grâce à AnyPin
pub struct Gamepad<'d> {
    top: Input<'d, AnyPin>,
    bottom: Input<'d, AnyPin>,
    left: Input<'d, AnyPin>,
    right: Input<'d, AnyPin>,
    center: Input<'d, AnyPin>,
}

impl<'d> Gamepad<'d> {
    // Le constructeur pour initialiser le gamepad
    pub fn new(
        top: Input<'d, AnyPin>,
        bottom: Input<'d, AnyPin>,
        left: Input<'d, AnyPin>,
        right: Input<'d, AnyPin>,
        center: Input<'d, AnyPin>,
    ) -> Self {
        Self { top, bottom, left, right, center }
    }

    // Méthode 1 : Vérifier si UN bouton spécifique est pressé
    // Rappel : avec un Pull::Up, le signal est bas (low) quand on appuie.
    pub fn is_pressed(&self, button: Button) -> bool {
        match button {
            Button::Top => self.top.is_low(),
            Button::Bottom => self.bottom.is_low(),
            Button::Left => self.left.is_low(),
            Button::Right => self.right.is_low(),
            Button::Center => self.center.is_low(),
        }
    }

    // Méthode 2 : Prendre une "photo" de l'état de tous les boutons
    pub fn poll(&self) -> GamepadState {
        GamepadState {
            top: self.is_pressed(Button::Top),
            bottom: self.is_pressed(Button::Bottom),
            left: self.is_pressed(Button::Left),
            right: self.is_pressed(Button::Right),
            center: self.is_pressed(Button::Center),
        }
    }
}