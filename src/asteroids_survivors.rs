use macroquad::color::colors;
use macroquad::text;
use macroquad::text::TextDimensions;
use macroquad::window;

// MODULES
pub mod scene;
pub mod util;

// TRAITS
pub trait Updatable {
    fn update(&mut self);
}
pub trait Drawable {
    fn draw(&self);
}
