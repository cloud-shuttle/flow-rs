//! Edge connection system tests
//!
//! Tests for the edge connection functionality including connection validation,
//! handle detection, edge creation, and connection management.

use crate::edge_connection::ConnectionResult;
use flow_rs_core::prelude::*;
use flow_rs_core::{Edge, EdgeId, Graph, Node, NodeId, Position, Size};
use wasm_bindgen_test::*;

/// Create a test graph with nodes for edge connection testing
fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Add nodes with different sizes and positions
    graph
        .add_node(Node::simple("node1", Position::new(100.0, 100.0)))
        .unwrap();
    graph
        .add_node(Node::simple("node2", Position::new(300.0, 150.0)))
        .unwrap();
    graph
        .add_node(Node::simple("node3", Position::new(500.0, 200.0)))
        .unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_connection_validation_basic() {
    // Test: Basic connection validation between two nodes
    use crate::edge_connection::{ConnectionResult, ConnectionValidator};

    let validator = ConnectionValidator::new();
    let graph = create_test_graph();

    let source_node = NodeId::new("node1");
    let target_node = NodeId::new("node2");

    // Test valid connection
    let result = validator.validate_connection(&graph, &source_node, &target_node);
    assert!(matches!(result, ConnectionResult::Valid));

    // Test self-connection (should be invalid)
    let result = validator.validate_connection(&graph, &source_node, &source_node);
    assert!(matches!(result, ConnectionResult::InvalidSelfConnection));
}

#[wasm_bindgen_test]
fn test_connection_validation_duplicate() {
    // Test: Connection validation should prevent duplicate edges
    use crate::edge_connection::{ConnectionResult, ConnectionValidator};

    let mut graph = create_test_graph();
    let validator = ConnectionValidator::new();

    let source_node = NodeId::new("node1");
    let target_node = NodeId::new("node2");

    // Add an existing edge
    graph
        .add_edge(Edge::new(
            "edge1",
            source_node.clone(),
            target_node.clone(),
            (),
        ))
        .unwrap();

    // Try to add duplicate edge
    let result = validator.validate_connection(&graph, &source_node, &target_node);
    assert!(matches!(result, ConnectionResult::DuplicateEdge));
}

#[wasm_bindgen_test]
fn test_connection_validation_circular() {
    // Test: Connection validation should prevent circular dependencies
    use crate::edge_connection::{ConnectionResult, ConnectionValidator};

    let mut graph = create_test_graph();
    let validator = ConnectionValidator::new();

    let node1 = NodeId::new("node1");
    let node2 = NodeId::new("node2");
    let node3 = NodeId::new("node3");

    // Create a chain: node1 -> node2 -> node3
    graph
        .add_edge(Edge::new("edge1", node1.clone(), node2.clone(), ()))
        .unwrap();
    graph
        .add_edge(Edge::new("edge2", node2.clone(), node3.clone(), ()))
        .unwrap();

    // Try to create circular connection: node3 -> node1
    let result = validator.validate_connection(&graph, &node3, &node1);
    assert!(matches!(result, ConnectionResult::CircularDependency));
}

#[wasm_bindgen_test]
fn test_handle_detection() {
    // Test: System should detect connection handles on nodes
    use crate::edge_connection::{ConnectionHandle, HandleDetector};

    let detector = HandleDetector::new();
    let graph = create_test_graph();

    let node_id = NodeId::new("node1");
    let node = graph.get_node(&node_id).unwrap();

    // Test detecting output handle (right side of node)
    let output_handle = detector.detect_handle(&node, Position::new(140.0, 120.0)); // Right side
    assert!(matches!(output_handle, Some(ConnectionHandle::Output)));

    // Test detecting input handle (left side of node)
    let input_handle = detector.detect_handle(&node, Position::new(60.0, 120.0)); // Left side
    assert!(matches!(input_handle, Some(ConnectionHandle::Input)));

    // Test no handle detected in center
    let no_handle = detector.detect_handle(&node, Position::new(100.0, 120.0)); // Center
    assert!(no_handle.is_none());
}

#[wasm_bindgen_test]
fn test_connection_preview() {
    // Test: Connection preview should show temporary edge during drag
    use crate::edge_connection::ConnectionPreview;

    let mut preview = ConnectionPreview::new();
    let graph = create_test_graph();

    let source_node = NodeId::new("node1");
    let start_pos = Position::new(140.0, 120.0); // Output handle
    let current_pos = Position::new(200.0, 150.0);

    // Start connection preview
    preview.start_connection(&graph, &source_node, start_pos);

    // Update preview position
    preview.update_preview(current_pos);

    // Check preview state
    assert!(preview.is_active());
    assert_eq!(preview.source_node(), Some(&source_node));
    assert_eq!(preview.current_position(), current_pos);

    // End preview
    preview.end_preview();
    assert!(!preview.is_active());
}

#[wasm_bindgen_test]
fn test_edge_creation() {
    // Test: System should create edges between nodes
    use crate::edge_connection::{ConnectionResult, EdgeCreator};

    let mut creator = EdgeCreator::new();
    let mut graph = create_test_graph();

    let source_node = NodeId::new("node1");
    let target_node = NodeId::new("node2");

    // Create edge
    let result = creator.create_edge(&mut graph, &source_node, &target_node);
    assert!(matches!(result, ConnectionResult::Valid));

    // Verify edge was created
    let edges: Vec<_> = graph.edges().collect();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].source, source_node);
    assert_eq!(edges[0].target, target_node);
}

#[wasm_bindgen_test]
fn test_edge_deletion() {
    // Test: System should delete edges
    use crate::edge_connection::EdgeCreator;

    let mut creator = EdgeCreator::new();
    let mut graph = create_test_graph();

    let source_node = NodeId::new("node1");
    let target_node = NodeId::new("node2");

    // Create edge first
    creator.create_edge(&mut graph, &source_node, &target_node);

    // Find the created edge
    let edges: Vec<_> = graph.edges().collect();
    let edge_id = edges[0].id.clone();

    // Delete edge
    let result = graph.remove_edge(&edge_id);
    assert!(result.is_ok());

    // Verify edge was deleted
    let edges: Vec<_> = graph.edges().collect();
    assert_eq!(edges.len(), 0);
}

#[wasm_bindgen_test]
fn test_connection_visual_feedback() {
    // Test: Connection should provide visual feedback during interaction
    use crate::edge_connection::ConnectionVisualizer;

    let visualizer = ConnectionVisualizer::new();
    let graph = create_test_graph();

    let source_node = NodeId::new("node1");
    let target_node = NodeId::new("node2");

    // Test valid connection visual feedback
    let feedback = visualizer.get_connection_feedback(&graph, &source_node, &target_node);
    assert!(feedback.is_valid);
    assert!(feedback.can_connect);

    // Test invalid connection visual feedback
    let feedback = visualizer.get_connection_feedback(&graph, &source_node, &source_node);
    assert!(!feedback.is_valid);
    assert!(!feedback.can_connect);
}

#[wasm_bindgen_test]
fn test_connection_handle_positions() {
    // Test: Connection handles should be positioned correctly on nodes
    use crate::edge_connection::HandleDetector;

    let detector = HandleDetector::new();
    let graph = create_test_graph();

    let node_id = NodeId::new("node1");
    let node = graph.get_node(&node_id).unwrap();

    // Get handle positions
    let input_handle_pos = detector.get_input_handle_position(&node);
    let output_handle_pos = detector.get_output_handle_position(&node);

    // Input handle should be on the left side
    assert_eq!(input_handle_pos.x, node.position.x);
    assert_eq!(input_handle_pos.y, node.position.y + node.size.height / 2.0);

    // Output handle should be on the right side
    assert_eq!(output_handle_pos.x, node.position.x + node.size.width);
    assert_eq!(
        output_handle_pos.y,
        node.position.y + node.size.height / 2.0
    );
}

#[wasm_bindgen_test]
fn test_connection_constraints() {
    // Test: Connection system should respect constraints
    use crate::edge_connection::{ConnectionConstraints, ConnectionResult, ConnectionValidator};

    let constraints = ConnectionConstraints {
        max_connections_per_node: 2,
        allow_self_connections: false,
        allow_circular_dependencies: false,
    };

    let validator = ConnectionValidator::with_constraints(constraints);
    let mut graph = create_test_graph();

    let node1 = NodeId::new("node1");
    let node2 = NodeId::new("node2");
    let node3 = NodeId::new("node3");

    // Add two connections from node1 (should be allowed)
    graph
        .add_edge(Edge::new("edge1", node1.clone(), node2.clone(), ()))
        .unwrap();
    graph
        .add_edge(Edge::new("edge2", node1.clone(), node3.clone(), ()))
        .unwrap();

    // Try to add third connection (should be rejected)
    let result = validator.validate_connection(&graph, &node1, &NodeId::new("node4"));
    assert!(matches!(result, ConnectionResult::MaxConnectionsExceeded));
}

#[wasm_bindgen_test]
fn test_connection_undo_redo() {
    // Test: Connection operations should support undo/redo
    use crate::edge_connection::{ConnectionHistory, EdgeCreator};

    let mut creator = EdgeCreator::new();
    let mut history = ConnectionHistory::new();
    let mut graph = create_test_graph();

    let source_node = NodeId::new("node1");
    let target_node = NodeId::new("node2");

    // Create edge and record in history
    creator.create_edge(&mut graph, &source_node, &target_node);
    history.record_edge_creation(&source_node, &target_node);

    // Verify edge exists
    assert_eq!(graph.edges().count(), 1);

    // Undo operation
    history.undo(&mut graph);
    assert_eq!(graph.edges().count(), 0);

    // Redo operation
    history.redo(&mut graph);
    assert_eq!(graph.edges().count(), 1);
}

#[wasm_bindgen_test]
fn test_connection_performance() {
    // Test: Connection system should handle large numbers of edges efficiently
    use crate::edge_connection::{ConnectionValidator, EdgeCreator};

    let validator = ConnectionValidator::new();
    let mut creator = EdgeCreator::new();
    let mut graph = create_test_graph();

    // Add many nodes
    for i in 4..100 {
        let node_id = NodeId::new(&format!("node{}", i));
        graph
            .add_node(Node::simple(
                format!("node{}", i),
                Position::new(i as f64 * 50.0, 100.0),
            ))
            .unwrap();
    }

    // Create many edges
    for i in 1..50 {
        let source = NodeId::new(&format!("node{}", i));
        let target = NodeId::new(&format!("node{}", i + 1));
        creator.create_edge(&mut graph, &source, &target);
    }

    // Validate connection should still be fast
    let start = std::time::Instant::now();
    let result =
        validator.validate_connection(&graph, &NodeId::new("node1"), &NodeId::new("node50"));
    let duration = start.elapsed();

    assert!(matches!(result, ConnectionResult::Valid));
    assert!(duration.as_millis() < 10); // Should be very fast
}
