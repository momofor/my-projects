use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = 1.;
    let mut y = 1.;
    let mut velocity = 1.;
    let acceleration = 1.0.1;
    loop {
        clear_background(GRAY);
        if x > 1920. || y > 1080. || x < 0. || y < 0. {
            x = 0.;
            y = 0.;

            velocity = 1.;
        }
        velocity = acceleration * velocity;
        if is_key_down(KeyCode::K) {
            y = y - 1. - velocity;
        }
        if is_key_down(KeyCode::J) {
            y = y + 1. + velocity;
        }
        if is_key_down(KeyCode::H) {
            x = x - 1. - velocity;
        }
        if is_key_down(KeyCode::L) {
            x = x + 1. + velocity;
        }

        draw_circle(x, y, 20., GREEN);

        draw_text(x.to_string().as_str(), x + 300., y + 20., 50., YELLOW);
        draw_text(y.to_string().as_str(), y + 100., y + 20., 50., GREEN);

        next_frame().await
    }
}
