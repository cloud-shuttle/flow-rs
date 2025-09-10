//! Core geometric and utility types

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
    pub fn add(self, other: Position) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract positions
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
        Self { x, y, width, height }
    }

    /// Create rectangle from position and size
    pub fn from_pos_size(position: Position, size: Size) -> Self {
        Self::new(position.x, position.y, size.width, size.height)
    }

    /// Create rectangle from center position and size
    pub fn from_center(center: Position, size: Size) -> Self {
        Self::new(
            center.x - size.width / 2.0,
            center.y - size.height / 2.0,
            size.width,
            size.height,
        )
    }

    /// Create rectangle from two points (bounding box)
    pub fn from_points(p1: Position, p2: Position) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);

        Self::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Get the position (top-left corner)
    pub fn position(self) -> Position {
        Position::new(self.x, self.y)
    }

    /// Get the size
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
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Calculate union with another rectangle
    pub fn union(self, other: Rect) -> Self {
        let min_x = self.x.min(other.x);
        let min_y = self.y.min(other.y);
        let max_x = (self.x + self.width).max(other.x + other.width);
        let max_y = (self.y + self.height).max(other.y + other.height);

        Self::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Calculate intersection with another rectangle
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

    /// Check if rectangle is valid
    pub fn is_valid(self) -> bool {
        self.width >= 0.0 && self.height >= 0.0
            && self.x.is_finite() && self.y.is_finite()
            && self.width.is_finite() && self.height.is_finite()
    }

    /// Expand rectangle by margin
    pub fn expand(self, margin: f64) -> Self {
        Self::new(
            self.x - margin,
            self.y - margin,
            self.width + 2.0 * margin,
            self.height + 2.0 * margin,
        )
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}x{}]", self.x, self.y, self.width, self.height)
    }
}

/// Viewport represents the visible area and zoom level
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub zoom: f64,
    pub offset: Position,
}

impl Viewport {
    /// Create a new viewport
    pub const fn new(x: f64, y: f64, width: f64, height: f64, zoom: f64) -> Self {
        Self { x, y, width, height, zoom, offset: Position::zero() }
    }

    /// Create viewport with default zoom
    pub const fn with_size(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::new(x, y, width, height, 1.0)
    }

    /// Create viewport with offset
    pub const fn with_offset(x: f64, y: f64, width: f64, height: f64, zoom: f64, offset: Position) -> Self {
        Self { x, y, width, height, zoom, offset }
    }

    /// Get the bounds as a rectangle
    pub fn bounds(self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// Get the center position
    pub fn center(self) -> Position {
        Position::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }

    /// Check if viewport contains a point
    pub fn contains_point(self, point: Position) -> bool {
        self.bounds().contains_point(point)
    }

    /// Check if viewport intersects with a rectangle
    pub fn intersects_rect(self, rect: Rect) -> bool {
        self.bounds().intersects(&rect)
    }

    /// Convert screen coordinates to flow coordinates
    pub fn screen_to_flow(self, screen_pos: Position) -> Position {
        Position::new(
            (screen_pos.x / self.zoom) + self.x,
            (screen_pos.y / self.zoom) + self.y,
        )
    }

    /// Convert flow coordinates to screen coordinates
    pub fn flow_to_screen(self, flow_pos: Position) -> Position {
        Position::new(
            (flow_pos.x - self.x) * self.zoom,
            (flow_pos.y - self.y) * self.zoom,
        )
    }

    /// Pan viewport by offset
    pub fn pan(self, pan_offset: Position) -> Self {
        Self::with_offset(
            self.x,
            self.y,
            self.width,
            self.height,
            self.zoom,
            self.offset.add(pan_offset),
        )
    }

    /// Zoom viewport around a point
    pub fn zoom_to_point(self, point: Position, new_zoom: f64) -> Self {
        let zoom_factor = new_zoom / self.zoom;
        let new_x = point.x - (point.x - self.x) * zoom_factor;
        let new_y = point.y - (point.y - self.y) * zoom_factor;

        Self::with_offset(new_x, new_y, self.width, self.height, new_zoom, self.offset)
    }

    /// Check if viewport is valid
    pub fn is_valid(self) -> bool {
        self.width > 0.0 && self.height > 0.0 && self.zoom > 0.0
            && self.x.is_finite() && self.y.is_finite()
            && self.width.is_finite() && self.height.is_finite()
            && self.zoom.is_finite()
    }
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Viewport[{}, {}, {}x{} @ {}x]",
               self.x, self.y, self.width, self.height, self.zoom)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(0.0, 0.0, 800.0, 600.0, 1.0)
    }
}

/// Node ID type for type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NodeId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for NodeId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Edge ID type for type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EdgeId(pub String);

impl EdgeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for EdgeId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for EdgeId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_operations() {
        let p1 = Position::new(1.0, 2.0);
        let p2 = Position::new(3.0, 4.0);

        assert_eq!(p1 + p2, Position::new(4.0, 6.0));
        assert_eq!(p2 - p1, Position::new(2.0, 2.0));
        assert_eq!(p1 * 2.0, Position::new(2.0, 4.0));
        assert_eq!(p1.distance_to(p2), (8.0_f64).sqrt());
    }

    #[test]
    fn test_rect_operations() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);

        assert!(rect1.contains_point(Position::new(5.0, 5.0)));
        assert!(!rect1.contains_point(Position::new(15.0, 15.0)));
        assert!(rect1.intersects(rect2));

        let union = rect1.union(rect2);
        assert_eq!(union, Rect::new(0.0, 0.0, 15.0, 15.0));
    }

    #[test]
    fn test_viewport_transformations() {
        let viewport = Viewport::new(10.0, 10.0, 100.0, 100.0, 2.0);

        let flow_pos = Position::new(60.0, 60.0);
        let screen_pos = viewport.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(100.0, 100.0));

        let back_to_flow = viewport.screen_to_flow(screen_pos);
        assert_eq!(back_to_flow, flow_pos);
    }
}
