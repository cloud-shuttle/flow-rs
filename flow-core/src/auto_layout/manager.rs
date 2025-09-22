//! Auto Layout Manager - Intelligent layout algorithm selection and management

use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

use crate::error::Result;
use crate::graph::Graph;
use crate::layout::{
    CircularLayout, ForceDirectedLayout, GridLayout, HierarchicalLayout, LayoutAlgorithm,
};
use crate::types::{NodeId, Position};

use super::analysis::GraphAnalysis;
use super::config::{AutoLayoutConfig, AutoLayoutStrategy};
use super::transitions::{TransitionState, update_transition};

// Layout algorithm names as constants
const ALGORITHM_HIERARCHICAL: &str = "Hierarchical";
const ALGORITHM_FORCE_DIRECTED: &str = "ForceDirected";
const ALGORITHM_GRID: &str = "Grid";
const ALGORITHM_CIRCULAR: &str = "Circular";

/// Auto Layout Manager - Intelligent layout algorithm selection and management
///
/// This manager automatically selects the best layout algorithm based on graph
/// characteristics and provides smooth transitions between different layouts.
#[derive(Debug)]
pub struct AutoLayoutManager {
    config: AutoLayoutConfig,
    current_algorithm: Option<String>,
    graph_hash: Option<u64>,
    transition_state: Option<TransitionState>,
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
            graph_hash: None,
            transition_state: None,
        }
    }

    /// Select the best layout algorithm for the given graph
    pub fn select_layout_algorithm<N, E>(&self, graph: &Graph<N, E>) -> Result<String> {
        let analysis = GraphAnalysis::analyze(graph);

        match self.config.strategy {
            AutoLayoutStrategy::Smart => self.select_smart_algorithm(&analysis),
            AutoLayoutStrategy::HierarchyFirst => self.select_hierarchy_first_algorithm(&analysis),
            AutoLayoutStrategy::ForceDirectedFirst => {
                self.select_force_directed_first_algorithm(&analysis)
            }
            AutoLayoutStrategy::SimpleFirst => self.select_simple_first_algorithm(&analysis),
        }
    }

    /// Apply automatic layout to the graph
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
    pub fn update_transition<N, E>(
        &mut self,
        graph: &mut Graph<N, E>,
        delta_time: f64,
    ) -> Result<bool> {
        update_transition(&mut self.transition_state, graph, delta_time)
    }

    // Private implementation methods

    fn select_smart_algorithm(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.node_count <= self.config.small_graph_threshold {
            if analysis.is_tree {
                Ok(ALGORITHM_HIERARCHICAL.to_string())
            } else {
                Ok(ALGORITHM_GRID.to_string())
            }
        } else if analysis.is_dag {
            Ok(ALGORITHM_HIERARCHICAL.to_string())
        } else if analysis.density > 0.3 {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        } else {
            Ok(ALGORITHM_GRID.to_string())
        }
    }

    fn select_hierarchy_first_algorithm(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.is_dag {
            Ok(ALGORITHM_HIERARCHICAL.to_string())
        } else if analysis.node_count <= self.config.small_graph_threshold {
            Ok(ALGORITHM_CIRCULAR.to_string())
        } else {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
    }

    fn select_force_directed_first_algorithm(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.node_count <= self.config.small_graph_threshold {
            Ok(ALGORITHM_CIRCULAR.to_string())
        } else {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
    }

    fn select_simple_first_algorithm(&self, analysis: &GraphAnalysis) -> Result<String> {
        if analysis.node_count <= self.config.small_graph_threshold {
            if analysis.is_tree {
                Ok(ALGORITHM_HIERARCHICAL.to_string())
            } else {
                Ok(ALGORITHM_GRID.to_string())
            }
        } else {
            Ok(ALGORITHM_FORCE_DIRECTED.to_string())
        }
    }

    fn has_graph_changed<N, E>(&self, graph: &Graph<N, E>) -> bool {
        let current_hash = self.calculate_graph_hash(graph);
        self.graph_hash != Some(current_hash)
    }

    fn update_graph_hash<N, E>(&mut self, graph: &Graph<N, E>) {
        self.graph_hash = Some(self.calculate_graph_hash(graph));
    }

    fn apply_layout_with_algorithm<N: Clone, E>(
        &mut self,
        graph: &mut Graph<N, E>,
        algorithm: &str,
    ) -> Result<()> {
        // Store current positions for transition if enabled
        let _from_positions = if self.config.enable_transitions && self.transition_state.is_none() {
            Some(
                graph
                    .nodes()
                    .map(|node| (node.id.clone(), node.position))
                    .collect::<HashMap<NodeId, Position>>(),
            )
        } else {
            None
        };

        // Apply the selected layout algorithm
        let result = match algorithm {
            ALGORITHM_HIERARCHICAL => {
                let mut layout = HierarchicalLayout::new();
                layout.apply(graph)
            }
            ALGORITHM_FORCE_DIRECTED => {
                let mut layout = ForceDirectedLayout::new();
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
                let mut layout = ForceDirectedLayout::new();
                layout.apply(graph)
            }
        };
        
        // Verify that nodes were actually moved
        let any_moved = graph.nodes().any(|node| node.position != Position::new(0.0, 0.0));
        if !any_moved {
            // Fallback: Simple grid layout
            self.apply_simple_grid_layout(graph)?;
        }
        
        result
    }
    
    /// Fallback simple grid layout when other algorithms fail
    fn apply_simple_grid_layout<N: Clone, E>(&self, graph: &mut Graph<N, E>) -> Result<()> {
        let node_ids: Vec<_> = graph.nodes().map(|node| node.id.clone()).collect();
        let cols = (node_ids.len() as f64).sqrt().ceil() as usize;
        
        for (i, node_id) in node_ids.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;
            let x = col as f64 * 150.0;
            let y = row as f64 * 100.0;
            
            if let Some(graph_node) = graph.get_node_mut(node_id) {
                graph_node.position = Position::new(x, y);
            }
        }
        
        Ok(())
    }

    /// Start transition if enabled and we have from positions
    fn start_transition_if_enabled<N: Clone, E>(
        &mut self,
        graph: &Graph<N, E>,
        from_positions: Option<HashMap<NodeId, Position>>,
    ) {
        if let Some(from_positions) = from_positions {
            let to_positions: HashMap<_, _> = graph
                .nodes()
                .map(|node| (node.id.clone(), node.position))
                .collect();

            self.transition_state = Some(TransitionState::new(
                self.config.transition_duration,
                from_positions,
                to_positions,
            ));
        }
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

impl Default for AutoLayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
