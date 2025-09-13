//! Common test utilities and helpers
//!
//! This module provides shared test utilities that can be used across
//! different test files.

use flow_core::{Graph, Node, Edge, Position, Viewport};
use flow_renderer::Renderer;
use flow_renderer::traits::{BackgroundConfig, BackgroundVariant};
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;

/// Create a test canvas element with default dimensions
pub fn create_test_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    canvas.set_width(800);
    canvas.set_height(600);
    canvas
}

/// Create a test canvas with custom dimensions
pub fn create_test_canvas_with_size(width: u32, height: u32) -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    canvas
}

/// Create a simple test graph with 3 nodes and 2 edges
pub fn create_simple_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    let node1 = Node::simple("node1", Position::new(100.0, 100.0));
    let node2 = Node::simple("node2", Position::new(300.0, 200.0));
    let node3 = Node::simple("node3", Position::new(500.0, 150.0));

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    let edge1 = Edge::simple("edge1", "node1", "node2");
    let edge2 = Edge::simple("edge2", "node2", "node3");

    graph.add_edge(edge1).unwrap();
    graph.add_edge(edge2).unwrap();

    graph
}

/// Create a complex test graph with many nodes and edges
pub fn create_complex_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Create a grid of nodes
    for i in 0..5 {
        for j in 0..5 {
            let node_id = format!("node_{}_{}", i, j);
            let position = Position::new(i as f64 * 100.0, j as f64 * 100.0);
            let node = Node::simple(node_id.clone(), position);
            graph.add_node(node).unwrap();
        }
    }

    // Create edges between adjacent nodes
    for i in 0..4 {
        for j in 0..5 {
            // Horizontal edges
            let edge_id = format!("edge_h_{}_{}", i, j);
            let source = format!("node_{}_{}", i, j);
            let target = format!("node_{}_{}", i + 1, j);
            let edge = Edge::simple(edge_id.clone(), source.clone(), target.clone());
            graph.add_edge(edge).unwrap();
        }
    }

    for i in 0..5 {
        for j in 0..4 {
            // Vertical edges
            let edge_id = format!("edge_v_{}_{}", i, j);
            let source = format!("node_{}_{}", i, j);
            let target = format!("node_{}_{}", i, j + 1);
            let edge = Edge::simple(edge_id.clone(), source.clone(), target.clone());
            graph.add_edge(edge).unwrap();
        }
    }

    graph
}

/// Create a default background configuration for tests
pub fn create_test_background_config() -> BackgroundConfig {
    BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.5,
    }
}

/// Create a test viewport
pub fn create_test_viewport() -> Viewport {
    Viewport::default()
}

/// Create a test viewport with custom settings
pub fn create_test_viewport_with_settings(x: f64, y: f64, width: f64, height: f64, zoom: f64) -> Viewport {
    Viewport::new(x, y, width, height, zoom)
}

/// Assert that a renderer operation succeeds
pub fn assert_renderer_operation_success<F, R>(operation: F)
where
    F: FnOnce() -> Result<R, Box<dyn std::error::Error>>,
{
    let result = operation();
    assert!(result.is_ok(), "Renderer operation should succeed: {:?}", result.err());
}

/// Assert that a renderer operation fails with expected error
pub fn assert_renderer_operation_fails<F, R>(operation: F, expected_error: &str)
where
    F: FnOnce() -> Result<R, Box<dyn std::error::Error>>,
{
    let result = operation();
    assert!(result.is_err(), "Renderer operation should fail");
    let error_msg = format!("{:?}", result.err().unwrap());
    assert!(error_msg.contains(expected_error),
           "Expected error containing '{}', got: {}", expected_error, error_msg);
}

/// Performance test helper - measure execution time
pub fn measure_execution_time<F, R>(operation: F) -> (R, f64)
where
    F: FnOnce() -> R,
{
    let start = web_sys::js_sys::Date::now();
    let result = operation();
    let end = web_sys::js_sys::Date::now();
    let duration = end - start;
    (result, duration)
}

/// Assert that an operation completes within a time limit (in milliseconds)
pub fn assert_operation_within_time_limit<F, R>(operation: F, max_time_ms: f64)
where
    F: FnOnce() -> R,
{
    let (_, duration) = measure_execution_time(operation);
    assert!(duration <= max_time_ms,
           "Operation took {}ms, expected <= {}ms", duration, max_time_ms);
}
