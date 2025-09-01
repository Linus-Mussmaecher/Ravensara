use std::hash::Hash;
use std::ops::DerefMut;

use super::sprite_manager;

use super::hexcoordinate;
use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::hash;

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
        ui.window(
            ui::hash!(&self),
            vec2(screen_width() - 3. * 128., screen_height() - 3. * 80.),
            vec2(3. * 128., 3. * 80.),
            |ui| {
                // Type
                ui::widgets::Label::new(self.tile_type.name())
                    .position(vec2(90., 9.))
                    .size(vec2(290., 30.))
                    .ui(ui);

                // Terrain
                ui::widgets::Label::new(format!("{}", self.tile_type.terrain()))
                    .position(vec2(50., 56.))
                    .size(vec2(42., 30.))
                    .ui(ui);

                // Defense
                ui::widgets::Label::new(format!("{}", self.tile_type.defense()))
                    .position(vec2(146., 56.))
                    .size(vec2(42., 30.))
                    .ui(ui);

                // Units
                ui::widgets::Label::new(format!("{} / {}", self.units, self.tile_type.capacity()))
                    .position(vec2(242., 56.))
                    .size(vec2(120., 30.))
                    .ui(ui);

                // Food
                ui::widgets::Label::new(format!("{}", self.tile_type.food()))
                    .position(vec2(50., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui);

                // Materials
                ui::widgets::Label::new(format!("{}", self.tile_type.material()))
                    .position(vec2(146., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui);

                // Funds
                ui::widgets::Label::new(format!("{}", self.tile_type.funds()))
                    .position(vec2(242., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui);

                // Unit Production
                ui::widgets::Label::new(format!("{}", self.tile_type.units()))
                    .position(vec2(338., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui);

                // if self
                //     .controller
                //     .is_some_and(|controller| controller == player)
                // {
                //     if ui.button(vec2(256.0, 40.), "Upgrade") {
                //         self.tile_type = crate::game::TileType::HOUSE
                //     }
                // }
            },
        );
    }
}
