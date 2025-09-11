//! DOM Rect Access Tests
//!
//! Tests for DOM element bounding rectangle access and coordinate conversion utilities.

use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};
use wasm_bindgen::{JsValue, JsCast};
use leptos_flow_core::{Position, Size, Rect};

// Import the DOM rect utilities (these will be implemented)
use crate::dom_rect::{DomRectUtils, ElementRect, CoordinateConverter};

/// Helper function to create a mock DOM element for testing
fn create_mock_element() -> web_sys::Element {
    // Create a mock element using js_sys
    let element = js_sys::Object::new();
    js_sys::Reflect::set(&element, &"getBoundingClientRect".into(), &js_sys::Function::new_no_args("return {x: 100, y: 200, width: 300, height: 150}")).unwrap();
    element.unchecked_into()
}

/// Helper function to create a mock DOM rect for testing
fn create_mock_dom_rect(x: f64, y: f64, width: f64, height: f64) -> JsValue {
    let rect = js_sys::Object::new();
    js_sys::Reflect::set(&rect, &"x".into(), &x.into()).unwrap();
    js_sys::Reflect::set(&rect, &"y".into(), &y.into()).unwrap();
    js_sys::Reflect::set(&rect, &"width".into(), &width.into()).unwrap();
    js_sys::Reflect::set(&rect, &"height".into(), &height.into()).unwrap();
    rect.into()
}

#[wasm_bindgen_test]
fn test_get_bounding_client_rect_basic() {
    // Test: Basic getBoundingClientRect functionality
    let element = create_mock_element();
    let mut rect_utils = DomRectUtils::new();

    let result = rect_utils.get_bounding_client_rect(&element);

    assert!(result.is_ok());
    let element_rect = result.unwrap();

    // Verify the rect has valid dimensions
    assert!(element_rect.width > 0.0);
    assert!(element_rect.height > 0.0);
    assert!(element_rect.x >= 0.0);
    assert!(element_rect.y >= 0.0);
}

#[wasm_bindgen_test]
fn test_element_rect_conversion() {
    // Test: Converting DOM rect to our internal Rect type
    let mock_dom_rect = create_mock_dom_rect(100.0, 200.0, 300.0, 150.0);
    let element_rect = ElementRect::from_dom_rect(&mock_dom_rect);

    assert_eq!(element_rect.x, 100.0);
    assert_eq!(element_rect.y, 200.0);
    assert_eq!(element_rect.width, 300.0);
    assert_eq!(element_rect.height, 150.0);

    // Test conversion to our Rect type
    let rect = element_rect.to_rect();
    assert_eq!(rect.position(), Position::new(100.0, 200.0));
    assert_eq!(rect.size(), Size::new(300.0, 150.0));
}

#[wasm_bindgen_test]
fn test_coordinate_conversion_viewport_to_element() {
    // Test: Converting viewport coordinates to element-relative coordinates
    let element_rect = ElementRect {
        x: 50.0,
        y: 100.0,
        width: 200.0,
        height: 150.0,
    };

    let converter = CoordinateConverter::new(element_rect);

    // Test point inside element
    let viewport_pos = Position::new(150.0, 175.0);
    let element_pos = converter.viewport_to_element(viewport_pos);

    assert_eq!(element_pos, Position::new(100.0, 75.0));

    // Test point outside element (should still convert correctly)
    let viewport_pos_outside = Position::new(10.0, 50.0);
    let element_pos_outside = converter.viewport_to_element(viewport_pos_outside);

    assert_eq!(element_pos_outside, Position::new(-40.0, -50.0));
}

#[wasm_bindgen_test]
fn test_coordinate_conversion_element_to_viewport() {
    // Test: Converting element-relative coordinates to viewport coordinates
    let element_rect = ElementRect {
        x: 50.0,
        y: 100.0,
        width: 200.0,
        height: 150.0,
    };

    let converter = CoordinateConverter::new(element_rect);

    // Test point inside element
    let element_pos = Position::new(100.0, 75.0);
    let viewport_pos = converter.element_to_viewport(element_pos);

    assert_eq!(viewport_pos, Position::new(150.0, 175.0));

    // Test point at element origin
    let element_origin = Position::new(0.0, 0.0);
    let viewport_origin = converter.element_to_viewport(element_origin);

    assert_eq!(viewport_origin, Position::new(50.0, 100.0));
}

#[wasm_bindgen_test]
fn test_point_in_element_bounds() {
    // Test: Checking if a point is within element bounds
    let element_rect = ElementRect {
        x: 100.0,
        y: 200.0,
        width: 300.0,
        height: 150.0,
    };

    let converter = CoordinateConverter::new(element_rect);

    // Test points inside bounds
    assert!(converter.is_point_in_bounds(Position::new(150.0, 250.0)));
    assert!(converter.is_point_in_bounds(Position::new(100.0, 200.0))); // Top-left corner
    assert!(converter.is_point_in_bounds(Position::new(400.0, 350.0))); // Bottom-right corner

    // Test points outside bounds
    assert!(!converter.is_point_in_bounds(Position::new(50.0, 150.0)));
    assert!(!converter.is_point_in_bounds(Position::new(450.0, 400.0)));
    assert!(!converter.is_point_in_bounds(Position::new(200.0, 100.0)));
}

#[wasm_bindgen_test]
fn test_element_rect_intersection() {
    // Test: Checking if two element rects intersect
    let rect1 = ElementRect {
        x: 100.0,
        y: 100.0,
        width: 200.0,
        height: 150.0,
    };

    let rect2 = ElementRect {
        x: 250.0,
        y: 200.0,
        width: 150.0,
        height: 100.0,
    };

    // Test intersection
    assert!(rect1.intersects(&rect2));
    assert!(rect2.intersects(&rect1));

    // Test non-intersecting rects
    let rect3 = ElementRect {
        x: 500.0,
        y: 500.0,
        width: 100.0,
        height: 100.0,
    };

    assert!(!rect1.intersects(&rect3));
    assert!(!rect3.intersects(&rect1));
}

#[wasm_bindgen_test]
fn test_element_rect_union() {
    // Test: Creating union of two element rects
    let rect1 = ElementRect {
        x: 100.0,
        y: 100.0,
        width: 200.0,
        height: 150.0,
    };

    let rect2 = ElementRect {
        x: 250.0,
        y: 200.0,
        width: 150.0,
        height: 100.0,
    };

    let union = rect1.union(&rect2);

    // Union should encompass both rects
    assert_eq!(union.x, 100.0); // Leftmost x
    assert_eq!(union.y, 100.0); // Topmost y
    assert_eq!(union.width, 300.0); // From 100 to 400
    assert_eq!(union.height, 200.0); // From 100 to 300
}

#[wasm_bindgen_test]
fn test_element_rect_contains() {
    // Test: Checking if one rect contains another
    let outer_rect = ElementRect {
        x: 100.0,
        y: 100.0,
        width: 300.0,
        height: 200.0,
    };

    let inner_rect = ElementRect {
        x: 150.0,
        y: 150.0,
        width: 100.0,
        height: 50.0,
    };

    assert!(outer_rect.contains(&inner_rect));
    assert!(!inner_rect.contains(&outer_rect));

    // Test partial overlap (should not contain)
    let partial_rect = ElementRect {
        x: 350.0,
        y: 250.0,
        width: 100.0,
        height: 50.0,
    };

    assert!(!outer_rect.contains(&partial_rect));
}

#[wasm_bindgen_test]
fn test_dom_rect_utils_caching() {
    // Test: Caching of DOM rect queries for performance
    let element = create_mock_element();
    let mut rect_utils = DomRectUtils::new();

    // First call should query DOM
    let rect1 = rect_utils.get_bounding_client_rect(&element).unwrap();

    // Second call should use cache (if within cache timeout)
    let rect2 = rect_utils.get_bounding_client_rect(&element).unwrap();

    // Results should be identical
    assert_eq!(rect1.x, rect2.x);
    assert_eq!(rect1.y, rect2.y);
    assert_eq!(rect1.width, rect2.width);
    assert_eq!(rect1.height, rect2.height);
}

#[wasm_bindgen_test]
fn test_dom_rect_utils_cache_invalidation() {
    // Test: Cache invalidation when element changes
    let element = create_mock_element();
    let mut rect_utils = DomRectUtils::new();

    // Get initial rect
    let initial_rect = rect_utils.get_bounding_client_rect(&element).unwrap();

    // Invalidate cache
    rect_utils.invalidate_cache(&element);

    // Next call should query DOM again
    let new_rect = rect_utils.get_bounding_client_rect(&element).unwrap();

    // Results should be fresh (even if identical)
    assert_eq!(initial_rect.x, new_rect.x);
    assert_eq!(initial_rect.y, new_rect.y);
}

#[wasm_bindgen_test]
fn test_coordinate_conversion_with_scroll() {
    // Test: Coordinate conversion accounting for scroll position
    let element_rect = ElementRect {
        x: 100.0,
        y: 200.0,
        width: 300.0,
        height: 150.0,
    };

    let scroll_offset = Position::new(50.0, 75.0);
    let converter = CoordinateConverter::with_scroll(element_rect, scroll_offset);

    // Test conversion with scroll offset
    let viewport_pos = Position::new(150.0, 175.0);
    let element_pos = converter.viewport_to_element(viewport_pos);

    // Should account for scroll offset
    assert_eq!(element_pos, Position::new(0.0, -100.0));
}

#[wasm_bindgen_test]
fn test_coordinate_conversion_with_zoom() {
    // Test: Coordinate conversion accounting for zoom level
    let element_rect = ElementRect {
        x: 100.0,
        y: 200.0,
        width: 300.0,
        height: 150.0,
    };

    let zoom_factor = 1.5;
    let converter = CoordinateConverter::with_zoom(element_rect, zoom_factor);

    // Test conversion with zoom
    let viewport_pos = Position::new(150.0, 175.0);
    let element_pos = converter.viewport_to_element(viewport_pos);

    // Should account for zoom factor
    assert_eq!(element_pos, Position::new(33.33, -16.67)); // Approximate due to zoom
}

#[wasm_bindgen_test]
fn test_element_rect_edge_cases() {
    // Test: Edge cases for element rect operations
    let zero_rect = ElementRect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    let normal_rect = ElementRect {
        x: 100.0,
        y: 100.0,
        width: 200.0,
        height: 150.0,
    };

    // Zero-size rect should not intersect with normal rect
    assert!(!zero_rect.intersects(&normal_rect));
    assert!(!normal_rect.intersects(&zero_rect));

    // Zero-size rect should not contain anything
    assert!(!zero_rect.contains(&normal_rect));

    // Union with zero rect should return the normal rect
    let union = zero_rect.union(&normal_rect);
    assert_eq!(union.x, 0.0);
    assert_eq!(union.y, 0.0);
    assert_eq!(union.width, 300.0);
    assert_eq!(union.height, 250.0);
}

#[wasm_bindgen_test]
fn test_dom_rect_utils_error_handling() {
    // Test: Error handling for invalid elements
    let mut rect_utils = DomRectUtils::new();

    // Test with null/invalid element (this should be handled gracefully)
    // Note: In a real implementation, this would test with a detached element
    // For now, we'll test the error handling structure

    // This test will be implemented when we have proper error handling
    assert!(true); // Placeholder
}

#[wasm_bindgen_test]
fn test_coordinate_conversion_precision() {
    // Test: Precision of coordinate conversions
    let element_rect = ElementRect {
        x: 100.5,
        y: 200.25,
        width: 300.75,
        height: 150.125,
    };

    let converter = CoordinateConverter::new(element_rect);

    // Test with fractional coordinates
    let viewport_pos = Position::new(200.75, 275.5);
    let element_pos = converter.viewport_to_element(viewport_pos);

    // Should maintain precision
    assert!((element_pos.x - 100.25).abs() < 0.01);
    assert!((element_pos.y - 75.25).abs() < 0.01);
}

#[wasm_bindgen_test]
fn test_element_rect_center_calculation() {
    // Test: Calculating center point of element rect
    let rect = ElementRect {
        x: 100.0,
        y: 200.0,
        width: 300.0,
        height: 150.0,
    };

    let center = rect.center();

    assert_eq!(center, Position::new(250.0, 275.0));
}

#[wasm_bindgen_test]
fn test_element_rect_area_calculation() {
    // Test: Calculating area of element rect
    let rect = ElementRect {
        x: 100.0,
        y: 200.0,
        width: 300.0,
        height: 150.0,
    };

    let area = rect.area();

    assert_eq!(area, 45000.0);
}
