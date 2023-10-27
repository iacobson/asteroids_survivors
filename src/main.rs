#![warn(clippy::all, clippy::pedantic)]

use asteroids_survivors::scene::home::Home;
use asteroids_survivors::scene::Scene;
use asteroids_survivors::Drawable;
use asteroids_survivors::Scenic;
use asteroids_survivors::Updatable;

use macroquad::color::colors;
use macroquad::file::set_pc_assets_folder;
use macroquad::window;

mod asteroids_survivors;

#[macroquad::main("Asteroids Survivors")]
async fn main() {
    set_pc_assets_folder("assets");

    window::clear_background(colors::BLACK);
    let mut current_scene = Scene::Home(Home::new().await);

    loop {
        current_scene.update();
        current_scene.draw();

        if let Some(next_scene) = current_scene.transition().await {
            current_scene = next_scene;
        }

        window::next_frame().await;
    }
}
