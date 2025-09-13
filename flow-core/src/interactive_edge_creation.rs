//! Tests for interactive edge creation system integration
//!
//! This module contains tests that define the expected behavior of interactive edge creation
//! before implementing the functionality (TDD Red phase).

#[cfg(test)]
mod tests {
    use crate::handle::{Handle, HandlePosition};
    use crate::types::{Position, Size};
    use crate::*;

    #[test]
    fn test_start_edge_creation_from_source_handle() {
        // Test: Starting edge creation from a source handle should create a preview edge
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create a node with a source handle
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(node).unwrap();

        // Start edge creation from the source handle
        let handle_pos = Position::new(180.0, 130.0); // Right side of node
        let result = edge_creator.start_edge_creation(&graph, "node1", "output", handle_pos);

        assert!(result.is_ok());
        assert!(edge_creator.is_creating_edge());

        let preview = edge_creator.get_preview_edge();
        assert!(preview.is_some());

        let preview = preview.unwrap();
        assert_eq!(preview.source_node, "node1".into());
        assert_eq!(preview.source_handle, Some("output".to_string()));
        assert_eq!(preview.start_position, handle_pos);
    }

    #[test]
    fn test_update_edge_creation_preview() {
        // Test: Moving mouse during edge creation should update preview edge end position
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Set up initial edge creation
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(node).unwrap();

        let start_pos = Position::new(180.0, 130.0);
        edge_creator
            .start_edge_creation(&graph, "node1", "output", start_pos)
            .unwrap();

        // Update preview edge position
        let mouse_pos = Position::new(250.0, 180.0);
        edge_creator.update_edge_preview(mouse_pos);

        let preview = edge_creator.get_preview_edge().unwrap();
        assert_eq!(preview.end_position, mouse_pos);
        assert_eq!(preview.start_position, start_pos);
    }

    #[test]
    fn test_complete_edge_creation_to_target_handle() {
        // Test: Completing edge creation to a valid target handle should create actual edge
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create nodes with compatible handles
        let mut source_node = Node::simple("node1", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("node2", Position::new(300.0, 150.0));
        target_node
            .add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Start edge creation
        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();

        // Complete edge creation to target handle
        let target_pos = Position::new(300.0, 180.0);
        let result =
            edge_creator.complete_edge_creation(&mut graph, "node2", Some("input"), target_pos);

        assert!(result.is_ok());
        assert!(!edge_creator.is_creating_edge());
        assert!(edge_creator.get_preview_edge().is_none());

        // Verify actual edge was created
        assert_eq!(graph.edge_count(), 1);
        let edge = graph.edges().next().unwrap();
        assert_eq!(edge.source, "node1".into());
        assert_eq!(edge.target, "node2".into());
        assert_eq!(edge.source_handle, Some("output".to_string()));
        assert_eq!(edge.target_handle, Some("input".to_string()));
    }

    #[test]
    fn test_cancel_edge_creation() {
        // Test: Canceling edge creation should clear preview edge
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Set up edge creation
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(node).unwrap();

        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();
        assert!(edge_creator.is_creating_edge());

        // Cancel edge creation
        edge_creator.cancel_edge_creation();

        assert!(!edge_creator.is_creating_edge());
        assert!(edge_creator.get_preview_edge().is_none());
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_edge_creation_handle_validation() {
        // Test: Edge creation should validate handle compatibility
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create nodes with incompatible handle types
        let mut source_node = Node::simple("node1", Position::new(100.0, 100.0));
        source_node
            .add_handle(
                Handle::source("data_out", HandlePosition::Right)
                    .with_connection_types(vec!["data".to_string()]),
            )
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("node2", Position::new(300.0, 150.0));
        target_node
            .add_handle(
                Handle::target("control_in", HandlePosition::Left)
                    .with_connection_types(vec!["control".to_string()]),
            )
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Start edge creation
        edge_creator
            .start_edge_creation(&graph, "node1", "data_out", Position::new(180.0, 130.0))
            .unwrap();

        // Try to complete with incompatible handle
        let result = edge_creator.complete_edge_creation(
            &mut graph,
            "node2",
            Some("control_in"),
            Position::new(300.0, 180.0),
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FlowError::InvalidConnection { .. }
        ));
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_edge_creation_connection_limits() {
        // Test: Edge creation should respect handle connection limits
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create source node with connection limit
        let mut source_node = Node::simple("node1", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("output", HandlePosition::Right).with_connection_limit(1))
            .unwrap();
        graph.add_node(source_node).unwrap();

        // Create two target nodes
        let mut target1 = Node::simple("node2", Position::new(300.0, 100.0));
        target1
            .add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();
        graph.add_node(target1).unwrap();

        let mut target2 = Node::simple("node3", Position::new(300.0, 200.0));
        target2
            .add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();
        graph.add_node(target2).unwrap();

        // Create first edge (should succeed)
        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();
        let result1 = edge_creator.complete_edge_creation(
            &mut graph,
            "node2",
            Some("input"),
            Position::new(300.0, 130.0),
        );
        assert!(result1.is_ok());

        // Try to create second edge from same handle (should fail)
        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();
        let result2 = edge_creator.complete_edge_creation(
            &mut graph,
            "node3",
            Some("input"),
            Position::new(300.0, 230.0),
        );

        assert!(result2.is_err());
        assert!(matches!(
            result2.unwrap_err(),
            FlowError::ConnectionLimitExceeded { .. }
        ));
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_edge_creation_to_node_without_handle() {
        // Test: Edge creation to a node without specifying handle should work
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create nodes
        let mut source_node = Node::simple("node1", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        graph
            .add_node(Node::simple("node2", Position::new(300.0, 150.0)))
            .unwrap();

        // Start edge creation
        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();

        // Complete to node without specific handle
        let result = edge_creator.complete_edge_creation(
            &mut graph,
            "node2",
            None,
            Position::new(350.0, 180.0),
        );

        assert!(result.is_ok());
        assert_eq!(graph.edge_count(), 1);

        let edge = graph.edges().next().unwrap();
        assert_eq!(edge.source_handle, Some("output".to_string()));
        assert!(edge.target_handle.is_none()); // No specific target handle
    }

    #[test]
    fn test_edge_creation_hit_detection() {
        // Test: Edge creation should detect when mouse is over valid drop targets
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create nodes with handles
        let mut source_node = Node::simple("node1", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("node2", Position::new(300.0, 150.0));
        target_node.set_size(Size::new(80.0, 60.0));
        target_node
            .add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Start edge creation
        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();

        // Test hit detection for target handle
        let target_handle_pos = Position::new(300.0, 180.0);
        let hit_result = edge_creator.get_drop_target(&graph, target_handle_pos, 10.0);

        assert!(hit_result.is_some());
        let (node_id, handle_id) = hit_result.unwrap();
        assert_eq!(node_id, "node2".into());
        assert_eq!(handle_id, Some("input".to_string()));

        // Test hit detection for node (not handle)
        let node_center = Position::new(340.0, 180.0);
        let node_hit = edge_creator.get_drop_target(&graph, node_center, 10.0);

        assert!(node_hit.is_some());
        let (node_id, handle_id) = node_hit.unwrap();
        assert_eq!(node_id, "node2".into());
        assert!(handle_id.is_none()); // Hit node, not specific handle
    }

    #[test]
    fn test_edge_creation_visual_feedback() {
        // Test: Edge creation should provide visual feedback for valid/invalid connections
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create compatible nodes
        let mut source_node = Node::simple("node1", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("node2", Position::new(300.0, 150.0));
        target_node
            .add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Start edge creation
        edge_creator
            .start_edge_creation(&graph, "node1", "output", Position::new(180.0, 130.0))
            .unwrap();

        // Get feedback for valid connection
        let feedback = edge_creator.get_connection_feedback(&graph, "node2", Some("input"));
        assert!(feedback.is_valid);
        assert!(feedback.can_connect);
        assert!(feedback.message.is_none());

        // Get feedback for invalid connection (self)
        let self_feedback = edge_creator.get_connection_feedback(&graph, "node1", Some("output"));
        assert!(!self_feedback.is_valid);
        assert!(!self_feedback.can_connect);
        assert!(self_feedback.message.is_some());
    }

    #[test]
    fn test_edge_creation_performance_with_many_nodes() {
        // Test: Edge creation should perform well with many nodes
        let mut graph: Graph<(), ()> = Graph::new();
        let mut edge_creator = graph.create_edge_creator();

        // Create many nodes
        for i in 0..100 {
            let mut node = Node::simple(
                format!("node{}", i),
                Position::new(i as f64 * 10.0, i as f64 * 10.0),
            );
            node.add_handle(Handle::source("output", HandlePosition::Right))
                .unwrap();
            node.add_handle(Handle::target("input", HandlePosition::Left))
                .unwrap();
            graph.add_node(node).unwrap();
        }

        // Start edge creation and measure performance
        let start_time = std::time::Instant::now();

        edge_creator
            .start_edge_creation(&graph, "node0", "output", Position::new(10.0, 5.0))
            .unwrap();

        // Test hit detection performance
        let hit_result = edge_creator.get_drop_target(&graph, Position::new(100.0, 105.0), 10.0);

        let elapsed = start_time.elapsed();

        // Should complete quickly even with many nodes
        assert!(
            elapsed.as_millis() < 50,
            "Edge creation took too long: {:?}",
            elapsed
        );
        assert!(hit_result.is_some());
    }
}
