use embassy_stm32::gpio::{AnyPin, Output}; // Level et Speed retirés
use embassy_stm32::peripherals::TIM3;
// CORRECTION : On utilise timer::simple_pwm et timer::Channel
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::timer::Channel;

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Copy)]
pub enum MicrosteppingMode {
    Full,    // MS1=L, MS2=L
    Half,    // MS1=H, MS2=L
    Quarter, // MS1=L, MS2=H
    Sixteenth, // MS1=H, MS2=H
}

pub struct Stepper<'d> {
    pwm: SimplePwm<'d, TIM3>,
    dir_pin: Output<'d, AnyPin>,
    enable_pin: Output<'d, AnyPin>,
    ms1_pin: Output<'d, AnyPin>,
    ms2_pin: Output<'d, AnyPin>,
}

impl<'d> Stepper<'d> {
    pub fn new(
        pwm: SimplePwm<'d, TIM3>,
        dir_pin: Output<'d, AnyPin>,
        enable_pin: Output<'d, AnyPin>,
        ms1_pin: Output<'d, AnyPin>,
        ms2_pin: Output<'d, AnyPin>,
    ) -> Self {
        let mut s = Self { pwm, dir_pin, enable_pin, ms1_pin, ms2_pin };
        s.disable();
        s
    }

    pub fn enable(&mut self) {
        self.enable_pin.set_low();
    }

    pub fn disable(&mut self) {
        self.enable_pin.set_high();
        self.set_speed(0, Direction::Forward); 
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => self.dir_pin.set_high(),
            Direction::Backward => self.dir_pin.set_low(),
        }
    }

    pub fn set_speed(&mut self, hz: u32, direction: Direction) {
        self.set_direction(direction);
        
        if hz == 0 {
            self.pwm.disable(Channel::Ch1);
        } else {
            // On utilise set_freq (ou set_frequency) en emballant hz dans la structure Hertz
            self.pwm.set_frequency(embassy_stm32::time::Hertz(hz));
            
            let max = self.pwm.get_max_duty();
            self.pwm.set_duty(Channel::Ch1, max / 2);
            self.pwm.enable(Channel::Ch1);
        }
    }

    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
        match mode {
            MicrosteppingMode::Full => {
                self.ms1_pin.set_low();
                self.ms2_pin.set_low();
            }
            MicrosteppingMode::Half => {
                self.ms1_pin.set_high();
                self.ms2_pin.set_low();
            }
            MicrosteppingMode::Quarter => {
                self.ms1_pin.set_low();
                self.ms2_pin.set_high();
            }
            MicrosteppingMode::Sixteenth => {
                self.ms1_pin.set_high();
                self.ms2_pin.set_high();
            }
        }
    }
}