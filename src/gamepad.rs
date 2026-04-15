use embassy_stm32::gpio::{AnyPin, Input};

// L'état de la manette, conçu pour être facilement copié et partagé
#[derive(Clone, Copy, defmt::Format)]
pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

impl GamepadState {
    // Une fonction const pour initialiser notre variable globale (le Mutex)
    pub const fn new() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            center: false,
        }
    }
}

// Le driver de la manette avec ses 5 broches d'entrée
pub struct Gamepad<'d> {
    btn_top: Input<'d, AnyPin>,
    btn_bottom: Input<'d, AnyPin>,
    btn_left: Input<'d, AnyPin>,
    btn_right: Input<'d, AnyPin>,
    btn_center: Input<'d, AnyPin>,
}

impl<'d> Gamepad<'d> {
    pub fn new(
        btn_top: Input<'d, AnyPin>,
        btn_bottom: Input<'d, AnyPin>,
        btn_left: Input<'d, AnyPin>,
        btn_right: Input<'d, AnyPin>,
        btn_center: Input<'d, AnyPin>,
    ) -> Self {
        Self {
            btn_top,
            btn_bottom,
            btn_left,
            btn_right,
            btn_center,
        }
    }

    // Fonction synchrone qui prend une "photo" de l'état des boutons
    pub fn poll(&self) -> GamepadState {
        // Comme on a configuré les boutons en Pull::Up dans le main,
        // le signal est "bas" (low) quand le bouton est pressé.
        GamepadState {
            top: self.btn_top.is_low(),
            bottom: self.btn_bottom.is_low(),
            left: self.btn_left.is_low(),
            right: self.btn_right.is_low(),
            center: self.btn_center.is_low(),
        }
    }
}