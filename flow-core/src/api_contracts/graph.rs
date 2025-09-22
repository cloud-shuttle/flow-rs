//! Graph API Contracts

use crate::graph::{Edge, Graph, Node};
use crate::types::{EdgeId, NodeId, Position, Rect, Size};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation_api_contract() {
        let graph = Graph::<(), ()>::new();

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_graph_node_operations_api_contract() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Test node addition
        let node = Node::new("test_node", Position::new(10.0, 20.0), ());
        assert!(graph.add_node(node).is_ok());
        assert_eq!(graph.node_count(), 1);

        // Test duplicate node handling
        let duplicate = Node::new("test_node", Position::new(30.0, 40.0), ());
        assert!(graph.add_node(duplicate).is_err());

        // Test node retrieval
        assert!(graph.get_node(&NodeId::new("test_node")).is_some());
        assert!(graph.get_node(&NodeId::new("nonexistent")).is_none());

        // Test node removal
        assert!(graph.remove_node(&NodeId::new("test_node")).is_ok());
        assert_eq!(graph.node_count(), 0);
        assert!(graph.remove_node(&NodeId::new("nonexistent")).is_err());
    }

    #[test]
    fn test_graph_edge_operations_api_contract() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Add nodes first
        graph.add_node(Node::new("node1", Position::new(10.0, 20.0), ())).unwrap();
        graph.add_node(Node::new("node2", Position::new(30.0, 40.0), ())).unwrap();

        // Test edge addition
        let edge1 = Edge::new("edge1", "node1", "node2", ());
        let edge2 = Edge::new("edge2", "node2", "node1", ());

        assert!(graph.add_edge(edge1).is_ok());
        assert!(graph.add_edge(edge2).is_ok());

        assert_eq!(graph.edge_count(), 2);

        // Test duplicate edge handling
        let duplicate = Edge::new("edge1", "node1", "node2", ());
        assert!(graph.add_edge(duplicate).is_err());

        // Test edge retrieval
        assert!(graph.get_edge(&EdgeId::new("edge1")).is_some());
        assert!(graph.get_edge(&EdgeId::new("nonexistent")).is_none());

        // Test edge removal
        assert!(graph.remove_edge(&EdgeId::new("edge1")).is_ok());
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.remove_edge(&EdgeId::new("nonexistent")).is_err());
    }

    #[test]
    fn test_graph_iteration_api_contract() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Add test data
        graph
            .add_node(Node::new("node1", Position::new(10.0, 20.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("node2", Position::new(30.0, 40.0), ()))
            .unwrap();
        graph
            .add_edge(Edge::new("edge1", "node1", "node2", ()))
            .unwrap();

        // Test node iteration
        let node_ids: Vec<_> = graph.node_ids().collect();
        assert_eq!(node_ids.len(), 2);
        assert!(node_ids.contains(&&NodeId::new("node1")));
        assert!(node_ids.contains(&&NodeId::new("node2")));

        // Test edge iteration
        let edge_ids: Vec<_> = graph.edge_ids().collect();
        assert_eq!(edge_ids.len(), 1);
        assert!(edge_ids.contains(&&EdgeId::new("edge1")));

        // Test node iteration with data
        let mut node_count = 0;
        for node in graph.nodes() {
            node_count += 1;
            assert!(node.id.as_str() == "node1" || node.id.as_str() == "node2");
        }
        assert_eq!(node_count, 2);

        // Test edge iteration with data
        let mut edge_count = 0;
        for edge in graph.edges() {
            edge_count += 1;
            assert_eq!(edge.id.as_str(), "edge1");
        }
        assert_eq!(edge_count, 1);
    }

    #[test]
    fn test_graph_bounds_api_contract() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Test empty graph bounds
        let empty_bounds = graph.bounds();
        assert_eq!(empty_bounds, None);

        // Add nodes and test bounds calculation
        let mut node1 = Node::new("node1", Position::new(10.0, 20.0), ());
        node1.set_size(Size::new(100.0, 50.0));
        let mut node2 = Node::new("node2", Position::new(50.0, 80.0), ());
        node2.set_size(Size::new(80.0, 40.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let bounds = graph.bounds().unwrap();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 120.0); // 50 + 80 - 10
        assert_eq!(bounds.height, 100.0); // 80 + 40 - 20
    }
}
