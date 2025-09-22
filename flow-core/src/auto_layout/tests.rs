//! Tests for auto layout system

// use crate::graph::{Graph, Node, Edge}; // Unused imports
// use crate::types::{NodeId, Position}; // Unused imports

// use super::config::{AutoLayoutConfig, AutoLayoutConfigBuilder, AutoLayoutStrategy}; // Unused imports
// use super::manager::AutoLayoutManager; // Unused imports

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NodeId, Position};
    use crate::graph::{Graph, Node, Edge};
    use crate::auto_layout::{AutoLayoutConfig, AutoLayoutConfigBuilder, AutoLayoutStrategy};
    use crate::auto_layout::manager::AutoLayoutManager;

    fn create_small_tree() -> Graph<(), ()> {
        let mut graph = Graph::new();
        let root = Node::simple("root", Position::new(0.0, 0.0));
        let child1 = Node::simple("child1", Position::new(0.0, 0.0));
        let child2 = Node::simple("child2", Position::new(0.0, 0.0));

        graph.add_node(root).unwrap();
        graph.add_node(child1).unwrap();
        graph.add_node(child2).unwrap();

        graph.add_edge(Edge::new("edge1", "root", "child1", ())).unwrap();
        graph.add_edge(Edge::new("edge2", "root", "child2", ())).unwrap();

        graph
    }

    fn create_cyclic_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        let node1 = Node::simple("node1", Position::new(0.0, 0.0));
        let node2 = Node::simple("node2", Position::new(0.0, 0.0));
        let node3 = Node::simple("node3", Position::new(0.0, 0.0));

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();
        graph.add_node(node3).unwrap();

        graph.add_edge(Edge::new("edge1", "node1", "node2", ())).unwrap();
        graph.add_edge(Edge::new("edge2", "node2", "node3", ())).unwrap();
        graph.add_edge(Edge::new("edge3", "node3", "node1", ())).unwrap();

        graph
    }

    fn create_large_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        for i in 0..20 {
            let node = Node::simple(format!("node{}", i), Position::new(0.0, 0.0));
            graph.add_node(node).unwrap();
        }

        // Add some edges to create a connected graph
        for i in 0..19 {
            graph.add_edge(Edge::new(format!("edge{}", i), format!("node{}", i), format!("node{}", i + 1), ())).unwrap();
        }

        graph
    }

    #[test]
    fn test_auto_layout_manager_creation() {
        let manager = AutoLayoutManager::new();
        assert!(manager.current_algorithm().is_none());
        assert!(!manager.is_transitioning());
        assert!(manager.transition_progress().is_none());
    }

    #[test]
    fn test_auto_layout_manager_with_config() {
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
    fn test_algorithm_selection_smart_strategy() {
        let manager = AutoLayoutManager::new();

        // Test with small tree
        let tree = create_small_tree();
        let algorithm = manager.select_layout_algorithm(&tree).unwrap();
        assert_eq!(algorithm, "Hierarchical");

        // Test with cyclic graph
        let cyclic = create_cyclic_graph();
        let algorithm = manager.select_layout_algorithm(&cyclic).unwrap();
        assert_eq!(algorithm, "Grid");

        // Test with large graph
        let large = create_large_graph();
        let algorithm = manager.select_layout_algorithm(&large).unwrap();
        assert_eq!(algorithm, "Hierarchical");
    }

    #[test]
    fn test_algorithm_selection_hierarchy_first() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::HierarchyFirst,
            ..Default::default()
        };
        let manager = AutoLayoutManager::with_config(config);

        // Test with DAG (tree)
        let tree = create_small_tree();
        let algorithm = manager.select_layout_algorithm(&tree).unwrap();
        assert_eq!(algorithm, "Hierarchical");

        // Test with cyclic graph
        let cyclic = create_cyclic_graph();
        let algorithm = manager.select_layout_algorithm(&cyclic).unwrap();
        assert_eq!(algorithm, "Circular");
    }

    #[test]
    fn test_algorithm_selection_force_directed_first() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::ForceDirectedFirst,
            ..Default::default()
        };
        let manager = AutoLayoutManager::with_config(config);

        // Test with small graph
        let tree = create_small_tree();
        let algorithm = manager.select_layout_algorithm(&tree).unwrap();
        assert_eq!(algorithm, "Circular");

        // Test with large graph
        let large = create_large_graph();
        let algorithm = manager.select_layout_algorithm(&large).unwrap();
        assert_eq!(algorithm, "ForceDirected");
    }

    #[test]
    fn test_algorithm_selection_simple_first() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::SimpleFirst,
            ..Default::default()
        };
        let manager = AutoLayoutManager::with_config(config);

        // Test with small tree
        let tree = create_small_tree();
        let algorithm = manager.select_layout_algorithm(&tree).unwrap();
        assert_eq!(algorithm, "Hierarchical");

        // Test with small cyclic graph
        let cyclic = create_cyclic_graph();
        let algorithm = manager.select_layout_algorithm(&cyclic).unwrap();
        assert_eq!(algorithm, "Grid");

        // Test with large graph
        let large = create_large_graph();
        let algorithm = manager.select_layout_algorithm(&large).unwrap();
        assert_eq!(algorithm, "ForceDirected");
    }

    #[test]
    fn test_apply_auto_layout() {
        let mut manager = AutoLayoutManager::new();
        let mut graph = create_small_tree();

        let result = manager.apply_auto_layout(&mut graph);
        assert!(result.is_ok());

        // Check that an algorithm was selected
        assert!(manager.current_algorithm().is_some());
    }

    #[test]
    fn test_algorithm_consistency() {
        let manager = AutoLayoutManager::new();
        let graph = create_small_tree();

        // Multiple calls should return same algorithm for same graph
        let algo1 = manager.select_layout_algorithm(&graph).unwrap();
        let algo2 = manager.select_layout_algorithm(&graph).unwrap();
        let algo3 = manager.select_layout_algorithm(&graph).unwrap();

        assert_eq!(algo1, algo2);
        assert_eq!(algo2, algo3);
    }

    #[test]
    fn test_all_strategy_types() {
        let strategies = [
            AutoLayoutStrategy::Smart,
            AutoLayoutStrategy::HierarchyFirst,
            AutoLayoutStrategy::ForceDirectedFirst,
            AutoLayoutStrategy::SimpleFirst,
        ];

        let graph = create_small_tree();

        for strategy in strategies.iter() {
            let config = AutoLayoutConfig {
                strategy: *strategy,
                ..Default::default()
            };
            let manager = AutoLayoutManager::with_config(config);

            let result = manager.select_layout_algorithm(&graph);
            assert!(result.is_ok(), "Strategy {:?} failed", strategy);

            let algorithm = result.unwrap();
            assert!(
                !algorithm.is_empty(),
                "Strategy {:?} returned empty algorithm",
                strategy
            );
        }
    }

    #[test]
    fn test_config_builder() {
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
    fn test_config_builder_defaults() {
        let config = AutoLayoutConfig::builder().build();
        let default_config = AutoLayoutConfig::default();

        assert_eq!(config.strategy, default_config.strategy);
        assert_eq!(
            config.small_graph_threshold,
            default_config.small_graph_threshold
        );
        assert_eq!(config.enable_transitions, default_config.enable_transitions);
        assert_eq!(
            config.transition_duration,
            default_config.transition_duration
        );
        assert_eq!(
            config.force_relayout_on_change,
            default_config.force_relayout_on_change
        );
    }

    #[test]
    fn test_config_builder_negative_transition_duration() {
        let config = AutoLayoutConfig::builder()
            .transition_duration(-1.0)
            .build();

        // Negative duration should be clamped to 0.0
        assert_eq!(config.transition_duration, 0.0);
    }
}
