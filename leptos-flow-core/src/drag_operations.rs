//! Drag operations for node manipulation
//!
//! Provides drag operation types and utilities for moving nodes in a graph.

use std::collections::HashSet;
use crate::{Result, NodeId, Position};
use crate::graph::Graph;

/// A drag operation that can be applied to nodes
#[derive(Debug, Clone)]
pub struct DragOperation {
    affected_nodes: HashSet<NodeId>,
    delta: Position,
}

impl DragOperation {
    /// Create a new drag operation
    pub fn new(affected_nodes: HashSet<NodeId>, delta: Position) -> Self {
        Self {
            affected_nodes,
            delta,
        }
    }

    /// Get the affected nodes
    pub fn affected_nodes(&self) -> &HashSet<NodeId> {
        &self.affected_nodes
    }

    /// Get the drag delta
    pub fn delta(&self) -> Position {
        self.delta
    }

    /// Check if this operation affects any nodes
    pub fn is_empty(&self) -> bool {
        self.affected_nodes.is_empty() || (self.delta.x == 0.0 && self.delta.y == 0.0)
    }

    /// Get the number of affected nodes
    pub fn node_count(&self) -> usize {
        self.affected_nodes.len()
    }

    /// Apply this operation to a graph using the graph's drag method
    pub fn apply_to_graph<N, E>(&self, graph: &mut Graph<N, E>) -> Result<()> {
        graph.apply_node_drag(&self.affected_nodes, self.delta)
    }

    /// Create an inverse operation for undo
    pub fn create_inverse(&self) -> Self {
        Self {
            affected_nodes: self.affected_nodes.clone(),
            delta: Position::new(-self.delta.x, -self.delta.y),
        }
    }

    /// Combine multiple drag operations into one
    pub fn combine(operations: Vec<Self>) -> Self {
        let mut all_nodes = HashSet::new();
        let mut total_delta = Position::new(0.0, 0.0);

        for op in operations {
            all_nodes.extend(op.affected_nodes);
            total_delta.x += op.delta.x;
            total_delta.y += op.delta.y;
        }

        Self::new(all_nodes, total_delta)
    }
}
