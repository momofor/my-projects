use eframe::egui;
use egui::Color32;
use nix::unistd::Uid;
use std::{
    io,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
use tokio::fs;
use tracing::{info, instrument};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

async fn change_state(state: u8, brightness: bool) {
    match brightness {
        true => {
            fs::write(
                "/sys/class/backlight/intel_backlight/brightness",
                state.to_string(),
            )
            .await
            .expect("Cannot open file make sure to run as root");
        }
        false => {
            fs::write(
                "/sys/class/leds/tpacpi::kbd_backlight/brightness",
                state.to_string(),
            )
            .await
            .expect("Cannot open file make sure to run as root");
        }
    }
}

#[instrument]
#[tokio::main]
async fn start_tokio(rx: Receiver<(u8, u8)>, state: Arc<Mutex<bool>>) {
    info!("started main tokio loop");
    loop {
        info!("listening to events");
        match rx.recv() {
            Ok(message) => {
                let mut state = state.lock().unwrap();
                match message {
                    (0, 1) | (0, 2) => {
                        *state = true;
                        change_state(message.1, false).await;
                        info!("Turned on");
                    }
                    (0, 0) => {
                        *state = false;
                        change_state(message.1, false).await;
                        info!("Turned off");
                    }
                    (1, brightness) => change_state(brightness, true).await,
                    _ => println!("TF is this ?"),
                }
            }
            Err(err) => println!("failed at unpacking with error: {}", err),
        }
    }
}

#[instrument]
fn fifo_handler(cloned_tx: Sender<(u8, u8)>) {
    loop {
        info!("Startig loop thread");
        let mut buffer = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut buffer) {
            Ok(_) => info!("Succesfully got input"),
            Err(err) => tracing::error!(error = %err),
        }
        info!("Listening for new input");
        info!("Got {}", buffer);
        match buffer.parse::<u8>() {
            Ok(val) => {
                info!("Got integer {}", val);
                cloned_tx.send((0, val)).unwrap();
            }
            Err(err) => {
                tracing::error!(error = %err);
            }
        };
    }
}

#[instrument]
fn main() {
    if !Uid::effective().is_root() {
        panic!("RUN AS ROOT!");
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO))
        .init();
    let (tx, rx) = channel();
    let cloned_tx = tx.clone();

    let state = Arc::new(Mutex::new(true));
    let cloned_state = Arc::clone(&state);

    thread::spawn(move || fifo_handler(cloned_tx));

    thread::spawn(move || start_tokio(rx, state));

    info!("started second thread");
    let options = eframe::NativeOptions::default();
    tracing::info!("starting stuff and eframe");
    eframe::run_native(
        "MyApp",
        options,
        Box::new(|_cc| Box::new(MyApp::new(tx, cloned_state))),
    );
}

struct MyApp {
    sender: Sender<(u8, u8)>,
    state: Arc<Mutex<bool>>,
    brightness: u8,
}

impl MyApp {
    fn new(sender: Sender<(u8, u8)>, state: Arc<Mutex<bool>>) -> MyApp {
        Self {
            sender,
            state,
            brightness: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut visuals = egui::Visuals::default();
            visuals.dark_mode = true;
            visuals.override_text_color = Some(Color32::GREEN);
            ctx.set_visuals(visuals);

            egui::widgets::global_dark_light_mode_switch(ui);

            let lights_state = match *self.state.lock().unwrap() {
                true => "ON",
                false => "OFF",
            };
            ui.heading(lights_state);

            ui.horizontal(|ui| {
                if ui.button("ON").clicked() {
                    self.sender.send((0, 2)).unwrap();
                }
                if ui.button("MEDIUM").clicked() {
                    self.sender.send((0, 1)).unwrap();
                }
                if ui.button("OFF").clicked() {
                    self.sender.send((0, 0)).unwrap();
                }
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.brightness, 0..=200)
                        .text("Brightness")
                        .suffix("%"),
                );
            });
            if ui.button("Set brightness").clicked() {
                self.sender.send((1, self.brightness)).unwrap();
                let cur_brightness =
                    std::fs::read_to_string("/sys/class/backlight/intel_backlight/brightness")
                        .unwrap();
                ui.heading(&cur_brightness);
                info!(cur_brightness = %cur_brightness);
            }
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}
