// Tests for selection animations and visual enhancements
//
// This module tests the animation system for node selection, including
// animated selection indicators, multi-selection effects, and hover animations.

use leptos::*;
use wasm_bindgen_test::*;
use web_sys::{HtmlCanvasElement, MouseEvent};
use js_sys::Object;

use leptos_flow_core::{Graph, Node, Position, NodeId, Rect};
use leptos_flow_renderer::{
    Canvas2DRenderer,
    traits::{AnimatedSelectionStyle, MultiSelectionStyle, SelectionHoverStyle, SelectionStyle}
};
use crate::signals::{FlowState, ViewportState};

wasm_bindgen_test_configure!(run_in_browser);

/// Test that animated selection style can be created and configured
#[wasm_bindgen_test]
fn test_animated_selection_style_creation() {
    let style = AnimatedSelectionStyle::new()
        .with_animation_duration(500.0)
        .with_pulse_enabled(true)
        .with_fade_in_enabled(true);

    assert_eq!(style.animation_duration_ms(), 500.0);
    assert!(style.pulse_enabled());
    assert!(style.fade_in_enabled());
    assert!(!style.is_animating());
    assert_eq!(style.animation_progress(), 0.0);
}

/// Test that animation can be started and progress tracked
#[wasm_bindgen_test]
fn test_animation_start_and_progress() {
    let mut style = AnimatedSelectionStyle::new()
        .with_animation_duration(1000.0);

    // Start animation
    style.start_animation();
    assert!(style.is_animating());
    assert_eq!(style.animation_progress(), 0.0);

    // Simulate time passing
    style.update_animation(500.0); // Half way through
    assert!(style.is_animating());
    assert_eq!(style.animation_progress(), 0.5);

    // Complete animation
    style.update_animation(1000.0); // Full duration
    assert!(!style.is_animating());
    assert_eq!(style.animation_progress(), 1.0);
}

/// Test that multi-selection style can be configured
#[wasm_bindgen_test]
fn test_multi_selection_style_creation() {
    let style = MultiSelectionStyle::new()
        .with_connection_lines(true)
        .with_selection_count_indicator(true);

    assert!(style.connection_lines);
    assert!(style.selection_count_indicator);
    assert_eq!(style.connection_color, "#1a73e8");
    assert_eq!(style.connection_width, 1.0);
}

/// Test that selection hover style can be configured
#[wasm_bindgen_test]
fn test_selection_hover_style_creation() {
    let style = SelectionHoverStyle::new()
        .with_hover_highlight_color("#ff0000".to_string())
        .with_hover_scale_factor(1.1)
        .with_hover_glow_enabled(true);

    assert_eq!(style.hover_highlight_color, "#ff0000");
    assert_eq!(style.hover_scale_factor, 1.1);
    assert!(style.hover_glow_enabled);
}

/// Test that Canvas2D renderer can render animated selection
#[wasm_bindgen_test]
fn test_canvas2d_animated_selection_rendering() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).expect("Failed to create renderer");

    // Create test selection bounds
    let bounds = vec![
        Rect::new(100.0, 100.0, 50.0, 50.0),
        Rect::new(200.0, 200.0, 50.0, 50.0),
    ];

    // Create animated selection style
    let mut style = AnimatedSelectionStyle::new()
        .with_animation_duration(300.0)
        .with_pulse_enabled(true)
        .with_fade_in_enabled(true);

    style.start_animation();

    // Render animated selection
    let result = renderer.render_animated_selection(&bounds, &style);
    assert!(result.is_ok());

    // Check that selection count is updated
    assert_eq!(renderer.get_selection_count(), 0); // Not set by animated selection
}

/// Test that Canvas2D renderer can render multi-selection indicators
#[wasm_bindgen_test]
fn test_canvas2d_multi_selection_rendering() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).expect("Failed to create renderer");

    // Create test selection bounds
    let bounds = vec![
        Rect::new(100.0, 100.0, 50.0, 50.0),
        Rect::new(200.0, 200.0, 50.0, 50.0),
        Rect::new(300.0, 300.0, 50.0, 50.0),
    ];

    // Create multi-selection style
    let style = MultiSelectionStyle::new()
        .with_connection_lines(true)
        .with_selection_count_indicator(true);

    // Render multi-selection
    let result = renderer.render_multi_selection(&bounds, &style);
    assert!(result.is_ok());

    // Check that selection count and multi-selection state are updated
    assert_eq!(renderer.get_selection_count(), 3);
    assert!(renderer.is_multi_selection_active());
}

/// Test that Canvas2D renderer can render selection hover effects
#[wasm_bindgen_test]
fn test_canvas2d_selection_hover_rendering() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).expect("Failed to create renderer");

    // Create test bounds and hover position
    let bounds = Rect::new(100.0, 100.0, 50.0, 50.0);
    let hover_position = Position::new(125.0, 125.0); // Inside bounds

    // Create hover style
    let style = SelectionHoverStyle::new()
        .with_hover_highlight_color("#ffff00".to_string())
        .with_hover_scale_factor(1.05)
        .with_hover_glow_enabled(true);

    // Render hover effect
    let result = renderer.render_selection_hover(&bounds, &hover_position, &style);
    assert!(result.is_ok());

    // Check that hover is active
    assert!(renderer.is_hover_active(&bounds));
}

/// Test that hover detection works correctly
#[wasm_bindgen_test]
fn test_hover_detection() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).expect("Failed to create renderer");

    let bounds = Rect::new(100.0, 100.0, 50.0, 50.0);
    let hover_style = SelectionHoverStyle::new();

    // Test hover inside bounds
    let inside_position = Position::new(125.0, 125.0);
    let _ = renderer.render_selection_hover(&bounds, &inside_position, &hover_style);
    assert!(renderer.is_hover_active(&bounds));

    // Test hover outside bounds
    let outside_position = Position::new(200.0, 200.0);
    let _ = renderer.render_selection_hover(&bounds, &outside_position, &hover_style);
    assert!(!renderer.is_hover_active(&bounds));
}

/// Test that selection count is properly tracked
#[wasm_bindgen_test]
fn test_selection_count_tracking() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).expect("Failed to create renderer");

    // Initially no selection
    assert_eq!(renderer.get_selection_count(), 0);
    assert!(!renderer.is_multi_selection_active());

    // Render multi-selection with 2 items
    let bounds_2 = vec![
        Rect::new(100.0, 100.0, 50.0, 50.0),
        Rect::new(200.0, 200.0, 50.0, 50.0),
    ];
    let style = MultiSelectionStyle::new();
    let _ = renderer.render_multi_selection(&bounds_2, &style);

    assert_eq!(renderer.get_selection_count(), 2);
    assert!(renderer.is_multi_selection_active());

    // Render multi-selection with 4 items
    let bounds_4 = vec![
        Rect::new(100.0, 100.0, 50.0, 50.0),
        Rect::new(200.0, 200.0, 50.0, 50.0),
        Rect::new(300.0, 300.0, 50.0, 50.0),
        Rect::new(400.0, 400.0, 50.0, 50.0),
    ];
    let _ = renderer.render_multi_selection(&bounds_4, &style);

    assert_eq!(renderer.get_selection_count(), 4);
    assert!(renderer.is_multi_selection_active());
}

/// Test that animation progress affects visual rendering
#[wasm_bindgen_test]
fn test_animation_progress_visual_effects() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).expect("Failed to create renderer");

    let bounds = vec![Rect::new(100.0, 100.0, 50.0, 50.0)];
    let mut style = AnimatedSelectionStyle::new()
        .with_animation_duration(1000.0)
        .with_pulse_enabled(true)
        .with_fade_in_enabled(true);

    // Start animation
    style.start_animation();

    // Test at different progress points
    style.update_animation(0.0); // Start
    let result1 = renderer.render_animated_selection(&bounds, &style);
    assert!(result1.is_ok());

    style.update_animation(500.0); // Middle
    let result2 = renderer.render_animated_selection(&bounds, &style);
    assert!(result2.is_ok());

    style.update_animation(1000.0); // End
    let result3 = renderer.render_animated_selection(&bounds, &style);
    assert!(result3.is_ok());
}

/// Helper function to create a test canvas element
fn create_test_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.dyn_into::<HtmlCanvasElement>().unwrap()
}
