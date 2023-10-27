use macroquad::audio;
use macroquad::audio::Sound;
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

pub struct Ship {
    hull_points: f32,
    position: Vec2,
    rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
    height: f32,
    base: f32,
    taking_damage: bool,
    assets: Assets,
}

struct Assets {
    sfx: Sfx,
}

struct Sfx {
    engine_running: Sound,
    engine_stops: Sound,
    engine_starts: Sound,
}

impl Assets {
    async fn new() -> Self {
        Self {
            sfx: Sfx {
                engine_starts: audio::load_sound("engine_start.wav").await.unwrap(),
                engine_running: audio::load_sound("engine.wav").await.unwrap(),
                engine_stops: audio::load_sound("engine_stop.wav").await.unwrap(),
            },
        }
    }
}

impl Ship {
    pub async fn new() -> Self {
        Self {
            hull_points: 100.,
            position: Vec2::new(window::screen_width() / 2., window::screen_height() / 2.),
            rotation: 0.,
            rotation_speed: 90.,
            velocity: Vec2::new(0., 0.),
            height: 25.,
            base: 22.,
            taking_damage: false,
            assets: Assets::new().await,
        }
    }

    // GETTERS

    pub fn is_destroyed(&self) -> bool {
        self.hull_points <= 0.
    }

    pub fn get_hull_points_display(&self) -> u8 {
        self.hull_points as u8
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_taking_damage(&self) -> bool {
        self.taking_damage
    }

    // SETTERS

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
            self.velocity += Vec2::new(rotation.sin(), -rotation.cos()) * (delta + 0.01);
            audio::play_sound_once(&self.assets.sfx.engine_running);
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

    pub fn take_damage(&mut self, damage: f32) {
        self.hull_points -= damage;
        self.taking_damage = true;
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
        self.taking_damage = false;
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

        if self.taking_damage {
            ship_color = colors::RED;
        }

        let rotation = self.rotation.to_radians();
        let v1 = Vec2::new(
            self.position.x + rotation.sin() * self.height / 2.,
            self.position.y - rotation.cos() * self.height / 2.,
        );
        let v2 = Vec2::new(
            self.position.x - rotation.cos() * self.base / 2. - rotation.sin() * self.height / 2.,
            self.position.y - rotation.sin() * self.base / 2. + rotation.cos() * self.height / 2.,
        );
        let v3 = Vec2::new(
            self.position.x + rotation.cos() * self.base / 2. - rotation.sin() * self.height / 2.,
            self.position.y + rotation.sin() * self.base / 2. + rotation.cos() * self.height / 2.,
        );
        shapes::draw_triangle_lines(v1, v2, v3, 2., ship_color);
    }
}
