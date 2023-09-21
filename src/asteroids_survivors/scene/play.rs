mod ship;

use crate::asteroids_survivors::scene::game_over::GameOver;
use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

use self::ship::Ship;

pub struct Play {
    ship: Ship,
}

impl Play {
    pub fn new() -> Self {
        Self { ship: Ship::new() }
    }
}

impl Updatable for Play {
    fn update(&mut self) {
        self.ship.update();
    }
}

impl Scenic for Play {
    fn transition(&self) -> Option<Scene> {
        if self.ship.is_destroyed() {
            Some(Scene::GameOver(GameOver::new()))
        } else {
            None
        }
    }
}

impl Drawable for Play {
    fn draw(&self) {
        self.ship.draw();
    }
}
