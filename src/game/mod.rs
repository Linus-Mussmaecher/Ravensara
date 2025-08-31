mod gamestate;
pub use gamestate::GameState;

mod hexcoordinate;

mod tile;
use macroquad::input;
use macroquad::prelude::*;
use macroquad::ui;
pub use tile::Tile;

use super::sprite_manager;

use super::scene_manager;

#[derive(Debug)]
pub struct Game {
    game_state: GameState,
    sprite_manager: sprite_manager::SpriteManager,
    camera: Camera2D,
}

impl Game {
    /// Creates a new game with n players and random map.
    pub fn new(_players: u8) -> Self {
        Self {
            game_state: GameState::new(),
            camera: Default::default(),
            sprite_manager: sprite_manager::SpriteManager::new(),
        }
    }
}

const CAMERA_SPEED: f32 = 0.6;
const EDGE_SCROLL_THRESHOLD: f32 = 0.99;
const EDGE_SCROLL: bool = false;
const ZOOM_FACTOR: f32 = 6.;

impl scene_manager::Scene for Game {
    fn update(&mut self) -> scene_manager::SceneSwitch {
        // === CAMERA ===
        self.camera.zoom = vec2(
            1. / ZOOM_FACTOR,
            screen_width() / screen_height() / ZOOM_FACTOR,
        );
        if input::is_key_down(input::KeyCode::Up)
            || input::mouse_position_local().y < -EDGE_SCROLL_THRESHOLD && EDGE_SCROLL
        {
            self.camera.target.y -= CAMERA_SPEED;
        }
        if input::is_key_down(input::KeyCode::Down)
            || input::mouse_position_local().y > EDGE_SCROLL_THRESHOLD && EDGE_SCROLL
        {
            self.camera.target.y += CAMERA_SPEED;
        }
        if input::is_key_down(input::KeyCode::Left)
            || input::mouse_position_local().x < -EDGE_SCROLL_THRESHOLD && EDGE_SCROLL
        {
            self.camera.target.x -= CAMERA_SPEED;
        }
        if input::is_key_down(input::KeyCode::Right)
            || input::mouse_position_local().x > EDGE_SCROLL_THRESHOLD && EDGE_SCROLL
        {
            self.camera.target.x += CAMERA_SPEED;
        }

        // Draggin

        if input::is_mouse_button_down(input::MouseButton::Middle) {
            self.camera.target += input::mouse_delta_position() * ZOOM_FACTOR;
        }

        // selecting
        if input::is_mouse_button_pressed(input::MouseButton::Left) {
            let pos = self.camera.screen_to_world(input::mouse_position().into());
            if let Some((x, y)) = hexcoordinate::from_world(
                pos.x,
                pos.y,
                self.game_state.width(),
                self.game_state.height(),
            ) {
                self.game_state.select(x, y);
            }
        }

        // === QUITTING ===

        if input::is_key_down(input::KeyCode::Q) {
            scene_manager::SceneSwitch::pop(1)
        } else {
            scene_manager::SceneSwitch::none()
        }
    }

    fn draw(&mut self, _mouse_listen: bool) {
        set_camera(&self.camera);

        self.game_state.draw(&mut self.sprite_manager);

        set_default_camera();

        if ui::root_ui().button(vec2(10.0, 10.0), "Click me!") {
            self.game_state
                .select(rand::rand() as usize % 5, rand::rand() as usize % 5);
        }
    }
}
