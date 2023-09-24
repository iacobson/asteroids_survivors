use macroquad::camera;
use macroquad::camera::Camera2D;
use macroquad::color::colors;
use macroquad::input;
use macroquad::math;
use macroquad::math::Vec2;
use macroquad::shapes;
use macroquad::time;
use macroquad::window;

use crate::asteroids_survivors::scene::play::Positionable;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Updatable;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

pub struct Ship {
    hull: i8,
    position: Vec2,
    rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            hull: 100,
            position: Vec2::new(window::screen_width() / 2., window::screen_height() / 2.),
            rotation: 0.,
            rotation_speed: 45.,
            velocity: Vec2::new(0., 0.),
        }
    }
    pub fn is_destroyed(&self) -> bool {
        self.hull <= 0
    }

    fn rotate(&mut self) {
        let delta = time::get_frame_time();

        if input::is_key_down(input::KeyCode::Right) {
            self.rotation += self.rotation_speed * delta;
        }
        if input::is_key_down(input::KeyCode::Left) {
            self.rotation -= self.rotation_speed * delta;
        }
    }

    fn accelerate(&mut self) {
        let delta = time::get_frame_time();
        let rotation = self.rotation.to_radians();

        if input::is_key_down(input::KeyCode::Space) {
            self.velocity += Vec2::new(rotation.sin(), -rotation.cos()) * delta;
        }

        self.position += self.velocity;

        let camera = Camera2D {
            zoom: math::vec2(
                1.0 / window::screen_width() * 2.,
                1.0 / window::screen_height() * 2.,
            ),
            target: self.position,
            ..Default::default()
        };

        camera::set_camera(&camera);
    }
}

impl Positionable for Ship {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

impl Updatable for Ship {
    fn update(&mut self) {
        self.rotate();
        self.accelerate();
    }
}

impl Drawable for Ship {
    fn draw(&self) {
        let mut ship_color = colors::WHITE;

        if input::is_key_down(input::KeyCode::Space) {
            ship_color = colors::SKYBLUE;
        }

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
        shapes::draw_triangle_lines(v1, v2, v3, 2., ship_color);
    }
}
