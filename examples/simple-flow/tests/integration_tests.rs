//! Integration tests for the simple flow example
//! 
//! These tests verify the complete application works end-to-end

use wasm_bindgen_test::*;
use leptos_flow_core::{Graph, Node, Edge, Position, Viewport};
use leptos_flow_renderer::{Canvas2DRenderer, Renderer};
use leptos_flow_renderer::traits::{BackgroundConfig, BackgroundVariant};
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

#[wasm_bindgen_test]
fn test_canvas2d_renderer_initialization() {
    // Test: Canvas2D renderer should initialize successfully
    let canvas = create_test_canvas();
    let renderer = Canvas2DRenderer::new(&canvas);
    
    assert!(renderer.is_ok(), "Canvas2D renderer should initialize successfully");
}

#[wasm_bindgen_test]
fn test_graph_rendering() {
    // Test: Should be able to render a graph with nodes and edges
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_test_graph();
    let viewport = Viewport::default();
    
    // Should render without errors
    let result = renderer.render_graph(&graph, &viewport);
    assert!(result.is_ok(), "Graph rendering should succeed");
    
    // Should present frame without errors
    let present_result = renderer.present();
    assert!(present_result.is_ok(), "Frame presentation should succeed");
}

#[wasm_bindgen_test]
fn test_background_rendering() {
    // Test: Should be able to render different background patterns
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let viewport = Viewport::default();
    
    let bg_config = BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.5,
    };
    
    let result = renderer.render_background(&bg_config, &viewport);
    assert!(result.is_ok(), "Background rendering should succeed");
}

#[wasm_bindgen_test]
fn test_viewport_operations() {
    // Test: Viewport should support panning and zooming operations
    let mut viewport = Viewport::default();
    
    // Test panning
    let pan_offset = Position::new(50.0, 30.0);
    let panned_viewport = viewport.pan(pan_offset);
    assert_eq!(panned_viewport.offset, pan_offset);
    
    // Test zooming to a point
    let zoom_point = Position::new(100.0, 100.0);
    let zoomed_viewport = panned_viewport.zoom_to_point(zoom_point, 2.0);
    assert_eq!(zoomed_viewport.zoom, 2.0);
}

#[wasm_bindgen_test]
fn test_graph_operations() {
    // Test: Graph should support adding and removing nodes and edges
    let mut graph = Graph::new();
    
    // Add nodes
    let node1 = Node::simple("node1", Position::new(100.0, 100.0));
    let node2 = Node::simple("node2", Position::new(200.0, 200.0));
    
    assert!(graph.add_node(node1).is_ok());
    assert!(graph.add_node(node2).is_ok());
    
    // Add edge
    let edge = Edge::simple("edge1", "node1", "node2");
    assert!(graph.add_edge(edge).is_ok());
    
    // Verify nodes exist
    assert!(graph.get_node(&"node1".into()).is_some());
    assert!(graph.get_node(&"node2".into()).is_some());
    
    // Verify edge exists
    assert!(graph.get_edge(&"edge1".into()).is_some());
}

#[wasm_bindgen_test]
fn test_canvas_resize() {
    // Test: Canvas should support resizing
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    
    let result = renderer.resize(1024, 768);
    assert!(result.is_ok(), "Canvas resize should succeed");
}

#[wasm_bindgen_test]
fn test_renderer_capabilities() {
    // Test: Renderer should report correct capabilities
    let canvas = create_test_canvas();
    let renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let capabilities = renderer.capabilities();
    
    // Should have basic capabilities
    assert!(capabilities.name.len() > 0, "Renderer should have a name");
    assert!(capabilities.max_texture_size > 0, "Renderer should support textures");
}
