use crate::sprite_manager;

use super::hexcoordinate;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
/// A hexagonal tile in the game world.
pub struct Tile {
    /// The sprite displayed to represent this tile.
    sprite_key: String,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            sprite_key: if rand::rand() % 3 == 0 {
                "house"
            } else {
                "forest"
            }
            .to_owned(),
        }
    }

    pub fn draw(
        &self,
        sprite_manager: &mut sprite_manager::SpriteManager,
        hex_x: usize,
        hex_y: usize,
        selected: bool,
    ) {
        let (x_pix, y_pix) = hexcoordinate::to_world(hex_x, hex_y);
        draw_texture_ex(
            sprite_manager.get_sprite(&self.sprite_key),
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
