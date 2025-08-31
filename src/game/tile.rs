use std::hash::Hash;
use std::ops::DerefMut;

use super::sprite_manager;

use super::hexcoordinate;
use macroquad::prelude::*;
use macroquad::ui;

#[derive(Debug, Clone, Hash)]
/// A hexagonal tile in the game world.
pub struct Tile {
    /// The type of this tile.
    tile_type: super::TileType,
    /// The player that currently controls this tile.
    controller: Option<super::Player>,
    /// The number of units currently in this tile.
    units: u8,
}

impl Tile {
    pub fn new(tile_type: super::TileType) -> Self {
        Self {
            tile_type,
            controller: None,
            units: 0,
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
            sprite_manager.get_sprite(&self.tile_type.sprite_key()),
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

    pub fn tile_type(&self) -> super::TileType {
        self.tile_type
    }

    pub fn capacity(&self, context: &super::TileGrid) -> u8 {
        self.tile_type.capacity()
    }

    pub fn set_controller(&mut self, player: super::Player) {
        self.controller = Some(player);
    }

    pub fn build_ui(
        &mut self,
        player: super::Player,
        mut ui: impl DerefMut<Target = macroquad::ui::Ui>,
    ) {
        ui::widgets::Group::new(0, vec2(512.0, 256.0)).ui(ui.deref_mut(), |ui| {
            ui.button(vec2(256.0, 50.0), self.tile_type.name());

            ui.button(
                vec2(256.0, 20.0),
                format!("Capacity: {}", self.tile_type.capacity()),
            );

            if self
                .controller
                .is_some_and(|controller| controller == player)
            {
                if ui.button(vec2(256.0, 40.), "Upgrade") {
                    self.tile_type = crate::game::TileType::HOUSE
                }
            }
        });
    }
}
