use embassy_stm32::gpio::AnyPin;
use embassy_stm32::Peripherals;

// 1. La boîte pour le Bargraph
pub struct BargraphPins {
    pub led7: AnyPin,
    pub led6: AnyPin,
    pub led5: AnyPin,
    pub led4: AnyPin,
    pub led3: AnyPin,
    pub led2: AnyPin,
    pub led1: AnyPin,
    pub led0: AnyPin,
}

// 2. La boîte pour le Gamepad
pub struct GamepadPins {
    pub top: AnyPin,
    pub bottom: AnyPin,
    pub right: AnyPin,
    pub left: AnyPin,
    pub center: AnyPin,
}

// 3. La boîte pour l'Encodeur Rotatif
pub struct EncoderPins {
    pub button: AnyPin,
    pub ch_a: AnyPin, // PA0
    pub ch_b: AnyPin, // PA1
}

pub struct Board {
    pub bargraph_pins: BargraphPins,
    pub gamepad_pins: GamepadPins,
    pub encoder_pins: EncoderPins,
    
    // On garde la place au chaud pour les prochains TP !
    // pub stepper_pins: StepperPins,
    // pub spi2_pins: Spi2Pins,
    // pub i2c1_pins: I2c1Pins,
    // pub usart1_pins: Usart1Pins,
}

impl Board {
    /// Prend possession de tous les périphériques matériels et les distribue
    pub fn new(p: Peripherals) -> Self {
        Self {
            bargraph_pins: BargraphPins {
                led7: p.PB5.into(),
                led6: p.PB14.into(),
                led5: p.PB4.into(),
                led4: p.PB15.into(),
                led3: p.PB1.into(),
                led2: p.PA8.into(),
                led1: p.PB2.into(),
                led0: p.PC7.into(),
            },
            gamepad_pins: GamepadPins {
                top: p.PC8.into(),
                bottom: p.PB11.into(),
                right: p.PC9.into(),
                left: p.PC6.into(),
                center: p.PC5.into(),
            },
            encoder_pins: EncoderPins {
                button: p.PA15.into(),
                ch_a: p.PA0.into(),
                ch_b: p.PA1.into(),
            },
        }
    }
}