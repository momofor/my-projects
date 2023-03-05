#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let tc1 = dp.TC1;
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b01).cs1().prescale_64());
    pins.d9.into_output();

    loop {
        for duty in 0u8..=255u8 {
            tc1.ocr1a.write(|w| unsafe { w.bits(duty as u16) });
            delay_ms(10);
        }
    }
}
