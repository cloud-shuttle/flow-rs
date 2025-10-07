//! Hello World Example
//!
//! The simplest possible Flow-RS example demonstrating:
//! - Creating a basic graph with nodes and edges
//! - Setting up a canvas renderer
//! - Rendering a static flow diagram
//!
//! This example shows the absolute minimum code needed to display
//! a flow diagram using Flow-RS.

use flow_rs_core::{Edge, Graph, Node, Position, Viewport};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::{Canvas2DRenderer, Renderer};
use wasm_bindgen::prelude::*;

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Get the canvas element from the DOM
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("flow-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // Create the renderer
    let mut renderer = Canvas2DRenderer::new(&canvas)
        .expect("Failed to create renderer");

    // Create a simple graph
    let graph = create_hello_world_graph();

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

    // Render the graph
    render_graph(&mut renderer, &graph, &viewport)
        .expect("Failed to render graph");
}

/// Create a simple "Hello World" graph with 3 nodes and 2 edges
fn create_hello_world_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Create three nodes in a simple flow
    let start_node = Node::simple("start", Position::new(150.0, 150.0));
    let process_node = Node::simple("process", Position::new(400.0, 150.0));
    let end_node = Node::simple("end", Position::new(650.0, 150.0));

    // Add nodes to the graph
    graph.add_node(start_node).expect("Failed to add start node");
    graph.add_node(process_node).expect("Failed to add process node");
    graph.add_node(end_node).expect("Failed to add end node");

    // Create edges connecting the nodes
    let edge1 = Edge::simple("start-to-process", "start", "process");
    let edge2 = Edge::simple("process-to-end", "process", "end");

    // Add edges to the graph
    graph.add_edge(edge1).expect("Failed to add first edge");
    graph.add_edge(edge2).expect("Failed to add second edge");

    graph
}

/// Render the graph to the canvas
fn render_graph(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<(), ()>,
    viewport: &Viewport,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear the canvas with a white background
    renderer.clear(Some("#ffffff"))?;

    // Set up a simple dotted background
    let background_config = BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.3,
    };

    // Render the background
    renderer.render_background(&background_config, viewport)?;

    // Render the graph (nodes and edges)
    renderer.render_graph_dyn(graph, viewport)?;

    // Present the frame
    renderer.present()?;

    Ok(())
}
