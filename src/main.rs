#![no_std]
#![no_main]

use panic_halt as _;

use push_button::PushButton;

#[macro_use]
pub mod serial;
pub mod push_button;
pub mod led;
use led::LightEmittingDiode;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial_interface = arduino_hal::default_serial!(dp, pins, 38400);
    serial::serial::init(serial_interface);
    
    let mut led_red = LightEmittingDiode { led: pins.d9.into_output(),  state: false, color: "red" };
    let mut led_yel = LightEmittingDiode { led: pins.d10.into_output(), state: false, color: "yellow" };
    let mut led_grn = LightEmittingDiode { led: pins.d11.into_output(), state: false, color: "green" };
    let mut led_blu = LightEmittingDiode { led: pins.d3.into_output(),  state: false, color: "blue" };

    let mut push_button_red = PushButton { pbt: pins.a0.into_pull_up_input(), was_pressed: false };
    let mut push_button_yel = PushButton { pbt: pins.a1.into_pull_up_input(), was_pressed: false };
    let mut push_button_grn = PushButton { pbt: pins.a2.into_pull_up_input(), was_pressed: false };
    let mut push_button_blu = PushButton { pbt: pins.a3.into_pull_up_input(), was_pressed: false };

    serial_println!("Rust application 'Advent wreath' is running. Enjoy.");

    // start-up light show
    for _n in 0..12 {
        led_red.generate_pulse();
        led_yel.generate_pulse();
        led_grn.generate_pulse();
        led_blu.generate_pulse();
        if push_button_red.is_pressed()
            || push_button_yel.is_pressed()
            || push_button_grn.is_pressed()
            || push_button_blu.is_pressed()
        {
            break;
        }
    }

    serial_println!("Jesus Christ is the Light of the world.");
    serial_println!("(John 8:12)");

    loop {
        let mut update_number = true;
        if push_button_red.pressed_transition() {
            led_red.toggle();
        } else if push_button_yel.pressed_transition() {
            led_yel.toggle();
        } else if push_button_grn.pressed_transition() {
            led_grn.toggle();
        } else if push_button_blu.pressed_transition() {
            led_blu.toggle();
        } else {
            // no event
            update_number = false;
        }
        if update_number {
            let candles_lit =
            led_red.state as u8 + led_yel.state as u8 + led_grn.state as u8 + led_blu.state as u8;
            serial_println!("Candles lit: {}", candles_lit);
        }
        arduino_hal::delay_ms(10);
    }
}
