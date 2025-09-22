//! Tests for graph data structures and operations

#[cfg(test)]
mod tests {
    use crate::error::FlowError;
    use crate::types::{EdgeId, NodeId, Position, Rect, Size};
    
    // Import graph types from crate root
    use crate::{Edge, Graph, Node};

    #[test]
    fn test_node_creation() {
        let node = Node::simple("test", Position::new(10.0, 20.0));
        assert_eq!(node.id.as_str(), "test");
        assert_eq!(node.position, Position::new(10.0, 20.0));
        assert!(node.selectable);
    }

    #[test]
    fn test_node_builder() {
        let node = Node::<()>::builder("test")
            .position(100.0, 200.0)
            .size(80.0, 40.0)
            .node_type("custom")
            .selectable(false)
            .build();

        assert_eq!(node.position, Position::new(100.0, 200.0));
        assert_eq!(node.size, Size::new(80.0, 40.0));
        assert_eq!(node.node_type, Some("custom".to_string()));
        assert!(!node.selectable);
    }

    #[test]
    fn test_edge_creation() {
        let edge = Edge::simple("e1", "node1", "node2");
        assert_eq!(edge.source.as_str(), "node1");
        assert_eq!(edge.target.as_str(), "node2");
        assert!(edge.connects(&"node1".into(), &"node2".into()));
    }

    #[test]
    fn test_edge_builder() {
        let edge = Edge::<()>::builder()
            .connect("source", "target")
            .animated(true)
            .label("test connection")
            .build()
            .unwrap();

        assert!(edge.animated);
        assert_eq!(edge.label, Some("test connection".to_string()));
    }

    #[test]
    fn test_edge_builder_self_connection() {
        let result = Edge::<()>::builder().connect("node1", "node1").build();

        assert!(matches!(result, Err(FlowError::SelfConnection)));
    }

    #[test]
    fn test_graph_operations() {
        let mut graph: Graph<(), ()> = Graph::new();

        let node1 = Node::simple("node1", Position::new(0.0, 0.0));
        let node2 = Node::simple("node2", Position::new(100.0, 100.0));

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        assert_eq!(graph.node_count(), 2);
        assert!(graph.get_node(&"node1".into()).is_some());

        let edge = Edge::simple("edge1", "node1", "node2");
        graph.add_edge(edge).unwrap();

        assert_eq!(graph.edge_count(), 1);
        assert!(graph.are_connected(&"node1".into(), &"node2".into()));
    }

    #[test]
    fn test_graph_cascade_delete() {
        let mut graph: Graph<(), ()> = Graph::new();

        graph
            .add_node(Node::simple("node1", Position::zero()))
            .unwrap();
        graph
            .add_node(Node::simple("node2", Position::zero()))
            .unwrap();
        graph
            .add_edge(Edge::simple("edge1", "node1", "node2"))
            .unwrap();

        assert_eq!(graph.edge_count(), 1);

        graph.remove_node(&"node1".into()).unwrap();

        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0); // Edge should be removed
    }

    #[test]
    fn test_graph_bounds() {
        let mut graph: Graph<(), ()> = Graph::new();

        graph
            .add_node(
                Node::builder("node1")
                    .position(0.0, 0.0)
                    .size(100.0, 50.0)
                    .build(),
            )
            .unwrap();

        graph
            .add_node(
                Node::builder("node2")
                    .position(200.0, 300.0)
                    .size(100.0, 50.0)
                    .build(),
            )
            .unwrap();

        let bounds = graph.bounds().unwrap();
        assert_eq!(bounds, Rect::new(0.0, 0.0, 300.0, 350.0));
    }

    // Comprehensive Graph Operations Tests

    #[test]
    fn test_comprehensive_node_operations() {
        let mut graph: Graph<i32, ()> = Graph::new();

        // Test adding nodes
        let node1 = Node::new("node1", Position::new(0.0, 0.0), 42);
        let node2 = Node::new("node2", Position::new(100.0, 100.0), 84);

        assert!(graph.add_node(node1.clone()).is_ok());
        assert!(graph.add_node(node2.clone()).is_ok());
        assert_eq!(graph.node_count(), 2);
        assert!(!graph.is_empty());

        // Test getting nodes
        let retrieved = graph.get_node(&NodeId::from("node1"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, 42);

        // Test duplicate node error
        let duplicate = Node::new("node1", Position::new(50.0, 50.0), 100);
        let result = graph.add_node(duplicate);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FlowError::DuplicateNodeId { .. }
        ));

        // Test node removal
        let removed = graph.remove_node(&NodeId::from("node1"));
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().data, 42);
        assert_eq!(graph.node_count(), 1);

        // Test removing non-existent node
        let not_found = graph.remove_node(&NodeId::from("nonexistent"));
        assert!(not_found.is_err());
        assert!(matches!(
            not_found.unwrap_err(),
            FlowError::NodeNotFound { .. }
        ));
    }

    #[test]
    fn test_comprehensive_edge_operations() {
        let mut graph: Graph<(), String> = Graph::new();

        // Add nodes first
        graph
            .add_node(Node::new("A", Position::new(0.0, 0.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("B", Position::new(100.0, 0.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("C", Position::new(200.0, 0.0), ()))
            .unwrap();

        // Test adding edges
        let edge1 = Edge::new("edge1", "A", "B", "connects_to".to_string());
        let edge2 = Edge::new("edge2", "B", "C", "flows_into".to_string());

        assert!(graph.add_edge(edge1.clone()).is_ok());
        assert!(graph.add_edge(edge2.clone()).is_ok());
        assert_eq!(graph.edge_count(), 2);

        // Test getting edges
        let retrieved = graph.get_edge(&EdgeId::from("edge1"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, "connects_to");

        // Test edge to non-existent node
        let invalid_edge = Edge::new("invalid", "A", "nonexistent", "error".to_string());
        let result = graph.add_edge(invalid_edge);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FlowError::NodeNotFound { .. }
        ));

        // Test edge removal
        let removed = graph.remove_edge(&EdgeId::from("edge1"));
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().data, "connects_to");
        assert_eq!(graph.edge_count(), 1);

        // Test removing non-existent edge
        let not_found = graph.remove_edge(&EdgeId::from("nonexistent"));
        assert!(not_found.is_err());
        assert!(matches!(
            not_found.unwrap_err(),
            FlowError::EdgeNotFound { .. }
        ));
    }

    #[test]
    fn test_graph_connectivity() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a simple graph: A -> B -> C
        graph
            .add_node(Node::simple("A", Position::new(0.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("B", Position::new(100.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("C", Position::new(200.0, 0.0)))
            .unwrap();

        graph
            .add_edge(Edge::simple("AB", "A", "B"))
            .unwrap();
        graph
            .add_edge(Edge::simple("BC", "B", "C"))
            .unwrap();

        // Test connectivity
        assert!(graph.are_connected(&"A".into(), &"B".into()));
        assert!(graph.are_connected(&"B".into(), &"C".into()));
        assert!(!graph.are_connected(&"A".into(), &"C".into()));

        // Test edge queries
        let a_edges = graph.get_connected_edges(&"A".into());
        assert_eq!(a_edges.len(), 1);
        assert_eq!(a_edges[0].id.as_str(), "AB");

        let b_incoming = graph.get_incoming_edges(&"B".into());
        assert_eq!(b_incoming.len(), 1);
        assert_eq!(b_incoming[0].id.as_str(), "AB");

        let b_outgoing = graph.get_outgoing_edges(&"B".into());
        assert_eq!(b_outgoing.len(), 1);
        assert_eq!(b_outgoing[0].id.as_str(), "BC");
    }

    #[test]
    fn test_graph_clear() {
        let mut graph: Graph<(), ()> = Graph::new();

        graph
            .add_node(Node::simple("A", Position::new(0.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("B", Position::new(100.0, 0.0)))
            .unwrap();
        graph
            .add_edge(Edge::simple("AB", "A", "B"))
            .unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(!graph.is_empty());

        graph.clear();

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a DAG: A -> B -> C, A -> D -> C
        graph
            .add_node(Node::simple("A", Position::new(0.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("B", Position::new(100.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("C", Position::new(200.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("D", Position::new(100.0, 100.0)))
            .unwrap();

        graph
            .add_edge(Edge::simple("AB", "A", "B"))
            .unwrap();
        graph
            .add_edge(Edge::simple("BC", "B", "C"))
            .unwrap();
        graph
            .add_edge(Edge::simple("AD", "A", "D"))
            .unwrap();
        graph
            .add_edge(Edge::simple("DC", "D", "C"))
            .unwrap();

        let sort_result = graph.topological_sort();
        assert!(sort_result.is_ok());

        let sorted = sort_result.unwrap();
        assert_eq!(sorted.len(), 4);

        // A should come before B and D
        let a_pos = sorted.iter().position(|id| id.as_str() == "A").unwrap();
        let b_pos = sorted.iter().position(|id| id.as_str() == "B").unwrap();
        let d_pos = sorted.iter().position(|id| id.as_str() == "D").unwrap();
        let c_pos = sorted.iter().position(|id| id.as_str() == "C").unwrap();

        assert!(a_pos < b_pos);
        assert!(a_pos < d_pos);
        assert!(b_pos < c_pos);
        assert!(d_pos < c_pos);
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a cycle: A -> B -> C -> A
        graph
            .add_node(Node::simple("A", Position::new(0.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("B", Position::new(100.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("C", Position::new(200.0, 0.0)))
            .unwrap();

        graph
            .add_edge(Edge::simple("AB", "A", "B"))
            .unwrap();
        graph
            .add_edge(Edge::simple("BC", "B", "C"))
            .unwrap();
        graph
            .add_edge(Edge::simple("CA", "C", "A"))
            .unwrap();

        let sort_result = graph.topological_sort();
        assert!(sort_result.is_err());
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Test acyclic graph
        graph
            .add_node(Node::simple("A", Position::new(0.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("B", Position::new(100.0, 0.0)))
            .unwrap();
        graph
            .add_edge(Edge::simple("AB", "A", "B"))
            .unwrap();

        assert!(!graph.has_cycle());

        // Add cycle
        graph
            .add_node(Node::simple("C", Position::new(200.0, 0.0)))
            .unwrap();
        graph
            .add_edge(Edge::simple("BC", "B", "C"))
            .unwrap();
        graph
            .add_edge(Edge::simple("CA", "C", "A"))
            .unwrap();

        assert!(graph.has_cycle());

        // Test cycle finding
        let cycle = graph.find_cycle();
        assert!(cycle.is_some());
        let cycle_path = cycle.unwrap();
        assert!(cycle_path.len() >= 3); // Should contain A, B, C
    }

    #[test]
    fn test_creates_cycle() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a DAG: A -> B -> C
        graph
            .add_node(Node::simple("A", Position::new(0.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("B", Position::new(100.0, 0.0)))
            .unwrap();
        graph
            .add_node(Node::simple("C", Position::new(200.0, 0.0)))
            .unwrap();

        graph
            .add_edge(Edge::simple("AB", "A", "B"))
            .unwrap();
        graph
            .add_edge(Edge::simple("BC", "B", "C"))
            .unwrap();

        // Adding C -> A would create a cycle
        assert!(graph.creates_cycle(&"C".into(), &"A".into()));

        // Adding A -> C would not create a cycle
        assert!(!graph.creates_cycle(&"A".into(), &"C".into()));

        // Adding B -> D (non-existent) would not create a cycle
        assert!(!graph.creates_cycle(&"B".into(), &"D".into()));
    }
}
