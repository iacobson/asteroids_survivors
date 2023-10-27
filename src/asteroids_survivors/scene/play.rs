use macroquad::camera;
use macroquad::color::colors;
use macroquad::math::Vec2;
use macroquad::text;
use macroquad::window;

use crate::asteroids_survivors::util;

use crate::asteroids_survivors::scene::game_over::GameOver;
use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

use self::asteroid::Asteroid;
use self::ship::Ship;

mod asteroid;
mod ship;

pub trait Positionable {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, position: Vec2);
}

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
            ],
            ship: ship,
        }
    }

    fn draw_hull_points(&self) {
        let hull = self.ship.get_hull_points_display();
        let screen_center = self.ship.get_position();

        let text = format!("Hull: {}", hull);
        let text_size = util::text_size(&text, 24.0);

        let mut text_color = colors::WHITE;

        if self.ship.get_taking_damage() {
            text_color = colors::RED;
        }

        text::draw_text(
            &text,
            screen_center.x + window::screen_width() / 2. - text_size.width - 10.,
            screen_center.y - window::screen_height() / 2. + text_size.height + 10.,
            24.0,
            text_color,
        );
    }
}

impl Updatable for Play {
    fn update(&mut self) {
        self.ship.update();

        for asteroid in &mut self.asteroids {
            asteroid.update();
            asteroid_wrap_around(self.ship.get_position(), asteroid)
        }

        for asteroid in &mut self.asteroids {
            if ship_colides_with_asteroid(&self.ship, asteroid) {
                let damage = asteroid.get_damage_dealt();
                self.ship.take_damage(damage);
            }
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

        self.draw_hull_points();
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

fn ship_colides_with_asteroid(ship: &Ship, asteroid: &Asteroid) -> bool {
    let ship_position = ship.get_position();
    let asteroid_position = asteroid.get_position();

    let ship_size = ship.get_height();
    let asteroid_size = asteroid.get_size();

    let distance = (asteroid_position - ship_position).length();

    distance < ship_size / 2. + asteroid_size / 1.05
}
