//! Tests for drag & drop node integration with Graph, Node, and Selection systems
//!
//! This module contains tests that define the expected behavior of drag & drop
//! before implementing the functionality (TDD Red phase).

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::types::{Position, Size};
    use crate::selection::{SelectionManager, SelectionMode};

    #[test]
    fn test_single_node_drag_updates_position() {
        // Test: Dragging a single node should update its position in the graph
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        // Create a node at initial position
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.set_size(Size::new(80.0, 60.0));
        graph.add_node(node).unwrap();

        // Select the node
        selection.select_node("node1".into());

        // Simulate drag operation
        let drag_delta = Position::new(50.0, 30.0);
        let result = graph.apply_node_drag(&selection.selected_nodes(), drag_delta);

        // Should successfully apply drag
        assert!(result.is_ok());

        // Node position should be updated
        let updated_node = graph.get_node(&"node1".into()).unwrap();
        assert_eq!(updated_node.position, Position::new(150.0, 130.0));
    }

    #[test]
    fn test_multi_node_drag_updates_all_positions() {
        // Test: Dragging multiple selected nodes should update all positions
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();
        selection.set_mode(SelectionMode::Multi);

        // Create multiple nodes
        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("node2", Position::new(200.0, 150.0))).unwrap();
        graph.add_node(Node::simple("node3", Position::new(300.0, 200.0))).unwrap();

        // Select multiple nodes
        selection.select_node("node1".into());
        selection.select_node("node2".into());
        selection.select_node("node3".into());

        // Apply drag to all selected nodes
        let drag_delta = Position::new(25.0, -15.0);
        let result = graph.apply_node_drag(&selection.selected_nodes(), drag_delta);

        assert!(result.is_ok());

        // All selected nodes should have updated positions
        let node1 = graph.get_node(&"node1".into()).unwrap();
        let node2 = graph.get_node(&"node2".into()).unwrap();
        let node3 = graph.get_node(&"node3".into()).unwrap();

        assert_eq!(node1.position, Position::new(125.0, 85.0));
        assert_eq!(node2.position, Position::new(225.0, 135.0));
        assert_eq!(node3.position, Position::new(325.0, 185.0));
    }

    #[test]
    fn test_drag_with_bounds_constraint() {
        // Test: Drag operations should respect boundary constraints
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        // Create node near the edge of bounds
        graph.add_node(Node::simple("node1", Position::new(10.0, 10.0))).unwrap();
        selection.select_node("node1".into());

        // Set canvas bounds
        let bounds = crate::types::Rect::new(0.0, 0.0, 400.0, 300.0);

        // Try to drag beyond bounds
        let large_negative_delta = Position::new(-20.0, -20.0);
        let result = graph.apply_node_drag_with_bounds(
            &selection.selected_nodes(),
            large_negative_delta,
            Some(bounds)
        );

        assert!(result.is_ok());

        // Node should be constrained to bounds minimum
        let node = graph.get_node(&"node1".into()).unwrap();
        assert!(node.position.x >= 0.0);
        assert!(node.position.y >= 0.0);
    }

    #[test]
    fn test_drag_with_grid_snapping() {
        // Test: Drag operations should support grid snapping
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        graph.add_node(Node::simple("node1", Position::new(115.0, 87.0))).unwrap();
        selection.select_node("node1".into());

        // Drag with small offset that should snap to grid
        let small_delta = Position::new(7.0, 18.0);
        let grid_size = 20.0;

        let result = graph.apply_node_drag_with_snap(
            &selection.selected_nodes(),
            small_delta,
            grid_size
        );

        assert!(result.is_ok());

        // Position should be snapped to grid
        let node = graph.get_node(&"node1".into()).unwrap();
        // Should snap to nearest 20px grid: (120, 100)
        assert_eq!(node.position.x, 120.0);
        assert_eq!(node.position.y, 100.0);
    }

    #[test]
    fn test_drag_nonexistent_node_returns_error() {
        // Test: Dragging nonexistent nodes should return error
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        selection.select_node("nonexistent".into());
        let delta = Position::new(10.0, 10.0);

        let result = graph.apply_node_drag(&selection.selected_nodes(), delta);

        // Should return error for nonexistent node
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FlowError::NodeNotFound { .. }));
    }

    #[test]
    fn test_drag_updates_connected_edges() {
        // Test: Dragging nodes should update positions of connected edges
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        // Create nodes and connect them
        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("node2", Position::new(200.0, 200.0))).unwrap();

        let edge = Edge::simple("edge1", "node1", "node2");
        graph.add_edge(edge).unwrap();

        // Drag node1
        selection.select_node("node1".into());
        let delta = Position::new(50.0, 25.0);

        let result = graph.apply_node_drag(&selection.selected_nodes(), delta);
        assert!(result.is_ok());

        // Verify node moved
        let node1 = graph.get_node(&"node1".into()).unwrap();
        assert_eq!(node1.position, Position::new(150.0, 125.0));

        // Edge should still connect the nodes (implicit visual update)
        let edge = graph.get_edge(&"edge1".into()).unwrap();
        assert_eq!(edge.source.as_str(), "node1");
        assert_eq!(edge.target.as_str(), "node2");
    }

    #[test]
    fn test_drag_with_constraint_functions() {
        // Test: Drag operations should support custom constraint functions
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        selection.select_node("node1".into());

        // Define constraint: only horizontal movement allowed
        let horizontal_only = |pos: Position| Position::new(pos.x, 100.0);

        let delta = Position::new(30.0, 50.0); // Try to move both x and y
        let result = graph.apply_node_drag_with_constraint(
            &selection.selected_nodes(),
            delta,
            horizontal_only
        );

        assert!(result.is_ok());

        // Y should remain unchanged due to constraint
        let node = graph.get_node(&"node1".into()).unwrap();
        assert_eq!(node.position, Position::new(130.0, 100.0)); // Only X changed
    }

    #[test]
    fn test_drag_operation_history() {
        // Test: Drag operations should be recordable for undo/redo
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        selection.select_node("node1".into());

        // Record initial state
        let initial_pos = graph.get_node(&"node1".into()).unwrap().position;

        // Apply drag operation
        let delta = Position::new(20.0, 15.0);
        let drag_operation = graph.create_drag_operation(&selection.selected_nodes(), delta);

        assert!(drag_operation.is_ok());
        let operation = drag_operation.unwrap();

        // Operation should contain enough info for undo/redo
        assert_eq!(operation.affected_nodes().len(), 1);
        assert!(operation.affected_nodes().contains(&"node1".into()));
        assert_eq!(operation.delta(), delta);

        // Apply the operation
        let result = operation.apply_to_graph(&mut graph);
        assert!(result.is_ok());

        // Verify position changed
        let final_pos = graph.get_node(&"node1".into()).unwrap().position;
        assert_eq!(final_pos, Position::new(120.0, 115.0));

        // Should be able to create inverse operation for undo
        let undo_operation = operation.create_inverse();
        let undo_result = undo_operation.apply_to_graph(&mut graph);
        assert!(undo_result.is_ok());

        // Position should be back to initial
        let undone_pos = graph.get_node(&"node1".into()).unwrap().position;
        assert_eq!(undone_pos, initial_pos);
    }

    #[test]
    fn test_drag_performance_with_many_nodes() {
        // Test: Drag operations should perform well with large numbers of nodes
        let mut graph: Graph<(), ()> = Graph::new();
        let mut selection = SelectionManager::new();
        selection.set_mode(SelectionMode::Multi);

        // Create many nodes
        for i in 0..100 {
            let node_id = format!("node{}", i);
            let pos = Position::new(i as f64 * 10.0, i as f64 * 10.0);
            graph.add_node(Node::simple(node_id.clone(), pos)).unwrap();

            // Select every 5th node
            if i % 5 == 0 {
                selection.select_node(node_id.into());
            }
        }

        // Should have 20 selected nodes
        assert_eq!(selection.selection_count(), 20);

        // Apply drag to all selected nodes - should complete quickly
        let delta = Position::new(5.0, 5.0);
        let start_time = std::time::Instant::now();

        let result = graph.apply_node_drag(&selection.selected_nodes(), delta);

        let elapsed = start_time.elapsed();

        // Should complete successfully and quickly (< 10ms for 20 nodes)
        assert!(result.is_ok());
        assert!(elapsed.as_millis() < 10, "Drag operation took too long: {:?}", elapsed);
    }
}
