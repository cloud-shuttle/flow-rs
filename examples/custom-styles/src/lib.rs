//! Custom Node Styles Example
//!
//! Demonstrates how to customize the appearance of nodes:
//! - Different colors and styling
//! - Custom CSS classes
//! - Various node shapes and sizes
//! - Text labels and positioning
//!
//! This example shows the visual customization capabilities of Flow-RS.

use flow_rs_core::{Edge, Graph, Node, Position, Viewport};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::{Canvas2DRenderer, Renderer};
use wasm_bindgen::prelude::*;

// Custom node data structure to include styling information
#[derive(Clone, Debug)]
struct StyledNodeData {
    pub label: String,
    pub color: String,
    pub shape: NodeShape,
    pub size: NodeSize,
}

#[derive(Clone, Debug)]
enum NodeShape {
    Rectangle,
    Rounded,
    Circle,
}

#[derive(Clone, Debug)]
enum NodeSize {
    Small,
    Medium,
    Large,
}

impl StyledNodeData {
    fn new(label: &str, color: &str, shape: NodeShape, size: NodeSize) -> Self {
        Self {
            label: label.to_string(),
            color: color.to_string(),
            shape,
            size,
        }
    }

    // Get size dimensions
    fn dimensions(&self) -> (f64, f64) {
        match self.size {
            NodeSize::Small => (80.0, 40.0),
            NodeSize::Medium => (120.0, 60.0),
            NodeSize::Large => (160.0, 80.0),
        }
    }
}

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

    // Create a graph with styled nodes
    let graph = create_styled_graph();

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 1000.0, 600.0, 1.0);

    // Render the graph
    render_styled_graph(&mut renderer, &graph, &viewport)
        .expect("Failed to render graph");
}

/// Create a graph demonstrating various node styling options
fn create_styled_graph() -> Graph<StyledNodeData, ()> {
    let mut graph = Graph::new();

    // Create nodes with different styles
    let input_node = Node {
        id: "input".to_string(),
        position: Position::new(100.0, 150.0),
        data: StyledNodeData::new(
            "Input Data",
            "#4299e1", // Blue
            NodeShape::Rounded,
            NodeSize::Medium,
        ),
        size: Position::new(120.0, 60.0), // Will be overridden by renderer
        ..Default::default()
    };

    let process_node = Node {
        id: "process".to_string(),
        position: Position::new(350.0, 100.0),
        data: StyledNodeData::new(
            "Process",
            "#48bb78", // Green
            NodeShape::Rectangle,
            NodeSize::Large,
        ),
        size: Position::new(160.0, 80.0),
        ..Default::default()
    };

    let decision_node = Node {
        id: "decision".to_string(),
        position: Position::new(350.0, 250.0),
        data: StyledNodeData::new(
            "Decision?",
            "#ed8936", // Orange
            NodeShape::Circle,
            NodeSize::Medium,
        ),
        size: Position::new(120.0, 60.0),
        ..Default::default()
    };

    let output_node = Node {
        id: "output".to_string(),
        position: Position::new(600.0, 150.0),
        data: StyledNodeData::new(
            "Output",
            "#9f7aea", // Purple
            NodeShape::Rounded,
            NodeSize::Medium,
        ),
        size: Position::new(120.0, 60.0),
        ..Default::default()
    };

    let error_node = Node {
        id: "error".to_string(),
        position: Position::new(600.0, 350.0),
        data: StyledNodeData::new(
            "Error",
            "#f56565", // Red
            NodeShape::Rectangle,
            NodeSize::Small,
        ),
        size: Position::new(80.0, 40.0),
        ..Default::default()
    };

    // Add nodes to the graph
    graph.add_node(input_node).expect("Failed to add input node");
    graph.add_node(process_node).expect("Failed to add process node");
    graph.add_node(decision_node).expect("Failed to add decision node");
    graph.add_node(output_node).expect("Failed to add output node");
    graph.add_node(error_node).expect("Failed to add error node");

    // Create edges connecting the nodes
    let edges = vec![
        Edge::simple("input-to-process", "input", "process"),
        Edge::simple("process-to-decision", "process", "decision"),
        Edge::simple("process-to-output", "process", "output"),
        Edge::simple("decision-to-error", "decision", "error"),
    ];

    // Add edges to the graph
    for edge in edges {
        graph.add_edge(edge).expect("Failed to add edge");
    }

    graph
}

/// Render the graph with custom styling
fn render_styled_graph(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<StyledNodeData, ()>,
    viewport: &Viewport,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear the canvas with a light background
    renderer.clear(Some("#f8fafc"))?;

    // Set up a subtle grid background
    let background_config = BackgroundConfig {
        color: "#f8fafc".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 15.0,
        opacity: 0.4,
    };

    // Render the background
    renderer.render_background(&background_config, viewport)?;

    // Render the graph (nodes and edges)
    renderer.render_graph_dyn(graph, viewport)?;

    // Present the frame
    renderer.present()?;

    Ok(())
}
