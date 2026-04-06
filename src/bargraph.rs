use embassy_stm32::gpio::{AnyPin, Output};

pub struct BarGraph<'d, const N: usize> {
    leds: [Output<'d, AnyPin>; N],
}

impl<'d, const N: usize> BarGraph<'d, N> {
    pub fn new(leds: [Output<'d, AnyPin>; N]) -> Self {
        Self { leds }
    }

    pub fn set_level(&mut self, level: usize) {
        for (i, led) in self.leds.iter_mut().enumerate() {
            if i < level {
                led.set_high(); // Allume la LED
            } else {
                led.set_low();  // Éteint la LED
            }
        }
    }
}