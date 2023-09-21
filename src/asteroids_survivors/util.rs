use macroquad::text;
use macroquad::text::TextDimensions;

pub fn text_size(text: &str, font_size: f32) -> TextDimensions {
    text::measure_text(text, None, font_size as u16, 1.0)
}
