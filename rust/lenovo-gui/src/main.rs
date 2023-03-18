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

enum Device {
    Screen,
    Keyboard,
}

async fn change_state(state: u8, device: Device) {
    match device {
        Device::Screen => {
            fs::write(
                "/sys/class/backlight/intel_backlight/brightness",
                state.to_string(),
            )
            .await
            .expect("Cannot open file make sure to run as root");
        }
        Device::Keyboard => {
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
async fn start_tokio(rx: Receiver<(u8, u8)>) {
    info!("started main tokio loop");
    loop {
        info!("listening to events");
        match rx.recv() {
            Ok(message) => match message {
                (0, 1) | (0, 2) => {
                    change_state(message.1, Device::Keyboard).await;
                    info!("Turned on");
                }
                (0, 0) => {
                    change_state(message.1, Device::Keyboard).await;
                    info!("Turned off");
                }
                (1, brightness) => change_state(brightness, Device::Screen).await,
                _ => tracing::error!("TF is this ?"),
            },
            Err(err) => println!("failed at unpacking with error: {}", err),
        }
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

    thread::spawn(move || start_tokio(rx));

    info!("started second thread");
    let options = eframe::NativeOptions::default();
    tracing::info!("starting stuff and eframe");
    eframe::run_native("MyApp", options, Box::new(|_cc| Box::new(MyApp::new(tx))));
}

struct MyApp {
    sender: Sender<(u8, u8)>,
    brightness: u8,
}

impl MyApp {
    fn new(sender: Sender<(u8, u8)>) -> MyApp {
        Self {
            sender,
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
            }
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}
