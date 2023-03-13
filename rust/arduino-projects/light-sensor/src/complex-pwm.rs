#![no_std]
#![no_main]

use arduino_hal::adc;
use arduino_hal::{delay_ms, prelude::*};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).void_unwrap();

    let pin = pins.a3.into_analog_input(&mut adc);
    let mut led = pins.d11.into_output();

    let tc1 = dp.TC1;
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b01).cs1().prescale_64());
    let pwm_pin = pins.d9.into_output();

    loop {
        let value = pin.analog_read(&mut adc);
        for duty in 0u8..=255u8 {
            ufmt::uwrite!(&mut serial, "Duty: {}", duty).void_unwrap();
            tc1.ocr1a.write(|w| unsafe { w.bits(duty as u16) });
        }
        if value > 0 {
            led.set_low();
        } else {
            led.set_high();
        }
        ufmt::uwriteln!(&mut serial, "{}", value / 2).void_unwrap();
        delay_ms(500);
    }
}
