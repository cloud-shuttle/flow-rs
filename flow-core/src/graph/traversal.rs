//! Graph traversal algorithms

use std::collections::{HashMap, HashSet, VecDeque};

use crate::error::{FlowError, Result};
use crate::types::NodeId;

use super::Graph;

impl<N, E> Graph<N, E>
where
    N: Clone,
    E: Clone,
{
    /// Check if the graph contains any cycles using DFS-based cycle detection
    ///
    /// Uses Depth-First Search with recursion stack tracking to detect back edges.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        // Check each node as a potential starting point
        for node_id in self.node_ids() {
            if !visited.contains(node_id)
                && self.has_cycle_dfs(node_id, &mut visited, &mut rec_stack)
            {
                return true;
            }
        }

        false
    }

    /// DFS helper for cycle detection
    fn has_cycle_dfs(
        &self,
        node_id: &NodeId,
        visited: &mut HashSet<NodeId>,
        rec_stack: &mut HashSet<NodeId>,
    ) -> bool {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        // Check all neighbors (nodes this node points to)
        // Optimize: only iterate through edges that start from this node
        for edge in self.get_outgoing_edges(node_id) {
            let neighbor = &edge.target;

            // If neighbor not visited, recurse
            if !visited.contains(neighbor) {
                if self.has_cycle_dfs(neighbor, visited, rec_stack) {
                    return true;
                }
            }
            // If neighbor is in recursion stack, we found a back edge (cycle)
            else if rec_stack.contains(neighbor) {
                return true;
            }
        }

        rec_stack.remove(node_id);
        false
    }

    /// Check if adding an edge from source to target would create a cycle
    ///
    /// This is useful for preventing cycles during interactive edge creation.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn creates_cycle(&self, source: &NodeId, target: &NodeId) -> bool {
        // If nodes don't exist, no cycle can be created
        if !self.nodes.contains_key(source) || !self.nodes.contains_key(target) {
            return false;
        }

        // Check if target can reach source (would create cycle if we add source -> target)
        self.can_reach(target, source)
    }

    /// Check if 'from' node can reach 'to' node through existing edges
    fn can_reach(&self, from: &NodeId, to: &NodeId) -> bool {
        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(from.clone());
        visited.insert(from.clone());

        while let Some(current) = queue.pop_front() {
            // Check all outgoing edges from current node
            // Optimize: only iterate through edges that start from this node
            for edge in self.get_outgoing_edges(&current) {
                let neighbor = &edge.target;

                if neighbor == to {
                    return true;
                }

                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }

        false
    }

    /// Find a cycle in the graph, returning the cycle path if found
    ///
    /// Returns the first cycle found, or None if the graph is acyclic.
    /// The returned path represents the nodes in the cycle.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn find_cycle(&self) -> Option<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut parent = HashMap::new();

        // Check each node as potential starting point
        for node_id in self.node_ids() {
            if !visited.contains(node_id) {
                if let Some(cycle) =
                    self.find_cycle_dfs(node_id, &mut visited, &mut rec_stack, &mut parent)
                {
                    return Some(cycle);
                }
            }
        }

        None
    }

    /// DFS helper for finding cycle path
    fn find_cycle_dfs(
        &self,
        node_id: &NodeId,
        visited: &mut HashSet<NodeId>,
        rec_stack: &mut HashSet<NodeId>,
        parent: &mut HashMap<NodeId, NodeId>,
    ) -> Option<Vec<NodeId>> {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        // Check all neighbors
        // Optimize: only iterate through edges that start from this node
        for edge in self.get_outgoing_edges(node_id) {
            let neighbor = &edge.target;

            // If neighbor not visited, recurse
            if !visited.contains(neighbor) {
                parent.insert(neighbor.clone(), node_id.clone());
                if let Some(cycle) = self.find_cycle_dfs(neighbor, visited, rec_stack, parent) {
                    return Some(cycle);
                }
            }
            // If neighbor is in recursion stack, we found a cycle
            else if rec_stack.contains(neighbor) {
                // Reconstruct cycle path
                let mut cycle = vec![neighbor.clone()];
                let mut current = node_id.clone();

                // Walk back through parents until we reach the cycle start
                while current != *neighbor {
                    cycle.push(current.clone());
                    current = parent.get(&current).unwrap_or(&current).clone();
                }

                cycle.reverse();
                return Some(cycle);
            }
        }

        rec_stack.remove(node_id);
        None
    }

    /// Perform topological sort on the graph using Kahn's algorithm
    ///
    /// Returns a valid topological ordering of nodes, or Err if the graph contains cycles.
    /// A topological sort is a linear ordering where for every directed edge (u, v),
    /// vertex u comes before v in the ordering.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn topological_sort(&self) -> Result<Vec<NodeId>> {
        // Calculate in-degrees
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();

        // Initialize all nodes with in-degree 0
        for node_id in self.node_ids() {
            in_degree.insert(node_id.clone(), 0);
        }

        // Count incoming edges for each node
        for edge in self.edges() {
            *in_degree.entry(edge.target.clone()).or_insert(0) += 1;
        }

        // Find nodes with in-degree 0
        let mut queue = VecDeque::new();
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }

        let mut result = Vec::new();

        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());

            // Reduce in-degree of neighbors
            for edge in self.edges() {
                if edge.source == node_id {
                    let neighbor = &edge.target;
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // If we didn't process all nodes, there must be a cycle
        if result.len() != self.node_count() {
            return Err(FlowError::invalid_operation(
                "Graph contains cycles - topological sort not possible",
            ));
        }

        Ok(result)
    }
}
