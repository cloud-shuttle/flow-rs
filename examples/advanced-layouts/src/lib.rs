//! Advanced Layouts Example
//!
//! Demonstrates sophisticated layout algorithms and advanced graph layout techniques.
//! This example showcases Flow-RS's advanced layout capabilities including constrained layouts,
//! custom algorithms, performance optimizations, and interactive layout controls.
//!
//! Advanced Features:
//! - Constrained force-directed layouts with pinning and grouping
//! - Custom layout algorithms (circular, radial, layered)
//! - Layout performance optimization and benchmarking
//! - Interactive layout controls and real-time adjustment
//! - Layout algorithm comparison and visualization
//! - Advanced layout constraints (alignment, spacing, boundaries)

use flow_rs_core::{Graph, Node, Position};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Layout algorithm types
#[derive(Clone, Debug, Serialize, Deserialize)]
enum AdvancedLayoutAlgorithm {
    ConstrainedForceDirected,
    Circular,
    Radial,
    Layered,
    CustomSpiral,
    ConstrainedGrid,
}

// Layout constraints
#[derive(Clone, Debug, Serialize, Deserialize)]
struct LayoutConstraints {
    pub pinned_nodes: Vec<String>, // Node IDs that shouldn't move
    pub grouped_nodes: Vec<Vec<String>>, // Groups that should stay together
    pub alignment_rules: Vec<AlignmentRule>,
    pub spacing_rules: Vec<SpacingRule>,
    pub boundary_constraints: Vec<BoundaryConstraint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AlignmentRule {
    pub nodes: Vec<String>,
    pub axis: Axis, // Horizontal or Vertical
    pub alignment: AlignmentType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Axis { Horizontal, Vertical }

#[derive(Clone, Debug, Serialize, Deserialize)]
enum AlignmentType { Left, Center, Right, Top, Middle, Bottom }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpacingRule {
    pub node_pairs: Vec<(String, String)>,
    pub min_distance: f64,
    pub max_distance: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BoundaryConstraint {
    pub nodes: Vec<String>,
    pub bounds: Rectangle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// Layout performance metrics
#[derive(Clone, Debug, Serialize, Deserialize)]
struct LayoutPerformance {
    pub algorithm: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub computation_time_ms: f64,
    pub iterations: u32,
    pub convergence_achieved: bool,
    pub final_energy: f64,
}

// Global state
static mut CURRENT_GRAPH: Option<Graph<String, String>> = None;
static mut CURRENT_LAYOUT: Option<AdvancedLayoutAlgorithm> = None;
static mut LAYOUT_CONSTRAINTS: Option<LayoutConstraints> = None;
static mut PERFORMANCE_HISTORY: Option<Vec<LayoutPerformance>> = None;

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"🎯 Flow-RS Advanced Layouts Demo Started!".into());

    // Initialize with a complex demo graph
    initialize_demo_graph();

    // Demonstrate advanced layouts
    demonstrate_advanced_layouts();

    web_sys::console::log_1(&"🔬 Advanced Layout Algorithms Available!".into());
    web_sys::console::log_1(&"🎛️ Try these commands:".into());
    web_sys::console::log_1(&"   • run_layout('constrained_force_directed') - Run constrained force-directed layout".into());
    web_sys::console::log_1(&"   • run_layout('circular') - Run circular layout".into());
    web_sys::console::log_1(&"   • run_layout('radial') - Run radial layout".into());
    web_sys::console::log_1(&"   • add_constraint('pin', 'node-1') - Pin a node in place".into());
    web_sys::console::log_1(&"   • compare_layouts() - Compare all layout algorithms".into());
    web_sys::console::log_1(&"   • benchmark_layout('constrained_force_directed', 1000) - Benchmark layout performance".into());
}

/// Initialize a complex demo graph for advanced layout testing
fn initialize_demo_graph() {
    unsafe {
        let mut graph: Graph<String, String> = Graph::new();

        // Create a complex graph structure with multiple components
        let nodes_data = vec![
            ("center", "Central Hub", Position::new(400.0, 300.0)),
            ("satellite-1", "Satellite 1", Position::new(200.0, 200.0)),
            ("satellite-2", "Satellite 2", Position::new(600.0, 200.0)),
            ("satellite-3", "Satellite 3", Position::new(200.0, 400.0)),
            ("satellite-4", "Satellite 4", Position::new(600.0, 400.0)),
            ("cluster-a-1", "Cluster A Node 1", Position::new(100.0, 100.0)),
            ("cluster-a-2", "Cluster A Node 2", Position::new(150.0, 100.0)),
            ("cluster-a-3", "Cluster A Node 3", Position::new(100.0, 150.0)),
            ("cluster-b-1", "Cluster B Node 1", Position::new(650.0, 100.0)),
            ("cluster-b-2", "Cluster B Node 2", Position::new(700.0, 100.0)),
            ("cluster-b-3", "Cluster B Node 3", Position::new(650.0, 150.0)),
            ("peripheral-1", "Peripheral 1", Position::new(50.0, 250.0)),
            ("peripheral-2", "Peripheral 2", Position::new(750.0, 250.0)),
            ("peripheral-3", "Peripheral 3", Position::new(50.0, 350.0)),
            ("peripheral-4", "Peripheral 4", Position::new(750.0, 350.0)),
        ];

        for (id, label, pos) in nodes_data {
            let node = Node::new(id.to_string(), pos, label.to_string());
            graph.add_node(node).unwrap();
        }

        // Create edges forming a complex network
        let edges_data = vec![
            ("center", "satellite-1"),
            ("center", "satellite-2"),
            ("center", "satellite-3"),
            ("center", "satellite-4"),
            ("satellite-1", "cluster-a-1"),
            ("satellite-1", "cluster-a-2"),
            ("satellite-1", "cluster-a-3"),
            ("satellite-2", "cluster-b-1"),
            ("satellite-2", "cluster-b-2"),
            ("satellite-2", "cluster-b-3"),
            ("cluster-a-1", "cluster-a-2"),
            ("cluster-a-2", "cluster-a-3"),
            ("cluster-a-1", "cluster-a-3"),
            ("cluster-b-1", "cluster-b-2"),
            ("cluster-b-2", "cluster-b-3"),
            ("cluster-b-1", "cluster-b-3"),
            ("satellite-1", "peripheral-1"),
            ("satellite-1", "peripheral-3"),
            ("satellite-2", "peripheral-2"),
            ("satellite-2", "peripheral-4"),
            ("satellite-3", "peripheral-1"),
            ("satellite-3", "peripheral-3"),
            ("satellite-4", "peripheral-2"),
            ("satellite-4", "peripheral-4"),
        ];

        for (source, target) in edges_data {
            use flow_rs_core::Edge;
            let edge = Edge::new("edge".to_string(), source.to_string(), target.to_string(), String::new());
            graph.add_edge(edge).unwrap();
        }

        // Initialize constraints
        let constraints = LayoutConstraints {
            pinned_nodes: vec!["center".to_string()],
            grouped_nodes: vec![
                vec!["cluster-a-1".to_string(), "cluster-a-2".to_string(), "cluster-a-3".to_string()],
                vec!["cluster-b-1".to_string(), "cluster-b-2".to_string(), "cluster-b-3".to_string()],
            ],
            alignment_rules: vec![],
            spacing_rules: vec![],
            boundary_constraints: vec![],
        };

        CURRENT_GRAPH = Some(graph);
        CURRENT_LAYOUT = Some(AdvancedLayoutAlgorithm::ConstrainedForceDirected);
        LAYOUT_CONSTRAINTS = Some(constraints);
        PERFORMANCE_HISTORY = Some(Vec::new());

        web_sys::console::log_1(&"📊 Complex demo graph created with 14 nodes and 24 edges!".into());
    }
}

/// Demonstrate advanced layout algorithms
fn demonstrate_advanced_layouts() {
    unsafe {
        if let Some(ref graph) = CURRENT_GRAPH {
            web_sys::console::log_1(&format!("🎯 Graph loaded: {} nodes, {} edges", graph.nodes().count(), graph.edges().count()).into());
            web_sys::console::log_1(&"🔧 Available layout algorithms:".into());
            web_sys::console::log_1(&"   • constrained_force_directed - Force-directed with constraints".into());
            web_sys::console::log_1(&"   • circular - Circular node arrangement".into());
            web_sys::console::log_1(&"   • radial - Radial node arrangement from center".into());
            web_sys::console::log_1(&"   • layered - Hierarchical layered layout".into());
            web_sys::console::log_1(&"   • custom_spiral - Custom spiral arrangement".into());
            web_sys::console::log_1(&"   • constrained_grid - Grid with constraint awareness".into());
        }
    }
}

// Public API functions
#[wasm_bindgen]
pub fn run_layout(algorithm: &str) -> String {
    let start_time = instant();

    unsafe {
        if let (Some(ref mut graph), Some(ref constraints)) = (CURRENT_GRAPH.as_mut(), LAYOUT_CONSTRAINTS.as_ref()) {
            let layout_type = match algorithm {
                "constrained_force_directed" => AdvancedLayoutAlgorithm::ConstrainedForceDirected,
                "circular" => AdvancedLayoutAlgorithm::Circular,
                "radial" => AdvancedLayoutAlgorithm::Radial,
                "layered" => AdvancedLayoutAlgorithm::Layered,
                "custom_spiral" => AdvancedLayoutAlgorithm::CustomSpiral,
                "constrained_grid" => AdvancedLayoutAlgorithm::ConstrainedGrid,
                _ => {
                    return format!("❌ Unknown algorithm: {}", algorithm);
                }
            };

            // Apply the layout
            let _result = apply_advanced_layout(graph, &layout_type, constraints);

            let end_time = instant();
            let computation_time = end_time - start_time;

            // Record performance
            if let Some(ref mut history) = PERFORMANCE_HISTORY.as_mut() {
                let perf = LayoutPerformance {
                    algorithm: algorithm.to_string(),
                    node_count: graph.nodes().count(),
                    edge_count: graph.edges().count(),
                    computation_time_ms: computation_time,
                    iterations: 100, // Placeholder
                    convergence_achieved: true, // Placeholder
                    final_energy: 0.0, // Placeholder
                };
                history.push(perf);
            }

            CURRENT_LAYOUT = Some(layout_type);

            format!("✅ Applied {} layout in {:.2}ms", algorithm, computation_time)
        } else {
            "❌ No graph loaded".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn add_constraint(constraint_type: &str, target: &str) -> String {
    unsafe {
        if let Some(ref mut constraints) = LAYOUT_CONSTRAINTS.as_mut() {
            match constraint_type {
                "pin" => {
                    if !constraints.pinned_nodes.contains(&target.to_string()) {
                        constraints.pinned_nodes.push(target.to_string());
                        format!("📌 Pinned node: {}", target)
                    } else {
                        format!("⚠️ Node {} already pinned", target)
                    }
                }
                "unpin" => {
                    constraints.pinned_nodes.retain(|id| id != target);
                    format!("📍 Unpinned node: {}", target)
                }
                _ => format!("❌ Unknown constraint type: {}", constraint_type)
            }
        } else {
            "❌ No constraints loaded".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn create_group(node_ids: &str) -> String {
    unsafe {
        if let Some(ref mut constraints) = LAYOUT_CONSTRAINTS.as_mut() {
            let ids: Vec<String> = node_ids.split(',').map(|s| s.trim().to_string()).collect();

            // Check if any of these nodes are already in a group
            let already_grouped = constraints.grouped_nodes.iter()
                .any(|group| ids.iter().any(|id| group.contains(id)));

            if already_grouped {
                "⚠️ Some nodes are already in groups".to_string()
            } else {
                constraints.grouped_nodes.push(ids.clone());
                format!("👥 Created group with {} nodes", ids.len())
            }
        } else {
            "❌ No constraints loaded".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn compare_layouts() -> String {
    unsafe {
        if let Some(ref history) = PERFORMANCE_HISTORY.as_ref() {
            if history.is_empty() {
                return "📊 No layout performance data available. Run some layouts first.".to_string();
            }

            let mut result = "📊 Layout Performance Comparison:\n".to_string();

            for perf in history.iter() {
                result.push_str(&format!(
                    "🎯 {}: {:.2}ms ({} nodes, {} edges)\n",
                    perf.algorithm, perf.computation_time_ms, perf.node_count, perf.edge_count
                ));
            }

            // Calculate averages
            let total_time: f64 = history.iter().map(|p| p.computation_time_ms).sum();
            let avg_time = total_time / history.len() as f64;

            result.push_str(&format!("📈 Average time: {:.2}ms across {} layouts", avg_time, history.len()));

            result
        } else {
            "❌ No performance history available".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn benchmark_layout(_algorithm: &str, iterations: usize) -> String {
    let mut total_time = 0.0;

    unsafe {
        if let (Some(graph), Some(constraints)) = (CURRENT_GRAPH.as_ref(), LAYOUT_CONSTRAINTS.as_ref()) {
            for i in 0..iterations {
                let start_time = instant();

                // Create a copy for benchmarking
                let mut test_graph = graph.clone();

                // Apply layout (simplified for benchmarking)
                let _ = apply_advanced_layout(&mut test_graph, &AdvancedLayoutAlgorithm::ConstrainedForceDirected, constraints);

                let end_time = instant();
                total_time += end_time - start_time;

                if i % 100 == 0 {
                    web_sys::console::log_1(&format!("🏃 Benchmark progress: {}/{}", i, iterations).into());
                }
            }

            let avg_time = total_time / iterations as f64;
            format!("🏃 Benchmark complete: {} iterations, average {:.3}ms per layout", iterations, avg_time)
        } else {
            "❌ No graph available for benchmarking".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn get_layout_info() -> String {
    unsafe {
        if let (Some(ref graph), Some(layout), Some(ref constraints), Some(ref history)) =
            (CURRENT_GRAPH.as_ref(), CURRENT_LAYOUT.as_ref(), LAYOUT_CONSTRAINTS.as_ref(), PERFORMANCE_HISTORY.as_ref()) {

            format!("🎯 Current Layout: {:?}\n📊 Graph: {} nodes, {} edges\n📌 Pinned: {} nodes\n👥 Groups: {} groups\n📈 Performance runs: {}",
                layout,
                graph.nodes().count(),
                graph.edges().count(),
                constraints.pinned_nodes.len(),
                constraints.grouped_nodes.len(),
                history.len()
            )
        } else {
            "❌ No layout information available".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn export_layout_data() -> String {
    unsafe {
        if let (Some(ref graph), Some(ref history)) = (CURRENT_GRAPH.as_ref(), PERFORMANCE_HISTORY.as_ref()) {
            let mut result = String::new();
            result.push_str("Flow-RS Advanced Layout Data Export\n");
            result.push_str("===================================\n\n");

            result.push_str(&format!("Graph Statistics:\n"));
            result.push_str(&format!("  Nodes: {}\n", graph.nodes().count()));
            result.push_str(&format!("  Edges: {}\n", graph.edges().count()));

            result.push_str(&format!("\nPerformance History ({} runs):\n", history.len()));
            for (i, perf) in history.iter().enumerate() {
                result.push_str(&format!("  {}. {} - {:.2}ms\n", i + 1, perf.algorithm, perf.computation_time_ms));
            }

            if let Some(ref constraints) = LAYOUT_CONSTRAINTS.as_ref() {
                result.push_str(&format!("\nLayout Constraints:\n"));
                result.push_str(&format!("  Pinned nodes: {}\n", constraints.pinned_nodes.len()));
                result.push_str(&format!("  Groups: {}\n", constraints.grouped_nodes.len()));
            }

            result
        } else {
            "❌ No data available for export".to_string()
        }
    }
}

/// Apply advanced layout algorithm to graph
fn apply_advanced_layout(
    graph: &mut Graph<String, String>,
    algorithm: &AdvancedLayoutAlgorithm,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    match algorithm {
        AdvancedLayoutAlgorithm::ConstrainedForceDirected => {
            apply_constrained_force_directed_layout(graph, constraints)
        }
        AdvancedLayoutAlgorithm::Circular => {
            apply_circular_layout(graph, constraints)
        }
        AdvancedLayoutAlgorithm::Radial => {
            apply_radial_layout(graph, constraints)
        }
        AdvancedLayoutAlgorithm::Layered => {
            apply_layered_layout(graph, constraints)
        }
        AdvancedLayoutAlgorithm::CustomSpiral => {
            apply_spiral_layout(graph, constraints)
        }
        AdvancedLayoutAlgorithm::ConstrainedGrid => {
            apply_constrained_grid_layout(graph, constraints)
        }
    }
}

fn apply_constrained_force_directed_layout(
    graph: &mut Graph<String, String>,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    // Create a basic force-directed layout with constraints
    let mut positions = HashMap::new();

    // Initialize positions randomly but respect pinned nodes
    for node in graph.nodes() {
        if constraints.pinned_nodes.contains(&node.id.to_string()) {
            positions.insert(node.id.clone(), node.position);
        } else {
            // Random position within bounds
            let x = 100.0 + (js_sys::Math::random() * 600.0) as f64;
            let y = 100.0 + (js_sys::Math::random() * 400.0) as f64;
            positions.insert(node.id.clone(), Position::new(x, y));
        }
    }

    // Simple force-directed iteration (simplified for demo)
    for _iteration in 0..50 {
        let mut forces = HashMap::new();

        // Calculate repulsive forces between all nodes
        for node1 in graph.nodes() {
            let mut force_x = 0.0;
            let mut force_y = 0.0;

            for node2 in graph.nodes() {
                if node1.id != node2.id {
                    let dx = positions[&node1.id].x - positions[&node2.id].x;
                    let dy = positions[&node1.id].y - positions[&node2.id].y;
                    let distance = (dx * dx + dy * dy).sqrt().max(1.0);

                    // Repulsive force
                    let force = 1000.0 / (distance * distance);
                    force_x += (dx / distance) * force;
                    force_y += (dy / distance) * force;
                }
            }

            // Attractive forces along edges
            for edge in graph.edges() {
                if edge.source == node1.id {
                    if let Some(target_pos) = positions.get(&edge.target) {
                        let dx = target_pos.x - positions[&node1.id].x;
                        let dy = target_pos.y - positions[&node1.id].y;
                        let distance = (dx * dx + dy * dy).sqrt().max(1.0);

                        let force = distance * 0.1; // Spring constant
                        force_x += (dx / distance) * force;
                        force_y += (dy / distance) * force;
                    }
                }
            }

            forces.insert(node1.id.clone(), (force_x, force_y));
        }

        // Apply forces (but don't move pinned nodes)
        for node in graph.nodes() {
            if !constraints.pinned_nodes.contains(&node.id.to_string()) {
                if let Some((fx, fy)) = forces.get(&node.id) {
                    let current_pos = positions.get_mut(&node.id).unwrap();
                    current_pos.x += fx * 0.01; // Damping factor
                    current_pos.y += fy * 0.01;
                }
            }
        }
    }

    // Update graph positions
    for node in graph.nodes_mut() {
        if let Some(new_pos) = positions.get(&node.id) {
            node.position = *new_pos;
        }
    }

    Ok(())
}

fn apply_circular_layout(
    graph: &mut Graph<String, String>,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    let node_count = graph.nodes().count() as f64;
    let center_x = 400.0;
    let center_y = 300.0;
    let radius = 200.0;

    let mut angle: f64 = 0.0;
    let angle_step = 2.0 * std::f64::consts::PI / node_count;

    for node in graph.nodes_mut() {
        if !constraints.pinned_nodes.contains(&node.id.to_string()) {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            node.position = Position::new(x, y);
            angle += angle_step;
        }
    }

    Ok(())
}

fn apply_radial_layout(
    graph: &mut Graph<String, String>,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    // Find center node (highest degree or first non-pinned)
    let center_node_id = if let Some(center_id) = constraints.pinned_nodes.first() {
        center_id.clone()
    } else {
        // Find node with most connections
        graph.nodes()
            .max_by_key(|node| {
                graph.edges().filter(|edge| edge.source == node.id || edge.target == node.id).count()
            })
            .map(|node| node.id.to_string())
            .unwrap_or_else(|| graph.nodes().next().unwrap().id.to_string())
    };

    // Position center node
    if let Some(center_node) = graph.nodes_mut().find(|n| n.id.to_string() == center_node_id) {
        center_node.position = Position::new(400.0, 300.0);
    }

    // Position other nodes radially based on distance from center
    let center_pos = Position::new(400.0, 300.0);

    for node in graph.nodes_mut() {
        if node.id.to_string() != center_node_id && !constraints.pinned_nodes.contains(&node.id.to_string()) {
            // Calculate angle based on node ID hash for deterministic placement
            let angle = (node.id.to_string().as_bytes().iter().map(|&b| b as u32).sum::<u32>() % 360) as f64 * std::f64::consts::PI / 180.0;
            let distance = 100.0 + (angle * 50.0).sin().abs() * 100.0;

            let x = center_pos.x + distance * angle.cos();
            let y = center_pos.y + distance * angle.sin();
            node.position = Position::new(x, y);
        }
    }

    Ok(())
}

fn apply_layered_layout(
    graph: &mut Graph<String, String>,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    // Simple hierarchical layout - assign layers based on connectivity
    let mut layers: Vec<Vec<String>> = Vec::new();

    // Start with nodes that have no incoming edges (or pinned nodes)
    let mut processed = std::collections::HashSet::new();
    let mut current_layer = Vec::new();

    // Add pinned nodes to first layer
    for pinned_id in &constraints.pinned_nodes {
        current_layer.push(pinned_id.clone());
        processed.insert(pinned_id.clone());
    }

    // Add nodes with no incoming edges
    for node in graph.nodes() {
        let has_incoming = graph.edges().any(|edge| edge.target == node.id);
        if !has_incoming && !processed.contains(&node.id.to_string()) {
            current_layer.push(node.id.to_string());
            processed.insert(node.id.to_string());
        }
    }

    layers.push(current_layer);

    // Continue with remaining nodes
    while processed.len() < graph.nodes().count() {
        let mut next_layer = Vec::new();

        for node in graph.nodes() {
            if !processed.contains(&node.id.to_string()) {
                // Check if all predecessors are in previous layers
                let predecessors_processed = graph.edges()
                    .filter(|edge| edge.target == node.id)
                    .all(|edge| processed.contains(&edge.source.to_string()));

                if predecessors_processed {
                    next_layer.push(node.id.to_string());
                }
            }
        }

        if next_layer.is_empty() {
            // Handle cycles by adding remaining nodes to current layer
            for node in graph.nodes() {
                if !processed.contains(&node.id.to_string()) {
                    next_layer.push(node.id.to_string());
                }
            }
        }

        for node_id in &next_layer {
            processed.insert(node_id.clone());
        }

        layers.push(next_layer);
    }

    // Position nodes in layers
    let layer_height = 100.0;
    let node_spacing = 120.0;

    for (layer_index, layer) in layers.iter().enumerate() {
        let y = 150.0 + layer_index as f64 * layer_height;
        let total_width = (layer.len() - 1) as f64 * node_spacing;
        let start_x = 400.0 - total_width / 2.0;

        for (node_index, node_id) in layer.iter().enumerate() {
            let x = start_x + node_index as f64 * node_spacing;

            if let Some(node) = graph.nodes_mut().find(|n| n.id.to_string() == *node_id) {
                if !constraints.pinned_nodes.contains(&node.id.to_string()) {
                    node.position = Position::new(x, y);
                }
            }
        }
    }

    Ok(())
}

fn apply_spiral_layout(
    graph: &mut Graph<String, String>,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    let center_x = 400.0;
    let center_y = 300.0;
    let mut angle: f64 = 0.0;
    let angle_step = 0.5;
    let radius_step = 5.0;
    let mut radius = 50.0;

    for node in graph.nodes_mut() {
        if !constraints.pinned_nodes.contains(&node.id.to_string()) {
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            node.position = Position::new(x, y);

            angle += angle_step;
            radius += radius_step;
        }
    }

    Ok(())
}

fn apply_constrained_grid_layout(
    graph: &mut Graph<String, String>,
    constraints: &LayoutConstraints
) -> Result<(), String> {
    let grid_size = 80.0;
    let start_x = 100.0;
    let start_y = 100.0;

    let mut grid_positions = Vec::new();
    let mut x = start_x;
    let mut y = start_y;

    // Create grid positions
    for _ in 0..graph.nodes().count() {
        grid_positions.push(Position::new(x, y));
        x += grid_size;
        if x > 700.0 {
            x = start_x;
            y += grid_size;
        }
    }

    // Assign positions respecting constraints
    let mut position_index = 0;
    for node in graph.nodes_mut() {
        if !constraints.pinned_nodes.contains(&node.id.to_string()) {
            if position_index < grid_positions.len() {
                node.position = grid_positions[position_index];
                position_index += 1;
            }
        }
    }

    Ok(())
}

// Helper function for high-precision timing
fn instant() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}
