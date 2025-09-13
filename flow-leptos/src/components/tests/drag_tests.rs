//! Drag and drop interaction tests
//!
//! Tests that verify drag and drop functionality works correctly for nodes

use leptos::*;
use flow_core::{Graph, Node, Position, NodeId};
use crate::signals::FlowState;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
    graph.add_node(Node::simple("node2", Position::new(200.0, 150.0))).unwrap();
    graph.add_node(Node::simple("node3", Position::new(300.0, 200.0))).unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_drag_state_initialization() {
    // Test: FlowState should properly initialize drag state
    let state = FlowState::new();

    assert!(!state.is_dragging);
    assert!(state.drag_start.is_none());
    assert!(state.last_mouse_pos.is_none());
}

#[wasm_bindgen_test]
fn test_start_drag_operation() {
    // Test: Starting a drag should set proper state
    let mut state = FlowState::new();
    let start_pos = Position::new(150.0, 100.0);

    state.start_drag(start_pos);

    assert!(state.is_dragging);
    assert_eq!(state.drag_start, Some(start_pos));
    assert_eq!(state.last_mouse_pos, Some(start_pos));
}

#[wasm_bindgen_test]
fn test_update_drag_position() {
    // Test: Updating drag position should track mouse movement
    let mut state = FlowState::new();
    let start_pos = Position::new(100.0, 100.0);
    let current_pos = Position::new(120.0, 110.0);

    state.start_drag(start_pos);
    state.update_drag(current_pos);

    assert!(state.is_dragging);
    assert_eq!(state.drag_start, Some(start_pos));
    assert_eq!(state.last_mouse_pos, Some(current_pos));

    let delta = state.drag_delta().unwrap();
    assert_eq!(delta.x, 20.0);
    assert_eq!(delta.y, 10.0);
}

#[wasm_bindgen_test]
fn test_end_drag_operation() {
    // Test: Ending drag should reset state properly
    let mut state = FlowState::new();
    let start_pos = Position::new(100.0, 100.0);

    state.start_drag(start_pos);
    assert!(state.is_dragging);

    state.end_drag();

    assert!(!state.is_dragging);
    assert!(state.drag_start.is_none());
    assert!(state.last_mouse_pos.is_none());
    assert!(state.drag_delta().is_none());
}

#[wasm_bindgen_test]
fn test_single_node_drag() {
    // Test: Dragging a single selected node should update its position
    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Select the node first
    state.select_node(node_id.clone());

    // Start drag
    let start_pos = Position::new(100.0, 100.0);
    state.start_drag(start_pos);

    // Move mouse
    let end_pos = Position::new(150.0, 120.0);
    state.update_drag(end_pos);

    // Apply drag to graph
    if let Some(delta) = state.drag_delta() {
        if let Some(node) = graph.get_node_mut(&node_id) {
            let new_pos = Position::new(
                node.position.x + delta.x,
                node.position.y + delta.y,
            );
            node.set_position(new_pos);
        }
    }

    // Verify node moved
    let node = graph.get_node(&node_id).unwrap();
    assert_eq!(node.position.x, 150.0); // 100 + 50
    assert_eq!(node.position.y, 120.0); // 100 + 20
}

#[wasm_bindgen_test]
fn test_multi_node_drag() {
    // Test: Dragging multiple selected nodes should move all of them
    let mut graph = create_test_graph();
    let mut state = FlowState::new();

    // Select multiple nodes
    state.select_node(NodeId::new("node1"));
    state.add_node_to_selection(NodeId::new("node2"));

    // Start drag
    let start_pos = Position::new(150.0, 125.0);
    state.start_drag(start_pos);

    // Move mouse
    let end_pos = Position::new(200.0, 175.0);
    state.update_drag(end_pos);

    // Apply drag to all selected nodes
    if let Some(delta) = state.drag_delta() {
        for node_id in &state.selected_nodes {
            if let Some(node) = graph.get_node_mut(node_id) {
                let new_pos = Position::new(
                    node.position.x + delta.x,
                    node.position.y + delta.y,
                );
                node.set_position(new_pos);
            }
        }
    }

    // Verify both nodes moved by the same delta
    let node1 = graph.get_node(&NodeId::new("node1")).unwrap();
    let node2 = graph.get_node(&NodeId::new("node2")).unwrap();

    assert_eq!(node1.position.x, 150.0); // 100 + 50
    assert_eq!(node1.position.y, 150.0); // 100 + 50
    assert_eq!(node2.position.x, 250.0); // 200 + 50
    assert_eq!(node2.position.y, 200.0); // 150 + 50
}

#[wasm_bindgen_test]
fn test_drag_bounds_validation() {
    // Test: Drag should respect canvas boundaries
    let mut state = FlowState::new();
    let start_pos = Position::new(10.0, 10.0);

    state.start_drag(start_pos);

    // Try to drag outside bounds (negative coordinates)
    let invalid_pos = Position::new(-50.0, -30.0);
    state.update_drag(invalid_pos);

    let delta = state.drag_delta().unwrap();

    // Should be able to calculate delta even if outside bounds
    assert_eq!(delta.x, -60.0);
    assert_eq!(delta.y, -40.0);

    // Note: Bounds validation should be applied when actually moving nodes
}

// ADVANCED DRAG FEATURES - TDD Implementation

#[wasm_bindgen_test]
fn test_snap_to_grid_functionality() {
    // Test: Nodes should snap to grid when snap_to_grid is enabled
    use crate::drag::{DragHandler, DragConfig};

    let config = DragConfig {
        snap_to_grid: true,
        grid_size: 20.0,
        ..Default::default()
    };
    let handler = DragHandler::with_config(config);

    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Select node and start drag
    state.select_node(node_id.clone());
    state.start_drag(Position::new(100.0, 100.0));

    // Move to position that should snap to grid
    state.update_drag(Position::new(123.0, 137.0)); // Should snap to 120, 140

    // Apply snap to grid
    handler.snap_selected_nodes_to_grid(&mut graph, &state);

    let node = graph.get_node(&node_id).unwrap();
    assert_eq!(node.position.x, 120.0); // Snapped to grid
    assert_eq!(node.position.y, 140.0); // Snapped to grid
}

#[wasm_bindgen_test]
fn test_drag_handles_precision() {
    // Test: Drag handles should allow precise manipulation of node edges
    use crate::drag::{DragHandler, DragConfig};

    let handler = DragHandler::new();
    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Test dragging from specific handle (e.g., bottom-right corner)
    let handle_type = crate::drag::DragHandle::BottomRight;
    state.select_node(node_id.clone());
    state.start_drag_with_handle(Position::new(180.0, 140.0), handle_type);

    // Move handle to resize node
    state.update_drag(Position::new(200.0, 160.0));

    // Apply handle drag
    handler.apply_handle_drag(&mut graph, &state);

    let node = graph.get_node(&node_id).unwrap();
    assert_eq!(node.size.width, 100.0); // 80 + 20
    assert_eq!(node.size.height, 60.0); // 40 + 20
}

#[wasm_bindgen_test]
fn test_collision_detection_during_drag() {
    // Test: System should detect and handle node collisions during drag
    use crate::drag::{DragHandler, DragConfig};

    let config = DragConfig {
        enforce_bounds: true,
        canvas_bounds: Some(flow_core::Rect::new(0.0, 0.0, 500.0, 400.0)),
        ..Default::default()
    };
    let handler = DragHandler::with_config(config);

    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Select node and start drag
    state.select_node(node_id.clone());
    state.start_drag(Position::new(100.0, 100.0));

    // Try to drag to position that would cause collision
    state.update_drag(Position::new(190.0, 150.0)); // Would overlap with node2

    // Check for collisions
    let collisions = handler.detect_collisions(&graph, &state);
    assert!(!collisions.is_empty());

    // Apply collision resolution
    handler.resolve_collisions(&mut graph, &state, &collisions);

    // Node should be positioned to avoid collision
    let node = graph.get_node(&node_id).unwrap();
    assert!(node.position.x < 200.0); // Should not overlap with node2
}

#[wasm_bindgen_test]
fn test_drag_constraints_axis_locking() {
    // Test: Drag constraints should allow axis locking (horizontal/vertical only)
    use crate::drag::{DragHandler, DragConfig, DragConstraint};

    let config = DragConfig {
        constraint: Some(crate::drag::DragConstraint::HorizontalOnly),
        ..Default::default()
    };
    let handler = DragHandler::with_config(config);

    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Select node and start drag
    state.select_node(node_id.clone());
    state.start_drag(Position::new(100.0, 100.0));

    // Try to drag diagonally
    state.update_drag(Position::new(150.0, 120.0));

    // Apply constraint
    handler.apply_drag_constraints(&mut graph, &state);

    let node = graph.get_node(&node_id).unwrap();
    assert_eq!(node.position.x, 150.0); // X should change
    assert_eq!(node.position.y, 100.0); // Y should remain unchanged
}

#[wasm_bindgen_test]
fn test_drag_bounds_enforcement() {
    // Test: Drag should respect canvas boundaries
    use crate::drag::{DragHandler, DragConfig};

    let config = DragConfig {
        enforce_bounds: true,
        canvas_bounds: Some(flow_core::Rect::new(0.0, 0.0, 300.0, 200.0)),
        ..Default::default()
    };
    let handler = DragHandler::with_config(config);

    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Select node and start drag
    state.select_node(node_id.clone());
    state.start_drag(Position::new(100.0, 100.0));

    // Try to drag outside bounds
    state.update_drag(Position::new(-50.0, 250.0));

    // Apply bounds constraints
    handler.apply_drag_to_nodes(&mut graph, &state, state.drag_delta().unwrap());

    let node = graph.get_node(&node_id).unwrap();
    assert!(node.position.x >= 0.0); // Should be clamped to bounds
    assert!(node.position.y <= 160.0); // 200 - 40 (node height)
}

#[wasm_bindgen_test]
fn test_drag_threshold_behavior() {
    // Test: Drag should only start after moving beyond threshold
    use crate::drag::{DragHandler, DragConfig};

    let config = DragConfig {
        drag_threshold: 10.0,
        ..Default::default()
    };
    let handler = DragHandler::with_config(config);

    let mut graph = create_test_graph();
    let mut state = FlowState::new();
    let node_id = NodeId::new("node1");

    // Select node and start drag
    state.select_node(node_id.clone());
    state.start_drag(Position::new(100.0, 100.0));

    // Move small amount (below threshold)
    state.update_drag(Position::new(105.0, 105.0)); // Distance: ~7.07

    // Should not trigger drag update
    let result = handler.handle_mouse_move(
        &create_mock_mouse_event(105.0, 105.0),
        &mut graph,
        &mut state,
        &create_mock_viewport_state(),
        None,
    );
    assert!(result.is_none());

    // Move beyond threshold
    state.update_drag(Position::new(115.0, 115.0)); // Distance: ~21.21

    // Should trigger drag update
    let result = handler.handle_mouse_move(
        &create_mock_mouse_event(115.0, 115.0),
        &mut graph,
        &mut state,
        &create_mock_viewport_state(),
        None,
    );
    assert!(result.is_some());
}

#[wasm_bindgen_test]
fn test_multi_node_drag_with_constraints() {
    // Test: Multiple node drag should respect individual constraints
    use crate::drag::{DragHandler, DragConfig};

    let handler = DragHandler::new();
    let mut graph = create_test_graph();
    let mut state = FlowState::new();

    // Select multiple nodes
    state.select_node(NodeId::new("node1"));
    state.add_node_to_selection(NodeId::new("node2"));

    // Start drag
    state.start_drag(Position::new(150.0, 125.0));

    // Move all nodes
    state.update_drag(Position::new(200.0, 175.0));

    // Apply drag to all selected nodes
    if let Some(delta) = state.drag_delta() {
        handler.apply_drag_to_nodes(&mut graph, &state, delta);
    }

    // Verify all nodes moved by same delta
    let node1 = graph.get_node(&NodeId::new("node1")).unwrap();
    let node2 = graph.get_node(&NodeId::new("node2")).unwrap();

    assert_eq!(node1.position.x, 150.0); // 100 + 50
    assert_eq!(node1.position.y, 150.0); // 100 + 50
    assert_eq!(node2.position.x, 250.0); // 200 + 50
    assert_eq!(node2.position.y, 200.0); // 150 + 50
}

// Helper functions for tests
fn create_mock_mouse_event(_x: f64, _y: f64) -> web_sys::MouseEvent {
    // This would need to be implemented with proper mocking
    // For now, we'll use a placeholder
    unimplemented!("Mock mouse event creation")
}

fn create_mock_viewport_state() -> crate::signals::ViewportState {
    crate::signals::ViewportState::new()
}
