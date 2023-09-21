use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Updatable;

pub struct YouWin {}

impl Updatable for YouWin {
    fn update(&mut self) {}
}

impl Drawable for YouWin {
    fn draw(&self) {}
}
