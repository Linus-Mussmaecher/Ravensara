use super::Tile;

#[derive(Debug)]
pub struct GameState {
    tiles: Vec<Vec<Tile>>,
    selected: Option<(usize, usize)>,
}

impl GameState {
    /// Creates a new game state initialized randomly.
    pub fn new() -> Self {
        let mut tiles = Vec::new();
        for _y in 0..20 {
            let mut row = Vec::new();
            for _x in 0..20 {
                row.push(Tile::new());
            }
            tiles.push(row);
        }
        Self {
            tiles,
            selected: Some((2, 2)),
        }
    }

    pub fn draw(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                tile.draw(
                    x,
                    y,
                    self.selected.map(|sel| sel == (x, y)).unwrap_or(false),
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

    pub fn select(&mut self, hex_x: usize, hex_y: usize) {
        self.selected = Some((hex_x, hex_y));
    }
}
