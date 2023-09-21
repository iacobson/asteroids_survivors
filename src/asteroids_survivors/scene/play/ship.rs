use macroquad::color::colors;
use macroquad::input;
use macroquad::math::Vec2;
use macroquad::shapes;
use macroquad::time;
use macroquad::window;

use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Updatable;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

pub struct Ship {
    position: Vec2,
    rotation: f32,
    rotation_angle_per_sec: f32,
    velocity: Vec2,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(window::screen_width() / 2., window::screen_height() / 2.),
            rotation: 0.,
            rotation_angle_per_sec: 45.,
            velocity: Vec2::new(0., 0.),
        }
    }

    pub fn rotate(&mut self) {
        let delta = time::get_frame_time();

        if input::is_key_down(input::KeyCode::Right) {
            self.rotation += self.rotation_angle_per_sec * delta;
        }
        if input::is_key_down(input::KeyCode::Left) {
            self.rotation -= self.rotation_angle_per_sec * delta;
        }
    }
}

impl Updatable for Ship {
    fn update(&mut self) {
        self.rotate();
    }
}

impl Drawable for Ship {
    fn draw(&self) {
        let rotation = self.rotation.to_radians();
        let v1 = Vec2::new(
            self.position.x + rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.position.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.position.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        shapes::draw_triangle_lines(v1, v2, v3, 2., colors::WHITE);
    }
}
