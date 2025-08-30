use macroquad::prelude::*;

#[derive(Debug, Clone)]
/// A hexagonal tile in the game world.
pub struct Tile {
    /// The position of the tile.
    pos: super::HexCoordinate,
    /// The sprite displayed to represent this tile.
    sprite: Texture2D,
}

impl Tile {
    pub fn new(pos: super::HexCoordinate) -> Self {
        Self {
            pos,
            sprite: Texture2D::from_file_with_format(
                if rand::rand() % 3 == 0 {
                    include_bytes!("../../resources/sprites/house.png")
                } else {
                    include_bytes!("../../resources/sprites/forest.png")
                },
                Some(ImageFormat::Png),
            ),
        }
    }

    pub fn draw(&self) {
        let (x_pix, y_pix) = self.pos.to_pixels();
        self.sprite.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            &self.sprite,
            x_pix,
            y_pix,
            Color::new(1f32, 1f32, 1f32, 1f32),
            DrawTextureParams {
                dest_size: Some(vec2(128.0, 128.0)),
                ..Default::default()
            },
        );
    }
}
