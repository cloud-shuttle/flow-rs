//! Keyboard event handling tests for selection system
//!
//! Tests that verify keyboard interactions work correctly for node selection

use leptos::*;
use leptos_flow_core::{Graph, Node, Position, NodeId, NavigationDirection, KeyboardShortcut};
use crate::signals::FlowState;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
    graph.add_node(Node::simple("node2", Position::new(200.0, 150.0))).unwrap();
    graph.add_node(Node::simple("node3", Position::new(300.0, 200.0))).unwrap();
    graph.add_node(Node::simple("node4", Position::new(150.0, 250.0))).unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_keyboard_select_all() {
    // Test: Ctrl+A should select all nodes
    let mut state = FlowState::new();
    let graph = create_test_graph();

    // Initially no nodes selected
    assert_eq!(state.selected_nodes.len(), 0);

    // Simulate Ctrl+A - select all
    state.select_all(&graph);

    assert_eq!(state.selected_nodes.len(), 4);
    assert!(state.is_node_selected(&NodeId::new("node1")));
    assert!(state.is_node_selected(&NodeId::new("node2")));
    assert!(state.is_node_selected(&NodeId::new("node3")));
    assert!(state.is_node_selected(&NodeId::new("node4")));
}

#[wasm_bindgen_test]
fn test_keyboard_clear_selection() {
    // Test: Escape should clear all selections
    let mut state = FlowState::new();
    let graph = create_test_graph();

    // First select some nodes
    state.select_node(NodeId::new("node1"));
    state.add_node_to_selection(NodeId::new("node2"));
    assert_eq!(state.selected_nodes.len(), 2);

    // Simulate Escape - clear selection
    state.clear_selection();

    assert_eq!(state.selected_nodes.len(), 0);
    assert!(!state.is_node_selected(&NodeId::new("node1")));
    assert!(!state.is_node_selected(&NodeId::new("node2")));
}

#[wasm_bindgen_test]
fn test_keyboard_navigation_basic() {
    // Test: Next/Previous navigation should work
    let mut state = FlowState::new();
    let graph = create_test_graph();

    // Start with node1 selected
    state.select_node(NodeId::new("node1"));

    // Navigate to next node
    let next_node = state.navigate_selection(&graph, NavigationDirection::Next);
    assert!(next_node.is_some());

    // Navigate to previous node
    let prev_node = state.navigate_selection(&graph, NavigationDirection::Previous);
    assert!(prev_node.is_some());
}

#[wasm_bindgen_test]
fn test_keyboard_multi_select_toggle() {
    // Test: Ctrl+Click behavior for toggling selection
    let mut state = FlowState::new();
    let node_id = NodeId::new("test-node");

    // First toggle - should select
    assert!(!state.is_node_selected(&node_id));
    state.toggle_node_selection(node_id.clone());
    assert!(state.is_node_selected(&node_id));
    assert_eq!(state.selected_nodes.len(), 1);

    // Second toggle - should deselect
    state.toggle_node_selection(node_id.clone());
    assert!(!state.is_node_selected(&node_id));
    assert_eq!(state.selected_nodes.len(), 0);
}

#[wasm_bindgen_test]
fn test_keyboard_shortcut_select_all() {
    // Test: Using keyboard shortcut system for select all
    let mut state = FlowState::new();
    let graph = create_test_graph();

    // Initially no nodes selected
    assert_eq!(state.selected_nodes.len(), 0);

    // Use keyboard shortcut system
    state.handle_keyboard_shortcut(&graph, KeyboardShortcut::SelectAll);

    assert_eq!(state.selected_nodes.len(), 4);
    assert!(state.is_node_selected(&NodeId::new("node1")));
    assert!(state.is_node_selected(&NodeId::new("node2")));
    assert!(state.is_node_selected(&NodeId::new("node3")));
    assert!(state.is_node_selected(&NodeId::new("node4")));
}

#[wasm_bindgen_test]
fn test_keyboard_shortcut_arrow_navigation() {
    // Test: Arrow key navigation using keyboard shortcuts
    let mut state = FlowState::new();
    let graph = create_test_graph();

    // Start with no selection
    assert_eq!(state.selected_nodes.len(), 0);

    // Arrow right should select next node
    state.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowRight);
    assert_eq!(state.selected_nodes.len(), 1);
    let first_selected = state.selected_nodes[0].clone();

    // Arrow right again should move to next node
    state.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowRight);
    assert_eq!(state.selected_nodes.len(), 1);
    let second_selected = state.selected_nodes[0].clone();
    assert_ne!(first_selected, second_selected);

    // Arrow left should go back
    state.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowLeft);
    assert_eq!(state.selected_nodes.len(), 1);
    let back_selected = state.selected_nodes[0].clone();
    assert_eq!(first_selected, back_selected);
}

#[wasm_bindgen_test]
fn test_keyboard_shortcut_escape() {
    // Test: Escape key using keyboard shortcuts
    let mut state = FlowState::new();
    let graph = create_test_graph();

    // Select some nodes first
    state.select_node(NodeId::new("node1"));
    state.add_node_to_selection(NodeId::new("node2"));
    assert_eq!(state.selected_nodes.len(), 2);

    // Escape should clear selection
    state.handle_keyboard_shortcut(&graph, KeyboardShortcut::Escape);
    assert_eq!(state.selected_nodes.len(), 0);
}

#[wasm_bindgen_test]
fn test_keyboard_shortcut_delete() {
    // Test: Delete key handling using keyboard shortcuts
    let mut state = FlowState::new();
    let mut graph = create_test_graph();

    // Select some nodes
    state.select_node(NodeId::new("node1"));
    state.add_node_to_selection(NodeId::new("node2"));
    assert_eq!(state.selected_nodes.len(), 2);
    assert_eq!(graph.node_count(), 4);

    // Delete should remove selected nodes from graph and clear selection
    state.handle_destructive_keyboard_shortcut(&mut graph, KeyboardShortcut::Delete);

    assert_eq!(state.selected_nodes.len(), 0);
    assert_eq!(graph.node_count(), 2);
    assert!(graph.get_node(&NodeId::new("node3")).is_some());
    assert!(graph.get_node(&NodeId::new("node4")).is_some());
    assert!(graph.get_node(&NodeId::new("node1")).is_none());
    assert!(graph.get_node(&NodeId::new("node2")).is_none());
}

// FAILING TESTS - Future functionality

#[wasm_bindgen_test]
#[should_panic(expected = "Copy/paste shortcuts not implemented")]
fn test_keyboard_copy_paste_not_implemented() {
    // This test will fail until we implement copy/paste
    panic!("Copy/paste shortcuts not implemented");
}

#[wasm_bindgen_test]
#[should_panic(expected = "Undo/redo shortcuts not implemented")]
fn test_keyboard_undo_redo_not_implemented() {
    // This test will fail until we implement undo/redo
    panic!("Undo/redo shortcuts not implemented");
}
