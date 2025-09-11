// Tests for mouse interactions and node selection
//
// This module tests the mouse event handling system and node selection functionality
// in the Leptos Flow editor.

use leptos::*;
use wasm_bindgen_test::*;
use web_sys::{MouseEvent, HtmlCanvasElement};
use js_sys::Object;

use leptos_flow_core::{Graph, Node, Position, NodeId};
use crate::signals::{FlowState, ViewportState};
use crate::events::{FlowEvent, NodeEvent, MouseButton, KeyboardModifiers};
use crate::drag::DragHandler;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that mouse down events on nodes trigger selection
#[wasm_bindgen_test]
fn test_mouse_down_node_selection() {
    // Create a test graph with a node
    let mut graph: Graph<(), ()> = Graph::new();
    let node = Node::simple("test-node", Position::new(100.0, 100.0));
    let _ = graph.add_node(node);

    // Create flow state
    let mut flow_state = FlowState::new();
    let viewport_state = ViewportState::new();
    let drag_handler = DragHandler::new();

    // Create a mock mouse event
    let mouse_event = create_mock_mouse_event(100, 100, 0); // Left click at node position

    // Handle mouse down
    let result = drag_handler.handle_mouse_down(
        &mouse_event,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );

    // Assert that drag started and node is selected
    assert!(result.is_some());
    assert!(flow_state.is_node_selected(&NodeId::from("test-node")));
}

/// Test that mouse down events on empty space clear selection
#[wasm_bindgen_test]
fn test_mouse_down_empty_space_clears_selection() {
    // Create a test graph with a node
    let mut graph: Graph<(), ()> = Graph::new();
    let node = Node::simple("test-node", Position::new(100.0, 100.0));
    let _ = graph.add_node(node);

    // Create flow state with node selected
    let mut flow_state = FlowState::new();
    flow_state.select_node(NodeId::from("test-node"));
    assert!(flow_state.is_node_selected(&NodeId::from("test-node")));

    let viewport_state = ViewportState::new();
    let drag_handler = DragHandler::new();

    // Create a mock mouse event on empty space
    let mouse_event = create_mock_mouse_event(50, 50, 0); // Left click away from node

    // Handle mouse down
    let result = drag_handler.handle_mouse_down(
        &mouse_event,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );

    // Assert that no drag started and selection is cleared
    assert!(result.is_none());
    assert!(!flow_state.is_node_selected(&NodeId::from("test-node")));
}

/// Test that Ctrl+Click toggles node selection
#[wasm_bindgen_test]
fn test_ctrl_click_toggles_selection() {
    // Create a test graph with a node
    let mut graph: Graph<(), ()> = Graph::new();
    let node = Node::simple("test-node", Position::new(100.0, 100.0));
    let _ = graph.add_node(node);

    // Create flow state
    let mut flow_state = FlowState::new();
    let viewport_state = ViewportState::new();
    let drag_handler = DragHandler::new();

    // First click - should select node
    let mouse_event1 = create_mock_mouse_event_with_modifiers(100, 100, 0, true, false, false, false);
    let _ = drag_handler.handle_mouse_down(
        &mouse_event1,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );
    assert!(flow_state.is_node_selected(&NodeId::from("test-node")));

    // Second Ctrl+Click - should deselect node
    let mouse_event2 = create_mock_mouse_event_with_modifiers(100, 100, 0, true, false, false, false);
    let _ = drag_handler.handle_mouse_down(
        &mouse_event2,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );
    assert!(!flow_state.is_node_selected(&NodeId::from("test-node")));
}

/// Test that Shift+Click adds to selection
#[wasm_bindgen_test]
fn test_shift_click_adds_to_selection() {
    // Create a test graph with two nodes
    let mut graph: Graph<(), ()> = Graph::new();
    let node1 = Node::simple("node1", Position::new(100.0, 100.0));
    let node2 = Node::simple("node2", Position::new(200.0, 200.0));
    let _ = graph.add_node(node1);
    let _ = graph.add_node(node2);

    // Create flow state
    let mut flow_state = FlowState::new();
    let viewport_state = ViewportState::new();
    let drag_handler = DragHandler::new();

    // First click - select first node
    let mouse_event1 = create_mock_mouse_event(100, 100, 0);
    let _ = drag_handler.handle_mouse_down(
        &mouse_event1,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );
    assert!(flow_state.is_node_selected(&NodeId::from("node1")));
    assert!(!flow_state.is_node_selected(&NodeId::from("node2")));

    // Shift+Click second node - should add to selection
    let mouse_event2 = create_mock_mouse_event_with_modifiers(200, 200, 0, false, true, false, false);
    let _ = drag_handler.handle_mouse_down(
        &mouse_event2,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );
    assert!(flow_state.is_node_selected(&NodeId::from("node1")));
    assert!(flow_state.is_node_selected(&NodeId::from("node2")));
}

/// Test that dragging moves selected nodes
#[wasm_bindgen_test]
fn test_drag_moves_selected_nodes() {
    // Create a test graph with a node
    let mut graph: Graph<(), ()> = Graph::new();
    let node = Node::simple("test-node", Position::new(100.0, 100.0));
    let _ = graph.add_node(node);

    // Create flow state with node selected
    let mut flow_state = FlowState::new();
    flow_state.select_node(NodeId::from("test-node"));

    let viewport_state = ViewportState::new();
    let drag_handler = DragHandler::new();

    // Start drag
    let mouse_down_event = create_mock_mouse_event(100, 100, 0);
    let _ = drag_handler.handle_mouse_down(
        &mouse_down_event,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );

    // Move mouse (drag)
    let mouse_move_event = create_mock_mouse_event(150, 150, 0);
    let _ = drag_handler.handle_mouse_move(
        &mouse_move_event,
        &mut graph,
        &mut flow_state,
        &viewport_state,
        None,
    );

    // Check that node position was updated
    let updated_node = graph.get_node(&NodeId::from("test-node")).unwrap();
    assert_eq!(updated_node.position.x, 150.0);
    assert_eq!(updated_node.position.y, 150.0);
}

/// Test that mouse up ends drag operation
#[wasm_bindgen_test]
fn test_mouse_up_ends_drag() {
    // Create a test graph with a node
    let mut graph: Graph<(), ()> = Graph::new();
    let node = Node::simple("test-node", Position::new(100.0, 100.0));
    let _ = graph.add_node(node);

    // Create flow state with node selected
    let mut flow_state = FlowState::new();
    flow_state.select_node(NodeId::from("test-node"));

    let viewport_state = ViewportState::new();
    let drag_handler = DragHandler::new();

    // Start drag
    let mouse_down_event = create_mock_mouse_event(100, 100, 0);
    let _ = drag_handler.handle_mouse_down(
        &mouse_down_event,
        &graph,
        &mut flow_state,
        &viewport_state,
        None,
    );

    assert!(flow_state.is_dragging);

    // End drag
    let mouse_up_event = create_mock_mouse_event(150, 150, 0);
    let _ = drag_handler.handle_mouse_up(
        &mouse_up_event,
        &mut graph,
        &mut flow_state,
        &viewport_state,
        None,
    );

    assert!(!flow_state.is_dragging);
}

/// Helper function to create a mock mouse event
fn create_mock_mouse_event(x: i32, y: i32, button: i16) -> MouseEvent {
    let event = MouseEvent::new("mousedown").unwrap();
    // Note: In a real test environment, we'd need to properly mock the event properties
    // For now, this is a placeholder that will fail until we implement proper mocking
    event
}

/// Helper function to create a mock mouse event with keyboard modifiers
fn create_mock_mouse_event_with_modifiers(
    x: i32,
    y: i32,
    button: i16,
    ctrl: bool,
    shift: bool,
    alt: bool,
    meta: bool
) -> MouseEvent {
    let event = MouseEvent::new("mousedown").unwrap();
    // Note: In a real test environment, we'd need to properly mock the event properties
    // For now, this is a placeholder that will fail until we implement proper mocking
    event
}
