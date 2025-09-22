//! Serialization API Contracts

use crate::graph::{Edge, Graph, Node};
use crate::types::{Position};

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_api_contract() {
        use serde_json;

        // Test Position serialization
        let pos = Position::new(10.0, 20.0);
        let serialized = serde_json::to_string(&pos).unwrap();
        let deserialized: Position = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pos, deserialized);

        // Test Node serialization
        let node = Node::new("test", Position::new(10.0, 20.0), "data");
        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: Node<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.position, deserialized.position);
        assert_eq!(node.data, deserialized.data);

        // Test Graph serialization
        let mut graph = Graph::new();
        graph
            .add_node(Node::new("node1", Position::new(10.0, 20.0), ()))
            .unwrap();
        graph
            .add_edge(Edge::new("edge1", "node1", "node1", ()))
            .unwrap();

        let serialized = serde_json::to_string(&graph).unwrap();
        let deserialized: Graph<(), ()> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(graph.node_count(), deserialized.node_count());
        assert_eq!(graph.edge_count(), deserialized.edge_count());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_complex_serialization_api_contract() {
        use serde_json;

        // Test complex graph serialization
        let mut graph = Graph::new();
        
        // Add nodes with different data types
        graph.add_node(Node::new("node1", Position::new(10.0, 20.0), "string_data")).unwrap();
        graph.add_node(Node::new("node2", Position::new(30.0, 40.0), 42)).unwrap();
        
        // Add edges
        graph.add_edge(Edge::new("edge1", "node1", "node2", "edge_data")).unwrap();

        // Serialize and deserialize
        let serialized = serde_json::to_string(&graph).unwrap();
        let deserialized: Graph<String, String> = serde_json::from_str(&serialized).unwrap();
        
        // Verify structure
        assert_eq!(graph.node_count(), deserialized.node_count());
        assert_eq!(graph.edge_count(), deserialized.edge_count());
        
        // Verify node data
        let node1 = deserialized.get_node(&"node1".into()).unwrap();
        assert_eq!(node1.data, "string_data");
        
        let node2 = deserialized.get_node(&"node2".into()).unwrap();
        assert_eq!(node2.data, "42"); // JSON deserializes numbers as strings
        
        // Verify edge data
        let edge1 = deserialized.get_edge(&"edge1".into()).unwrap();
        assert_eq!(edge1.data, "edge_data");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_error_handling_api_contract() {
        use serde_json;

        // Test invalid JSON handling
        let invalid_json = r#"{"invalid": "json"}"#;
        let result: Result<Position, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());

        // Test type mismatch handling
        let wrong_type_json = r#"{"x": "not_a_number", "y": 20.0}"#;
        let result: Result<Position, _> = serde_json::from_str(wrong_type_json);
        assert!(result.is_err());
    }
}
