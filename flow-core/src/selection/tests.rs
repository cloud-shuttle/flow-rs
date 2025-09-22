//! Tests for selection system

#[cfg(test)]
mod tests {
    use crate::graph::{Graph, Node};
    use crate::types::{NodeId, Position};
    
    use crate::selection::SelectionManager;
    use crate::selection::{KeyboardShortcut, NavigationDirection, SelectionMode};
    // use crate::selection::VisualFeedback; // Not used in tests

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

    #[test]
    fn test_new_selection_manager() {
        let manager = SelectionManager::new();
        assert_eq!(manager.selection_count(), 0);
        assert_eq!(manager.mode(), &SelectionMode::Single);
        assert!(manager.selected_nodes().is_empty());
    }

    #[test]
    fn test_single_node_selection() {
        let mut manager = SelectionManager::new();
        let node_id: NodeId = "node1".into();

        manager.select_node(node_id.clone());

        assert_eq!(manager.selection_count(), 1);
        assert!(manager.is_selected(&node_id));
    }

    #[test]
    fn test_single_mode_replaces_selection() {
        let mut manager = SelectionManager::new();
        let node1: NodeId = "node1".into();
        let node2: NodeId = "node2".into();

        manager.select_node(node1.clone());
        manager.select_node(node2.clone());

        assert_eq!(manager.selection_count(), 1);
        assert!(!manager.is_selected(&node1));
        assert!(manager.is_selected(&node2));
    }

    #[test]
    fn test_multi_mode_accumulates_selection() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);

        let node1: NodeId = "node1".into();
        let node2: NodeId = "node2".into();

        manager.select_node(node1.clone());
        manager.select_node(node2.clone());

        assert_eq!(manager.selection_count(), 2);
        assert!(manager.is_selected(&node1));
        assert!(manager.is_selected(&node2));
    }

    #[test]
    fn test_toggle_node_selection() {
        let mut manager = SelectionManager::new();
        let node_id: NodeId = "node1".into();

        // Toggle on
        manager.toggle_node(node_id.clone());
        assert!(manager.is_selected(&node_id));
        assert_eq!(manager.selection_count(), 1);

        // Toggle off
        manager.toggle_node(node_id.clone());
        assert!(!manager.is_selected(&node_id));
        assert_eq!(manager.selection_count(), 0);
    }

    #[test]
    fn test_clear_selection() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);

        manager.select_node("node1".into());
        manager.select_node("node2".into());

        assert_eq!(manager.selection_count(), 2);

        manager.clear_selection();

        assert_eq!(manager.selection_count(), 0);
        assert!(manager.selected_nodes().is_empty());
    }

    #[test]
    fn test_rectangle_selection_basic() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Start rectangle selection
        manager.start_rectangle_selection(Position::new(50.0, 50.0));
        assert_eq!(manager.mode(), &SelectionMode::Rectangle);

        // Update rectangle to cover nodes 1 and 2
        manager.update_rectangle_selection(Position::new(250.0, 175.0));

        // Complete selection
        let selected = manager.complete_rectangle_selection(&graph);
        assert_eq!(selected.len(), 2);
        assert_eq!(manager.mode(), &SelectionMode::Single);
    }

    #[test]
    fn test_rectangle_selection_bounds() {
        let mut manager = SelectionManager::new();

        // Start rectangle selection
        manager.start_rectangle_selection(Position::new(100.0, 100.0));
        assert_eq!(manager.mode(), &SelectionMode::Rectangle);

        // Update rectangle
        manager.update_rectangle_selection(Position::new(200.0, 200.0));

        // Check bounds
        let bounds = manager.rectangle_bounds();
        assert!(bounds.is_some());
        let (start, end) = bounds.unwrap();
        assert_eq!(start, Position::new(100.0, 100.0));
        assert_eq!(end, Position::new(200.0, 200.0));
    }

    #[test]
    fn test_keyboard_navigation() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Navigate to first node
        let first = manager.navigate_selection(&graph, NavigationDirection::Next);
        assert!(first.is_some());
        assert_eq!(manager.selection_count(), 1);

        // Navigate to next node
        let second = manager.navigate_selection(&graph, NavigationDirection::Next);
        assert!(second.is_some());
        assert_eq!(manager.selection_count(), 1);
        assert_ne!(first, second);

        // Navigate back
        let back = manager.navigate_selection(&graph, NavigationDirection::Previous);
        assert!(back.is_some());
        assert_eq!(back, first);
    }

    #[test]
    fn test_keyboard_shortcuts() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Test select all
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::SelectAll);
        assert_eq!(manager.selection_count(), 4);

        // Test escape
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::Escape);
        assert_eq!(manager.selection_count(), 0);

        // Test arrow navigation
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowRight);
        assert_eq!(manager.selection_count(), 1);
    }

    #[test]
    fn test_destructive_keyboard_shortcuts() {
        let mut manager = SelectionManager::new();
        let mut graph = create_test_graph();

        // Select a node
        manager.select_node("node1".into());
        assert_eq!(manager.selection_count(), 1);

        // Delete selected node
        manager.handle_destructive_keyboard_shortcut(&mut graph, KeyboardShortcut::Delete);
        assert_eq!(manager.selection_count(), 0);
        assert_eq!(graph.node_count(), 3);
    }

    #[test]
    fn test_visual_feedback_basic() {
        let mut manager = SelectionManager::new();
        let node_id: NodeId = "node1".into();

        // Set hover state
        manager.set_hover_state(&node_id, true);
        assert!(manager.has_visual_feedback(&node_id));
        assert!(manager.get_visual_feedback(&node_id).unwrap().is_hovered());

        // Set highlight state
        manager.set_highlight_state(&node_id, true);
        assert!(manager.get_visual_feedback(&node_id).unwrap().is_highlighted());

        // Clear hover state
        manager.set_hover_state(&node_id, false);
        assert!(manager.has_visual_feedback(&node_id)); // Still has highlight

        // Clear highlight state
        manager.set_highlight_state(&node_id, false);
        assert!(!manager.has_visual_feedback(&node_id)); // No more feedback
    }

    #[test]
    fn test_visual_feedback_animation() {
        let mut manager = SelectionManager::new();
        let node_id: NodeId = "node1".into();

        // Set hover state first to create visual feedback
        manager.set_hover_state(&node_id, true);
        
        // Set animation progress
        manager.set_animation_progress(&node_id, 0.5);
        assert!(manager.has_visual_feedback(&node_id));
        assert_eq!(manager.get_visual_feedback(&node_id).unwrap().animation_progress(), 0.5);

        // Clamp animation progress
        manager.set_animation_progress(&node_id, 1.5);
        assert_eq!(manager.get_visual_feedback(&node_id).unwrap().animation_progress(), 1.0);

        manager.set_animation_progress(&node_id, -0.5);
        assert_eq!(manager.get_visual_feedback(&node_id).unwrap().animation_progress(), 0.0);
    }

    #[test]
    fn test_visual_feedback_selection_integration() {
        let mut manager = SelectionManager::new();
        let node1: NodeId = "node1".into();
        let node2: NodeId = "node2".into();

        // Select first node
        manager.select_node(node1.clone());
        assert!(manager.get_visual_feedback(&node1).unwrap().is_selected());

        // Select second node - should clear first node's selection state
        manager.select_node(node2.clone());

        // First node should lose selection visual feedback
        if let Some(feedback1) = manager.get_visual_feedback(&node1) {
            assert!(!feedback1.is_selected());
        } else {
            // Or might be completely removed if no other states
            assert!(!manager.has_visual_feedback(&node1));
        }

        // Second node should have selection visual feedback
        assert!(manager.get_visual_feedback(&node2).unwrap().is_selected());
    }

    #[test]
    fn test_visual_feedback_batch_operations() {
        let mut manager = SelectionManager::new();
        let nodes: Vec<NodeId> = (0..10)
            .map(|i| NodeId::from(format!("node{}", i)))
            .collect();

        // Set all nodes to different states
        for (i, node_id) in nodes.iter().enumerate() {
            if i % 3 == 0 {
                manager.select_node(node_id.clone());
            }
            if i % 2 == 0 {
                manager.set_hover_state(node_id, true);
            }
            if i % 5 == 0 {
                manager.set_highlight_state(node_id, true);
            }
        }

        // Verify states
        let feedback_nodes = manager.nodes_with_visual_feedback();
        assert!(!feedback_nodes.is_empty());

        // Clear all visual feedback
        manager.clear_all_visual_feedbacks();

        // Verify all feedback is cleared
        for node_id in &nodes {
            assert!(!manager.has_visual_feedback(node_id));
        }

        // But selection state should remain in the selection manager
        assert!(!manager.selected_nodes().is_empty()); // Some nodes were selected
    }
}
