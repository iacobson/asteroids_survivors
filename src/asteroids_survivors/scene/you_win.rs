use async_trait::async_trait;

use macroquad::camera;

use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

pub struct YouWin {}

impl YouWin {
    pub async fn new() -> Self {
        camera::set_default_camera();
        Self {}
    }
}

impl Updatable for YouWin {
    fn update(&mut self) {}
}

#[async_trait]
impl Scenic for YouWin {
    async fn transition(&self) -> Option<Scene> {
        todo!()
    }
}

impl Drawable for YouWin {
    fn draw(&self) {}
}
