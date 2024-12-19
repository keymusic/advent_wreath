
// Code is taken from https://github.com/Rahix/avr-hal/issues/115 and slightly modified.

pub mod serial {
    use avr_device::interrupt::Mutex;
    use core::cell::RefCell;

    pub type Usart = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
    pub static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

    pub fn init(serial: Usart) {
        avr_device::interrupt::free(|cs| {
            GLOBAL_SERIAL.borrow(cs).replace(Some(serial));
        })
    }

    #[macro_export]
    macro_rules! serial_println {
        ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
                if let Some(serial) = &mut *serial::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                    ::ufmt::uwriteln!(serial, $($arg)*).unwrap();
                }
            })
        }
    }
}
