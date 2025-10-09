//! Comprehensive integration tests for Flow-RS core functionality
//!
//! These tests validate end-to-end workflows and interactions between
//! different components of the flow editor system.

use flow_rs_core::*;

/// Test basic graph operations work together
#[test]
fn test_basic_graph_operations() {
    // Create a new graph
    let mut graph: Graph<String, String> = Graph::new();

    // Create nodes
    let node1 = Node::new("node1", Position::new(0.0, 0.0), "Start".to_string());
    let node2 = Node::new("node2", Position::new(100.0, 0.0), "Process".to_string());

    // Add nodes
    graph.add_node(node1.clone()).unwrap();
    graph.add_node(node2.clone()).unwrap();

    assert_eq!(graph.node_count(), 2);

    // Test node retrieval
    let retrieved_node = graph.get_node(&node1.id).unwrap();
    assert_eq!(retrieved_node.data, "Start".to_string());

    // Test node removal
    let removed_node = graph.remove_node(&node1.id).unwrap();
    assert_eq!(removed_node.data, "Start".to_string());
    assert_eq!(graph.node_count(), 1);
}

/// Test auto-layout transitions work
#[test]
fn test_auto_layout_transitions() {
    use crate::auto_layout::transitions::{TransitionState, ease_in_out, interpolate_position};

    // Test easing function
    assert_eq!(ease_in_out(0.0), 0.0);
    assert_eq!(ease_in_out(1.0), 1.0);
    assert_eq!(ease_in_out(0.5), 0.5);

    // Test position interpolation
    let from = Position::new(0.0, 0.0);
    let to = Position::new(100.0, 100.0);
    let result = interpolate_position(from, to, 0.5);
    assert_eq!(result, Position::new(50.0, 50.0));

    // Test transition state creation
    let mut from_positions = std::collections::HashMap::new();
    let mut to_positions = std::collections::HashMap::new();

    let node_id = NodeId::new("test");
    from_positions.insert(node_id.clone(), Position::new(0.0, 0.0));
    to_positions.insert(node_id, Position::new(100.0, 100.0));

    let state = TransitionState::new(1.0, from_positions, to_positions);
    assert_eq!(state.progress, 0.0);
    assert_eq!(state.duration, 1.0);
}

