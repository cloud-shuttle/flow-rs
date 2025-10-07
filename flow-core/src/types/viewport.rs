//! Viewport and coordinate transformation types

use super::geometry::{Position, Rect};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

impl Default for Viewport {
    fn default() -> Self {
        Self::new(0.0, 0.0, 800.0, 600.0, 1.0)
    }
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

    /// Zoom viewport by factor
    pub fn zoom(self, zoom_factor: f64) -> Self {
        Self::with_offset(
            self.x,
            self.y,
            self.width,
            self.height,
            self.zoom * zoom_factor,
            self.offset,
        )
    }

    /// Set zoom level
    pub fn set_zoom(self, zoom: f64) -> Self {
        Self::with_offset(
            self.x,
            self.y,
            self.width,
            self.height,
            zoom,
            self.offset,
        )
    }

    /// Set position
    pub fn set_position(self, x: f64, y: f64) -> Self {
        Self::with_offset(
            x,
            y,
            self.width,
            self.height,
            self.zoom,
            self.offset,
        )
    }

    /// Set size
    pub fn set_size(self, width: f64, height: f64) -> Self {
        Self::with_offset(
            self.x,
            self.y,
            width,
            height,
            self.zoom,
            self.offset,
        )
    }

    /// Zoom to a specific point with zoom factor
    pub fn zoom_to_point(self, point: Position, zoom_factor: f64) -> Self {
        let new_zoom = self.zoom * zoom_factor;
        let zoom_ratio = new_zoom / self.zoom;

        // Calculate new position to keep the point under the mouse at the same screen position
        let new_x = point.x - (point.x - self.x) * zoom_ratio;
        let new_y = point.y - (point.y - self.y) * zoom_ratio;

        Self::with_offset(
            new_x,
            new_y,
            self.width,
            self.height,
            new_zoom,
            self.offset,
        )
    }

    /// Check if viewport is valid (positive dimensions and finite values)
    pub fn is_valid(self) -> bool {
        self.width > 0.0
            && self.height > 0.0
            && self.width.is_finite()
            && self.height.is_finite()
            && self.zoom > 0.0
            && self.zoom.is_finite()
            && self.offset.is_valid()
    }
}
