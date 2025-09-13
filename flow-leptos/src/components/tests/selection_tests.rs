//! Integration tests for Node Selection System in Leptos Flow
//!
//! These tests verify the selection system works correctly with the Leptos reactive system

use crate::signals::ViewportState;
use flow_core::{Graph, NavigationDirection, Node, Position, SelectionManager, SelectionMode};
use leptos::*;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    graph
        .add_node(Node::simple("node1", Position::new(100.0, 100.0)))
        .unwrap();
    graph
        .add_node(Node::simple("node2", Position::new(200.0, 150.0)))
        .unwrap();
    graph
        .add_node(Node::simple("node3", Position::new(300.0, 200.0)))
        .unwrap();
    graph
        .add_node(Node::simple("node4", Position::new(150.0, 250.0)))
        .unwrap();

    graph
}

#[wasm_bindgen_test]
fn test_selection_manager_integration_with_leptos_signals() {
    // Test: Selection manager should work with Leptos reactive signals
    let _graph = create_rw_signal(create_test_graph());
    let selection_manager = create_rw_signal(SelectionManager::new());

    // Select a node
    selection_manager.update(|manager| {
        manager.select_node("node1".into());
    });

    // Verify selection
    let selected_nodes = selection_manager.with(|manager| manager.selected_nodes().clone());

    assert_eq!(selected_nodes.len(), 1);
    assert!(selected_nodes.contains(&"node1".into()));
}

#[wasm_bindgen_test]
fn test_multi_selection_with_ctrl_click_simulation() {
    // Test: Multi-selection should work with simulated Ctrl+Click
    let selection_manager = create_rw_signal(SelectionManager::new());

    // Enable multi-selection mode
    selection_manager.update(|manager| {
        manager.set_mode(SelectionMode::Multi);
    });

    // Simulate Ctrl+Click on multiple nodes
    selection_manager.update(|manager| {
        manager.select_node("node1".into());
        manager.select_node("node2".into());
        manager.select_node("node3".into());
    });

    let selection_count = selection_manager.with(|manager| manager.selection_count());
    assert_eq!(selection_count, 3);
}

#[wasm_bindgen_test]
fn test_rectangle_selection_integration() {
    // Test: Rectangle selection should work with graph data
    let graph = create_test_graph();
    let selection_manager = create_rw_signal(SelectionManager::new());

    selection_manager.update(|manager| {
        // Start rectangle selection
        manager.start_rectangle_selection(Position::new(50.0, 50.0));

        // Update to cover first two nodes
        manager.update_rectangle_selection(Position::new(250.0, 175.0));

        // Complete selection
        let selected = manager.complete_rectangle_selection(&graph);

        assert!(selected.len() >= 2);
        assert!(selected.contains(&"node1".into()));
        assert!(selected.contains(&"node2".into()));
    });
}

#[wasm_bindgen_test]
fn test_keyboard_navigation_integration() {
    // Test: Keyboard navigation should work with reactive signals
    let graph = create_rw_signal(create_test_graph());
    let selection_manager = create_rw_signal(SelectionManager::new());

    // Test navigation
    selection_manager.update(|manager| {
        graph.with(|g| manager.navigate_selection(g, NavigationDirection::Next));
    });

    // Verify a node was selected
    let has_selection = selection_manager.with(|manager| manager.selection_count() > 0);
    assert!(has_selection);

    let selection_count = selection_manager.with(|manager| manager.selection_count());
    assert_eq!(selection_count, 1);
}

#[wasm_bindgen_test]
fn test_selection_state_reactivity() {
    // Test: Selection changes should trigger reactive updates
    let selection_manager = create_rw_signal(SelectionManager::new());
    let selection_changed = create_rw_signal(false);

    // Create a derived signal that tracks selection changes
    create_effect(move |_| {
        selection_manager.track();
        selection_changed.set(true);
    });

    // Initial state should not have triggered change yet
    assert!(!selection_changed.get_untracked());

    // Change selection
    selection_manager.update(|manager| {
        manager.select_node("node1".into());
    });

    // Should have triggered reactive update
    assert!(selection_changed.get_untracked());
}

// FAILING TESTS - RED PHASE
#[wasm_bindgen_test]
#[should_panic(expected = "Visual selection feedback not implemented")]
fn test_visual_selection_feedback_not_implemented() {
    // This test will fail until we implement visual feedback for selection
    // TODO: Test that selected nodes show visual indication (highlight, border, etc.)
    panic!("Visual selection feedback not implemented");
}

#[wasm_bindgen_test]
#[should_panic(expected = "Keyboard shortcuts not implemented")]
fn test_keyboard_shortcuts_not_implemented() {
    // This test will fail until we implement keyboard shortcuts
    // TODO: Test Ctrl+A (select all), Delete (delete selected), etc.
    panic!("Keyboard shortcuts not implemented");
}

#[wasm_bindgen_test]
#[should_panic(expected = "Selection persistence not implemented")]
fn test_selection_persistence_not_implemented() {
    // This test will fail until we implement selection persistence
    // TODO: Test that selection persists across viewport changes, saves/loads
    panic!("Selection persistence not implemented");
}

#[wasm_bindgen_test]
#[should_panic(expected = "Drag selection not implemented")]
fn test_drag_selection_rectangle_not_implemented() {
    // This test will fail until we implement drag selection rectangle
    // TODO: Test mouse drag creates selection rectangle and selects nodes
    panic!("Drag selection not implemented");
}
