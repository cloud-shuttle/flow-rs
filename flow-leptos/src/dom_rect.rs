//! DOM Rect Access Utilities
//!
//! Provides utilities for accessing DOM element bounding rectangles and coordinate conversion.

use flow_core::{Position, Rect, Size};
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;

/// Error types for DOM rect operations
#[derive(Debug, Clone)]
pub enum DomRectError {
    ElementNotFound,
    InvalidElement,
    DomAccessFailed(String),
    ConversionError(String),
}

impl std::fmt::Display for DomRectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomRectError::ElementNotFound => write!(f, "Element not found"),
            DomRectError::InvalidElement => write!(f, "Invalid element"),
            DomRectError::DomAccessFailed(msg) => write!(f, "DOM access failed: {}", msg),
            DomRectError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
        }
    }
}

/// Represents an element's bounding rectangle
#[derive(Debug, Clone, PartialEq)]
pub struct ElementRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl ElementRect {
    /// Create a new ElementRect
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create from a DOM rect (JsValue)
    pub fn from_dom_rect(dom_rect: &JsValue) -> Self {
        Self {
            x: js_sys::Reflect::get(dom_rect, &"x".into())
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            y: js_sys::Reflect::get(dom_rect, &"y".into())
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            width: js_sys::Reflect::get(dom_rect, &"width".into())
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            height: js_sys::Reflect::get(dom_rect, &"height".into())
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
        }
    }

    /// Convert to our internal Rect type
    pub fn to_rect(&self) -> Rect {
        Rect::from_pos_size(
            Position::new(self.x, self.y),
            Size::new(self.width, self.height),
        )
    }

    /// Check if this rect intersects with another
    pub fn intersects(&self, other: &ElementRect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Check if this rect contains another rect
    pub fn contains(&self, other: &ElementRect) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.x + other.width <= self.x + self.width
            && other.y + other.height <= self.y + self.height
    }

    /// Create union of this rect with another
    pub fn union(&self, other: &ElementRect) -> ElementRect {
        let left = self.x.min(other.x);
        let top = self.y.min(other.y);
        let right = (self.x + self.width).max(other.x + other.width);
        let bottom = (self.y + self.height).max(other.y + other.height);

        ElementRect::new(left, top, right - left, bottom - top)
    }

    /// Get the center point of this rect
    pub fn center(&self) -> Position {
        Position::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Get the area of this rect
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// Coordinate converter for viewport and element-relative coordinates
#[derive(Debug, Clone)]
pub struct CoordinateConverter {
    element_rect: ElementRect,
    scroll_offset: Position,
    zoom_factor: f64,
}

impl CoordinateConverter {
    /// Create a new coordinate converter
    pub fn new(element_rect: ElementRect) -> Self {
        Self {
            element_rect,
            scroll_offset: Position::zero(),
            zoom_factor: 1.0,
        }
    }

    /// Create with scroll offset
    pub fn with_scroll(element_rect: ElementRect, scroll_offset: Position) -> Self {
        Self {
            element_rect,
            scroll_offset,
            zoom_factor: 1.0,
        }
    }

    /// Create with zoom factor
    pub fn with_zoom(element_rect: ElementRect, zoom_factor: f64) -> Self {
        Self {
            element_rect,
            scroll_offset: Position::zero(),
            zoom_factor,
        }
    }

    /// Convert viewport coordinates to element-relative coordinates
    pub fn viewport_to_element(&self, viewport_pos: Position) -> Position {
        let adjusted_x = (viewport_pos.x - self.scroll_offset.x) / self.zoom_factor;
        let adjusted_y = (viewport_pos.y - self.scroll_offset.y) / self.zoom_factor;

        Position::new(
            adjusted_x - self.element_rect.x,
            adjusted_y - self.element_rect.y,
        )
    }

    /// Convert element-relative coordinates to viewport coordinates
    pub fn element_to_viewport(&self, element_pos: Position) -> Position {
        let adjusted_x = element_pos.x + self.element_rect.x;
        let adjusted_y = element_pos.y + self.element_rect.y;

        Position::new(
            adjusted_x * self.zoom_factor + self.scroll_offset.x,
            adjusted_y * self.zoom_factor + self.scroll_offset.y,
        )
    }

    /// Check if a point is within the element bounds
    pub fn is_point_in_bounds(&self, point: Position) -> bool {
        point.x >= 0.0
            && point.y >= 0.0
            && point.x <= self.element_rect.width
            && point.y <= self.element_rect.height
    }

    /// Update the element rect
    pub fn update_element_rect(&mut self, element_rect: ElementRect) {
        self.element_rect = element_rect;
    }

    /// Update the scroll offset
    pub fn update_scroll_offset(&mut self, scroll_offset: Position) {
        self.scroll_offset = scroll_offset;
    }

    /// Update the zoom factor
    pub fn update_zoom_factor(&mut self, zoom_factor: f64) {
        self.zoom_factor = zoom_factor;
    }
}

/// Cache entry for DOM rect queries
#[derive(Debug, Clone)]
struct CacheEntry {
    rect: ElementRect,
    timestamp: f64,
}

/// DOM rect utilities with caching for performance
#[derive(Debug)]
pub struct DomRectUtils {
    cache: HashMap<String, CacheEntry>,
    cache_timeout: f64, // in milliseconds
}

impl DomRectUtils {
    /// Create a new DOM rect utils instance
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            cache_timeout: 16.0, // ~60fps
        }
    }

    /// Create with custom cache timeout
    pub fn with_cache_timeout(cache_timeout: f64) -> Self {
        Self {
            cache: HashMap::new(),
            cache_timeout,
        }
    }

    /// Get bounding client rect for an element
    pub fn get_bounding_client_rect(
        &mut self,
        element: &Element,
    ) -> Result<ElementRect, DomRectError> {
        let element_id = self.get_element_id(element)?;

        // Check cache first
        if let Some(entry) = self.cache.get(&element_id) {
            if self.is_cache_valid(entry.timestamp) {
                return Ok(entry.rect.clone());
            }
        }

        // Query DOM for fresh rect using js_sys
        let method = js_sys::Reflect::get(element, &"getBoundingClientRect".into())
            .ok()
            .and_then(|m| m.dyn_into::<js_sys::Function>().ok())
            .ok_or(DomRectError::DomAccessFailed(
                "Failed to get getBoundingClientRect method".to_string(),
            ))?;

        let dom_rect =
            js_sys::Reflect::apply(&method, element, &js_sys::Array::new()).map_err(|_| {
                DomRectError::DomAccessFailed("Failed to call getBoundingClientRect".to_string())
            })?;

        let element_rect = ElementRect::from_dom_rect(&dom_rect);

        // Cache the result
        let timestamp = self.get_current_timestamp();
        self.cache.insert(
            element_id,
            CacheEntry {
                rect: element_rect.clone(),
                timestamp,
            },
        );

        Ok(element_rect)
    }

    /// Invalidate cache for a specific element
    pub fn invalidate_cache(&mut self, element: &Element) {
        if let Ok(element_id) = self.get_element_id(element) {
            self.cache.remove(&element_id);
        }
    }

    /// Clear all cached entries
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get element ID for caching (fallback to pointer address)
    fn get_element_id(&self, element: &Element) -> Result<String, DomRectError> {
        // Try to get a unique identifier for the element
        // In a real implementation, this might use element.id or a custom attribute
        // For now, we'll use the element's pointer address as a fallback
        let ptr = element as *const Element as usize;
        Ok(format!("element_{}", ptr))
    }

    /// Check if cache entry is still valid
    fn is_cache_valid(&self, timestamp: f64) -> bool {
        let current_time = self.get_current_timestamp();
        current_time - timestamp < self.cache_timeout
    }

    /// Get current timestamp in milliseconds
    fn get_current_timestamp(&self) -> f64 {
        // In a real implementation, this would use performance.now() or similar
        // For now, we'll use a simple counter
        0.0 // Placeholder - will be implemented with proper timing
    }
}

impl Default for DomRectUtils {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for DOM rect operations
pub mod utils {
    use super::*;

    /// Get bounding client rect for an element without caching
    pub fn get_bounding_client_rect(element: &Element) -> Result<ElementRect, DomRectError> {
        let method = js_sys::Reflect::get(element, &"getBoundingClientRect".into())
            .ok()
            .and_then(|m| m.dyn_into::<js_sys::Function>().ok())
            .ok_or(DomRectError::DomAccessFailed(
                "Failed to get getBoundingClientRect method".to_string(),
            ))?;

        let dom_rect =
            js_sys::Reflect::apply(&method, element, &js_sys::Array::new()).map_err(|_| {
                DomRectError::DomAccessFailed("Failed to call getBoundingClientRect".to_string())
            })?;

        Ok(ElementRect::from_dom_rect(&dom_rect))
    }

    /// Check if a point is within an element's bounds
    pub fn is_point_in_element(element: &Element, point: Position) -> Result<bool, DomRectError> {
        let rect = get_bounding_client_rect(element)?;
        Ok(point.x >= rect.x
            && point.y >= rect.y
            && point.x <= rect.x + rect.width
            && point.y <= rect.y + rect.height)
    }

    /// Get the center point of an element
    pub fn get_element_center(element: &Element) -> Result<Position, DomRectError> {
        let rect = get_bounding_client_rect(element)?;
        Ok(rect.center())
    }

    /// Check if two elements intersect
    pub fn elements_intersect(
        element1: &Element,
        element2: &Element,
    ) -> Result<bool, DomRectError> {
        let rect1 = get_bounding_client_rect(element1)?;
        let rect2 = get_bounding_client_rect(element2)?;
        Ok(rect1.intersects(&rect2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_rect_creation() {
        let rect = ElementRect::new(100.0, 200.0, 300.0, 150.0);
        assert_eq!(rect.x, 100.0);
        assert_eq!(rect.y, 200.0);
        assert_eq!(rect.width, 300.0);
        assert_eq!(rect.height, 150.0);
    }

    #[test]
    fn test_element_rect_intersection() {
        let rect1 = ElementRect::new(100.0, 100.0, 200.0, 150.0);
        let rect2 = ElementRect::new(250.0, 200.0, 150.0, 100.0);

        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));

        let rect3 = ElementRect::new(500.0, 500.0, 100.0, 100.0);
        assert!(!rect1.intersects(&rect3));
    }

    #[test]
    fn test_element_rect_contains() {
        let outer = ElementRect::new(100.0, 100.0, 300.0, 200.0);
        let inner = ElementRect::new(150.0, 150.0, 100.0, 50.0);

        assert!(outer.contains(&inner));
        assert!(!inner.contains(&outer));
    }

    #[test]
    fn test_element_rect_union() {
        let rect1 = ElementRect::new(100.0, 100.0, 200.0, 150.0);
        let rect2 = ElementRect::new(250.0, 200.0, 150.0, 100.0);

        let union = rect1.union(&rect2);
        assert_eq!(union.x, 100.0);
        assert_eq!(union.y, 100.0);
        assert_eq!(union.width, 300.0);
        assert_eq!(union.height, 200.0);
    }

    #[test]
    fn test_coordinate_converter_basic() {
        let rect = ElementRect::new(50.0, 100.0, 200.0, 150.0);
        let converter = CoordinateConverter::new(rect);

        let viewport_pos = Position::new(150.0, 175.0);
        let element_pos = converter.viewport_to_element(viewport_pos);

        assert_eq!(element_pos, Position::new(100.0, 75.0));

        let back_to_viewport = converter.element_to_viewport(element_pos);
        assert_eq!(back_to_viewport, viewport_pos);
    }

    #[test]
    fn test_coordinate_converter_with_scroll() {
        let rect = ElementRect::new(100.0, 200.0, 300.0, 150.0);
        let scroll_offset = Position::new(50.0, 75.0);
        let converter = CoordinateConverter::with_scroll(rect, scroll_offset);

        let viewport_pos = Position::new(150.0, 175.0);
        let element_pos = converter.viewport_to_element(viewport_pos);

        assert_eq!(element_pos, Position::new(0.0, -100.0));
    }

    #[test]
    fn test_coordinate_converter_with_zoom() {
        let rect = ElementRect::new(100.0, 200.0, 300.0, 150.0);
        let zoom_factor = 2.0;
        let converter = CoordinateConverter::with_zoom(rect, zoom_factor);

        let viewport_pos = Position::new(200.0, 300.0);
        let element_pos = converter.viewport_to_element(viewport_pos);

        assert_eq!(element_pos, Position::new(0.0, -50.0));
    }

    #[test]
    fn test_dom_rect_utils_creation() {
        let utils = DomRectUtils::new();
        assert_eq!(utils.cache_timeout, 16.0);
        assert!(utils.cache.is_empty());
    }

    #[test]
    fn test_dom_rect_utils_with_timeout() {
        let utils = DomRectUtils::with_cache_timeout(100.0);
        assert_eq!(utils.cache_timeout, 100.0);
    }
}
