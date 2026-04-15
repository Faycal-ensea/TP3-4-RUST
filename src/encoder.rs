use embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::qei::Qei;
use embassy_stm32::pac::timer::regs::{Arr32, Cnt32};

pub struct RotaryEncoder<'d> {
    qei: Qei<'d, TIM2>,
}

impl<'d> RotaryEncoder<'d> {
    pub fn new(qei: Qei<'d, TIM2>) -> Self {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.arr().write_value(Arr32(10_000));
        Self { qei }
    }

    pub fn get_position(&self) -> i32 {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.cnt().read().0 as i32
    }

    pub fn set_position(&self, position: i32) {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.cnt().write_value(Cnt32(position as u32));
    }

    pub fn reset(&self) {
        self.set_position(5_000);
    }
}