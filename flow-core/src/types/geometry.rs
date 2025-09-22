//! Geometric types and operations

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 2D position coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    /// Create a new position
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a position at the origin
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Calculate distance to another position
    pub fn distance_to(self, other: Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate squared distance (faster than distance_to)
    pub fn distance_squared_to(self, other: Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Add positions together
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Position) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract positions
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Position) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Scale position by a factor
    pub fn scale(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    /// Check if position is valid (not NaN or infinite)
    pub fn is_valid(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(other)
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.sub(other)
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<f64> for Position {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        self.scale(factor)
    }
}

impl std::ops::Div<f64> for Position {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        Self {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }
}

/// 2D size dimensions
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    /// Create a new size
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Create a zero size
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Create a square size
    pub const fn square(size: f64) -> Self {
        Self::new(size, size)
    }

    /// Calculate area
    pub fn area(self) -> f64 {
        self.width * self.height
    }

    /// Check if size is valid (positive dimensions)
    pub fn is_valid(self) -> bool {
        self.width >= 0.0 && self.height >= 0.0 && self.width.is_finite() && self.height.is_finite()
    }

    /// Scale size by a factor
    pub fn scale(self, factor: f64) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(100.0, 50.0) // Default node size
    }
}

/// 2D rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    /// Create a new rectangle
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create rectangle from position and size
    pub fn from_pos_size(position: Position, size: Size) -> Self {
        Self {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Create rectangle from two points
    pub fn from_points(p1: Position, p2: Position) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        
        Self::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Expand rectangle by padding
    pub fn expand(self, padding: f64) -> Self {
        Self::new(
            self.x - padding,
            self.y - padding,
            self.width + 2.0 * padding,
            self.height + 2.0 * padding,
        )
    }

    /// Get the position of the rectangle
    pub fn position(self) -> Position {
        Position::new(self.x, self.y)
    }

    /// Get the size of the rectangle
    pub fn size(self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Get the center position
    pub fn center(self) -> Position {
        Position::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }

    /// Check if rectangle contains a point
    pub fn contains_point(self, point: Position) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    /// Check if rectangle intersects with another rectangle
    pub fn intersects(self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Get the union of two rectangles
    pub fn union(self, other: Rect) -> Self {
        let min_x = self.x.min(other.x);
        let min_y = self.y.min(other.y);
        let max_x = (self.x + self.width).max(other.x + other.width);
        let max_y = (self.y + self.height).max(other.y + other.height);

        Self::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Get the intersection of two rectangles
    pub fn intersection(self, other: Rect) -> Option<Self> {
        let min_x = self.x.max(other.x);
        let min_y = self.y.max(other.y);
        let max_x = (self.x + self.width).min(other.x + other.width);
        let max_y = (self.y + self.height).min(other.y + other.height);

        if min_x < max_x && min_y < max_y {
            Some(Self::new(min_x, min_y, max_x - min_x, max_y - min_y))
        } else {
            None
        }
    }

    /// Check if rectangle is valid (positive dimensions)
    pub fn is_valid(self) -> bool {
        self.width >= 0.0 && self.height >= 0.0 && self.width.is_finite() && self.height.is_finite()
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}x{}]",
            self.x, self.y, self.width, self.height
        )
    }
}
