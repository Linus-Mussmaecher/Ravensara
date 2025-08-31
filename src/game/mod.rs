mod gamestate;
pub use gamestate::GameState;

mod hexcoordinate;

mod tile;
use macroquad::input;
use macroquad::prelude::*;
use macroquad::ui;
pub use tile::Tile;

use crate::scene::Scene;

#[derive(Debug)]
pub struct Game {
    game_state: GameState,
    camera: Camera2D,
}

impl Game {
    /// Creates a new game with n players and random map.
    pub fn new(players: u8) -> Self {
        Self {
            game_state: GameState::new(),
            camera: Default::default(),
        }
    }
}

const CAMERA_SPEED: f32 = 0.6;
const EDGE_SCROLL_THRESHOLD: f32 = 0.99;
const EDGE_SCROLL: bool = false;
const ZOOM_FACTOR: f32 = 6.;

impl Scene for Game {
    fn update(&mut self) -> crate::scene::SceneSwitch {
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

        // === QUITTING ===

        if input::is_key_down(input::KeyCode::Q) {
            crate::scene::SceneSwitch::pop(1)
        } else {
            crate::scene::SceneSwitch::none()
        }
    }

    fn draw(&mut self, _mouse_listen: bool) {
        set_camera(&self.camera);

        self.game_state.draw();

        set_default_camera();

        if ui::root_ui().button(vec2(10.0, 10.0), "Click me!") {
            self.game_state
                .select(rand::rand() as usize % 5, rand::rand() as usize % 5);
        }
    }
}
