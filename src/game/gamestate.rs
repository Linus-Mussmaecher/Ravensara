use super::HexCoordinate;
use super::Tile;

#[derive(Debug)]
pub struct GameState {
    tiles: Vec<Tile>,
}

impl GameState {
    /// Creates a new game state initialized randomly.
    pub fn new() -> Self {
        let mut test_vec = Vec::new();
        for y in 0..20 {
            for x in 0..20 {
                test_vec.push(Tile::new(HexCoordinate::new(x, y)));
            }
        }
        Self { tiles: test_vec }
    }

    pub fn draw(&self) {
        for tile in &self.tiles {
            tile.draw();
        }
    }
}
