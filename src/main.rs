mod game;
mod scene_manager;
mod skins;
mod sprite_manager;

#[macroquad::main("ravensara")]
async fn main() {
    let mut scene_manager = scene_manager::SceneManager::new(game::Game::new(4).await);

    while !scene_manager.update() {
        macroquad::window::next_frame().await
    }
}
