use embassy_stm32::gpio::{AnyPin, Input};
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::qei::Qei;

// CORRECTION ICI : On utilise `pac` au lieu de `stm32_metapac`
use embassy_stm32::pac::timer::regs::{Arr32, Cnt32};

pub struct RotaryEncoder<'d> {
    qei: Qei<'d, TIM2>,
    button: Input<'d, AnyPin>,
}

impl<'d> RotaryEncoder<'d> {
    pub fn new(qei: Qei<'d, TIM2>, button: Input<'d, AnyPin>) -> Self {
        let tim2 = embassy_stm32::pac::TIM2;
        
        // On emballe 10_000 dans la structure Arr32
        tim2.arr().write_value(Arr32(10_000));

        Self { qei, button }
    }

    pub fn is_pressed(&self) -> bool {
        self.button.is_low() 
    }

    pub fn get_position(&self) -> i32 {
        let tim2 = embassy_stm32::pac::TIM2;
        // read() renvoie un struct Cnt32. On utilise .0 pour extraire le nombre
        tim2.cnt().read().0 as i32
    }

    pub fn set_position(&self, position: i32) {
        let tim2 = embassy_stm32::pac::TIM2;
        // On emballe notre position dans Cnt32
        tim2.cnt().write_value(Cnt32(position as u32));
    }

    pub fn reset(&self) {
        self.set_position(5_000);
    }
}