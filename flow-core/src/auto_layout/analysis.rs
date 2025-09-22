//! Graph analysis for automatic layout selection

use std::collections::HashSet;

use crate::graph::Graph;
use crate::types::NodeId;

/// Results of graph structure analysis
#[derive(Debug)]
pub struct GraphAnalysis {
    pub node_count: usize,
    pub edge_count: usize,
    pub is_tree: bool,
    pub is_dag: bool,
    pub has_cycles: bool,
    pub max_degree: usize,
    pub density: f64,
    pub clustering_coefficient: f64,
}

impl GraphAnalysis {
    /// Analyze a graph structure to determine layout characteristics
    pub fn analyze<N, E>(graph: &Graph<N, E>) -> Self {
        let node_count = graph.node_count();
        let edge_count = graph.edge_count();
        let density = if node_count > 1 {
            (2.0 * edge_count as f64) / (node_count as f64 * (node_count - 1) as f64)
        } else {
            0.0
        };

        // Basic cycle detection
        let has_cycles = Self::detect_cycles(graph);
        let is_dag = !has_cycles;

        // Check if it's a tree (connected, acyclic, n-1 edges)
        let is_tree = is_dag && edge_count == node_count.saturating_sub(1);

        // Calculate max degree
        let max_degree = graph
            .nodes()
            .map(|node| {
                graph.get_incoming_edges(&node.id).len() + graph.get_outgoing_edges(&node.id).len()
            })
            .max()
            .unwrap_or(0);

        // Simplified clustering coefficient calculation
        let clustering_coefficient = Self::calculate_clustering_coefficient(graph);

        Self {
            node_count,
            edge_count,
            is_tree,
            is_dag,
            has_cycles,
            max_degree,
            density,
            clustering_coefficient,
        }
    }

    /// Detect cycles in the graph using DFS
    fn detect_cycles<N, E>(graph: &Graph<N, E>) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        // Simple DFS-based cycle detection
        for node in graph.nodes() {
            if !visited.contains(&node.id)
                && Self::dfs_cycle_detection(graph, &node.id, &mut visited, &mut rec_stack)
            {
                return true;
            }
        }
        false
    }

    #[allow(clippy::only_used_in_recursion)]
    fn dfs_cycle_detection<N, E>(
        graph: &Graph<N, E>,
        node_id: &NodeId,
        visited: &mut HashSet<NodeId>,
        rec_stack: &mut HashSet<NodeId>,
    ) -> bool {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        for edge in graph.get_outgoing_edges(node_id) {
            if !visited.contains(&edge.target) {
                if Self::dfs_cycle_detection(graph, &edge.target, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&edge.target) {
                return true; // Back edge found = cycle
            }
        }

        rec_stack.remove(node_id);
        false
    }

    /// Calculate clustering coefficient (simplified implementation)
    fn calculate_clustering_coefficient<N, E>(graph: &Graph<N, E>) -> f64 {
        let mut total_coefficient = 0.0;
        let mut node_count = 0;

        for node in graph.nodes() {
            let neighbors: HashSet<_> = graph
                .get_outgoing_edges(&node.id)
                .into_iter()
                .map(|e| &e.target)
                .chain(graph.get_incoming_edges(&node.id).into_iter().map(|e| &e.source))
                .collect();

            let k = neighbors.len();
            if k < 2 {
                continue; // Skip nodes with less than 2 neighbors
            }

            // Count edges between neighbors
            let mut edges_between_neighbors = 0;
            for neighbor in &neighbors {
                for edge in graph.get_outgoing_edges(neighbor) {
                    if neighbors.contains(&edge.target) {
                        edges_between_neighbors += 1;
                    }
                }
            }

            // Clustering coefficient for this node
            let max_possible_edges = k * (k - 1);
            if max_possible_edges > 0 {
                total_coefficient += edges_between_neighbors as f64 / max_possible_edges as f64;
                node_count += 1;
            }
        }

        if node_count > 0 {
            total_coefficient / node_count as f64
        } else {
            0.0
        }
    }
}
