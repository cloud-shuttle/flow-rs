//! Auto Layout API Contracts

use crate::auto_layout::{AutoLayoutConfig, AutoLayoutManager, AutoLayoutStrategy};
use crate::graph::{Edge, Graph, Node};
use crate::types::{Position};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        graph.add_node(Node::new("node1", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("node2", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("node3", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_edge(Edge::new("edge1", "node1", "node2", ())).unwrap();
        graph.add_edge(Edge::new("edge2", "node2", "node3", ())).unwrap();
        graph
    }

    #[test]
    fn test_auto_layout_manager_creation_api_contract() {
        let manager = AutoLayoutManager::new();
        
        assert!(manager.current_algorithm().is_none());
        assert!(!manager.is_transitioning());
        assert!(manager.transition_progress().is_none());
    }

    #[test]
    fn test_auto_layout_manager_with_config_api_contract() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::HierarchyFirst,
            small_graph_threshold: 5,
            enable_transitions: false,
            transition_duration: 1.0,
            force_relayout_on_change: true,
        };

        let manager = AutoLayoutManager::with_config(config);
        assert!(manager.current_algorithm().is_none());
    }

    #[test]
    fn test_auto_layout_algorithm_selection_api_contract() {
        let manager = AutoLayoutManager::new();
        let graph = create_test_graph();

        // Test algorithm selection
        let algorithm = manager.select_layout_algorithm(&graph).unwrap();
        assert!(!algorithm.is_empty());
        assert!(algorithm == "Hierarchical" || algorithm == "Grid" || algorithm == "ForceDirected" || algorithm == "Circular");
    }

    #[test]
    fn test_auto_layout_application_api_contract() {
        let mut manager = AutoLayoutManager::new();
        let mut graph = create_test_graph();

        // Test layout application
        assert!(manager.apply_auto_layout(&mut graph).is_ok());
        
        // Verify algorithm was selected
        assert!(manager.current_algorithm().is_some());
        
        // Verify nodes have been positioned (at least some nodes should move)
        let moved_nodes = graph.nodes().filter(|node| node.position != Position::new(0.0, 0.0)).count();
        assert!(moved_nodes > 0, "At least some nodes should have been moved from (0.0, 0.0)");
    }

    #[test]
    fn test_auto_layout_strategy_api_contract() {
        let strategies = [
            AutoLayoutStrategy::Smart,
            AutoLayoutStrategy::HierarchyFirst,
            AutoLayoutStrategy::ForceDirectedFirst,
            AutoLayoutStrategy::SimpleFirst,
        ];

        let graph = create_test_graph();

        for strategy in strategies.iter() {
            let config = AutoLayoutConfig {
                strategy: *strategy,
                ..Default::default()
            };
            let manager = AutoLayoutManager::with_config(config);

            let result = manager.select_layout_algorithm(&graph);
            assert!(result.is_ok(), "Strategy {:?} failed", strategy);

            let algorithm = result.unwrap();
            assert!(!algorithm.is_empty(), "Strategy {:?} returned empty algorithm", strategy);
        }
    }

    #[test]
    fn test_auto_layout_config_builder_api_contract() {
        let config = AutoLayoutConfig::builder()
            .strategy(AutoLayoutStrategy::HierarchyFirst)
            .small_graph_threshold(5)
            .enable_transitions(false)
            .transition_duration(1.0)
            .force_relayout_on_change(true)
            .build();

        assert_eq!(config.strategy, AutoLayoutStrategy::HierarchyFirst);
        assert_eq!(config.small_graph_threshold, 5);
        assert!(!config.enable_transitions);
        assert_eq!(config.transition_duration, 1.0);
        assert!(config.force_relayout_on_change);
    }

    #[test]
    fn test_auto_layout_transition_api_contract() {
        let mut manager = AutoLayoutManager::new();
        let mut graph = create_test_graph();

        // Test transition state
        assert!(!manager.is_transitioning());
        assert!(manager.transition_progress().is_none());

        // Apply layout (should not start transition by default)
        assert!(manager.apply_auto_layout(&mut graph).is_ok());
        assert!(!manager.is_transitioning());
    }
}
