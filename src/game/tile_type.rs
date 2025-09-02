use tinyvec::{ArrayVec, array_vec};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TileType {
    #[default]
    PLAINS,
    FOREST,
    HOUSE,
    TRENCH,
}

impl TileType {
    /// Returns the name of this type.
    pub fn name(&self) -> &str {
        match self {
            TileType::PLAINS => "Plains",
            TileType::FOREST => "Forest",
            TileType::HOUSE => "House",
            TileType::TRENCH => "Trench",
        }
    }

    /// Returns the key of the sprite to draw for tiles of this type.
    pub fn sprite_key(&self) -> &str {
        match self {
            TileType::PLAINS => "plains",
            TileType::FOREST => "forest",
            TileType::HOUSE => "house",
            TileType::TRENCH => "trench",
        }
    }

    /// Returns the base capacity of tiles of this type.
    pub fn capacity(&self) -> u8 {
        match self {
            TileType::PLAINS => 20,
            TileType::FOREST => 15,
            TileType::HOUSE => 30,
            TileType::TRENCH => 40,
        }
    }

    /// Returns the base terrain score of tiles of this type.
    pub fn terrain(&self) -> u8 {
        match self {
            TileType::PLAINS => 20,
            TileType::FOREST => 15,
            TileType::HOUSE => 30,
            TileType::TRENCH => 40,
        }
    }

    /// Returns the base defense score of tiles of this type.
    pub fn defense(&self) -> u8 {
        match self {
            TileType::PLAINS => 0,
            TileType::FOREST => 0,
            TileType::HOUSE => 10,
            TileType::TRENCH => 50,
        }
    }

    /// Returns the base food production of tiles of this type.
    pub fn food(&self) -> u8 {
        match self {
            TileType::PLAINS => 5,
            TileType::FOREST => 0,
            TileType::HOUSE => 0,
            TileType::TRENCH => 0,
        }
    }

    /// Returns the base material production of tiles of this type.
    pub fn material(&self) -> u8 {
        match self {
            TileType::PLAINS => 0,
            TileType::FOREST => 5,
            TileType::HOUSE => 0,
            TileType::TRENCH => 0,
        }
    }

    /// Returns the base funds production of tiles of this type.
    pub fn funds(&self) -> u8 {
        match self {
            TileType::PLAINS => 0,
            TileType::FOREST => 0,
            TileType::HOUSE => 10,
            TileType::TRENCH => 0,
        }
    }

    /// Returns the base unit production of tiles of this type.
    pub fn units(&self) -> u8 {
        match self {
            TileType::PLAINS => 0,
            TileType::FOREST => 0,
            TileType::HOUSE => 1,
            TileType::TRENCH => 0,
        }
    }

    pub fn upgrades(&self) -> ArrayVec<[TileType; 5]> {
        match self {
            TileType::PLAINS => {
                array_vec!([TileType;5] => TileType::HOUSE, TileType::TRENCH)
            }
            TileType::FOREST => array_vec!([TileType;5] => TileType::PLAINS),
            TileType::HOUSE => array_vec!([TileType;5] => TileType::PLAINS),
            TileType::TRENCH => array_vec!([TileType;5] => TileType::PLAINS),
        }
    }
}
