//! Layout API Contracts

use crate::graph::{Edge, Graph, Node};
use crate::layout::{CircularLayout, ForceDirectedLayout, GridLayout, LayoutAlgorithm};
use crate::types::{Position};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        
        // Add nodes
        graph.add_node(Node::new("node1", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("node2", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("node3", Position::new(0.0, 0.0), ())).unwrap();
        
        // Add edges
        graph.add_edge(Edge::new("edge1", "node1", "node2", ())).unwrap();
        graph.add_edge(Edge::new("edge2", "node2", "node3", ())).unwrap();
        
        graph
    }

    #[test]
    fn test_grid_layout_api_contract() {
        let mut graph = create_test_graph();
        let mut layout = GridLayout::new();

        // Test layout application
        assert!(layout.apply(&mut graph).is_ok());

        // Verify nodes have been positioned (at least some nodes should move)
        let moved_nodes = graph.nodes().filter(|node| node.position != Position::new(0.0, 0.0)).count();
        assert!(moved_nodes > 0, "At least some nodes should have been moved from (0.0, 0.0)");
    }

    #[test]
    fn test_circular_layout_api_contract() {
        let mut graph = create_test_graph();
        let mut layout = CircularLayout::new();

        // Test layout application
        assert!(layout.apply(&mut graph).is_ok());

        // Verify nodes have been positioned (at least some nodes should move)
        let moved_nodes = graph.nodes().filter(|node| node.position != Position::new(0.0, 0.0)).count();
        assert!(moved_nodes > 0, "At least some nodes should have been moved from (0.0, 0.0)");
    }

    #[test]
    fn test_force_directed_layout_api_contract() {
        let mut graph = create_test_graph();
        let mut layout = ForceDirectedLayout::new();

        // Test layout application
        assert!(layout.apply(&mut graph).is_ok());

        // Verify nodes have been positioned (at least some nodes should move)
        let moved_nodes = graph.nodes().filter(|node| node.position != Position::new(0.0, 0.0)).count();
        assert!(moved_nodes > 0, "At least some nodes should have been moved from (0.0, 0.0)");
    }

    #[test]
    fn test_layout_algorithm_trait_api_contract() {
        let mut graph = create_test_graph();
        
        // Test that all layout algorithms implement the trait
        let mut grid_layout = GridLayout::new();
        let mut circular_layout = CircularLayout::new();
        let mut force_layout = ForceDirectedLayout::new();

        // Test apply method
        assert!(grid_layout.apply(&mut graph).is_ok());
        assert!(circular_layout.apply(&mut graph).is_ok());
        assert!(force_layout.apply(&mut graph).is_ok());
    }
}
