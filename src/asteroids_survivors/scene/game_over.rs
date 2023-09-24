use macroquad::camera;

use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

pub struct GameOver {}

impl GameOver {
    pub fn new() -> Self {
        camera::set_default_camera();
        Self {}
    }
}

impl Updatable for GameOver {
    fn update(&mut self) {}
}

impl Scenic for GameOver {
    fn transition(&self) -> Option<Scene> {
        todo!()
    }
}

impl Drawable for GameOver {
    fn draw(&self) {}
}
