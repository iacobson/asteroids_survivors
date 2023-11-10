use async_trait::async_trait;

use crate::Scene;

// MODULES
pub mod scene;
pub mod util;

pub struct Frame {
    pub delta: f32,
    pub screen_width: f32,
    pub screen_height: f32,
}

// TRAITS
pub trait Updatable {
    fn update(&mut self, frame: &Frame);
}
pub trait Drawable {
    fn draw(&self);
}

#[async_trait]
pub trait Scenic {
    async fn transition(&self) -> Option<Scene>;
}
