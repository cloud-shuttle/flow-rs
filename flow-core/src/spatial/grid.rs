//! Grid-based spatial partitioning

use crate::types::Position;

/// Grid cell coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

impl GridCell {
    /// Create a new grid cell
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Create a grid cell from a position and cell size
    pub fn from_position(pos: Position, cell_size: f64) -> Self {
        Self {
            x: (pos.x / cell_size).floor() as i32,
            y: (pos.y / cell_size).floor() as i32,
        }
    }
}
