use super::hexcoordinate;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
/// A hexagonal tile in the game world.
pub struct Tile {
    /// The sprite displayed to represent this tile.
    sprite: Texture2D,
}

impl Tile {
    pub fn new() -> Self {
        Self {
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

    pub fn draw(&self, hex_x: usize, hex_y: usize, selected: bool) {
        let (x_pix, y_pix) = hexcoordinate::to_pixels(hex_x, hex_y);
        self.sprite.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            &self.sprite,
            x_pix,
            y_pix
                - if selected {
                    hexcoordinate::HEX_SIZE / 8.0
                } else {
                    0.0
                },
            Color::new(1f32, 1f32, 1f32, 1f32),
            DrawTextureParams {
                dest_size: Some(vec2(hexcoordinate::HEX_SIZE, hexcoordinate::HEX_SIZE)),
                ..Default::default()
            },
        );
    }
}
