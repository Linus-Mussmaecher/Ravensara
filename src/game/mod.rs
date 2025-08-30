mod gamestate;
pub use gamestate::GameState;

mod hexcoordinate;
pub use hexcoordinate::HexCoordinate;

mod tile;
use macroquad::input;
pub use tile::Tile;

use crate::scene::Scene;

#[derive(Debug, Clone)]
pub struct Game {
    game_state: GameState,
}

impl Game {
    /// Creates a new game with n players and random map.
    pub fn new(players: u8) -> Self {
        Self {
            game_state: GameState::new(),
        }
    }
}

impl Scene for Game {
    fn update(&mut self) -> crate::scene::SceneSwitch {
        if input::is_key_down(input::KeyCode::Q) {
            crate::scene::SceneSwitch::pop(1)
        } else {
            crate::scene::SceneSwitch::none()
        }
    }

    fn draw(&mut self, mouse_listen: bool) {
        self.game_state.draw();
    }
}
