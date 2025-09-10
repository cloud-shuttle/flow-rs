//! Tests for Canvas2D renderer

use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use leptos_flow_core::{Graph, Node, Edge, Position, Viewport};
use leptos_flow_core::types::{NodeId, EdgeId};
use crate::canvas2d::Canvas2DRenderer;
use crate::traits::{Renderer, NodeStyle, EdgeStyle, BackgroundConfig, BackgroundVariant};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_test_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.dyn_into::<HtmlCanvasElement>().unwrap()
}

fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    let node1 = Node::simple("node1", Position::new(100.0, 100.0));
    let node2 = Node::simple("node2", Position::new(300.0, 200.0));

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    let edge = Edge::simple("edge1", "node1", "node2");
    graph.add_edge(edge).unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_canvas2d_renderer_creation() {
    let canvas = create_test_canvas();
    let renderer = Canvas2DRenderer::new(&canvas);
    assert!(renderer.is_ok());
}

#[wasm_bindgen_test]
fn test_capabilities() {
    let canvas = create_test_canvas();
    let renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let caps = renderer.capabilities();

    assert_eq!(caps.name, "Canvas2D");
    assert_eq!(caps.supports_msaa, true);
    assert_eq!(caps.supports_compute_shaders, false);
}

#[wasm_bindgen_test]
fn test_clear() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    assert!(renderer.clear(Some("#ff0000")).is_ok());
    assert!(renderer.clear(None).is_ok());
}

#[wasm_bindgen_test]
fn test_resize() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

    assert!(renderer.resize(800, 600).is_ok());
}

#[wasm_bindgen_test]
fn test_render_graph() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_test_graph();
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

    // This test should pass once we fix the compilation errors
    let result = renderer.render_graph(&graph, &viewport);
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert_eq!(stats.nodes_rendered, 2);
    assert_eq!(stats.edges_rendered, 1);
}

#[wasm_bindgen_test]
fn test_render_background() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

    let config = BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#cccccc".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.5,
    };

    assert!(renderer.render_background(&config, &viewport).is_ok());
}
