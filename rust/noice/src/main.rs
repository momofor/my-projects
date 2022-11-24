use std::{
    io::{self, Write},
    process::Command,
};

use inputbot::KeybdKey::*;

fn change_state(state: u8) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            " sudo light -S {} -s sysfs/leds/tpacpi::kbd_backlight",
            state,
        ))
        .output()
        .expect("noice");
    io::stdout().write_all(&output.stdout).unwrap();
}

#[tokio::main]
async fn main() {
    // loop {
    //     change_state(0);
    //     thread::sleep(Duration::from_millis(500));
    //     change_state(50);
    //     thread::sleep(Duration::from_millis(500));
    //     change_state(100);
    //     thread::sleep(Duration::from_millis(500));
    // }

    F9Key.bind(|| change_state(0));
    F10Key.bind(|| change_state(100));

    inputbot::handle_input_events();
}
