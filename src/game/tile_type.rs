#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TileType {
    FOREST,
    PLAINS,
    HOUSE,
    TRENCH,
}

impl TileType {
    /// Returns the key of the sprite to draw for tiles of this type.
    pub fn sprite_key(&self) -> &str {
        match self {
            TileType::FOREST => "forest",
            TileType::PLAINS => "plains",
            TileType::HOUSE => "house",
            TileType::TRENCH => "trench",
        }
    }

    /// Returns the base capacity of tiles of this type.
    pub fn capacity(&self) -> u8 {
        match self {
            TileType::FOREST => 15,
            TileType::PLAINS => 20,
            TileType::HOUSE => 30,
            TileType::TRENCH => 40,
        }
    }

    /// Returns the name of this type.
    pub fn name(&self) -> &str {
        match self {
            TileType::FOREST => "Forest",
            TileType::PLAINS => "Plains",
            TileType::HOUSE => "House",
            TileType::TRENCH => "Trench",
        }
    }
}
