use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    // main loop
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
