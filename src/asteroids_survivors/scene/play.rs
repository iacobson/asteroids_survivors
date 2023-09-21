mod ship;

use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Updatable;

pub struct Play {}

pub struct GameOver {}
impl Updatable for Play {
    fn update(&mut self) {}
}

impl Drawable for Play {
    fn draw(&self) {}
}
