use super::skins;
use macroquad::{prelude::*, ui};
use std::collections::HashMap;

/// Manages a collection of GPU-stored sprite textures and ui skins.
#[derive(Debug)]
pub struct SpriteManager {
    sprites: HashMap<String, Texture2D>,
    skins: HashMap<String, ui::Skin>,
}

impl SpriteManager {
    /// Creates a new sprite manager with empty storage.
    pub async fn new() -> Self {
        let mut skins = HashMap::new();
        skins.insert("tile-info".to_owned(), skins::tile_info().await);
        for upgrade in ["House", "Plains", "Trench"] {
            skins.insert(
                format!("upgrade-{}", upgrade),
                skins::upgrade_button(upgrade).await,
            );
        }
        Self {
            sprites: HashMap::new(),
            skins,
        }
    }

    /// Loads the sprite with the given key from the file system.
    fn load(&mut self, key: &str) {
        // TODO: Make async and replace with
        // let sprite = load_texture(&format!("resources/sprites/{}.png", key)).await;
        if let Ok(bytes) = std::fs::read(format!("resources/sprites/{}.png", key)) {
            let sprite = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            sprite.set_filter(FilterMode::Nearest);
            self.sprites.insert(key.to_owned(), sprite);
        }
    }

    /// Gets the sprite with the requested key from the storage.
    /// If the sprite is not yet in storage, it will be loaded.
    /// The key needs to be the subpath of the sprite starting from `resources/sprites` without the `.png` ending.
    /// If the sprite file does not exist, this function will panic.
    pub fn get_sprite(&mut self, key: &str) -> &Texture2D {
        if !self.sprites.contains_key(key) {
            self.load(key);
        }

        self.sprites.get(key).unwrap()
    }

    pub fn get_skin(&mut self, key: &str) -> &ui::Skin {
        self.skins.get(key).unwrap()
    }
}
