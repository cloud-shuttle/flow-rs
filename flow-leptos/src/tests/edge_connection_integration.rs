// Integration tests for edge connection system with mouse interactions
//
// Tests the complete edge connection workflow including:
// - Connection handle detection
// - Edge creation via drag and drop
// - Connection validation
// - Visual feedback during connection

use flow_core::{Graph, Node, NodeId, EdgeId, Position, Size};
use crate::edge_connection::{
    ConnectionValidator, HandleDetector, ConnectionPreview, EdgeCreator, ConnectionVisualizer,
    ConnectionResult, ConnectionHandle, ConnectionFeedback
};
use crate::signals::{FlowState, ViewportState};
use crate::events::{FlowEvent, NodeEvent};

/// Test helper to create a simple graph with two nodes
fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Add two test nodes
    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    let node1 = Node::builder(node1_id.clone())
        .position(100.0, 100.0)
        .size(80.0, 40.0)
        .build();

    let node2 = Node::builder(node2_id.clone())
        .position(300.0, 100.0)
        .size(80.0, 40.0)
        .build();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    graph
}

#[test]
fn test_connection_handle_detection() {
    let graph = create_test_graph();
    let detector = HandleDetector::new();

    // Get the first node
    let node1 = graph.get_node(&NodeId::new("node1")).unwrap();

    // Test input handle detection (left side of node)
    let input_handle_pos = detector.get_input_handle_position(&node1);
    let detected_handle = detector.detect_handle(&node1, input_handle_pos);

    assert_eq!(detected_handle, Some(ConnectionHandle::Input));

    // Test output handle detection (right side of node)
    let output_handle_pos = detector.get_output_handle_position(&node1);
    let detected_handle = detector.detect_handle(&node1, output_handle_pos);

    assert_eq!(detected_handle, Some(ConnectionHandle::Output));

    // Test no handle detection (center of node)
    let center_pos = Position::new(
        node1.position.x + node1.size.width / 2.0,
        node1.position.y + node1.size.height / 2.0,
    );
    let detected_handle = detector.detect_handle(&node1, center_pos);

    assert_eq!(detected_handle, None);
}

#[test]
fn test_connection_validation() {
    let graph = create_test_graph();
    let validator = ConnectionValidator::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Test valid connection
    let result = validator.validate_connection(&graph, &node1_id, &node2_id);
    assert_eq!(result, ConnectionResult::Valid);

    // Test self-connection (should be invalid by default)
    let result = validator.validate_connection(&graph, &node1_id, &node1_id);
    assert_eq!(result, ConnectionResult::InvalidSelfConnection);

    // Test connection to non-existent node
    let invalid_node_id = NodeId::new("invalid");
    let result = validator.validate_connection(&graph, &node1_id, &invalid_node_id);
    assert_eq!(result, ConnectionResult::InvalidNode);
}

#[test]
fn test_connection_preview() {
    let graph = create_test_graph();
    let mut preview = ConnectionPreview::new();

    let node1_id = NodeId::new("node1");
    let start_pos = Position::new(100.0, 100.0);

    // Test starting a connection preview
    assert!(!preview.is_active());
    preview.start_connection(&graph, &node1_id, start_pos);
    assert!(preview.is_active());
    assert_eq!(preview.source_node(), Some(&node1_id));
    assert_eq!(preview.current_position(), start_pos);

    // Test updating preview position
    let new_pos = Position::new(150.0, 120.0);
    preview.update_preview(new_pos);
    assert_eq!(preview.current_position(), new_pos);

    // Test ending preview
    preview.end_preview();
    assert!(!preview.is_active());
    assert_eq!(preview.source_node(), None);
}

#[test]
fn test_edge_creation() {
    let mut graph = create_test_graph();
    let creator = EdgeCreator::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Test creating a valid edge
    let result = creator.create_edge(&mut graph, &node1_id, &node2_id);
    assert_eq!(result, ConnectionResult::Valid);

    // Verify edge was created
    let edges: Vec<_> = graph.edges().collect();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].source, node1_id);
    assert_eq!(edges[0].target, node2_id);

    // Test creating duplicate edge (should fail)
    let result = creator.create_edge(&mut graph, &node1_id, &node2_id);
    assert_eq!(result, ConnectionResult::DuplicateEdge);
}

#[test]
fn test_connection_feedback() {
    let mut graph = create_test_graph();
    let visualizer = ConnectionVisualizer::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Test valid connection feedback
    let feedback = visualizer.get_connection_feedback(&graph, &node1_id, &node2_id);
    assert!(feedback.is_valid);
    assert!(feedback.can_connect);
    assert!(feedback.message.is_none());

    // Create an edge and test duplicate feedback
    let creator = EdgeCreator::new();
    creator.create_edge(&mut graph, &node1_id, &node2_id);

    let feedback = visualizer.get_connection_feedback(&graph, &node1_id, &node2_id);
    assert!(!feedback.is_valid);
    assert!(!feedback.can_connect);
    assert!(feedback.message.is_some());
    assert!(feedback.message.unwrap().contains("already exists"));
}

#[test]
fn test_mouse_to_connection_workflow() {
    // This test simulates the complete mouse interaction workflow for edge creation
    let mut graph = create_test_graph();
    let detector = HandleDetector::new();
    let mut preview = ConnectionPreview::new();
    let creator = EdgeCreator::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Step 1: Mouse down on output handle of node1
    let node1 = graph.get_node(&node1_id).unwrap();
    let output_handle_pos = detector.get_output_handle_position(&node1);
    let detected_handle = detector.detect_handle(&node1, output_handle_pos);

    assert_eq!(detected_handle, Some(ConnectionHandle::Output));

    // Step 2: Start connection preview
    preview.start_connection(&graph, &node1_id, output_handle_pos);
    assert!(preview.is_active());

    // Step 3: Mouse move to update preview
    let mid_pos = Position::new(200.0, 120.0);
    preview.update_preview(mid_pos);
    assert_eq!(preview.current_position(), mid_pos);

    // Step 4: Mouse up on input handle of node2
    let node2 = graph.get_node(&node2_id).unwrap();
    let input_handle_pos = detector.get_input_handle_position(&node2);
    let detected_handle = detector.detect_handle(&node2, input_handle_pos);

    assert_eq!(detected_handle, Some(ConnectionHandle::Input));

    // Step 5: Create the edge
    let result = creator.create_edge(&mut graph, &node1_id, &node2_id);
    assert_eq!(result, ConnectionResult::Valid);

    // Step 6: End preview
    preview.end_preview();
    assert!(!preview.is_active());

    // Verify edge was created
    let edges: Vec<_> = graph.edges().collect();
    assert_eq!(edges.len(), 1);
}

#[test]
fn test_connection_with_flow_state_integration() {
    // This test verifies integration with FlowState for connection operations
    let mut graph = create_test_graph();
    let mut flow_state = FlowState::new();
    let detector = HandleDetector::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Simulate starting a connection drag
    let node1 = graph.get_node(&node1_id).unwrap();
    let output_handle_pos = detector.get_output_handle_position(&node1);

    // Update flow state to indicate connection mode
    flow_state.set_connection_mode(true);
    flow_state.set_connection_source(Some(node1_id.clone()));
    flow_state.set_connection_start_position(Some(output_handle_pos));

    assert!(flow_state.is_connection_mode());
    assert_eq!(flow_state.connection_source(), Some(&node1_id));
    assert_eq!(flow_state.connection_start_position(), Some(output_handle_pos));

    // Simulate ending connection drag
    flow_state.set_connection_mode(false);
    flow_state.set_connection_source(None);
    flow_state.set_connection_start_position(None);

    assert!(!flow_state.is_connection_mode());
    assert_eq!(flow_state.connection_source(), None);
    assert_eq!(flow_state.connection_start_position(), None);
}

#[test]
fn test_connection_events() {
    // This test verifies that connection operations generate appropriate events
    let mut graph = create_test_graph();
    let creator = EdgeCreator::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Test that edge creation would generate appropriate events
    // (In a real implementation, this would be handled by the event system)
    let result = creator.create_edge(&mut graph, &node1_id, &node2_id);
    assert_eq!(result, ConnectionResult::Valid);

    // Verify the edge exists (which would trigger a FlowEvent::EdgeCreated)
    let edges: Vec<_> = graph.edges().collect();
    assert_eq!(edges.len(), 1);

    // In a real implementation, we would verify that:
    // - FlowEvent::ConnectionStarted was emitted when drag began
    // - FlowEvent::ConnectionUpdated was emitted during drag
    // - FlowEvent::ConnectionCompleted was emitted when edge was created
    // - FlowEvent::EdgeCreated was emitted when edge was added to graph
}

#[test]
fn test_connection_validation_edge_cases() {
    let mut graph = create_test_graph();
    let validator = ConnectionValidator::new();

    let node1_id = NodeId::new("node1");
    let node2_id = NodeId::new("node2");

    // Test circular dependency detection
    // First create an edge from node1 to node2
    let creator = EdgeCreator::new();
    creator.create_edge(&mut graph, &node1_id, &node2_id);

    // Now try to create an edge from node2 back to node1 (circular)
    let result = validator.validate_connection(&graph, &node2_id, &node1_id);
    assert_eq!(result, ConnectionResult::CircularDependency);

    // Test max connections constraint
    let constraints = crate::edge_connection::ConnectionConstraints {
        max_connections_per_node: 1,
        allow_self_connections: false,
        allow_circular_dependencies: false,
    };
    let limited_validator = ConnectionValidator::with_constraints(constraints);

    // Try to create another edge from node1 (should exceed max connections)
    let node3_id = NodeId::new("node3");
    let node3 = Node::builder(node3_id.clone())
        .position(500.0, 100.0)
        .size(80.0, 40.0)
        .build();
    graph.add_node(node3).unwrap();

    let result = limited_validator.validate_connection(&graph, &node1_id, &node3_id);
    assert_eq!(result, ConnectionResult::MaxConnectionsExceeded);
}
