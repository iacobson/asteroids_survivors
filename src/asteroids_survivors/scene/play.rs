use macroquad::camera;
use macroquad::math::Vec2;
use macroquad::window;

use crate::asteroids_survivors::scene::game_over::GameOver;
use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

use self::asteroid::Asteroid;
use self::ship::Ship;

mod asteroid;
mod ship;

pub struct Play {
    ship: Ship,
    asteroids: Vec<Asteroid>,
}

impl Play {
    pub fn new() -> Self {
        camera::set_default_camera();

        let ship = Ship::new();

        let ship_position = ship.get_position();

        Self {
            asteroids: vec![
                Asteroid::new(&ship_position),
                Asteroid::new(&ship_position),
                Asteroid::new(&ship_position),
                Asteroid::new(&ship_position),
                Asteroid::new(&ship_position),
            ],
            ship: ship,
        }
    }
}

impl Updatable for Play {
    fn update(&mut self) {
        self.ship.update();

        for asteroid in &mut self.asteroids {
            asteroid.update();
            asteroid_wrap_around(self.ship.get_position(), asteroid)
        }
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

        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }
}

fn asteroid_wrap_around(screen_center: Vec2, asteroid: &mut Asteroid) {
    let mut vr = asteroid.get_position();

    if vr.x > screen_center.x + window::screen_width() / 2. {
        vr.x = screen_center.x - window::screen_width() / 2.;
    }

    if vr.x < screen_center.x - window::screen_width() / 2. {
        vr.x = screen_center.x + window::screen_width() / 2.;
    }

    if vr.y > screen_center.y + window::screen_height() / 2. {
        vr.y = screen_center.y - window::screen_height() / 2.;
    }

    if vr.y < screen_center.y - window::screen_height() / 2. {
        vr.y = screen_center.y + window::screen_height() / 2.;
    }

    asteroid.set_position(vr);
}

pub trait Positionable {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, position: Vec2);
}
