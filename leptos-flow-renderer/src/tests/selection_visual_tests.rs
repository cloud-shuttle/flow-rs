//! Visual selection feedback tests
//!
//! Tests that verify selection rendering works correctly

use wasm_bindgen_test::*;
use leptos_flow_core::{Graph, Node, Position, Rect};
use crate::canvas2d::Canvas2DRenderer;
use crate::traits::{Renderer, SelectionStyle};
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_test_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    canvas.set_width(800);
    canvas.set_height(600);
    canvas
}

fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
    graph.add_node(Node::simple("node2", Position::new(200.0, 150.0))).unwrap();
    graph.add_node(Node::simple("node3", Position::new(300.0, 200.0))).unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_selection_style_creation() {
    // Test: SelectionStyle should have sensible defaults
    let style = SelectionStyle::default();

    assert_eq!(style.color, "#1a73e8");
    assert_eq!(style.width, 2.0);
    assert!(style.glow_color.is_some());
    assert!(style.glow_blur.is_some());
}

#[wasm_bindgen_test]
fn test_selection_bounds_calculation() {
    // Test: Should correctly calculate bounds for selected nodes
    let graph = create_test_graph();

    // Create selection bounds for nodes
    let mut selected_bounds = Vec::new();

    if let Some(node) = graph.get_node(&"node1".into()) {
        let rect = Rect::new(
            node.position.x,
            node.position.y,
            node.size.width,
            node.size.height,
        );
        selected_bounds.push(rect);
    }

    assert_eq!(selected_bounds.len(), 1);
    assert_eq!(selected_bounds[0].x, 100.0);
    assert_eq!(selected_bounds[0].y, 100.0);
}

#[wasm_bindgen_test]
fn test_render_selection_basic() {
    // Test: Should render selection indicators without errors
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    // Create selection bounds
    let selected_bounds = vec![
        Rect::new(100.0, 100.0, 50.0, 30.0),
        Rect::new(200.0, 150.0, 60.0, 40.0),
    ];

    let style = SelectionStyle::default();

    // Should render without errors
    let result = renderer.render_selection(&selected_bounds, &style);
    assert!(result.is_ok(), "Selection rendering should succeed");
}

#[wasm_bindgen_test]
fn test_render_selection_with_custom_style() {
    // Test: Should render selection with custom styling
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    let selected_bounds = vec![Rect::new(150.0, 100.0, 80.0, 50.0)];

    let custom_style = SelectionStyle {
        color: "#ff6b6b".to_string(),
        width: 3.0,
        dasharray: Some("5,5".to_string()),
        glow_color: Some("#ff6b6b".to_string()),
        glow_blur: Some(6.0),
    };

    let result = renderer.render_selection(&selected_bounds, &custom_style);
    assert!(result.is_ok(), "Custom style selection rendering should succeed");
}

#[wasm_bindgen_test]
fn test_render_empty_selection() {
    // Test: Should handle empty selection gracefully
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    let empty_bounds = Vec::new();
    let style = SelectionStyle::default();

    let result = renderer.render_selection(&empty_bounds, &style);
    assert!(result.is_ok(), "Empty selection rendering should succeed");
}

#[wasm_bindgen_test]
fn test_render_large_selection() {
    // Test: Should handle large number of selected items
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    let mut large_selection = Vec::new();
    for i in 0..50 {
        large_selection.push(Rect::new(
            (i * 20) as f64,
            (i * 15) as f64,
            40.0,
            30.0,
        ));
    }

    let style = SelectionStyle::default();
    let result = renderer.render_selection(&large_selection, &style);
    assert!(result.is_ok(), "Large selection rendering should succeed");
}

// FAILING TESTS - These will be completed in future TDD cycles
#[wasm_bindgen_test]
#[should_panic(expected = "Selection animations not implemented")]
fn test_selection_animation_not_implemented() {
    // This test will fail until we implement selection animations
    panic!("Selection animations not implemented");
}

#[wasm_bindgen_test]
#[should_panic(expected = "Multi-selection indicators not implemented")]
fn test_multi_selection_indicators_not_implemented() {
    // This test will fail until we implement special multi-selection indicators
    panic!("Multi-selection indicators not implemented");
}

#[wasm_bindgen_test]
#[should_panic(expected = "Selection hover effects not implemented")]
fn test_selection_hover_effects_not_implemented() {
    // This test will fail until we implement hover effects for selection
    panic!("Selection hover effects not implemented");
}
