//! Visual selection feedback tests
//!
//! Tests that verify selection rendering works correctly

use wasm_bindgen_test::*;
use flow_core::{Graph, Node, Position, Rect};
use crate::canvas2d::Canvas2DRenderer;
use crate::traits::{Renderer, SelectionStyle, AnimatedSelectionStyle, MultiSelectionStyle, SelectionHoverStyle};
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use js_sys;

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

// TDD FAILING TESTS - These define our new selection animation features

#[wasm_bindgen_test]
fn test_animated_selection_style_creation() {
    // Test creating animated selection styles with timing
    let animated_style = AnimatedSelectionStyle::new()
        .with_animation_duration(300.0)  // 300ms
        .with_pulse_enabled(true)
        .with_fade_in_enabled(true);

    assert_eq!(animated_style.animation_duration_ms(), 300.0);
    assert!(animated_style.pulse_enabled());
    assert!(animated_style.fade_in_enabled());
    assert!(!animated_style.is_animating()); // not started yet
}

#[wasm_bindgen_test]
fn test_selection_animation_state_machine() {
    // Test selection animation state transitions
    let mut animated_style = AnimatedSelectionStyle::new();

    // Initially not animating
    assert!(!animated_style.is_animating());

    // Start animation
    animated_style.start_animation();
    assert!(animated_style.is_animating());

    // Update animation progress
    animated_style.update_animation(150.0); // 50% through 300ms animation
    assert_eq!(animated_style.animation_progress(), 0.5);

    // Finish animation
    animated_style.update_animation(300.0);
    assert!(!animated_style.is_animating());
    assert_eq!(animated_style.animation_progress(), 1.0);
}

#[wasm_bindgen_test]
fn test_render_animated_selection() {
    // Test rendering animated selection with Canvas2D
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_test_graph();

    // Create animated selection style
    let mut animated_style = AnimatedSelectionStyle::new()
        .with_pulse_enabled(true)
        .with_animation_duration(200.0);

    // Start animation
    animated_style.start_animation();
    animated_style.update_animation(100.0); // 50% progress

    // Get node bounds for rendering
    let bounds = vec![
        Rect::new(100.0, 100.0, 80.0, 40.0), // node1 bounds
        Rect::new(200.0, 150.0, 80.0, 40.0), // node2 bounds
    ];

    // Render animated selection - should not panic
    let result = renderer.render_animated_selection(&bounds, &animated_style);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_multi_selection_indicators() {
    // Test special indicators for multi-selection
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    let bounds = vec![
        Rect::new(100.0, 100.0, 80.0, 40.0),
        Rect::new(200.0, 150.0, 80.0, 40.0),
        Rect::new(300.0, 200.0, 80.0, 40.0),
    ];

    let multi_style = MultiSelectionStyle::new()
        .with_connection_lines(true)
        .with_selection_count_indicator(true);

    // Render multi-selection indicators
    let result = renderer.render_multi_selection(&bounds, &multi_style);
    assert!(result.is_ok());

    // Check that multi-selection state is tracked
    assert_eq!(renderer.get_selection_count(), 3);
    assert!(renderer.is_multi_selection_active());
}

#[wasm_bindgen_test]
fn test_selection_hover_effects() {
    // Test hover effects on selectable elements
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    let hover_style = SelectionHoverStyle::new()
        .with_hover_highlight_color("#ffeb3b".to_string())
        .with_hover_scale_factor(1.05)
        .with_hover_glow_enabled(true);

    let node_bounds = Rect::new(100.0, 100.0, 80.0, 40.0);
    let hover_position = Position::new(140.0, 120.0); // Inside node

    // Render hover effects
    let result = renderer.render_selection_hover(&node_bounds, &hover_position, &hover_style);
    assert!(result.is_ok());

    // Check hover state
    assert!(renderer.is_hover_active(&node_bounds));
}

#[wasm_bindgen_test]
fn test_animation_performance_optimization() {
    // Test that animations are optimized for large selections
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    // Create large selection (100 nodes)
    let mut bounds = Vec::new();
    for i in 0..100 {
        bounds.push(Rect::new(
            (i % 10) as f64 * 100.0,
            (i / 10) as f64 * 60.0,
            80.0,
            40.0
        ));
    }

    let animated_style = AnimatedSelectionStyle::new()
        .with_performance_mode(true) // Enable optimizations
        .with_batch_rendering(true);

    // Should render efficiently without dropping frames
    let start_time = js_sys::Date::now();
    let result = renderer.render_animated_selection(&bounds, &animated_style);
    let end_time = js_sys::Date::now();

    assert!(result.is_ok());
    assert!((end_time - start_time) < 16.0); // < 16ms for 60fps

    // Check that batching was used
    let stats = renderer.get_stats();
    assert!(stats.draw_calls < bounds.len()); // Fewer draw calls than nodes
}
