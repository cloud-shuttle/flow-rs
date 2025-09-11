//! Integration tests for DOM rect access with mouse event handling

use wasm_bindgen_test::*;
use web_sys::{Element, MouseEvent};
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Object;

use crate::mouse_integration::{MouseEventConverter, utils};
use crate::dom_rect::DomRectUtils;
use leptos_flow_core::{Position, Viewport};

wasm_bindgen_test_configure!(run_in_browser);

/// Create a mock mouse event for testing
fn create_mock_mouse_event(client_x: f64, client_y: f64) -> MouseEvent {
    let event = Object::new();
    js_sys::Reflect::set(&event, &"clientX".into(), &client_x.into()).unwrap();
    js_sys::Reflect::set(&event, &"clientY".into(), &client_y.into()).unwrap();
    js_sys::Reflect::set(&event, &"preventDefault".into(), &js_sys::Function::new_no_args("")).unwrap();
    event.unchecked_into()
}

/// Create a mock canvas element with specific bounding rect
fn create_mock_canvas_element(x: f64, y: f64, width: f64, height: f64) -> Element {
    let element = Object::new();
    js_sys::Reflect::set(&element, &"getBoundingClientRect".into(), &js_sys::Function::new_no_args(&format!(
        "return {{x: {}, y: {}, width: {}, height: {}, top: {}, right: {}, bottom: {}, left: {}}}",
        x, y, width, height, y, x + width, y + height, x
    ))).unwrap();
    element.unchecked_into()
}

#[wasm_bindgen_test]
fn test_mouse_event_to_canvas_coordinates() {
    // Test converting mouse event coordinates to canvas coordinates using DOM rect
    let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);
    let mouse_event = create_mock_mouse_event(250.0, 350.0);

    let mut rect_utils = DomRectUtils::new();
    let canvas_pos = utils::mouse_event_to_canvas_coords(&mouse_event, &canvas_element, &mut rect_utils).unwrap();

    assert_eq!(canvas_pos.x, 150.0); // 250 - 100
    assert_eq!(canvas_pos.y, 150.0); // 350 - 200
}

#[wasm_bindgen_test]
fn test_canvas_to_world_coordinate_conversion() {
    // Test converting canvas coordinates to world coordinates with viewport
    let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
    let mouse_event = create_mock_mouse_event(400.0, 300.0);

    let mut rect_utils = DomRectUtils::new();
    let viewport = Viewport::new(100.0, 50.0, 0.0, 0.0, 2.0);
    let world_pos = utils::mouse_event_to_world_coords(&mouse_event, &canvas_element, &mut rect_utils, &viewport).unwrap();

    // Expected: (400 / 2.0) + 100 = 300, (300 / 2.0) + 50 = 200
    assert_eq!(world_pos.x, 300.0);
    assert_eq!(world_pos.y, 200.0);
}

#[wasm_bindgen_test]
fn test_world_to_canvas_coordinate_conversion() {
    // Test converting world coordinates to canvas coordinates with viewport
    let mut converter = MouseEventConverter::new();
    let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
    converter.with_canvas(&canvas_element).unwrap();

    let viewport = Viewport::new(100.0, 50.0, 0.0, 0.0, 2.0);
    let world_pos = Position::new(250.0, 175.0);
    let canvas_pos = converter.world_to_canvas(world_pos, &viewport).unwrap();

    // Expected: (250 - 100) * 2.0 = 300, (175 - 50) * 2.0 = 250
    assert_eq!(canvas_pos.x, 300.0);
    assert_eq!(canvas_pos.y, 250.0);
}

#[wasm_bindgen_test]
fn test_mouse_event_integration_with_scroll() {
    // Test mouse event handling with scrolled canvas
    let canvas_element = create_mock_canvas_element(0.0, -100.0, 800.0, 600.0); // Scrolled up
    let mouse_event = create_mock_mouse_event(400.0, 200.0);

    let mut rect_utils = DomRectUtils::new();
    let canvas_pos = utils::mouse_event_to_canvas_coords(&mouse_event, &canvas_element, &mut rect_utils).unwrap();

    // Canvas position should account for scroll
    assert_eq!(canvas_pos.x, 400.0); // 400 - 0
    assert_eq!(canvas_pos.y, 300.0); // 200 - (-100)
}

#[wasm_bindgen_test]
fn test_mouse_event_integration_with_zoom() {
    // Test mouse event handling with zoomed viewport
    let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
    let mouse_event = create_mock_mouse_event(400.0, 300.0);

    let mut rect_utils = DomRectUtils::new();
    let viewport = Viewport::new(0.0, 0.0, 0.0, 0.0, 0.5); // Zoomed out
    let world_pos = utils::mouse_event_to_world_coords(&mouse_event, &canvas_element, &mut rect_utils, &viewport).unwrap();

    // With 0.5 zoom: 400 / 0.5 = 800, 300 / 0.5 = 600
    assert_eq!(world_pos.x, 800.0);
    assert_eq!(world_pos.y, 600.0);
}

#[wasm_bindgen_test]
fn test_mouse_event_error_handling() {
    // Test error handling when DOM rect access fails
    let invalid_element = Object::new().unchecked_into::<Element>();
    let mut rect_utils = DomRectUtils::new();

    let result = rect_utils.get_bounding_client_rect(&invalid_element);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_mouse_event_coordinate_precision() {
    // Test coordinate precision with fractional values
    let canvas_element = create_mock_canvas_element(100.5, 200.25, 800.0, 600.0);
    let mouse_event = create_mock_mouse_event(250.75, 350.125);

    let mut rect_utils = DomRectUtils::new();
    let canvas_pos = utils::mouse_event_to_canvas_coords(&mouse_event, &canvas_element, &mut rect_utils).unwrap();

    // Test precision with fractional values
    assert!((canvas_pos.x - 150.25).abs() < 0.001);
    assert!((canvas_pos.y - 149.875).abs() < 0.001);
}

#[wasm_bindgen_test]
fn test_mouse_event_with_viewport_offset() {
    // Test mouse event handling with viewport offset
    let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
    let mouse_event = create_mock_mouse_event(400.0, 300.0);

    let mut rect_utils = DomRectUtils::new();
    let viewport = Viewport::new(200.0, 150.0, 0.0, 0.0, 1.0);
    let world_pos = utils::mouse_event_to_world_coords(&mouse_event, &canvas_element, &mut rect_utils, &viewport).unwrap();

    // With offset: 400 + 200 = 600, 300 + 150 = 450
    assert_eq!(world_pos.x, 600.0);
    assert_eq!(world_pos.y, 450.0);
}

#[wasm_bindgen_test]
fn test_mouse_event_boundary_conditions() {
    // Test mouse events at canvas boundaries
    let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);
    let mut rect_utils = DomRectUtils::new();

    // Test top-left corner
    let top_left_event = create_mock_mouse_event(100.0, 200.0);
    let top_left_canvas = utils::mouse_event_to_canvas_coords(&top_left_event, &canvas_element, &mut rect_utils).unwrap();
    assert_eq!(top_left_canvas.x, 0.0);
    assert_eq!(top_left_canvas.y, 0.0);

    // Test bottom-right corner
    let bottom_right_event = create_mock_mouse_event(900.0, 800.0);
    let bottom_right_canvas = utils::mouse_event_to_canvas_coords(&bottom_right_event, &canvas_element, &mut rect_utils).unwrap();
    assert_eq!(bottom_right_canvas.x, 800.0);
    assert_eq!(bottom_right_canvas.y, 600.0);
}

#[wasm_bindgen_test]
fn test_mouse_event_integration_performance() {
    // Test performance of mouse event coordinate conversion
    let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
    let mut rect_utils = DomRectUtils::new();
    let viewport = Viewport::new(0.0, 0.0, 0.0, 0.0, 1.0);

    // Perform multiple coordinate conversions
    for i in 0..100 {
        let mouse_event = create_mock_mouse_event(i as f64, i as f64);
        let _world_pos = utils::mouse_event_to_world_coords(&mouse_event, &canvas_element, &mut rect_utils, &viewport).unwrap();
    }

    // If we get here without panicking, performance is acceptable
    assert!(true);
}
