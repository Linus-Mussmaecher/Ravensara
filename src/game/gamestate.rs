use crate::sprite_manager;

use super::Tile;

pub type TileGrid = Vec<Vec<Tile>>;

#[derive(Debug)]
pub struct GameState {
    tiles: TileGrid,
}

impl GameState {
    /// Creates a new game state initialized randomly.
    pub fn new() -> Self {
        let mut tiles = Vec::new();
        for _y in 0..20 {
            let mut row = Vec::new();
            for _x in 0..20 {
                row.push(Tile::new(super::TileType::FOREST));
            }
            tiles.push(row);
        }
        tiles[0][0].set_controller(0);
        Self { tiles }
    }

    pub fn draw(
        &self,
        sprite_manager: &mut sprite_manager::SpriteManager,
        selected: Option<(usize, usize)>,
    ) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                tile.draw(
                    sprite_manager,
                    x,
                    y,
                    selected.map(|sel| sel == (x, y)).unwrap_or(false),
                );
            }
        }
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn width(&self) -> usize {
        self.tiles
            .first()
            .map(|first_row| first_row.len())
            .unwrap_or(0)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }
}
