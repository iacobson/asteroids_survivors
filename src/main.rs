#![warn(clippy::all, clippy::pedantic)]

use asteroids_survivors::scene::home::Home;
use asteroids_survivors::scene::Scene;
use asteroids_survivors::Drawable;
use asteroids_survivors::Updatable;

use macroquad::color::colors;
use macroquad::window;

mod asteroids_survivors;

#[macroquad::main("Asteroids Survivors")]
async fn main() {
    window::clear_background(colors::BLACK);
    // let mut ship = ship::Ship::new();
    let mut current_scene = Scene::Home(Home::new());

    loop {
        current_scene.update();
        current_scene.draw();

        window::next_frame().await;
    }
}
