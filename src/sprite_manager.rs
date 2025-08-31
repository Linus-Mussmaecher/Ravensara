use macroquad::prelude::*;
use std::collections::HashMap;

/// Manages a collection of GPU-stored sprite textures.
#[derive(Debug)]
pub struct SpriteManager {
    sprites: HashMap<String, Texture2D>,
}

impl SpriteManager {
    /// Creates a new sprite manager with empty storage.
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }

    /// Loads the sprite with the given key from the file system.
    fn load(&mut self, key: &str) {
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
}
