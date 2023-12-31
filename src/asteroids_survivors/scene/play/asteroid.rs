use macroquad::color::colors;
use macroquad::math::Vec2;
use macroquad::rand;
use macroquad::shapes;
use macroquad::time;
use macroquad::window;

use crate::asteroids_survivors::scene::play::Positionable;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Updatable;

use crate::asteroids_survivors::Frame;

#[derive(Debug)]
pub struct Asteroid {
    position: Vec2,
    rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
    size: f32,
    sides: u8,
}

impl Asteroid {
    pub async fn new(screen_center: &Vec2) -> Self {
        let base_size = window::screen_width().min(window::screen_height());
        Self {
            position: *screen_center
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * window::screen_width().min(window::screen_height())
                    / 2.,
            rotation: 0.,
            rotation_speed: rand::gen_range(-360., 360.),
            velocity: Vec2::new(rand::gen_range(-100., 100.), rand::gen_range(-100., 100.)),
            size: rand::gen_range(base_size / 6., base_size / 20.),
            sides: rand::gen_range(4, 18),
        }
    }
    // GETTERS

    pub fn get_size(&self) -> f32 {
        self.size
    }

    pub fn get_damage_dealt(&self) -> f32 {
        let delta = time::get_frame_time();
        self.size / 2. * delta
    }

    // SETTERS

    fn rotate(&mut self, delta: f32) {
        self.rotation += self.rotation_speed * delta;
    }

    fn accelerate(&mut self, delta: f32) {
        self.position += self.velocity * delta;
    }
}

impl Positionable for Asteroid {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

impl Updatable for Asteroid {
    fn update(&mut self, frame: &Frame) {
        self.rotate(frame.delta);
        self.accelerate(frame.delta);
    }
}

impl Drawable for Asteroid {
    fn draw(&self) {
        shapes::draw_poly_lines(
            self.position.x,
            self.position.y,
            self.sides,
            self.size,
            self.rotation,
            2.,
            colors::WHITE,
        )
    }
}
