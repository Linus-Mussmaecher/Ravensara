#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TileType {
    FOREST,
    PLAINS,
    HOUSE,
    TRENCH,
}

impl TileType {
    /// Returns the name of this type.
    pub fn name(&self) -> &str {
        match self {
            TileType::FOREST => "Forest",
            TileType::PLAINS => "Plains",
            TileType::HOUSE => "House",
            TileType::TRENCH => "Trench",
        }
    }

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

    /// Returns the base terrain score of tiles of this type.
    pub fn terrain(&self) -> u8 {
        match self {
            TileType::FOREST => 15,
            TileType::PLAINS => 20,
            TileType::HOUSE => 30,
            TileType::TRENCH => 40,
        }
    }

    /// Returns the base defense score of tiles of this type.
    pub fn defense(&self) -> u8 {
        match self {
            TileType::FOREST => 0,
            TileType::PLAINS => 0,
            TileType::HOUSE => 10,
            TileType::TRENCH => 50,
        }
    }

    /// Returns the base food production of tiles of this type.
    pub fn food(&self) -> u8 {
        match self {
            TileType::FOREST => 0,
            TileType::PLAINS => 5,
            TileType::HOUSE => 0,
            TileType::TRENCH => 0,
        }
    }

    /// Returns the base material production of tiles of this type.
    pub fn material(&self) -> u8 {
        match self {
            TileType::FOREST => 5,
            TileType::PLAINS => 0,
            TileType::HOUSE => 0,
            TileType::TRENCH => 0,
        }
    }

    /// Returns the base funds production of tiles of this type.
    pub fn funds(&self) -> u8 {
        match self {
            TileType::FOREST => 0,
            TileType::PLAINS => 0,
            TileType::HOUSE => 10,
            TileType::TRENCH => 0,
        }
    }

    /// Returns the base unit production of tiles of this type.
    pub fn units(&self) -> u8 {
        match self {
            TileType::FOREST => 0,
            TileType::PLAINS => 0,
            TileType::HOUSE => 1,
            TileType::TRENCH => 0,
        }
    }
}
