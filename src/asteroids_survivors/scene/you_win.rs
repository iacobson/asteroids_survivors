use macroquad::camera;

use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

pub struct YouWin {}

impl YouWin {
    pub fn new() -> Self {
        camera::set_default_camera();
        Self {}
    }
}

impl Updatable for YouWin {
    fn update(&mut self) {}
}

impl Scenic for YouWin {
    fn transition(&self) -> Option<Scene> {
        todo!()
    }
}

impl Drawable for YouWin {
    fn draw(&self) {}
}
