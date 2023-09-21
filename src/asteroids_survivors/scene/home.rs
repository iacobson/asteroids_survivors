use macroquad::color::colors;
use macroquad::input;
use macroquad::text;
use macroquad::window;

use crate::asteroids_survivors::scene::play::Play;
use crate::asteroids_survivors::scene::Scene;
use crate::asteroids_survivors::util;
use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

pub struct Home {
    start_game: bool,
    title: String,
    title_font_size: f32,
}
impl Home {
    pub fn new() -> Self {
        window::clear_background(colors::BLACK);
        Self {
            start_game: false,
            title: String::from("Asteroids Survivors"),
            title_font_size: 36.0,
        }
    }
}

impl Updatable for Home {
    fn update(&mut self) {
        if input::is_key_pressed(input::KeyCode::Enter) {
            self.start_game = true;
        }
    }
}

impl Scenic for Home {
    fn transition(&self) -> Option<Scene> {
        if self.start_game {
            Some(Scene::Play(Play::new()))
        } else {
            None
        }
    }
}

impl Drawable for Home {
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
