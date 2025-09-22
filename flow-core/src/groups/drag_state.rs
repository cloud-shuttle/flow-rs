//! Group drag state management

use crate::graph::Graph;
use crate::types::{GroupId, NodeId, Position};
use std::collections::HashMap;

use super::group::Group;

/// Group drag state for tracking ongoing drag operations
#[derive(Debug, Clone)]
pub struct GroupDragState {
    /// Group being dragged
    pub dragging_group: GroupId,
    /// Starting position when drag began
    pub start_position: Position,
    /// Current drag position
    pub current_position: Position,
    /// Original positions of all nodes before drag started
    pub original_node_positions: HashMap<NodeId, Position>,
}

impl GroupDragState {
    /// Create a new group drag state
    pub fn new(group_id: GroupId, start_position: Position) -> Self {
        Self {
            dragging_group: group_id,
            start_position,
            current_position: start_position,
            original_node_positions: HashMap::new(),
        }
    }

    /// Update the current drag position and return the delta
    pub fn update_position(&mut self, new_position: Position) -> Position {
        self.current_position = new_position;
        self.delta_from_start()
    }

    /// Get the delta from the starting position
    pub fn delta_from_start(&self) -> Position {
        Position::new(
            self.current_position.x - self.start_position.x,
            self.current_position.y - self.start_position.y,
        )
    }

    /// Store original positions of all nodes in the group
    pub fn store_original_positions<N, E>(
        &mut self,
        group: &Group,
        graph: &Graph<N, E>,
    ) -> Result<(), crate::error::FlowError>
    where
        N: Clone,
        E: Clone,
    {
        for node_id in &group.members {
            if let Some(node) = graph.get_node(node_id) {
                self.original_node_positions.insert(node_id.clone(), node.position);
            } else {
                return Err(crate::error::FlowError::invalid_operation(format!(
                    "Node {} not found in graph",
                    node_id
                )));
            }
        }
        Ok(())
    }
}
