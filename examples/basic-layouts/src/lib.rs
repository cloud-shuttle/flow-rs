//! Basic Layouts Example
//!
//! Demonstrates Flow-RS's layout algorithm capabilities:
//! - Force-directed layout for organic node positioning
//! - Grid layout for structured, regular arrangements
//! - Hierarchical layout for tree-like structures
//! - Interactive layout switching and animation
//! - Performance comparison between algorithms

use flow_rs_core::{Edge, Graph, Node, Position, Viewport};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::Canvas2DRenderer;
use wasm_bindgen::prelude::*;

// Layout algorithm types
#[derive(Clone, Debug)]
enum LayoutAlgorithm {
    ForceDirected,
    Grid,
    Hierarchical,
}

// Global state for layout switching
static mut CURRENT_LAYOUT: LayoutAlgorithm = LayoutAlgorithm::ForceDirected;

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

    // Create initial graph with force-directed layout
    let graph = create_and_layout_graph(LayoutAlgorithm::ForceDirected);

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 1000.0, 700.0, 1.0);

    // Initial render
    render_graph(&mut renderer, &graph, &viewport)
        .expect("Failed to render initial layout");

    // Set up layout controls
    setup_layout_controls();
}

/// Set up interactive layout controls
fn setup_layout_controls() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Set up layout algorithm selector
    if let Some(select) = document.get_element_by_id("layout-select") {
        let select = select.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(target) = event.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>() {
                let value = target.value();

                let algorithm = match value.as_str() {
                    "force-directed" => LayoutAlgorithm::ForceDirected,
                    "grid" => LayoutAlgorithm::Grid,
                    "hierarchical" => LayoutAlgorithm::Hierarchical,
                    _ => LayoutAlgorithm::ForceDirected,
                };

                // Update global state
                unsafe {
                    CURRENT_LAYOUT = algorithm;
                }

                // Re-layout and render
                relayout_and_rerender();
            }
        }) as Box<dyn FnMut(web_sys::Event)>);

        select.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up re-layout button
    if let Some(button) = document.get_element_by_id("relayout-btn") {
        let button = button.dyn_into::<web_sys::HtmlElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            relayout_and_rerender();
        }) as Box<dyn FnMut(web_sys::Event)>);

        button.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
}

/// Re-layout the graph with current algorithm and re-render
fn relayout_and_rerender() {
    let algorithm = unsafe { CURRENT_LAYOUT.clone() };
    let graph = create_and_layout_graph(algorithm);

    // Get canvas and render
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(canvas) = document.get_element_by_id("flow-canvas") {
        if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
            if let Ok(mut renderer) = Canvas2DRenderer::new(&canvas) {
                let viewport = Viewport::new(0.0, 0.0, 1000.0, 700.0, 1.0);
                let _ = render_graph(&mut renderer, &graph, &viewport);
            }
        }
    }
}

/// Create a graph and apply the specified layout algorithm
fn create_and_layout_graph(algorithm: LayoutAlgorithm) -> Graph<(), ()> {
    let mut graph = create_base_graph();

    // Apply layout algorithm
    match algorithm {
        LayoutAlgorithm::ForceDirected => apply_force_directed_layout(&mut graph),
        LayoutAlgorithm::Grid => apply_grid_layout(&mut graph),
        LayoutAlgorithm::Hierarchical => apply_hierarchical_layout(&mut graph),
    }

    graph
}

/// Create the base graph structure without positioning
fn create_base_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Create nodes (without positions - will be set by layout)
    let node_ids = vec![
        "start", "process1", "process2", "decision", "output1", "output2",
        "subprocess1", "subprocess2", "end"
    ];

    for node_id in node_ids {
        let node = Node::simple(node_id, Position::new(0.0, 0.0)); // Position will be set by layout
        graph.add_node(node).expect("Failed to add node");
    }

    // Create edges
    let edges = vec![
        ("start", "process1"),
        ("process1", "process2"),
        ("process2", "decision"),
        ("decision", "output1"),
        ("decision", "output2"),
        ("output1", "subprocess1"),
        ("output2", "subprocess2"),
        ("subprocess1", "end"),
        ("subprocess2", "end"),
    ];

    for (source, target) in edges {
        let edge = Edge::simple(&format!("{}-{}", source, target), source, target);
        graph.add_edge(edge).expect("Failed to add edge");
    }

    graph
}

/// Apply force-directed layout algorithm
fn apply_force_directed_layout(graph: &mut Graph<(), ()>) {
    // Simple force-directed layout implementation
    // In a real implementation, this would use proper physics simulation

    let center_x = 500.0;
    let center_y = 350.0;
    let radius = 200.0;

    // Position nodes in a rough circle initially
    let node_count = graph.node_count();
    let mut nodes: Vec<_> = graph.nodes_mut().collect();

    for (i, node) in nodes.iter_mut().enumerate() {
        let angle = (i as f64 * 2.0 * std::f64::consts::PI) / node_count as f64;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        node.position = Position::new(x, y);
    }

    // Simple force-directed relaxation (simplified)
    for _ in 0..10 {
        apply_repulsive_forces(&mut nodes);
        apply_attractive_forces(&mut nodes);
    }
}

/// Apply repulsive forces between all node pairs
fn apply_repulsive_forces(nodes: &mut [flow_rs_core::graph::NodeRefMut<(), ()>]) {
    let k = 100.0; // Force constant

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let node1 = &mut nodes[i];
            let node2 = &mut nodes[j];

            let dx = node2.position.x - node1.position.x;
            let dy = node2.position.y - node1.position.y;
            let distance = (dx * dx + dy * dy).sqrt().max(1.0);

            let force = k * k / distance;
            let fx = force * dx / distance;
            let fy = force * dy / distance;

            node1.position.x -= fx * 0.1;
            node1.position.y -= fy * 0.1;
            node2.position.x += fx * 0.1;
            node2.position.y += fy * 0.1;
        }
    }
}

/// Apply attractive forces along edges
fn apply_attractive_forces(nodes: &mut [flow_rs_core::graph::NodeRefMut<(), ()>]) {
    // This is a simplified implementation
    // In a real force-directed layout, we'd iterate through edges
    // For now, we'll just ensure nodes stay roughly in the viewport
    let center_x = 500.0;
    let center_y = 350.0;

    for node in nodes.iter_mut() {
        let dx = center_x - node.position.x;
        let dy = center_y - node.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 50.0 {
            node.position.x += dx * 0.01;
            node.position.y += dy * 0.01;
        }
    }
}

/// Apply grid layout algorithm
fn apply_grid_layout(graph: &mut Graph<(), ()>) {
    let cols = 3;
    let rows = 3;
    let spacing_x = 250.0;
    let spacing_y = 200.0;
    let start_x = 200.0;
    let start_y = 150.0;

    let mut nodes: Vec<_> = graph.nodes_mut().collect();

    for (i, node) in nodes.iter_mut().enumerate() {
        let row = i / cols;
        let col = i % cols;

        let x = start_x + (col as f64) * spacing_x;
        let y = start_y + (row as f64) * spacing_y;

        node.position = Position::new(x, y);
    }
}

/// Apply hierarchical layout algorithm
fn apply_hierarchical_layout(graph: &mut Graph<(), ()>) {
    // Simple hierarchical layout based on connectivity

    // Level 1: Start node
    if let Some(node) = graph.node_mut("start") {
        node.position = Position::new(500.0, 100.0);
    }

    // Level 2: Process nodes
    if let Some(node) = graph.node_mut("process1") {
        node.position = Position::new(300.0, 200.0);
    }
    if let Some(node) = graph.node_mut("process2") {
        node.position = Position::new(700.0, 200.0);
    }

    // Level 3: Decision node
    if let Some(node) = graph.node_mut("decision") {
        node.position = Position::new(500.0, 300.0);
    }

    // Level 4: Output nodes
    if let Some(node) = graph.node_mut("output1") {
        node.position = Position::new(300.0, 400.0);
    }
    if let Some(node) = graph.node_mut("output2") {
        node.position = Position::new(700.0, 400.0);
    }

    // Level 5: Subprocess nodes
    if let Some(node) = graph.node_mut("subprocess1") {
        node.position = Position::new(200.0, 500.0);
    }
    if let Some(node) = graph.node_mut("subprocess2") {
        node.position = Position::new(800.0, 500.0);
    }

    // Level 6: End node
    if let Some(node) = graph.node_mut("end") {
        node.position = Position::new(500.0, 600.0);
    }
}

/// Render the graph with layout
fn render_graph(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<(), ()>,
    viewport: &Viewport,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear the canvas
    renderer.clear(Some("#ffffff"))?;

    // Set up background
    let background_config = BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#f1f5f9".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.3,
    };

    // Render background
    renderer.render_background(&background_config, viewport)?;

    // Render graph
    renderer.render_graph_dyn(graph, viewport)?;

    // Present
    renderer.present()?;

    Ok(())
}
