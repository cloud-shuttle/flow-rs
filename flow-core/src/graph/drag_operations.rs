//! Drag and drop operations for graph nodes

use std::collections::HashSet;

use crate::error::{FlowError, Result};
use crate::types::{NodeId, Position, Rect};

use super::{Graph, Node};

impl<N, E> Graph<N, E> {
    /// Apply drag operation to selected nodes
    pub fn apply_node_drag(
        &mut self,
        selected_nodes: &HashSet<NodeId>,
        delta: Position,
    ) -> Result<()> {
        self.apply_node_drag_with_transform(selected_nodes, delta, |pos, _| pos)
    }

    /// Apply drag operation with bounds constraint
    pub fn apply_node_drag_with_bounds(
        &mut self,
        selected_nodes: &HashSet<NodeId>,
        delta: Position,
        bounds: Option<Rect>,
    ) -> Result<()> {
        self.apply_node_drag_with_transform(selected_nodes, delta, |new_pos, node| {
            if let Some(bounds) = bounds {
                Position::new(
                    new_pos
                        .x
                        .max(bounds.x)
                        .min(bounds.x + bounds.width - node.size.width),
                    new_pos
                        .y
                        .max(bounds.y)
                        .min(bounds.y + bounds.height - node.size.height),
                )
            } else {
                new_pos
            }
        })
    }

    /// Apply drag operation with grid snapping
    pub fn apply_node_drag_with_snap(
        &mut self,
        selected_nodes: &HashSet<NodeId>,
        delta: Position,
        grid_size: f64,
    ) -> Result<()> {
        self.apply_node_drag_with_transform(selected_nodes, delta, |new_pos, _| {
            Position::new(
                (new_pos.x / grid_size).round() * grid_size,
                (new_pos.y / grid_size).round() * grid_size,
            )
        })
    }

    /// Apply drag operation with custom constraint function
    pub fn apply_node_drag_with_constraint<F>(
        &mut self,
        selected_nodes: &HashSet<NodeId>,
        delta: Position,
        constraint: F,
    ) -> Result<()>
    where
        F: Fn(Position) -> Position,
    {
        self.apply_node_drag_with_transform(selected_nodes, delta, |new_pos, _| constraint(new_pos))
    }

    /// Internal method for applying drag operations with position transformation
    fn apply_node_drag_with_transform<F>(
        &mut self,
        selected_nodes: &HashSet<NodeId>,
        delta: Position,
        transform: F,
    ) -> Result<()>
    where
        F: Fn(Position, &Node<N>) -> Position,
    {
        // Pre-validate all nodes exist to fail fast
        for node_id in selected_nodes {
            if !self.nodes.contains_key(node_id) {
                return Err(FlowError::node_not_found(node_id.as_str()));
            }
        }

        // Apply transformations
        for node_id in selected_nodes {
            if let Some(node) = self.get_node_mut(node_id) {
                let new_pos = Position::new(node.position.x + delta.x, node.position.y + delta.y);
                node.position = transform(new_pos, node);
            }
        }

        Ok(())
    }

    /// Create a drag operation for undo/redo support
    pub fn create_drag_operation(
        &self,
        selected_nodes: &HashSet<NodeId>,
        delta: Position,
    ) -> Result<crate::drag_operations::DragOperation> {
        // Validate all nodes exist before creating operation
        for node_id in selected_nodes {
            if !self.nodes.contains_key(node_id) {
                return Err(FlowError::node_not_found(node_id.as_str()));
            }
        }

        Ok(crate::drag_operations::DragOperation::new(
            selected_nodes.clone(),
            delta,
        ))
    }
}
