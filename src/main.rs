#![no_std]
#![no_main]
#[macro_use]

pub mod serial;
pub mod push_button;
pub mod led;

use panic_halt as _;
use push_button::PushButton;
use led::LightEmittingDiode;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial_interface = arduino_hal::default_serial!(dp, pins, 38400);
    serial::serial::init(serial_interface);
    
    let mut led_red = LightEmittingDiode { led: pins.d9.into_output().downgrade(),  state: false, color: "red" };
    let mut led_yel = LightEmittingDiode { led: pins.d10.into_output().downgrade(), state: false, color: "yellow" };
    let mut led_grn = LightEmittingDiode { led: pins.d11.into_output().downgrade(), state: false, color: "green" };
    let mut led_blu = LightEmittingDiode { led: pins.d3.into_output().downgrade(),  state: false, color: "blue" };
    let mut leds = [&mut led_red, &mut led_yel, &mut led_grn, &mut led_blu];

    let mut push_button_red = PushButton { pbt: pins.a0.into_pull_up_input().downgrade(), was_pressed: false };
    let mut push_button_yel = PushButton { pbt: pins.a1.into_pull_up_input().downgrade(), was_pressed: false };
    let mut push_button_grn = PushButton { pbt: pins.a2.into_pull_up_input().downgrade(), was_pressed: false };
    let mut push_button_blu = PushButton { pbt: pins.a3.into_pull_up_input().downgrade(), was_pressed: false };
    let mut push_buttons = [&mut push_button_red, &mut push_button_yel, &mut push_button_grn, &mut push_button_blu];

    serial_println!("Rust application 'Advent wreath' is running. Enjoy.");

    // start-up light show
    'outer: for _n in 0..12 {
        for led in &mut leds {
            led.generate_pulse();
        }
        for pb in &mut push_buttons {
            if pb.is_pressed() {
                break 'outer;
            }
        }
    }

    serial_println!("Jesus Christ is the Light of the world.");
    serial_println!("(John 8:12)");

    loop {
        let mut update_number = false;
        for (i, pb) in &mut push_buttons.iter_mut().enumerate() {
            if pb.pressed_transition() {
                // We assume the same order within the arrays 'leds' and 'push_buttons'.
                leds[i].toggle();
                update_number = true;
            }
        }
        if update_number {
            let mut candles_lit = 0;
            for led in &mut leds {
                candles_lit += led.state as u8;
            }
            serial_println!("Candles lit: {}", candles_lit);
        }
        arduino_hal::delay_ms(10);
    }
}
