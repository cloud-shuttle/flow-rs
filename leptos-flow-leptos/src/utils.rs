//! Utility functions for DOM manipulation and coordinate conversion
//!
//! This module provides helper functions for working with DOM elements,
//! particularly for canvas-based interactions in flow editors.

use web_sys::{HtmlCanvasElement, DomRect};
use wasm_bindgen::JsValue;

/// Canvas rectangle bounds (left, top, width, height)
#[derive(Debug, Clone, PartialEq)]
pub struct CanvasRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl CanvasRect {
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> Self {
        Self { left, top, width, height }
    }

    /// Convert from DomRect to CanvasRect
    pub fn from_dom_rect(dom_rect: &DomRect) -> Self {
        Self {
            left: dom_rect.left(),
            top: dom_rect.top(),
            width: dom_rect.width(),
            height: dom_rect.height(),
        }
    }
}

/// Get the bounding rectangle of a canvas element
///
/// This function properly accesses the DOM to get the actual canvas bounds,
/// which is essential for accurate mouse coordinate transformation.
pub fn get_canvas_rect(canvas: &HtmlCanvasElement) -> Result<CanvasRect, JsValue> {
    // TODO: Implement this function
    Err(JsValue::from_str("get_canvas_rect not implemented yet"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::{window, Document, HtmlCanvasElement};

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_test_canvas() -> Result<HtmlCanvasElement, JsValue> {
        let window = window().ok_or_else(|| JsValue::from_str("no window found"))?;
        let document = window.document().ok_or_else(|| JsValue::from_str("no document found"))?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("failed to create canvas"))?;

        canvas.set_width(800);
        canvas.set_height(600);

        // Add to document body to get real DOM rect
        document.body()
            .ok_or_else(|| JsValue::from_str("no body found"))?
            .append_child(&canvas)?;

        Ok(canvas)
    }

    #[wasm_bindgen_test]
    fn test_canvas_rect_creation() {
        let rect = CanvasRect::new(10.0, 20.0, 300.0, 200.0);
        assert_eq!(rect.left, 10.0);
        assert_eq!(rect.top, 20.0);
        assert_eq!(rect.width, 300.0);
        assert_eq!(rect.height, 200.0);
    }

    #[wasm_bindgen_test]
    fn test_get_canvas_rect_fails_initially() {
        // RED phase: This test should fail initially
        let canvas = create_test_canvas().expect("Failed to create test canvas");

        let result = get_canvas_rect(&canvas);

        // This should fail because get_canvas_rect is not implemented yet
        assert!(result.is_err(), "get_canvas_rect should fail when not implemented");

        // Clean up
        let _ = canvas.remove();
    }

    #[wasm_bindgen_test]
    fn test_get_canvas_rect_returns_correct_bounds() {
        // This test will fail initially but defines our expected behavior
        let canvas = create_test_canvas().expect("Failed to create test canvas");

        // Position the canvas with CSS for testing
        canvas.style().set_property("position", "absolute").unwrap();
        canvas.style().set_property("left", "50px").unwrap();
        canvas.style().set_property("top", "100px").unwrap();

        let result = get_canvas_rect(&canvas);

        // When implemented, this should return the actual DOM bounds
        match result {
            Ok(rect) => {
                // These values should match the actual DOM positioning
                assert!(rect.left >= 0.0, "Left should be non-negative");
                assert!(rect.top >= 0.0, "Top should be non-negative");
                assert_eq!(rect.width, 800.0, "Width should match canvas width");
                assert_eq!(rect.height, 600.0, "Height should match canvas height");
            }
            Err(_) => {
                // Expected to fail initially in RED phase
                panic!("get_canvas_rect not implemented yet - this is expected in RED phase");
            }
        }

        // Clean up
        let _ = canvas.remove();
    }

    #[wasm_bindgen_test]
    fn test_canvas_rect_from_dom_rect() {
        // Test the conversion helper function
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");
        let canvas = document.create_element("canvas").expect("should create canvas")
            .dyn_into::<HtmlCanvasElement>().expect("should be canvas");

        canvas.set_width(400);
        canvas.set_height(300);

        document.body().expect("should have body").append_child(&canvas).expect("should append");

        let dom_rect = canvas.get_bounding_client_rect();
        let canvas_rect = CanvasRect::from_dom_rect(&dom_rect);

        assert_eq!(canvas_rect.width, dom_rect.width());
        assert_eq!(canvas_rect.height, dom_rect.height());
        assert_eq!(canvas_rect.left, dom_rect.left());
        assert_eq!(canvas_rect.top, dom_rect.top());

        // Clean up
        let _ = canvas.remove();
    }
}
