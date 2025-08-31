pub const HEX_SIZE: f32 = 1.0;

/// Returns if the two given hex coordinates are adjacent to one another.
#[allow(dead_code)]
pub fn adjacent(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    (y1 % 2 == 1
        && ((x2 == x1 && y2 == y1 + 1)
            || (x2 == x1 - 1 && y2 == y1)
            || (x2 == x1 && y2 == y1 - 1)
            || (x2 == x1 + 1 && y2 == y1 + 1)
            || (x2 == x1 + 1 && y2 == y1 - 1)
            || (x2 == x1 + 1 && y2 == y1)))
        || (y1 % 2 == 0
            && (!(x2 != x1 - 1 || y2 != y1 + 1 && y2 != y1 && y2 != y1 - 1)
                || (x2 == x1 && y2 == y1 + 1)
                || (x2 == x1 && y2 == y1 - 1)
                || (x2 == x1 + 1 && y2 == y1)))
}

/// Returns all hex coordinates adjacent to the passed one.
#[allow(dead_code)]
pub fn get_adjacents(x: usize, y: usize) -> [(usize, usize); 6] {
    if y % 2 == 1 {
        [
            (x, y - 1),
            (x - 1, y),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    } else {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x + 1, y),
            (x, y + 1),
        ]
    }
}

/// Converts this hex coordinate to the top right of its position in the world.
pub fn to_world(x: usize, y: usize) -> (f32, f32) {
    (
        (x as f32 * HEX_SIZE + if y % 2 == 1 { HEX_SIZE * 0.5 } else { 0. }),
        (y as f32 * HEX_SIZE * 0.75),
    )
}

/// Converts a world location to a hex coordinate in the grid.
pub fn from_world(
    x_world: f32,
    y_world: f32,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    if y_world < 0.0 {
        return None;
    }
    let y = (y_world / 0.75).floor() as usize;

    // correct for the fact that every second row is offset.
    let x_cor = if y % 2 == 1 { x_world - 0.5 } else { x_world };

    if x_cor < 0.0 {
        return None;
    }

    let x = x_cor.floor() as usize;

    if x < width && y < height {
        Some((x, y))
    } else {
        None
    }
}
