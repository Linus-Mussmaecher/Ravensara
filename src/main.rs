use macroquad::prelude::*;
mod game;
mod scene_manager;
mod skins;
mod sprite_manager;

#[macroquad::main("ravensara")]
async fn main() {
    let mut scene_manager = scene_manager::SceneManager::new(game::Game::new(4));

    let skin = skins::create_skin().await;
    macroquad::ui::root_ui().push_skin(&skin);

    while !scene_manager.update() {
        scene_manager.draw();

        next_frame().await
    }
}
