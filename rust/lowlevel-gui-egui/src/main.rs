use egui::Context;

fn main() {
    ui_loop();
}

fn ui_loop() {
    let mut ctx = egui::Context::default();

    // Game loop:
    loop {
        let raw_input = egui::RawInput::default();
        let full_output = ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.label("Hello world!");
                if ui.button("Click me").clicked() {
                    println!("Cool")
                    // take some action here
                }
            });
        });
        let clipped_primitives = ctx.tessellate(full_output.shapes); // create triangles to paint
    }
}
