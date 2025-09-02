use std::hash::Hash;

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

    pub fn capacity(&self, context: &super::TileGrid) -> u8 {
        self.tile_type.capacity()
    }

    pub fn set_controller(&mut self, player: super::Player) {
        self.controller = Some(player);
    }

    /// Builds the information UI for this tile and returns wether the user interacted.
    pub fn build_ui(
        &mut self,
        player: super::Player,
        ui: &mut ui::Ui,
        sprite_manager: &mut sprite_manager::SpriteManager,
    ) -> bool {
        let mut interaction = false;
        ui.push_skin(sprite_manager.get_skin("tile-info"));
        ui.window(
            ui::hash!(&self),
            vec2(screen_width() - 3. * 128., screen_height() - 3. * 80.),
            vec2(3. * 128., 3. * 80.),
            |ui_inner| {
                // Type
                ui::widgets::Label::new(self.tile_type.name())
                    .position(vec2(90., 9.))
                    .size(vec2(290., 30.))
                    .ui(ui_inner);

                // Terrain
                ui::widgets::Label::new(format!("{}", self.tile_type.terrain()))
                    .position(vec2(50., 56.))
                    .size(vec2(42., 30.))
                    .ui(ui_inner);

                // Defense
                ui::widgets::Label::new(format!("{}", self.tile_type.defense()))
                    .position(vec2(146., 56.))
                    .size(vec2(42., 30.))
                    .ui(ui_inner);

                // Units
                ui::widgets::Label::new(format!("{} / {}", self.units, self.tile_type.capacity()))
                    .position(vec2(242., 56.))
                    .size(vec2(120., 30.))
                    .ui(ui_inner);

                // Food
                ui::widgets::Label::new(format!("{}", self.tile_type.food()))
                    .position(vec2(50., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui_inner);

                // Materials
                ui::widgets::Label::new(format!("{}", self.tile_type.material()))
                    .position(vec2(146., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui_inner);

                // Funds
                ui::widgets::Label::new(format!("{}", self.tile_type.funds()))
                    .position(vec2(242., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui_inner);

                // Unit Production
                ui::widgets::Label::new(format!("{}", self.tile_type.units()))
                    .position(vec2(338., 104.))
                    .size(vec2(42., 30.))
                    .ui(ui_inner);

                if self
                    .controller
                    .is_some_and(|controller| controller == player)
                {
                    for (index, upgrade_type) in self.tile_type.upgrades().iter().enumerate() {
                        ui_inner.push_skin(
                            sprite_manager.get_skin(&format!("upgrade-{}", upgrade_type.name())),
                        );

                        if ui::widgets::Button::new("")
                            .position(vec2(3. * 9. + index as f32 * 3. * 23., 3. * 55.))
                            .size(vec2(48., 57.))
                            .ui(ui_inner)
                        {
                            self.tile_type = *upgrade_type;
                            interaction = true;
                        }

                        ui_inner.pop_skin();
                    }
                }
            },
        );
        ui.pop_skin();
        interaction
    }
}
