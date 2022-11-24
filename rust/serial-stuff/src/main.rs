use std::{
    fs,
    io::{BufRead, BufReader, Read},
    time::Duration,
};

fn change_state(state: u8, brightness: bool) {
    match brightness {
        true => {
            fs::write(
                "/sys/class/backlight/intel_backlight/brightness",
                state.to_string(),
            )
            .expect("Cannot open file make sure to run as root");
        }
        false => {
            fs::write(
                "/sys/class/leds/tpacpi::kbd_backlight/brightness",
                state.to_string(),
            )
            .expect("Cannot open file make sure to run as root");
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut port = serialport::new("/dev/ttyACM0", 57600)
        .timeout(Duration::from_millis(5000))
        .open()
        .unwrap();
    let mut reader = BufReader::new(port);
    let mut last: u8 = 0;

    for line in reader.lines() {
        let noice = match line.unwrap().parse::<u8>() {
            Ok(num) => num,
            Err(err) => {
                println!("ERROR:{}", err);
                0
            }
        };
        if noice > 0 && last == 0 {
            change_state(0, false);
            change_state(100, true);
            println!("Light Is On");
        } else if noice == 0 && last > 0 {
            change_state(2, false);
            change_state(30, true);
            println!("Light is OFF");
        }
        last = noice;
    }
}
