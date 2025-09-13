//! Minimap component tests
//!
//! Tests for the minimap functionality including rendering, interaction,
//! viewport synchronization, and configuration.

use wasm_bindgen_test::*;
use flow_core::{Graph, Node, Edge, NodeId, EdgeId, Position, Size, Viewport};
use flow_core::prelude::*;

/// Create a test graph with nodes for minimap testing
fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Add nodes with different sizes and positions
    graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
    graph.add_node(Node::simple("node2", Position::new(300.0, 150.0))).unwrap();
    graph.add_node(Node::simple("node3", Position::new(500.0, 200.0))).unwrap();
    graph.add_node(Node::simple("node4", Position::new(700.0, 100.0))).unwrap();

    // Add some edges
    graph.add_edge(Edge::new("edge1", "node1", "node2", ())).unwrap();
    graph.add_edge(Edge::new("edge2", "node2", "node3", ())).unwrap();
    graph.add_edge(Edge::new("edge3", "node3", "node4", ())).unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_minimap_rendering_basic() {
    // Test: Minimap should render nodes and edges correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

    // Test that minimap can render the graph
    let result = renderer.render_minimap(&graph, &viewport);
    assert!(result.is_ok());

    // Test that all nodes are rendered
    let rendered_nodes = renderer.get_rendered_nodes();
    assert_eq!(rendered_nodes.len(), 4);

    // Test that all edges are rendered
    let rendered_edges = renderer.get_rendered_edges();
    assert_eq!(rendered_edges.len(), 3);
}

#[wasm_bindgen_test]
fn test_minimap_viewport_sync() {
    // Test: Minimap should sync with main viewport
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();

    // Test initial viewport
    let viewport1 = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);
    renderer.update_viewport(&viewport1);

    // Test viewport change
    let viewport2 = Viewport::new(100.0, 50.0, 800.0, 600.0, 1.5);
    renderer.update_viewport(&viewport2);

    // Verify viewport was updated
    let current_viewport = renderer.get_current_viewport();
    assert_eq!(current_viewport.x, viewport2.x);
    assert_eq!(current_viewport.y, viewport2.y);
    assert_eq!(current_viewport.zoom, viewport2.zoom);
}

#[wasm_bindgen_test]
fn test_minimap_click_navigation() {
    // Test: Clicking on minimap should navigate main viewport
    use crate::minimap::{MinimapRenderer, MinimapConfig, MinimapInteraction};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

    // Render minimap
    renderer.render_minimap(&graph, &viewport).unwrap();

    // Test click interaction
    let click_pos = Position::new(150.0, 125.0); // Click on node1 area
    let interaction = MinimapInteraction::new();
    let result = interaction.handle_click(&renderer, click_pos);

    assert!(result.is_ok());

    // Verify navigation occurred
    let new_viewport = result.unwrap();
    assert!(new_viewport.x > 0.0); // Should have moved
}

#[wasm_bindgen_test]
fn test_minimap_zoom_handling() {
    // Test: Minimap should handle different zoom levels correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();

    // Test with different zoom levels
    let viewports = vec![
        Viewport::new(0.0, 0.0, 800.0, 600.0, 0.5),  // Zoomed out
        Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0),  // Normal
        Viewport::new(0.0, 0.0, 800.0, 600.0, 2.0),  // Zoomed in
    ];

    for viewport in viewports {
        let result = renderer.render_minimap(&graph, &viewport);
        assert!(result.is_ok());

        // Verify zoom level is handled correctly
        let rendered_viewport = renderer.get_rendered_viewport();
        assert_eq!(rendered_viewport.zoom, viewport.zoom);
    }
}

#[wasm_bindgen_test]
fn test_minimap_bounds_calculation() {
    // Test: Minimap should calculate correct bounds for the graph
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();

    // Calculate bounds
    let bounds = renderer.calculate_graph_bounds(&graph);

    // Verify bounds include all nodes
    assert!(bounds.min_x <= 100.0); // node1 position
    assert!(bounds.max_x >= 700.0); // node4 position
    assert!(bounds.min_y <= 100.0); // node1 position
    assert!(bounds.max_y >= 200.0); // node3 position
}

#[wasm_bindgen_test]
fn test_minimap_scale_calculation() {
    // Test: Minimap should calculate correct scale for fitting graph
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig {
        width: 200.0,
        height: 150.0,
        ..Default::default()
    };
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();

    // Calculate scale
    let scale = renderer.calculate_scale(&graph);

    // Verify scale is reasonable (should fit graph in minimap)
    assert!(scale > 0.0);
    assert!(scale <= 1.0); // Should not be larger than 1:1
}

#[wasm_bindgen_test]
fn test_minimap_node_visibility() {
    // Test: Minimap should handle node visibility correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let mut graph = create_test_graph();

    // Hide a node
    if let Some(node) = graph.get_node_mut(&NodeId::new("node2")) {
        node.hidden = true;
    }

    // Render minimap
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);
    let result = renderer.render_minimap(&graph, &viewport);
    assert!(result.is_ok());

    // Verify only visible nodes are rendered
    let rendered_nodes = renderer.get_rendered_nodes();
    assert_eq!(rendered_nodes.len(), 3); // Should be 3 visible nodes
}

#[wasm_bindgen_test]
fn test_minimap_edge_visibility() {
    // Test: Minimap should handle edge visibility correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let mut graph = create_test_graph();

    // Hide an edge
    if let Some(edge) = graph.get_edge_mut(&EdgeId::new("edge1")) {
        edge.hidden = true;
    }

    // Render minimap
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);
    let result = renderer.render_minimap(&graph, &viewport);
    assert!(result.is_ok());

    // Verify only visible edges are rendered
    let rendered_edges = renderer.get_rendered_edges();
    assert_eq!(rendered_edges.len(), 2); // Should be 2 visible edges
}

#[wasm_bindgen_test]
fn test_minimap_configuration() {
    // Test: Minimap should respect configuration settings
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig {
        width: 300.0,
        height: 200.0,
        background_color: "rgba(0, 0, 0, 0.1)".to_string(),
        node_color: "blue".to_string(),
        edge_color: "gray".to_string(),
        viewport_color: "red".to_string(),
        show_grid: true,
        grid_size: 50.0,
    };

    let renderer = MinimapRenderer::new(config.clone());

    // Verify configuration is applied
    assert_eq!(renderer.get_config().width, 300.0);
    assert_eq!(renderer.get_config().height, 200.0);
    assert_eq!(renderer.get_config().background_color, "rgba(0, 0, 0, 0.1)");
    assert_eq!(renderer.get_config().node_color, "blue");
    assert_eq!(renderer.get_config().edge_color, "gray");
    assert_eq!(renderer.get_config().viewport_color, "red");
    assert!(renderer.get_config().show_grid);
    assert_eq!(renderer.get_config().grid_size, 50.0);
}

#[wasm_bindgen_test]
fn test_minimap_performance() {
    // Test: Minimap should handle large graphs efficiently
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let mut graph = Graph::new();

    // Add many nodes
    for i in 0..100 {
        let node_id = format!("node{}", i);
        let x = (i % 10) as f64 * 100.0;
        let y = (i / 10) as f64 * 100.0;
        graph.add_node(Node::simple(node_id, Position::new(x, y))).unwrap();
    }

    // Add many edges
    for i in 0..99 {
        let edge_id = format!("edge{}", i);
        let source = format!("node{}", i);
        let target = format!("node{}", i + 1);
        graph.add_edge(Edge::new(edge_id, source, target, ())).unwrap();
    }

    // Test rendering performance
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);
    let start = std::time::Instant::now();
    let result = renderer.render_minimap(&graph, &viewport);
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration.as_millis() < 100); // Should render quickly
}

#[wasm_bindgen_test]
fn test_minimap_viewport_rectangle() {
    // Test: Minimap should render viewport rectangle correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig::default();
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();
    let viewport = Viewport::new(100.0, 100.0, 800.0, 600.0, 1.5);

    // Render minimap
    let result = renderer.render_minimap(&graph, &viewport);
    assert!(result.is_ok());

    // Get viewport rectangle
    let viewport_rect = renderer.get_viewport_rectangle();

    // Verify viewport rectangle properties
    assert!(viewport_rect.x >= 0.0);
    assert!(viewport_rect.y >= 0.0);
    assert!(viewport_rect.width > 0.0);
    assert!(viewport_rect.height > 0.0);
}

#[wasm_bindgen_test]
fn test_minimap_coordinate_conversion() {
    // Test: Minimap should convert coordinates correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig {
        width: 200.0,
        height: 150.0,
        ..Default::default()
    };
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();

    // Test world to minimap coordinate conversion
    let world_pos = Position::new(300.0, 150.0);
    let minimap_pos = renderer.world_to_minimap(world_pos, &graph);

    // Verify conversion is within minimap bounds
    assert!(minimap_pos.x >= 0.0);
    assert!(minimap_pos.x <= 200.0);
    assert!(minimap_pos.y >= 0.0);
    assert!(minimap_pos.y <= 150.0);

    // Test minimap to world coordinate conversion
    let minimap_pos2 = Position::new(100.0, 75.0);
    let world_pos2 = renderer.minimap_to_world(minimap_pos2, &graph);

    // Verify conversion is reasonable
    assert!(world_pos2.x >= 0.0);
    assert!(world_pos2.y >= 0.0);
}

#[wasm_bindgen_test]
fn test_minimap_resize_handling() {
    // Test: Minimap should handle resize events correctly
    use crate::minimap::{MinimapRenderer, MinimapConfig};

    let config = MinimapConfig {
        width: 200.0,
        height: 150.0,
        ..Default::default()
    };
    let mut renderer = MinimapRenderer::new(config);
    let graph = create_test_graph();

    // Resize minimap
    let new_config = MinimapConfig {
        width: 300.0,
        height: 200.0,
        ..Default::default()
    };
    renderer.resize(new_config);

    // Verify resize was applied
    assert_eq!(renderer.get_config().width, 300.0);
    assert_eq!(renderer.get_config().height, 200.0);

    // Test that rendering still works after resize
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);
    let result = renderer.render_minimap(&graph, &viewport);
    assert!(result.is_ok());
}
