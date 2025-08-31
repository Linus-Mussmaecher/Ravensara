use macroquad::prelude::*;
mod game;
mod scene_manager;
mod sprite_manager;

#[macroquad::main("ravensara")]
async fn main() {
    let mut scene_manager = scene_manager::SceneManager::new(game::Game::new(4));

    while !scene_manager.update() {
        scene_manager.draw();

        next_frame().await
    }
}
