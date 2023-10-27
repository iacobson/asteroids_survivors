use macroquad::camera;
use macroquad::color::colors;
use macroquad::input;
use macroquad::text;
use macroquad::window;

use crate::asteroids_survivors::scene::home::Home;
use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::util;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

pub struct GameOver {
    home: bool,
    title: String,
    title_font_size: f32,
}

impl GameOver {
    pub fn new() -> Self {
        window::clear_background(colors::BLACK);
        camera::set_default_camera();
        Self {
            home: false,
            title: String::from("Game Over"),
            title_font_size: 36.0,
        }
    }
}

impl Updatable for GameOver {
    fn update(&mut self) {
        if input::is_key_pressed(input::KeyCode::Enter) {
            self.home = true;
        }
    }
}

impl Scenic for GameOver {
    fn transition(&self) -> Option<Scene> {
        if self.home {
            Some(Scene::Home(Home::new()))
        } else {
            None
        }
    }
}

impl Drawable for GameOver {
    fn draw(&self) {
        let title_text_size = util::text_size(&self.title, self.title_font_size);
        text::draw_text(
            &self.title,
            window::screen_width() / 2. - title_text_size.width / 2.,
            window::screen_height() / 2. - title_text_size.height / 2.,
            self.title_font_size,
            colors::WHITE,
        );
    }
}
