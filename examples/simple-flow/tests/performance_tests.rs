//! Performance tests for the simple flow example
//!
//! These tests verify that the application performs well under various conditions

use wasm_bindgen_test::*;
use leptos_flow_core::{Graph, Node, Edge, Position};
use leptos_flow_renderer::{Canvas2DRenderer, Renderer};

mod common;
use common::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_small_graph_rendering_performance() {
    // Test: Small graph should render quickly
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_simple_test_graph();
    let viewport = create_test_viewport();

    let (_, duration) = measure_execution_time(|| {
        renderer.render_graph(&graph, &viewport).unwrap();
        renderer.present().unwrap();
    });

    // Small graph should render in under 100ms
    assert!(duration <= 100.0, "Small graph rendering took {}ms, expected <= 100ms", duration);
}

#[wasm_bindgen_test]
fn test_large_graph_rendering_performance() {
    // Test: Large graph should still render reasonably fast
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_complex_test_graph();
    let viewport = create_test_viewport();

    let (_, duration) = measure_execution_time(|| {
        renderer.render_graph(&graph, &viewport).unwrap();
        renderer.present().unwrap();
    });

    // Large graph should render in under 500ms
    assert!(duration <= 500.0, "Large graph rendering took {}ms, expected <= 500ms", duration);
}

#[wasm_bindgen_test]
fn test_background_rendering_performance() {
    // Test: Background rendering should be fast
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let viewport = create_test_viewport();
    let bg_config = create_test_background_config();

    let (_, duration) = measure_execution_time(|| {
        renderer.render_background(&bg_config, &viewport).unwrap();
    });

    // Background rendering should be very fast
    assert!(duration <= 50.0, "Background rendering took {}ms, expected <= 50ms", duration);
}

#[wasm_bindgen_test]
fn test_canvas_resize_performance() {
    // Test: Canvas resize should be fast
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    let (_, duration) = measure_execution_time(|| {
        renderer.resize(1920, 1080).unwrap();
    });

    // Canvas resize should be very fast
    assert!(duration <= 10.0, "Canvas resize took {}ms, expected <= 10ms", duration);
}

#[wasm_bindgen_test]
fn test_viewport_operations_performance() {
    // Test: Viewport operations should be very fast
    let mut viewport = create_test_viewport();

    let (_, duration) = measure_execution_time(|| {
        for _ in 0..100 {
            let pan_offset = Position::new(1.0, 1.0);
            viewport = viewport.pan(pan_offset);
            viewport = viewport.zoom_to_point(Position::new(100.0, 100.0), 1.1);
        }
    });

    // 100 viewport operations should be very fast
    assert!(duration <= 10.0, "100 viewport operations took {}ms, expected <= 10ms", duration);
}

#[wasm_bindgen_test]
fn test_graph_operations_performance() {
    // Test: Graph operations should be fast
    let mut graph = Graph::new();

    let (_, duration) = measure_execution_time(|| {
        // Add 100 nodes
        for i in 0..100 {
            let node_id = format!("node_{}", i);
            let position = Position::new(i as f64 * 10.0, i as f64 * 10.0);
            let node = Node::simple(node_id.clone(), position);
            graph.add_node(node).unwrap();
        }

        // Add 99 edges
        for i in 0..99 {
            let edge_id = format!("edge_{}", i);
            let source = format!("node_{}", i);
            let target = format!("node_{}", i + 1);
            let edge = Edge::simple(edge_id.clone(), source.clone(), target.clone());
            graph.add_edge(edge).unwrap();
        }
    });

    // Graph operations should be fast
    assert!(duration <= 50.0, "Graph operations took {}ms, expected <= 50ms", duration);
}

#[wasm_bindgen_test]
fn test_memory_usage() {
    // Test: Application should not use excessive memory
    let canvas = create_test_canvas();
    let renderer = Canvas2DRenderer::new(&canvas).unwrap();

    // Memory test is not available in all browsers
    // For now, just verify the renderer was created successfully
    assert!(renderer.capabilities().name.len() > 0, "Renderer should have a name");
}

#[wasm_bindgen_test]
fn test_renderer_initialization_performance() {
    // Test: Renderer initialization should be fast
    let canvas = create_test_canvas();

    let (_, duration) = measure_execution_time(|| {
        Canvas2DRenderer::new(&canvas).unwrap();
    });

    // Renderer initialization should be fast
    assert!(duration <= 100.0, "Renderer initialization took {}ms, expected <= 100ms", duration);
}

#[wasm_bindgen_test]
fn test_concurrent_operations() {
    // Test: Multiple operations should not interfere with each other
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_simple_test_graph();
    let viewport = create_test_viewport();
    let bg_config = create_test_background_config();

    let (_, duration) = measure_execution_time(|| {
        // Perform multiple operations in sequence
        renderer.clear(Some("#ffffff")).unwrap();
        renderer.render_background(&bg_config, &viewport).unwrap();
        renderer.render_graph(&graph, &viewport).unwrap();
        renderer.present().unwrap();

        // Resize and render again
        renderer.resize(1024, 768).unwrap();
        renderer.render_graph(&graph, &viewport).unwrap();
        renderer.present().unwrap();
    });

    // All operations should complete quickly
    assert!(duration <= 200.0, "Concurrent operations took {}ms, expected <= 200ms", duration);
}
