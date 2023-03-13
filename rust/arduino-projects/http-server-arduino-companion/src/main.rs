#![no_std]
#![no_main]

use arduino_hal::{delay_ms, prelude::*};
use panic_halt as _;

use embedded_hal::{digital::v2::OutputPin, serial::Read};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    loop {
        // Read a byte from the serial connection

        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        if b == 1 {
            ufmt::uwriteln!(&mut serial, "ON").void_unwrap();
        } else if b == 0 {
            ufmt::uwriteln!(&mut serial, "OFFFFF").void_unwrap();
        } else if b != 1 && b != 0 {
            ufmt::uwriteln!(&mut serial, "DIFERENT: {}", b).void_unwrap();
        }
    }
}
