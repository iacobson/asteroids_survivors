use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Updatable;

pub struct GameOver {}
impl Updatable for GameOver {
    fn update(&mut self) {}
}

impl Drawable for GameOver {
    fn draw(&self) {}
}
