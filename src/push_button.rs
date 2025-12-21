use arduino_hal::port::mode::{Input};
use arduino_hal::port::{Pin, PinOps};
    
pub struct PushButton<PIN: PinOps> {
    pub pbt: Pin<Input<arduino_hal::port::mode::PullUp>, PIN>,
    pub was_pressed: bool,
}

impl<PIN: PinOps> PushButton<PIN> {
    pub fn is_pressed(&self) -> bool {
        self.pbt.is_low()
    }

    // returns true if a transition from 'not pressed' to 'pressed' was detected
    pub fn pressed_transition(&mut self) -> bool {
        let mut transition = false;
        if self.is_pressed() {
            if !self.was_pressed {
                self.was_pressed = true;
                transition = true;
            }
        } else {
            self.was_pressed = false;
        }
        transition
    }
}
