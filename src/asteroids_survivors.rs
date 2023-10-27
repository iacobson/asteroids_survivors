use async_trait::async_trait;

use crate::Scene;

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

#[async_trait]
pub trait Scenic {
    async fn transition(&self) -> Option<Scene>;
}
