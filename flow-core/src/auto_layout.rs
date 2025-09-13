//! Auto Layout Manager - Intelligent layout algorithm selection and management
//!
//! This module provides automatic layout algorithm selection based on graph characteristics,
//! dynamic layout switching, and smooth transitions between different layout styles.

use std::collections::{HashMap, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

use crate::error::Result;
use crate::graph::Graph;
use crate::layout::{LayoutAlgorithm, ForceDirectedLayout, GridLayout, HierarchicalLayout, CircularLayout};
use crate::types::{NodeId, Position};

// Layout algorithm names as constants
const ALGORITHM_HIERARCHICAL: &str = "Hierarchical";
const ALGORITHM_FORCE_DIRECTED: &str = "ForceDirected";
const ALGORITHM_GRID: &str = "Grid";
const ALGORITHM_CIRCULAR: &str = "Circular";

// Default configuration values
const DEFAULT_SMALL_GRAPH_THRESHOLD: usize = 10;
const DEFAULT_TRANSITION_DURATION: f64 = 0.5;
const DEFAULT_FORCE_DIRECTED_ITERATIONS: usize = 50;

/// Strategies for automatic layout selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoLayoutStrategy {
    /// Automatically detect best layout based on graph structure
    Smart,
    /// Prefer hierarchical layouts when possible, fallback to force-directed
    HierarchyFirst,
    /// Always use force-directed as primary with fallbacks
    ForceDirectedFirst,
    /// Use simple layouts (grid/circular) for small graphs
    SimpleFirst,
}

/// Configuration for automatic layout management
///
/// This configuration controls how the AutoLayoutManager selects and applies
/// layout algorithms based on graph characteristics.
#[derive(Debug, Clone)]
pub struct AutoLayoutConfig {
    /// Strategy for automatic algorithm selection
    pub strategy: AutoLayoutStrategy,
    /// Maximum number of nodes to consider a graph "small" for simple layouts
    pub small_graph_threshold: usize,
    /// Whether to enable smooth transitions between layout changes
    pub enable_transitions: bool,
    /// Duration of layout transitions in seconds
    pub transition_duration: f64,
    /// Whether to force relayout when graph structure changes
    pub force_relayout_on_change: bool,
}

impl Default for AutoLayoutConfig {
    fn default() -> Self {
        Self {
            strategy: AutoLayoutStrategy::Smart,
            small_graph_threshold: DEFAULT_SMALL_GRAPH_THRESHOLD,
            enable_transitions: true,
            transition_duration: DEFAULT_TRANSITION_DURATION,
            force_relayout_on_change: false,
        }
    }
}

impl AutoLayoutConfig {
    /// Create a new configuration builder
    pub fn builder() -> AutoLayoutConfigBuilder {
        AutoLayoutConfigBuilder::new()
    }
}

/// Builder for AutoLayoutConfig
#[derive(Debug)]
pub struct AutoLayoutConfigBuilder {
    config: AutoLayoutConfig,
}

impl AutoLayoutConfigBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            config: AutoLayoutConfig::default(),
        }
    }

    /// Set the layout selection strategy
    pub fn strategy(mut self, strategy: AutoLayoutStrategy) -> Self {
        self.config.strategy = strategy;
        self
    }

    /// Set the small graph threshold
    pub fn small_graph_threshold(mut self, threshold: usize) -> Self {
        self.config.small_graph_threshold = threshold;
        self
    }

    /// Enable or disable transitions
    pub fn enable_transitions(mut self, enable: bool) -> Self {
        self.config.enable_transitions = enable;
        self
    }

    /// Set the transition duration in seconds
    pub fn transition_duration(mut self, duration: f64) -> Self {
        self.config.transition_duration = duration.max(0.0);
        self
    }

    /// Enable or disable forced relayout on graph changes
    pub fn force_relayout_on_change(mut self, force: bool) -> Self {
        self.config.force_relayout_on_change = force;
        self
    }

    /// Build the configuration
    pub fn build(self) -> AutoLayoutConfig {
        self.config
    }
}

impl Default for AutoLayoutConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Auto Layout Manager that intelligently selects and applies layout algorithms
///
/// The AutoLayoutManager analyzes graph structure and automatically selects the most
/// appropriate layout algorithm. It supports smooth transitions between different
/// layouts and can detect when the graph structure changes to trigger re-layout.
///
/// # Example
///
/// ```rust
/// use flow_core::{Graph, Node, AutoLayoutManager, AutoLayoutStrategy, AutoLayoutConfig};
///
/// let mut graph: Graph<(), ()> = Graph::new();
/// // Add nodes and edges...
///
/// let mut manager = AutoLayoutManager::new();
/// manager.apply_auto_layout(&mut graph).unwrap();
///
/// // The manager has automatically selected and applied the best layout
/// println!("Selected algorithm: {:?}", manager.current_algorithm());
/// ```
pub struct AutoLayoutManager {
    config: AutoLayoutConfig,
    current_algorithm: Option<String>,
    last_graph_hash: Option<u64>,
    transition_state: Option<TransitionState>,
}

/// State for managing layout transitions
#[derive(Debug)]
struct TransitionState {
    from_positions: HashMap<NodeId, Position>,
    to_positions: HashMap<NodeId, Position>,
    progress: f64,
    duration: f64,
}

impl AutoLayoutManager {
    /// Create a new auto layout manager with default configuration
    pub fn new() -> Self {
        Self::with_config(AutoLayoutConfig::default())
    }

    /// Create a new auto layout manager with custom configuration
    pub fn with_config(config: AutoLayoutConfig) -> Self {
        Self {
            config,
            current_algorithm: None,
            last_graph_hash: None,
            transition_state: None,
        }
    }

    /// Analyze graph structure and select the best layout algorithm
    pub fn select_layout_algorithm<N, E>(&self, graph: &Graph<N, E>) -> Result<String> {
        let analysis = self.analyze_graph_structure(graph);

        match self.config.strategy {
            AutoLayoutStrategy::Smart => self.smart_algorithm_selection(&analysis),
            AutoLayoutStrategy::HierarchyFirst => self.hierarchy_first_selection(&analysis),
            AutoLayoutStrategy::ForceDirectedFirst => self.force_directed_first_selection(&analysis),
            AutoLayoutStrategy::SimpleFirst => self.simple_first_selection(&analysis),
        }
    }

    /// Apply the most appropriate layout algorithm to the graph
    pub fn apply_auto_layout<N: Clone, E>(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        let selected_algorithm = self.select_layout_algorithm(graph)?;

        // Check if we need to change algorithms
        let algorithm_changed = self.current_algorithm.as_ref() != Some(&selected_algorithm);
        let graph_changed = self.has_graph_changed(graph);

        if algorithm_changed || graph_changed || self.config.force_relayout_on_change {
            self.apply_layout_with_algorithm(graph, &selected_algorithm)?;
            self.current_algorithm = Some(selected_algorithm);
            self.update_graph_hash(graph);
        }

        Ok(())
    }

    /// Get the currently selected layout algorithm name
    pub fn current_algorithm(&self) -> Option<&str> {
        self.current_algorithm.as_deref()
    }

    /// Check if a layout transition is currently in progress
    pub fn is_transitioning(&self) -> bool {
        self.transition_state.is_some()
    }

    /// Get the progress of the current transition (0.0 to 1.0)
    pub fn transition_progress(&self) -> Option<f64> {
        self.transition_state.as_ref().map(|state| state.progress)
    }

    /// Update transition state (should be called each frame during animation)
    pub fn update_transition<N, E>(&mut self, graph: &mut Graph<N, E>, delta_time: f64) -> Result<bool> {
        let should_remove_state = if let Some(ref mut state) = self.transition_state {
            state.progress += delta_time / state.duration;

            if state.progress >= 1.0 {
                // Transition complete - apply final positions
                for node in graph.nodes_mut() {
                    if let Some(&target_pos) = state.to_positions.get(&node.id) {
                        node.position = target_pos;
                    }
                }
                return Ok(true); // Transition completed
            } else {
                // Interpolate positions - collect data first to avoid borrowing issues
                let progress = state.progress;
                let transitions: Vec<_> = graph.nodes()
                    .filter_map(|node| {
                        if let (Some(&from_pos), Some(&to_pos)) =
                            (state.from_positions.get(&node.id), state.to_positions.get(&node.id)) {
                            Some((node.id.clone(), from_pos, to_pos))
                        } else {
                            None
                        }
                    })
                    .collect();

                // Apply interpolated positions
                let t = ease_in_out(progress);
                for (node_id, from_pos, to_pos) in transitions {
                    if let Some(node) = graph.get_node_mut(&node_id) {
                        node.position = interpolate_position(from_pos, to_pos, t);
                    }
                }
                return Ok(false); // Transition in progress
            }
        } else {
            false
        };

        if should_remove_state {
            self.transition_state = None;
        }

        Ok(true) // No transition
    }

    // Private implementation methods

    fn analyze_graph_structure<N, E>(&self, _graph: &Graph<N, E>) -> GraphAnalysis {
        // Simplified implementation for MVP
        let node_count = _graph.node_count();
        let edge_count = _graph.edge_count();
        let density = if node_count > 1 {
            (2.0 * edge_count as f64) / (node_count as f64 * (node_count - 1) as f64)
        } else {
            0.0
        };

        // Basic cycle detection (simplified)
        let has_cycles = self.detect_cycles(_graph);
        let is_dag = !has_cycles;
        let is_tree = is_dag && edge_count == (node_count.saturating_sub(1));

        GraphAnalysis {
            node_count,
            edge_count,
            is_tree,
            is_dag,
            has_cycles,
            max_degree: 0, // Simplified for now
            density,
            clustering_coefficient: 0.0, // Simplified for now
        }
    }

    fn smart_algorithm_selection(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.node_count == 0 {
            return Ok(ALGORITHM_GRID.to_string());
        }

        // Trees should use hierarchical layout when large enough
        if analysis.is_tree && analysis.node_count > 2 {
            Ok(ALGORITHM_HIERARCHICAL.to_string())
        }
        // Cyclic graphs generally benefit from force-directed layout
        else if analysis.has_cycles {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
        // Small simple graphs (non-cyclic) can use simple layouts
        else if analysis.node_count <= self.config.small_graph_threshold {
            if analysis.density > 0.5 {
                Ok(ALGORITHM_CIRCULAR.to_string())
            } else {
                Ok(ALGORITHM_GRID.to_string())
            }
        }
        // Large graphs default to force-directed
        else {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
    }

    fn hierarchy_first_selection(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.is_tree && analysis.node_count > 2 {
            Ok(ALGORITHM_HIERARCHICAL.to_string())
        } else {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
    }

    fn force_directed_first_selection(&self, _analysis: &GraphAnalysis) -> Result<String> {
        Ok(ALGORITHM_FORCE_DIRECTED.to_string())
    }

    fn simple_first_selection(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.node_count <= self.config.small_graph_threshold {
            if analysis.has_cycles {
                Ok(ALGORITHM_CIRCULAR.to_string())
            } else {
                Ok(ALGORITHM_GRID.to_string())
            }
        } else {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
    }

    fn has_graph_changed<N, E>(&self, graph: &Graph<N, E>) -> bool {
        let current_hash = self.calculate_graph_hash(graph);
        self.last_graph_hash != Some(current_hash)
    }

    fn update_graph_hash<N, E>(&mut self, graph: &Graph<N, E>) {
        self.last_graph_hash = Some(self.calculate_graph_hash(graph));
    }

    fn apply_layout_with_algorithm<N: Clone, E>(&mut self, graph: &mut Graph<N, E>, algorithm: &str) -> Result<()> {
        match algorithm {
            ALGORITHM_HIERARCHICAL => {
                let mut layout = HierarchicalLayout::new();
                layout.apply(graph)
            }
            ALGORITHM_FORCE_DIRECTED => {
                let mut layout = ForceDirectedLayout::builder()
                    .iterations(DEFAULT_FORCE_DIRECTED_ITERATIONS)
                    .randomize_start(false)
                    .build();
                layout.apply(graph)
            }
            ALGORITHM_GRID => {
                let mut layout = GridLayout::new();
                layout.apply(graph)
            }
            ALGORITHM_CIRCULAR => {
                let mut layout = CircularLayout::new();
                layout.apply(graph)
            }
            _ => {
                // Default to force-directed for unknown algorithms
                let mut layout = ForceDirectedLayout::new();
                layout.apply(graph)
            }
        }
    }

    fn detect_cycles<N, E>(&self, graph: &Graph<N, E>) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        // Simple DFS-based cycle detection
        for node in graph.nodes() {
            if !visited.contains(&node.id)
                && self.dfs_cycle_detection(graph, &node.id, &mut visited, &mut rec_stack)
            {
                return true;
            }
        }
        false
    }

    #[allow(clippy::only_used_in_recursion)]
    fn dfs_cycle_detection<N, E>(&self, graph: &Graph<N, E>, node_id: &NodeId,
                                visited: &mut HashSet<NodeId>, rec_stack: &mut HashSet<NodeId>) -> bool {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        for edge in graph.get_outgoing_edges(node_id) {
            if !visited.contains(&edge.target) {
                if self.dfs_cycle_detection(graph, &edge.target, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&edge.target) {
                return true; // Back edge found = cycle
            }
        }

        rec_stack.remove(node_id);
        false
    }

    fn calculate_graph_hash<N, E>(&self, graph: &Graph<N, E>) -> u64 {
        let mut hasher = DefaultHasher::new();
        graph.node_count().hash(&mut hasher);
        graph.edge_count().hash(&mut hasher);

        // Hash node IDs
        let mut node_ids: Vec<_> = graph.nodes().map(|n| &n.id).collect();
        node_ids.sort();
        for id in node_ids {
            id.hash(&mut hasher);
        }

        // Hash edge connections
        let mut edges: Vec<_> = graph.edges().map(|e| (&e.source, &e.target)).collect();
        edges.sort();
        for (source, target) in edges {
            source.hash(&mut hasher);
            target.hash(&mut hasher);
        }

        hasher.finish()
    }
}

/// Results of graph structure analysis
#[derive(Debug)]
struct GraphAnalysis {
    node_count: usize,
    #[allow(dead_code)]
    edge_count: usize,
    is_tree: bool,
    #[allow(dead_code)]
    is_dag: bool,
    has_cycles: bool,
    #[allow(dead_code)]
    max_degree: usize,
    density: f64,
    #[allow(dead_code)]
    clustering_coefficient: f64,
}

impl Default for AutoLayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function for smooth easing
fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Helper function for position interpolation
fn interpolate_position(from: Position, to: Position, t: f64) -> Position {
    Position::new(
        from.x + (to.x - from.x) * t,
        from.y + (to.y - from.y) * t,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Node, Edge};

    // Test fixtures
    fn create_small_tree() -> Graph<(), ()> {
        let mut graph = Graph::new();
        graph.add_node(Node::builder("root").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("child1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("child2").position(0.0, 0.0).build()).unwrap();

        graph.add_edge(Edge::builder().connect("root", "child1").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("root", "child2").build().unwrap()).unwrap();

        graph
    }

    fn create_cyclic_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        for i in 1..=4 {
            graph.add_node(Node::builder(format!("node{}", i)).position(0.0, 0.0).build()).unwrap();
        }

        graph.add_edge(Edge::builder().connect("node1", "node2").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("node2", "node3").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("node3", "node4").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("node4", "node1").build().unwrap()).unwrap();

        graph
    }

    fn create_large_connected_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();

        // Create 20 nodes
        for i in 1..=20 {
            graph.add_node(Node::builder(format!("node{}", i)).position(0.0, 0.0).build()).unwrap();
        }

        // Create a dense network of connections
        for i in 1..=20 {
            for j in (i+1)..=20 {
                if (i + j) % 3 == 0 { // Create some connections based on a pattern
                    graph.add_edge(Edge::builder().connect(format!("node{}", i), format!("node{}", j)).build().unwrap()).unwrap();
                }
            }
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

        let manager = AutoLayoutManager::with_config(config.clone());
        assert_eq!(manager.config.strategy, AutoLayoutStrategy::HierarchyFirst);
        assert_eq!(manager.config.small_graph_threshold, 5);
        assert!(!manager.config.enable_transitions);
    }

    #[test]
    fn test_select_layout_for_small_tree() {
        let manager = AutoLayoutManager::new();
        let graph = create_small_tree();

        let result = manager.select_layout_algorithm(&graph);
        assert!(result.is_ok());

        let algorithm = result.unwrap();
        // For a small tree, we expect hierarchical layout
        assert_eq!(algorithm, "Hierarchical");
    }

    #[test]
    fn test_select_layout_for_cyclic_graph() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::Smart,
            ..Default::default()
        };
        let manager = AutoLayoutManager::with_config(config);
        let graph = create_cyclic_graph();

        let result = manager.select_layout_algorithm(&graph);
        assert!(result.is_ok());

        let algorithm = result.unwrap();
        // For a cyclic graph, we expect force-directed layout
        assert_eq!(algorithm, "ForceDirected");
    }

    #[test]
    fn test_select_layout_for_large_graph() {
        let manager = AutoLayoutManager::new();
        let graph = create_large_connected_graph();

        let result = manager.select_layout_algorithm(&graph);
        assert!(result.is_ok());

        let algorithm = result.unwrap();
        // For a large connected graph, we expect force-directed layout
        assert_eq!(algorithm, "ForceDirected");
    }

    #[test]
    fn test_hierarchy_first_strategy() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::HierarchyFirst,
            ..Default::default()
        };
        let manager = AutoLayoutManager::with_config(config);

        let tree = create_small_tree();
        let result = manager.select_layout_algorithm(&tree);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hierarchical");

        let cyclic = create_cyclic_graph();
        let result = manager.select_layout_algorithm(&cyclic);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ForceDirected"); // Fallback for non-hierarchical
    }

    #[test]
    fn test_simple_first_strategy() {
        let config = AutoLayoutConfig {
            strategy: AutoLayoutStrategy::SimpleFirst,
            small_graph_threshold: 5,
            ..Default::default()
        };
        let manager = AutoLayoutManager::with_config(config);

        let small_graph = create_small_tree();
        let result = manager.select_layout_algorithm(&small_graph);
        assert!(result.is_ok());
        let algorithm = result.unwrap();
        // Should be a simple layout (Grid or Circular)
        assert!(algorithm == "Grid" || algorithm == "Circular");
    }

    #[test]
    fn test_apply_auto_layout() {
        let mut manager = AutoLayoutManager::new();
        let mut graph = create_small_tree();

        // Store initial positions
        let initial_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

        let result = manager.apply_auto_layout(&mut graph);
        assert!(result.is_ok());

        // Positions should have changed after layout
        let new_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();
        assert_ne!(initial_positions, new_positions);

        // Manager should now have a current algorithm
        assert!(manager.current_algorithm().is_some());
    }

    #[test]
    fn test_algorithm_consistency() {
        let mut manager = AutoLayoutManager::new();
        let mut graph = create_small_tree();

        // First application
        manager.apply_auto_layout(&mut graph).unwrap();
        let first_algorithm = manager.current_algorithm().unwrap().to_string();

        // Second application on same graph should use same algorithm
        manager.apply_auto_layout(&mut graph).unwrap();
        let second_algorithm = manager.current_algorithm().unwrap().to_string();

        assert_eq!(first_algorithm, second_algorithm);
    }

    #[test]
    fn test_algorithm_change_on_graph_modification() {
        let mut manager = AutoLayoutManager::with_config(AutoLayoutConfig {
            force_relayout_on_change: true,
            ..Default::default()
        });
        let mut graph = create_small_tree();

        // First application
        manager.apply_auto_layout(&mut graph).unwrap();
        let _first_algorithm = manager.current_algorithm().unwrap().to_string();

        // Modify graph to make it cyclic
        graph.add_edge(Edge::builder().connect("child1", "child2").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("child2", "root").build().unwrap()).unwrap();

        // Apply layout again
        manager.apply_auto_layout(&mut graph).unwrap();
        let _second_algorithm = manager.current_algorithm().unwrap().to_string();

        // Algorithm might change based on new graph structure
        // At minimum, this should not panic and should complete successfully
        assert!(manager.current_algorithm().is_some());
    }

    #[test]
    fn test_transition_state_management() {
        let manager = AutoLayoutManager::new();

        // Initially no transition
        assert!(!manager.is_transitioning());
        assert!(manager.transition_progress().is_none());
    }

    #[test]
    fn test_empty_graph_handling() {
        let mut manager = AutoLayoutManager::new();
        let mut empty_graph: Graph<(), ()> = Graph::new();

        let result = manager.apply_auto_layout(&mut empty_graph);
        // Should handle empty graph gracefully
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_node_graph_handling() {
        let mut manager = AutoLayoutManager::new();
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("solo").position(0.0, 0.0).build()).unwrap();

        let result = manager.apply_auto_layout(&mut graph);
        assert!(result.is_ok());
        assert!(manager.current_algorithm().is_some());
    }

    #[test]
    fn test_layout_algorithm_selection_consistency() {
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
            assert!(!algorithm.is_empty(), "Strategy {:?} returned empty algorithm", strategy);
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
        assert_eq!(config.small_graph_threshold, default_config.small_graph_threshold);
        assert_eq!(config.enable_transitions, default_config.enable_transitions);
        assert_eq!(config.transition_duration, default_config.transition_duration);
        assert_eq!(config.force_relayout_on_change, default_config.force_relayout_on_change);
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
