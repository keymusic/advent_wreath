use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};

pub struct LightEmittingDiode<PIN: PinOps> {
    pub led: Pin<Output, PIN>,
    pub state: bool,
    pub color: &'static str,
}

impl<PIN: PinOps> LightEmittingDiode<PIN> {
    fn set_low(&mut self) {
        self.led.set_low();
        self.state = false;
    }

    fn set_high(&mut self) {
        self.led.set_high();
        self.state = true;
    }

    pub fn generate_pulse(&mut self) {
        self.set_high();
        arduino_hal::delay_ms(25);
        self.set_low();
        arduino_hal::delay_ms(25);
    }

    pub fn toggle(&mut self) {
        if self.state {
            self.set_low();
        } else {
            self.set_high();
        }
        /*
        let state_str = if self.state { "on" } else { "off" };
        serial_println!("Toggle {} LED {}.", self.color, state_str);
        */
    }
}
