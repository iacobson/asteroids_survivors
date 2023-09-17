#![warn(clippy::all, clippy::pedantic)]

use macroquad::color::colors;
use macroquad::window;

mod asteroids_survivors;

use asteroids_survivors::Drawable;

use asteroids_survivors::ship;

#[macroquad::main("Asteroids Survivors")]
async fn main() {
    window::clear_background(colors::BLACK);
    let mut ship = ship::Ship::new();

    loop {
        ship.draw();
        window::next_frame().await;
    }
}
