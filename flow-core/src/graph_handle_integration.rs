//! Tests for handle integration with Graph, Node, and Edge structures
//!
//! This module contains tests that define the expected behavior of handle-based
//! edge connections before implementing the functionality.

#[cfg(test)]
mod tests {
    use crate::handle::*;
    use crate::*;

    #[test]
    fn test_node_with_handles() {
        // This test will fail until we implement handle integration
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));

        // Should be able to add handles to a node
        node.add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        node.add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();

        assert_eq!(node.handles().len(), 2);
        assert!(node.get_handle(&"output".into()).is_some());
        assert!(node.get_handle(&"input".into()).is_some());
    }

    #[test]
    fn test_node_handle_hit_detection() {
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.set_size(Size::new(80.0, 60.0));
        node.add_handle(Handle::source("right", HandlePosition::Right))
            .unwrap();

        // Should find handle at correct position
        let handle = node.handle_at_position(Position::new(180.0, 130.0), 10.0);
        assert!(handle.is_some());
        assert_eq!(handle.unwrap().id.as_str(), "right");

        // Should not find handle at wrong position
        let no_handle = node.handle_at_position(Position::new(150.0, 130.0), 10.0);
        assert!(no_handle.is_none());
    }

    #[test]
    fn test_edge_with_handle_references() {
        // Edge should be able to reference specific handles on nodes
        let edge = Edge::simple("edge1", "node1", "node2")
            .with_source_handle("output")
            .with_target_handle("input");

        assert_eq!(edge.source_handle, Some("output".to_string()));
        assert_eq!(edge.target_handle, Some("input".to_string()));
    }

    #[test]
    fn test_graph_handle_based_connection() {
        let mut graph = Graph::new();

        // Create nodes with handles
        let mut node1 = Node::simple("node1", Position::new(0.0, 0.0));
        node1
            .add_handle(Handle::source("out", HandlePosition::Right))
            .unwrap();

        let mut node2 = Node::simple("node2", Position::new(200.0, 0.0));
        node2
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // Create handle-based edge
        let edge = Edge::simple("edge1", "node1", "node2")
            .with_source_handle("out")
            .with_target_handle("in");

        // Should validate handle-based connections
        let result = graph.add_handle_edge(edge);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_handle_connection() {
        let mut graph = Graph::new();

        // Create nodes with handles
        let mut node1 = Node::simple("node1", Position::new(0.0, 0.0));
        node1
            .add_handle(Handle::source("out", HandlePosition::Right))
            .unwrap();

        let mut node2 = Node::simple("node2", Position::new(200.0, 0.0));
        node2
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // Try to create edge with non-existent handle
        let edge = Edge::simple("edge1", "node1", "node2")
            .with_source_handle("nonexistent")
            .with_target_handle("in");

        let result = graph.add_handle_edge(edge);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_connection_type_validation() {
        let mut graph = Graph::new();

        // Create nodes with typed handles
        let mut node1 = Node::simple("node1", Position::new(0.0, 0.0));
        node1
            .add_handle(
                Handle::source("data_out", HandlePosition::Right)
                    .with_connection_types(vec!["data".to_string()]),
            )
            .unwrap();

        let mut node2 = Node::simple("node2", Position::new(200.0, 0.0));
        node2
            .add_handle(
                Handle::target("control_in", HandlePosition::Left)
                    .with_connection_types(vec!["control".to_string()]),
            )
            .unwrap();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // Try to connect incompatible types
        let edge = Edge::simple("edge1", "node1", "node2")
            .with_source_handle("data_out")
            .with_target_handle("control_in");

        let result = graph.add_handle_edge(edge);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FlowError::InvalidConnection { .. }
        ));
    }

    #[test]
    fn test_handle_connection_limit_enforcement() {
        let mut graph = Graph::new();

        // Create node with limited handle
        let mut node1 = Node::simple("node1", Position::new(0.0, 0.0));
        node1
            .add_handle(Handle::source("out", HandlePosition::Right).with_connection_limit(1))
            .unwrap();

        let mut node2 = Node::simple("node2", Position::new(200.0, 0.0));
        node2
            .add_handle(Handle::target("in1", HandlePosition::Left))
            .unwrap();

        let mut node3 = Node::simple("node3", Position::new(200.0, 100.0));
        node3
            .add_handle(Handle::target("in2", HandlePosition::Left))
            .unwrap();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();
        graph.add_node(node3).unwrap();

        // First connection should succeed
        let edge1 = Edge::simple("edge1", "node1", "node2")
            .with_source_handle("out")
            .with_target_handle("in1");
        assert!(graph.add_handle_edge(edge1).is_ok());

        // Second connection should fail (exceeds limit)
        let edge2 = Edge::simple("edge2", "node1", "node3")
            .with_source_handle("out")
            .with_target_handle("in2");
        let result = graph.add_handle_edge(edge2);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FlowError::ConnectionLimitExceeded { .. }
        ));
    }

    #[test]
    fn test_graph_get_handle_connections() {
        let mut graph = Graph::new();

        // Set up nodes with handles
        let mut node1 = Node::simple("node1", Position::new(0.0, 0.0));
        node1
            .add_handle(Handle::source("out", HandlePosition::Right))
            .unwrap();

        let mut node2 = Node::simple("node2", Position::new(200.0, 0.0));
        node2
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let edge = Edge::simple("edge1", "node1", "node2")
            .with_source_handle("out")
            .with_target_handle("in");
        graph.add_handle_edge(edge).unwrap();

        // Should be able to get connections for a specific handle
        let connections = graph.get_handle_connections(&"node1".into(), &"out".into());
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0].id.as_str(), "edge1");
    }
}
