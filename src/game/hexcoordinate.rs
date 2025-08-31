#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A struct representing a position in a grid of hexagonal objects.
pub struct HexCoordinate {
    pub x: i32,
    pub y: i32,
}

pub const HEX_SIZE: f32 = 1.0;

impl HexCoordinate {
    /// Creates a new coordinate.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns if the given hex coordinate is adjacent to this one.
    pub fn is_adjacent(&self, other: Self) -> bool {
        (self.y % 2 == 1
            && ((other.x == self.x && other.y == self.y + 1)
                || (other.x == self.x - 1 && other.y == self.y)
                || (other.x == self.x && other.y == self.y - 1)
                || (other.x == self.x + 1 && other.y == self.y + 1)
                || (other.x == self.x + 1 && other.y == self.y - 1)
                || (other.x == self.x + 1 && other.y == self.y)))
            || (self.y % 2 == 0
                && ((other.x == self.x - 1 && other.y == self.y + 1)
                    || (other.x == self.x - 1 && other.y == self.y)
                    || (other.x == self.x - 1 && other.y == self.y - 1)
                    || (other.x == self.x && other.y == self.y + 1)
                    || (other.x == self.x && other.y == self.y - 1)
                    || (other.x == self.x + 1 && other.y == self.y)))
    }

    /// Returns all hex coordinates adjacent to this one.
    pub fn adjacents(&self) -> [HexCoordinate; 6] {
        if self.y % 2 == 1 {
            [
                Self::new(self.x, self.y - 1),
                Self::new(self.x - 1, self.y),
                Self::new(self.x, self.y + 1),
                Self::new(self.x + 1, self.y - 1),
                Self::new(self.x + 1, self.y),
                Self::new(self.x + 1, self.y + 1),
            ]
        } else {
            [
                Self::new(self.x - 1, self.y - 1),
                Self::new(self.x - 1, self.y),
                Self::new(self.x - 1, self.y + 1),
                Self::new(self.x, self.y - 1),
                Self::new(self.x + 1, self.y),
                Self::new(self.x, self.y + 1),
            ]
        }
    }

    /// Converts this hex coordinate to pixels.
    pub fn to_pixels(&self) -> (f32, f32) {
        (
            (self.x as f32 * HEX_SIZE + if self.y % 2 == 1 { HEX_SIZE * 0.5 } else { 0. }),
            (self.y as f32 * HEX_SIZE * 0.75),
        )
    }
}
