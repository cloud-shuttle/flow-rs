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
        Position::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
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
        self.width >= 0.0
            && self.height >= 0.0
            && self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
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
        write!(
            f,
            "[{}, {}, {}x{}]",
            self.x, self.y, self.width, self.height
        )
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
        Self {
            x,
            y,
            width,
            height,
            zoom,
            offset: Position::zero(),
        }
    }

    /// Create viewport with default zoom
    pub const fn with_size(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::new(x, y, width, height, 1.0)
    }

    /// Create viewport with offset
    pub const fn with_offset(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        zoom: f64,
        offset: Position,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            zoom,
            offset,
        }
    }

    /// Get the bounds as a rectangle
    pub fn bounds(self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// Get the center position
    pub fn center(self) -> Position {
        Position::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
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
        self.width > 0.0
            && self.height > 0.0
            && self.zoom > 0.0
            && self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.zoom.is_finite()
    }
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Viewport[{}, {}, {}x{} @ {}x]",
            self.x, self.y, self.width, self.height, self.zoom
        )
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

/// Unique identifier for node groups
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroupId(pub String);

impl GroupId {
    /// Create a new group ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Generate a random group ID
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for GroupId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for GroupId {
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
        assert!(rect1.intersects(&rect2));

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

    // Targeted unit tests for mathematical operations (mutation testing coverage)

    #[test]
    fn test_position_distance_calculation() {
        // Test basic distance calculation
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        assert_eq!(p1.distance_to(p2), 5.0);

        // Test zero distance
        let p3 = Position::new(1.0, 1.0);
        assert_eq!(p1.distance_to(p1), 0.0);
        assert_eq!(p3.distance_to(p3), 0.0);

        // Test negative coordinates
        let p4 = Position::new(-1.0, -1.0);
        let p5 = Position::new(1.0, 1.0);
        assert_eq!(p4.distance_to(p5), 2.0_f64.sqrt() * 2.0);

        // Test large coordinates
        let p6 = Position::new(1000.0, 1000.0);
        let p7 = Position::new(1003.0, 1004.0);
        assert_eq!(p6.distance_to(p7), 5.0);
    }

    #[test]
    fn test_position_distance_squared_calculation() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        assert_eq!(p1.distance_squared_to(p2), 25.0);

        // Test zero distance squared
        assert_eq!(p1.distance_squared_to(p1), 0.0);

        // Test negative coordinates
        let p3 = Position::new(-1.0, -1.0);
        let p4 = Position::new(1.0, 1.0);
        assert_eq!(p3.distance_squared_to(p4), 8.0);
    }

    #[test]
    fn test_position_arithmetic_operations() {
        let p1 = Position::new(1.0, 2.0);
        let p2 = Position::new(3.0, 4.0);

        // Test addition
        assert_eq!(p1.add(p2), Position::new(4.0, 6.0));
        assert_eq!(p1 + p2, Position::new(4.0, 6.0));

        // Test subtraction
        assert_eq!(p2.sub(p1), Position::new(2.0, 2.0));
        assert_eq!(p2 - p1, Position::new(2.0, 2.0));

        // Test scaling
        assert_eq!(p1.scale(2.0), Position::new(2.0, 4.0));
        assert_eq!(p1 * 2.0, Position::new(2.0, 4.0));
        assert_eq!(p1 / 2.0, Position::new(0.5, 1.0));

        // Test zero scaling
        assert_eq!(p1.scale(0.0), Position::new(0.0, 0.0));

        // Test negative scaling
        assert_eq!(p1.scale(-1.0), Position::new(-1.0, -2.0));
    }

    #[test]
    fn test_position_assignment_operations() {
        let mut p1 = Position::new(1.0, 2.0);
        let p2 = Position::new(3.0, 4.0);

        // Test add_assign
        p1 += p2;
        assert_eq!(p1, Position::new(4.0, 6.0));

        // Test sub_assign
        p1 -= p2;
        assert_eq!(p1, Position::new(1.0, 2.0));
    }

    #[test]
    fn test_position_validity() {
        // Test valid positions
        assert!(Position::new(0.0, 0.0).is_valid());
        assert!(Position::new(1.0, 1.0).is_valid());
        assert!(Position::new(-1.0, -1.0).is_valid());
        assert!(Position::new(f64::MAX, f64::MAX).is_valid());
        assert!(Position::new(f64::MIN, f64::MIN).is_valid());

        // Test invalid positions (NaN or infinite)
        assert!(!Position::new(f64::NAN, 0.0).is_valid());
        assert!(!Position::new(0.0, f64::NAN).is_valid());
        assert!(!Position::new(f64::INFINITY, 0.0).is_valid());
        assert!(!Position::new(0.0, f64::NEG_INFINITY).is_valid());
    }

    #[test]
    fn test_rect_intersection_logic() {
        // Test overlapping rectangles
        let rect1 = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(10.0, 10.0));
        let rect2 = Rect::from_pos_size(Position::new(5.0, 5.0), Size::new(10.0, 10.0));
        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));

        // Test non-overlapping rectangles
        let rect3 = Rect::from_pos_size(Position::new(20.0, 20.0), Size::new(10.0, 10.0));
        assert!(!rect1.intersects(&rect3));
        assert!(!rect3.intersects(&rect1));

        // Test touching rectangles (edge case) - they should not intersect if they only touch at edges
        let rect4 = Rect::from_pos_size(Position::new(10.0, 0.0), Size::new(10.0, 10.0));
        assert!(!rect1.intersects(&rect4));

        // Test contained rectangles
        let rect5 = Rect::from_pos_size(Position::new(2.0, 2.0), Size::new(5.0, 5.0));
        assert!(rect1.intersects(&rect5));
        assert!(rect5.intersects(&rect1));

        // Test zero-size rectangles
        let rect6 = Rect::from_pos_size(Position::new(5.0, 5.0), Size::new(0.0, 0.0));
        assert!(rect1.intersects(&rect6));
    }

    #[test]
    fn test_rect_contains_point_logic() {
        let rect = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(10.0, 10.0));

        // Test points inside
        assert!(rect.contains_point(Position::new(5.0, 5.0)));
        assert!(rect.contains_point(Position::new(0.0, 0.0)));
        assert!(rect.contains_point(Position::new(10.0, 10.0)));

        // Test points outside
        assert!(!rect.contains_point(Position::new(-1.0, 5.0)));
        assert!(!rect.contains_point(Position::new(5.0, -1.0)));
        assert!(!rect.contains_point(Position::new(11.0, 5.0)));
        assert!(!rect.contains_point(Position::new(5.0, 11.0)));

        // Test edge cases
        assert!(rect.contains_point(Position::new(0.0, 5.0)));
        assert!(rect.contains_point(Position::new(5.0, 0.0)));
        assert!(rect.contains_point(Position::new(10.0, 5.0)));
        assert!(rect.contains_point(Position::new(5.0, 10.0)));
    }

    #[test]
    fn test_rect_union_operations() {
        let rect1 = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(10.0, 10.0));
        let rect2 = Rect::from_pos_size(Position::new(5.0, 5.0), Size::new(10.0, 10.0));

        let union = rect1.union(rect2);
        assert_eq!(
            union,
            Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(15.0, 15.0))
        );

        // Test with non-overlapping rectangles
        let rect3 = Rect::from_pos_size(Position::new(20.0, 20.0), Size::new(5.0, 5.0));
        let union2 = rect1.union(rect3);
        assert_eq!(
            union2,
            Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(25.0, 25.0))
        );
    }

    #[test]
    fn test_rect_intersection_operations() {
        let rect1 = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(10.0, 10.0));
        let rect2 = Rect::from_pos_size(Position::new(5.0, 5.0), Size::new(10.0, 10.0));

        let intersection = rect1.intersection(rect2);
        assert_eq!(
            intersection,
            Some(Rect::from_pos_size(
                Position::new(5.0, 5.0),
                Size::new(5.0, 5.0)
            ))
        );

        // Test with non-overlapping rectangles
        let rect3 = Rect::from_pos_size(Position::new(20.0, 20.0), Size::new(5.0, 5.0));
        let intersection2 = rect1.intersection(rect3);
        assert_eq!(intersection2, None);
    }

    #[test]
    fn test_size_operations() {
        let size = Size::new(10.0, 20.0);

        // Test area calculation
        assert_eq!(size.area(), 200.0);

        // Test scaling
        let scaled = size.scale(2.0);
        assert_eq!(scaled, Size::new(20.0, 40.0));

        // Test validity
        assert!(size.is_valid());
        assert!(!Size::new(-1.0, 10.0).is_valid());
        assert!(!Size::new(10.0, -1.0).is_valid());
        assert!(Size::new(0.0, 0.0).is_valid()); // Zero size should be valid
    }

    #[test]
    fn test_viewport_coordinate_transformations() {
        let viewport = Viewport::new(10.0, 10.0, 100.0, 100.0, 2.0);

        // Test flow to screen transformation
        let flow_pos = Position::new(60.0, 60.0);
        let screen_pos = viewport.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(100.0, 100.0));

        // Test screen to flow transformation
        let back_to_flow = viewport.screen_to_flow(screen_pos);
        assert_eq!(back_to_flow, flow_pos);

        // Test with different zoom levels
        let viewport2 = Viewport::new(0.0, 0.0, 100.0, 100.0, 1.0);
        let screen_pos2 = viewport2.flow_to_screen(Position::new(50.0, 50.0));
        assert_eq!(screen_pos2, Position::new(50.0, 50.0));

        // Test with zero zoom (edge case)
        let viewport3 = Viewport::new(0.0, 0.0, 100.0, 100.0, 0.0);
        let screen_pos3 = viewport3.flow_to_screen(Position::new(50.0, 50.0));
        assert_eq!(screen_pos3, Position::new(0.0, 0.0));
    }

    #[test]
    fn test_viewport_bounds_operations() {
        let viewport = Viewport::new(10.0, 10.0, 100.0, 100.0, 2.0);

        // Test bounds calculation
        let bounds = viewport.bounds();
        assert_eq!(
            bounds,
            Rect::from_pos_size(Position::new(10.0, 10.0), Size::new(100.0, 100.0))
        );

        // Test center calculation
        let center = viewport.center();
        assert_eq!(center, Position::new(60.0, 60.0));

        // Test contains point
        assert!(viewport.contains_point(Position::new(60.0, 60.0)));
        assert!(!viewport.contains_point(Position::new(5.0, 5.0)));

        // Test intersects rect
        let test_rect = Rect::from_pos_size(Position::new(50.0, 50.0), Size::new(20.0, 20.0));
        assert!(viewport.intersects_rect(test_rect));

        let outside_rect = Rect::from_pos_size(Position::new(200.0, 200.0), Size::new(20.0, 20.0));
        assert!(!viewport.intersects_rect(outside_rect));
    }

    // Edge case tests for boundary conditions (mutation testing coverage)

    #[test]
    fn test_position_edge_cases() {
        // Test with very small values
        let p1 = Position::new(1e-10, 1e-10);
        let p2 = Position::new(0.0, 0.0);
        assert!(p1.distance_to(p2) > 0.0);

        // Test with very large values
        let p3 = Position::new(1e10, 1e10);
        let p4 = Position::new(0.0, 0.0);
        assert!(p3.distance_to(p4) > 0.0);

        // Test with mixed large and small values
        let p5 = Position::new(1e10, 1e-10);
        let p6 = Position::new(0.0, 0.0);
        assert!(p5.distance_to(p6) > 0.0);

        // Test with negative values
        let p7 = Position::new(-1e10, -1e10);
        let p8 = Position::new(0.0, 0.0);
        assert!(p7.distance_to(p8) > 0.0);
    }

    #[test]
    fn test_rect_edge_cases() {
        // Test with very small rectangles
        let tiny_rect = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(1e-10, 1e-10));
        assert!(tiny_rect.contains_point(Position::new(0.0, 0.0)));
        assert!(!tiny_rect.contains_point(Position::new(1e-9, 0.0)));

        // Test with very large rectangles
        let huge_rect = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(1e10, 1e10));
        assert!(huge_rect.contains_point(Position::new(1e9, 1e9)));
        assert!(!huge_rect.contains_point(Position::new(1e11, 1e11)));

        // Test with negative positions
        let neg_rect = Rect::from_pos_size(Position::new(-100.0, -100.0), Size::new(50.0, 50.0));
        assert!(neg_rect.contains_point(Position::new(-75.0, -75.0)));
        assert!(!neg_rect.contains_point(Position::new(-25.0, -25.0)));

        // Test intersection with zero-size rectangles
        let zero_rect = Rect::from_pos_size(Position::new(10.0, 10.0), Size::new(0.0, 0.0));
        let normal_rect = Rect::from_pos_size(Position::new(5.0, 5.0), Size::new(20.0, 20.0));
        assert!(normal_rect.intersects(&zero_rect));
    }

    #[test]
    fn test_size_edge_cases() {
        // Test with very small sizes
        let tiny_size = Size::new(1e-10, 1e-10);
        assert!(tiny_size.is_valid());
        // Use approximate equality for floating point precision
        let expected_area = 1e-20;
        let actual_area = tiny_size.area();
        assert!(
            (actual_area - expected_area).abs() < f64::EPSILON,
            "Expected area {} but got {}",
            expected_area,
            actual_area
        );

        // Test with very large sizes
        let huge_size = Size::new(1e10, 1e10);
        assert!(huge_size.is_valid());
        assert_eq!(huge_size.area(), 1e20);

        // Test scaling with very small factors
        let normal_size = Size::new(100.0, 100.0);
        let scaled_tiny = normal_size.scale(1e-10);
        assert_eq!(scaled_tiny, Size::new(1e-8, 1e-8));

        // Test scaling with very large factors
        let scaled_huge = normal_size.scale(1e10);
        assert_eq!(scaled_huge, Size::new(1e12, 1e12));
    }

    #[test]
    fn test_viewport_edge_cases() {
        // Test with zero zoom
        let viewport_zero = Viewport::new(0.0, 0.0, 100.0, 100.0, 0.0);
        let flow_pos = Position::new(50.0, 50.0);
        let screen_pos = viewport_zero.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(0.0, 0.0));

        // Test with very large zoom
        let viewport_huge = Viewport::new(0.0, 0.0, 100.0, 100.0, 1e10);
        let screen_pos_huge = viewport_huge.flow_to_screen(flow_pos);
        assert!(screen_pos_huge.x > 1e9);
        assert!(screen_pos_huge.y > 1e9);

        // Test with negative offset - fix calculation logic
        let viewport_neg = Viewport::new(-100.0, -100.0, 100.0, 100.0, 1.0);
        let screen_pos_neg = viewport_neg.flow_to_screen(Position::new(0.0, 0.0));
        // flow_to_screen: (flow_pos.x - viewport.x) * zoom = (0.0 - (-100.0)) * 1.0 = 100.0
        assert_eq!(screen_pos_neg, Position::new(100.0, 100.0));

        // Test with very small viewport
        let viewport_tiny = Viewport::new(0.0, 0.0, 1e-10, 1e-10, 1.0);
        let bounds = viewport_tiny.bounds();
        assert_eq!(
            bounds,
            Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(1e-10, 1e-10))
        );
    }

    #[test]
    fn test_arithmetic_edge_cases() {
        // Test addition with zero
        let p1 = Position::new(10.0, 20.0);
        let p_zero = Position::zero();
        assert_eq!(p1.add(p_zero), p1);
        assert_eq!(p_zero.add(p1), p1);

        // Test subtraction with zero
        assert_eq!(p1.sub(p_zero), p1);
        assert_eq!(p_zero.sub(p1), Position::new(-10.0, -20.0));

        // Test scaling with zero
        assert_eq!(p1.scale(0.0), Position::zero());

        // Test scaling with one
        assert_eq!(p1.scale(1.0), p1);

        // Test division by zero (should panic or handle gracefully)
        // Note: This depends on implementation - might need to adjust based on actual behavior

        // Test with very small scaling factors
        let scaled_tiny = p1.scale(1e-10);
        assert_eq!(scaled_tiny, Position::new(1e-9, 2e-9));

        // Test with very large scaling factors
        let scaled_huge = p1.scale(1e10);
        assert_eq!(scaled_huge, Position::new(1e11, 2e11));
    }

    #[test]
    fn test_distance_edge_cases() {
        // Test distance between identical points
        let p1 = Position::new(10.0, 20.0);
        assert_eq!(p1.distance_to(p1), 0.0);
        assert_eq!(p1.distance_squared_to(p1), 0.0);

        // Test distance with very close points
        let p2 = Position::new(10.0 + 1e-10, 20.0 + 1e-10);
        let distance = p1.distance_to(p2);
        assert!(distance > 0.0);
        assert!(distance < 1e-9);

        // Test distance with very far points
        let p3 = Position::new(1e10, 1e10);
        let p4 = Position::new(-1e10, -1e10);
        let distance_far = p3.distance_to(p4);
        assert!(distance_far > 1e10);

        // Test distance squared with very close points
        let distance_sq = p1.distance_squared_to(p2);
        assert!(distance_sq > 0.0);
        assert!(distance_sq < 1e-18);
    }
}
