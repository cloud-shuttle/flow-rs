//! Custom Edges Example
//!
//! Demonstrates Flow-RS's edge customization capabilities:
//! - Different edge types (straight, bezier, step)
//! - Custom styling (colors, thickness, dash patterns)
//! - Edge labels and markers
//! - Connection validation and feedback
//!
//! This example shows how to create visually rich and interactive edge connections.

use flow_rs_core::{Edge, Graph, Node, Position, Viewport};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::Canvas2DRenderer;
use wasm_bindgen::prelude::*;

// Custom edge data structure for styling
#[derive(Clone, Debug)]
struct StyledEdgeData {
    pub edge_type: EdgeType,
    pub color: String,
    pub thickness: f64,
    pub dash_pattern: Option<Vec<f64>>,
    pub label: Option<String>,
    pub animated: bool,
}

#[derive(Clone, Debug)]
enum EdgeType {
    Straight,
    Bezier,
    Step,
    SmoothStep,
}

impl StyledEdgeData {
    fn new(
        edge_type: EdgeType,
        color: &str,
        thickness: f64,
        label: Option<&str>,
        animated: bool,
    ) -> Self {
        Self {
            edge_type,
            color: color.to_string(),
            thickness,
            dash_pattern: None,
            label: label.map(|s| s.to_string()),
            animated,
        }
    }

    fn with_dash_pattern(mut self, pattern: Vec<f64>) -> Self {
        self.dash_pattern = Some(pattern);
        self
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

    // Create a graph with styled edges
    let graph = create_styled_edges_graph();

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 1000.0, 700.0, 1.0);

    // Render the graph
    render_styled_edges_graph(&mut renderer, &graph, &viewport)
        .expect("Failed to render graph");
}

/// Create a graph demonstrating various edge styling options
fn create_styled_edges_graph() -> Graph<(), StyledEdgeData> {
    let mut graph = Graph::new();

    // Create nodes arranged in different patterns
    let nodes = vec![
        // Top row - input nodes
        Node::simple("input1", Position::new(100.0, 100.0)),
        Node::simple("input2", Position::new(350.0, 100.0)),
        Node::simple("input3", Position::new(600.0, 100.0)),
        Node::simple("input4", Position::new(850.0, 100.0)),

        // Middle row - processing nodes
        Node::simple("process1", Position::new(100.0, 300.0)),
        Node::simple("process2", Position::new(350.0, 300.0)),
        Node::simple("process3", Position::new(600.0, 300.0)),
        Node::simple("process4", Position::new(850.0, 300.0)),

        // Bottom row - output nodes
        Node::simple("output1", Position::new(225.0, 500.0)),
        Node::simple("output2", Position::new(575.0, 500.0)),
        Node::simple("output3", Position::new(725.0, 500.0)),
    ];

    // Add nodes to the graph
    for node in nodes {
        graph.add_node(node).expect("Failed to add node");
    }

    // Create edges with different styles
    let edges = vec![
        // Straight edge - basic connection
        Edge {
            id: "straight".to_string(),
            source: "input1".to_string(),
            target: "process1".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Straight,
                "#4299e1", // Blue
                2.0,
                Some("Straight"),
                false,
            ),
        },

        // Bezier edge - smooth curved connection
        Edge {
            id: "bezier".to_string(),
            source: "input2".to_string(),
            target: "process2".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Bezier,
                "#48bb78", // Green
                3.0,
                Some("Bezier"),
                false,
            ),
        },

        // Step edge - angular connection
        Edge {
            id: "step".to_string(),
            source: "input3".to_string(),
            target: "process3".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Step,
                "#ed8936", // Orange
                2.5,
                Some("Step"),
                false,
            ),
        },

        // Smooth step edge - rounded angular
        Edge {
            id: "smooth-step".to_string(),
            source: "input4".to_string(),
            target: "process4".to_string(),
            data: StyledEdgeData::new(
                EdgeType::SmoothStep,
                "#9f7aea", // Purple
                2.0,
                Some("Smooth Step"),
                false,
            ),
        },

        // Dashed edge - conditional flow
        Edge {
            id: "dashed".to_string(),
            source: "process1".to_string(),
            target: "output1".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Bezier,
                "#f56565", // Red
                2.0,
                Some("Dashed"),
                false,
            )
            .with_dash_pattern(vec![10.0, 5.0]),
        },

        // Thick edge - main data flow
        Edge {
            id: "thick".to_string(),
            source: "process2".to_string(),
            target: "output2".to_string(),
            data: StyledEdgeData::new(
                EdgeType::SmoothStep,
                "#38b2ac", // Teal
                5.0,
                Some("Thick"),
                false,
            ),
        },

        // Animated edge - active processing
        Edge {
            id: "animated".to_string(),
            source: "process3".to_string(),
            target: "output3".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Bezier,
                "#e53e3e", // Red
                3.0,
                Some("Animated"),
                true,
            ),
        },

        // Multiple connections to same node
        Edge {
            id: "multi1".to_string(),
            source: "process4".to_string(),
            target: "output2".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Step,
                "#805ad5", // Purple
                1.5,
                Some("Multi-1"),
                false,
            ),
        },

        Edge {
            id: "multi2".to_string(),
            source: "process4".to_string(),
            target: "output3".to_string(),
            data: StyledEdgeData::new(
                EdgeType::Straight,
                "#d53f8c", // Pink
                1.5,
                Some("Multi-2"),
                false,
            ),
        },
    ];

    // Add edges to the graph
    for edge in edges {
        graph.add_edge(edge).expect("Failed to add edge");
    }

    graph
}

/// Render the graph with custom edge styling
fn render_styled_edges_graph(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<(), StyledEdgeData>,
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
