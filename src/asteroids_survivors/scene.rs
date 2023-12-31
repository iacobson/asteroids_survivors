use async_trait::async_trait;

use crate::asteroids_survivors::Drawable;
use crate::asteroids_survivors::Scenic;
use crate::asteroids_survivors::Updatable;

use crate::asteroids_survivors::Frame;

pub mod game_over;
pub mod home;
pub mod play;
pub mod you_win;

pub enum Scene {
    Home(home::Home),
    Play(play::Play),
    YouWin(you_win::YouWin),
    GameOver(game_over::GameOver),
}

impl Updatable for Scene {
    fn update(&mut self, frame: &Frame) {
        match self {
            Scene::Home(home) => home.update(frame),
            Scene::Play(play) => play.update(frame),
            Scene::YouWin(you_win) => you_win.update(frame),
            Scene::GameOver(game_over) => game_over.update(frame),
        }
    }
}

#[async_trait]
impl Scenic for Scene {
    async fn transition(&self) -> Option<Scene> {
        match self {
            Scene::Home(home) => home.transition().await,
            Scene::Play(play) => play.transition().await,
            Scene::YouWin(you_win) => you_win.transition().await,
            Scene::GameOver(game_over) => game_over.transition().await,
        }
    }
}

impl Drawable for Scene {
    fn draw(&self) {
        match self {
            Scene::Home(home) => home.draw(),
            Scene::Play(play) => play.draw(),
            Scene::YouWin(you_win) => you_win.draw(),
            Scene::GameOver(game_over) => game_over.draw(),
        }
    }
}
