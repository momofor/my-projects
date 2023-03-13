#![no_std]
#![no_main]

use arduino_hal::adc;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer2Pwm};
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

    let a3 = pins.a3.into_analog_input(&mut adc);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut led = pins.d11.into_output().into_pwm(&timer2);
    let mut state: u8 = 0;

    loop {
        let voltage = nb::block!(adc.read_nonblocking(&a3)).void_unwrap();
        if voltage != 0 && state == 0 {
            ufmt::uwriteln!(&mut serial, "Light is on and voltage is {}", &voltage).void_unwrap();
            state = 1;
            led.set_duty(voltage as u8 * 10);
            led.enable();
        } else if voltage == 0 && state == 1 {
            ufmt::uwriteln!(&mut serial, "Light is now off").void_unwrap();
            state = 0;
            led.disable();
        }
        delay_ms(100);
    }
}
