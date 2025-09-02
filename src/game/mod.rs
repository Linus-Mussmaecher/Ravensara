mod gamestate;
use std::ops::DerefMut;

pub use gamestate::GameState;
pub use gamestate::TileGrid;

mod hexcoordinate;

mod tile;
pub use tile::Tile;

mod tile_type;
pub use tile_type::TileType;

use macroquad::input;
use macroquad::prelude::*;
use macroquad::ui;

use super::sprite_manager;

use super::scene_manager;

type Player = usize;

#[derive(Debug)]
pub struct Game {
    game_state: GameState,
    sprite_manager: sprite_manager::SpriteManager,
    camera: Camera2D,
    selected: Option<(usize, usize)>,
    player: Player,
}

impl Game {
    /// Creates a new game with n players and random map.
    pub async fn new(_players: u8) -> Self {
        Self {
            game_state: GameState::new(),
            camera: Default::default(),
            sprite_manager: sprite_manager::SpriteManager::new().await,
            selected: None,
            player: 0,
        }
    }
}

const CAMERA_SPEED: f32 = 0.6;
const EDGE_SCROLL_THRESHOLD: f32 = 0.99;
const EDGE_SCROLL: bool = false;
const ZOOM_FACTOR: f32 = 8.;

impl scene_manager::Scene for Game {
    fn update(&mut self) -> scene_manager::SceneSwitch {
        // === UPDATE ===

        // Todo

        // === DRAWING ===

        clear_background(Color::from_rgba(0, 0, 0, 0));

        // Game

        set_camera(&self.camera);

        self.game_state
            .draw(&mut self.sprite_manager, self.selected);

        set_default_camera();

        // UI

        let mut ui_interaction = false;

        if let Some(tile) = self
            .selected
            .and_then(|(x, y)| self.game_state.get_tile_mut(x, y))
        {
            ui_interaction |= tile.build_ui(
                self.player,
                ui::root_ui().deref_mut(),
                &mut self.sprite_manager,
            );
        }

        // === INPUT ===

        // Camera
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

        // Dragging

        if input::is_mouse_button_down(input::MouseButton::Middle) {
            self.camera.target += input::mouse_delta_position() * ZOOM_FACTOR;
        }

        // Selecting
        if input::is_mouse_button_pressed(input::MouseButton::Left) && !ui_interaction {
            let pos = self.camera.screen_to_world(input::mouse_position().into());
            if let Some((x, y)) = hexcoordinate::from_world(
                pos.x,
                pos.y,
                self.game_state.width(),
                self.game_state.height(),
            ) {
                self.selected = Some((x, y));
            }
        }

        // === QUITTING ===

        if input::is_key_down(input::KeyCode::Q) {
            scene_manager::SceneSwitch::pop(1)
        } else {
            scene_manager::SceneSwitch::none()
        }
    }
}
