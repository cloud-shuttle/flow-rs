//! Tests for Handle Connection Counting system integration
//!
//! This module contains tests that define the expected behavior of connection counting
//! before implementing the functionality (TDD Red phase).

#[cfg(test)]
mod tests {
    use crate::handle::{Handle, HandlePosition};
    use crate::types::Position;
    use crate::*;

    #[test]
    fn test_connection_count_with_no_edges() {
        // Test: Handle with no connections should return count of 0
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a node with handles
        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        node.add_handle(Handle::target("input", HandlePosition::Left))
            .unwrap();
        graph.add_node(node).unwrap();

        // Get the node and check connection counts
        let output_count = graph
            .get_handle_connections(&"node1".into(), &"output".into())
            .len();
        let input_count = graph
            .get_handle_connections(&"node1".into(), &"input".into())
            .len();

        assert_eq!(output_count, 0);
        assert_eq!(input_count, 0);
    }

    #[test]
    fn test_connection_count_with_single_edge() {
        // Test: Handle connected to one edge should return count of 1
        let mut graph: Graph<(), ()> = Graph::new();

        // Create source and target nodes with handles
        let mut source_node = Node::simple("source", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("out", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("target", Position::new(300.0, 100.0));
        target_node
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Create an edge connecting the handles
        let edge = Edge::new("edge1", "source", "target", ())
            .with_source_handle("out")
            .with_target_handle("in");
        graph.add_edge(edge).unwrap();

        // Check connection counts
        let source_count = graph
            .get_handle_connections(&"source".into(), &"out".into())
            .len();
        let target_count = graph
            .get_handle_connections(&"target".into(), &"in".into())
            .len();

        assert_eq!(source_count, 1);
        assert_eq!(target_count, 1);
    }

    #[test]
    fn test_connection_count_with_multiple_edges() {
        // Test: Handle connected to multiple edges should return correct count
        let mut graph: Graph<(), ()> = Graph::new();

        // Create hub node with one output handle
        let mut hub_node = Node::simple("hub", Position::new(100.0, 100.0));
        hub_node
            .add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(hub_node).unwrap();

        // Create multiple target nodes
        for i in 1..=3 {
            let mut target_node = Node::simple(
                format!("target{}", i),
                Position::new(300.0, i as f64 * 100.0),
            );
            target_node
                .add_handle(Handle::target("input", HandlePosition::Left))
                .unwrap();
            graph.add_node(target_node).unwrap();

            // Connect hub to each target
            let edge = Edge::new(format!("edge{}", i), "hub", format!("target{}", i), ())
                .with_source_handle("output")
                .with_target_handle("input");
            graph.add_edge(edge).unwrap();
        }

        // Check hub output handle has 3 connections
        let output_count = graph
            .get_handle_connections(&"hub".into(), &"output".into())
            .len();
        assert_eq!(output_count, 3);

        // Check each target has 1 connection
        for i in 1..=3 {
            let input_count = graph
                .get_handle_connections(&format!("target{}", i).into(), &"input".into())
                .len();
            assert_eq!(input_count, 1);
        }
    }

    #[test]
    fn test_connection_count_excludes_unrelated_handles() {
        // Test: Connection count should only count edges for specific handle
        let mut graph: Graph<(), ()> = Graph::new();

        // Create node with multiple handles
        let mut node = Node::simple("multi_handle", Position::new(100.0, 100.0));
        node.add_handle(Handle::source("output1", HandlePosition::Right))
            .unwrap();
        node.add_handle(Handle::source("output2", HandlePosition::Top))
            .unwrap();
        node.add_handle(Handle::target("input1", HandlePosition::Left))
            .unwrap();
        graph.add_node(node).unwrap();

        // Create target nodes for each handle
        let mut target1 = Node::simple("target1", Position::new(300.0, 100.0));
        target1
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();
        graph.add_node(target1).unwrap();

        let mut target2 = Node::simple("target2", Position::new(100.0, 300.0));
        target2
            .add_handle(Handle::target("in", HandlePosition::Bottom))
            .unwrap();
        graph.add_node(target2).unwrap();

        // Connect only output1 to target1
        let edge1 = Edge::new("edge1", "multi_handle", "target1", ())
            .with_source_handle("output1")
            .with_target_handle("in");
        graph.add_edge(edge1).unwrap();

        // Connect only output2 to target2
        let edge2 = Edge::new("edge2", "multi_handle", "target2", ())
            .with_source_handle("output2")
            .with_target_handle("in");
        graph.add_edge(edge2).unwrap();

        // Check each handle has exactly 1 connection
        assert_eq!(
            graph
                .get_handle_connections(&"multi_handle".into(), &"output1".into())
                .len(),
            1
        );
        assert_eq!(
            graph
                .get_handle_connections(&"multi_handle".into(), &"output2".into())
                .len(),
            1
        );
        assert_eq!(
            graph
                .get_handle_connections(&"multi_handle".into(), &"input1".into())
                .len(),
            0
        );
    }

    #[test]
    fn test_connection_count_with_edge_removal() {
        // Test: Connection count should decrease when edges are removed
        let mut graph: Graph<(), ()> = Graph::new();

        // Create connected nodes
        let mut source_node = Node::simple("source", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("out", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("target", Position::new(300.0, 100.0));
        target_node
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Create edge
        let edge = Edge::new("edge1", "source", "target", ())
            .with_source_handle("out")
            .with_target_handle("in");
        graph.add_edge(edge).unwrap();

        // Verify initial counts
        assert_eq!(
            graph
                .get_handle_connections(&"source".into(), &"out".into())
                .len(),
            1
        );
        assert_eq!(
            graph
                .get_handle_connections(&"target".into(), &"in".into())
                .len(),
            1
        );

        // Remove edge
        graph.remove_edge(&"edge1".into()).unwrap();

        // Verify counts are now zero
        assert_eq!(
            graph
                .get_handle_connections(&"source".into(), &"out".into())
                .len(),
            0
        );
        assert_eq!(
            graph
                .get_handle_connections(&"target".into(), &"in".into())
                .len(),
            0
        );
    }

    #[test]
    fn test_connection_count_with_nonexistent_handle() {
        // Test: Connection count for non-existent handle should return 0
        let mut graph: Graph<(), ()> = Graph::new();

        let node = Node::simple("node1", Position::new(100.0, 100.0));
        graph.add_node(node).unwrap();

        let count = graph
            .get_handle_connections(&"node1".into(), &"nonexistent".into())
            .len();

        assert_eq!(count, 0);
    }

    #[test]
    fn test_can_accept_connection_with_no_limit() {
        // Test: Handle without connection limit should always accept connections
        let mut graph: Graph<(), ()> = Graph::new();

        let mut node = Node::simple("node1", Position::new(100.0, 100.0));
        node.add_handle(Handle::source("unlimited", HandlePosition::Right))
            .unwrap();
        graph.add_node(node).unwrap();

        let can_accept = graph.can_handle_accept_connection(&"node1".into(), &"unlimited".into());

        assert!(can_accept);
    }

    #[test]
    fn test_can_accept_connection_with_limit_not_reached() {
        // Test: Handle with connection limit should accept when under limit
        let mut graph: Graph<(), ()> = Graph::new();

        let mut source_node = Node::simple("source", Position::new(100.0, 100.0));
        source_node
            .add_handle(
                Handle::source("limited_out", HandlePosition::Right).with_connection_limit(2),
            )
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("target", Position::new(300.0, 100.0));
        target_node
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Add one connection (under limit of 2)
        let edge = Edge::new("edge1", "source", "target", ())
            .with_source_handle("limited_out")
            .with_target_handle("in");
        graph.add_edge(edge).unwrap();

        let can_accept =
            graph.can_handle_accept_connection(&"source".into(), &"limited_out".into());

        assert!(can_accept);
    }

    #[test]
    fn test_can_accept_connection_with_limit_reached() {
        // Test: Handle with connection limit should reject when limit reached
        let mut graph: Graph<(), ()> = Graph::new();

        let mut source_node = Node::simple("source", Position::new(100.0, 100.0));
        source_node
            .add_handle(
                Handle::source("limited_out", HandlePosition::Right).with_connection_limit(1),
            )
            .unwrap();
        graph.add_node(source_node).unwrap();

        let mut target_node = Node::simple("target", Position::new(300.0, 100.0));
        target_node
            .add_handle(Handle::target("in", HandlePosition::Left))
            .unwrap();
        graph.add_node(target_node).unwrap();

        // Add connection that reaches limit
        let edge = Edge::new("edge1", "source", "target", ())
            .with_source_handle("limited_out")
            .with_target_handle("in");
        graph.add_edge(edge).unwrap();

        let can_accept =
            graph.can_handle_accept_connection(&"source".into(), &"limited_out".into());

        assert!(!can_accept);
    }

    #[test]
    fn test_connection_count_performance_with_many_edges() {
        // Test: Connection counting should perform well with many edges in graph
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a hub node
        let mut hub = Node::simple("hub", Position::new(100.0, 100.0));
        hub.add_handle(Handle::source("output", HandlePosition::Right))
            .unwrap();
        graph.add_node(hub).unwrap();

        // Create many target nodes and edges
        let edge_count = 100;
        for i in 0..edge_count {
            let mut target = Node::simple(
                format!("target{}", i),
                Position::new(300.0, i as f64 * 10.0),
            );
            target
                .add_handle(Handle::target("input", HandlePosition::Left))
                .unwrap();
            graph.add_node(target).unwrap();

            let edge = Edge::new(format!("edge{}", i), "hub", format!("target{}", i), ())
                .with_source_handle("output")
                .with_target_handle("input");
            graph.add_edge(edge).unwrap();
        }

        // Measure performance of connection counting
        let start_time = std::time::Instant::now();

        let connection_count = graph
            .get_handle_connections(&"hub".into(), &"output".into())
            .len();

        let elapsed = start_time.elapsed();

        assert_eq!(connection_count, edge_count);
        assert!(
            elapsed.as_millis() < 50,
            "Connection counting took too long: {:?}",
            elapsed
        );
    }

    #[test]
    fn test_connection_count_with_edges_without_handles() {
        // Test: Connection count should only count edges with matching handle references
        let mut graph: Graph<(), ()> = Graph::new();

        // Create nodes with handles
        let mut source_node = Node::simple("source", Position::new(100.0, 100.0));
        source_node
            .add_handle(Handle::source("specific_handle", HandlePosition::Right))
            .unwrap();
        graph.add_node(source_node).unwrap();

        let target_node = Node::simple("target", Position::new(300.0, 100.0));
        graph.add_node(target_node).unwrap();

        // Create edge with specific handle reference
        let edge_with_handle = Edge::new("edge_with_handle", "source", "target", ())
            .with_source_handle("specific_handle");
        graph.add_edge(edge_with_handle).unwrap();

        // Create edge without handle reference (node-to-node connection)
        let edge_without_handle = Edge::new("edge_without_handle", "source", "target", ());
        graph.add_edge(edge_without_handle).unwrap();

        // Connection count for specific handle should only count the edge that references it
        let specific_count = graph
            .get_handle_connections(&"source".into(), &"specific_handle".into())
            .len();

        assert_eq!(specific_count, 1); // Only the edge that specifically references this handle
    }
}
