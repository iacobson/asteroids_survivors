use async_trait::async_trait;

use macroquad::camera;
use macroquad::color::colors;
use macroquad::math::Vec2;
use macroquad::text;
use macroquad::time;
use macroquad::window;
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::prelude::ParallelIterator;

use crate::asteroids_survivors::util;

use crate::asteroids_survivors::scene::game_over::GameOver;
use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

use crate::asteroids_survivors::Frame;

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
    fps: u16,
    fps_acc: u16,
    timer_second: u16,
}

impl Play {
    pub async fn new() -> Self {
        camera::set_default_camera();

        let ship = Ship::new().await;

        let ship_position = ship.get_position();

        Self {
            asteroids: vec![
                Asteroid::new(&ship_position).await,
                Asteroid::new(&ship_position).await,
                Asteroid::new(&ship_position).await,
            ],
            ship: ship,
            fps: 0,
            fps_acc: 0,
            timer_second: 0,
        }
    }

    fn calculate_fps(&mut self) {
        let delta_ms = (time::get_frame_time() * 1000.) as u16;

        self.timer_second += delta_ms;

        if self.timer_second >= 1000 {
            self.fps = self.fps_acc;
            self.fps_acc = 0;
            self.timer_second = self.timer_second - 1000;
        } else {
            self.fps_acc += 1;
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

    fn draw_fps(&self) {
        let fps = self.fps;
        let text = format!("{} FPS", fps);
        let text_size = util::text_size(&text, 24.0);

        let screen_center = self.ship.get_position();
        text::draw_text(
            &text,
            screen_center.x + window::screen_width() / 2. - text_size.width - 10.,
            screen_center.y + window::screen_height() / 2. - text_size.height - 10.,
            24.0,
            colors::WHITE,
        );
    }
}

impl Updatable for Play {
    fn update(&mut self, frame: &Frame) {
        self.calculate_fps();
        self.ship.update(frame);

        self.asteroids.par_iter_mut().for_each(|asteroid| {
            asteroid.update(frame);
            asteroid_wrap_around(self.ship.get_position(), asteroid, frame)
        });

        for asteroid in &mut self.asteroids {
            if ship_colides_with_asteroid(&self.ship, asteroid) {
                let damage = asteroid.get_damage_dealt();
                self.ship.take_damage(damage);
            }
        }
    }
}

#[async_trait]
impl Scenic for Play {
    async fn transition(&self) -> Option<Scene> {
        if self.ship.is_destroyed() {
            Some(Scene::GameOver(GameOver::new().await))
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
        self.draw_fps();
    }
}

fn asteroid_wrap_around(screen_center: Vec2, asteroid: &mut Asteroid, frame: &Frame) {
    let mut vr = asteroid.get_position();

    if vr.x > screen_center.x + frame.screen_width / 2. {
        vr.x = screen_center.x - frame.screen_width / 2.;
    }

    if vr.x < screen_center.x - frame.screen_width / 2. {
        vr.x = screen_center.x + frame.screen_width / 2.;
    }

    if vr.y > screen_center.y + frame.screen_height / 2. {
        vr.y = screen_center.y - frame.screen_height / 2.;
    }

    if vr.y < screen_center.y - frame.screen_height / 2. {
        vr.y = screen_center.y + frame.screen_height / 2.;
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
