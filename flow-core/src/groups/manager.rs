//! Group manager for handling group operations

use crate::error::{FlowError, Result};
use crate::graph::Graph;
use crate::types::{GroupId, NodeId, Position};
use std::collections::{HashMap, HashSet};

use super::drag_state::GroupDragState;
use super::group::Group;

/// Group manager for handling group operations
#[derive(Debug, Clone)]
pub struct GroupManager {
    groups: HashMap<GroupId, Group>,
    node_to_group: HashMap<NodeId, GroupId>,
    drag_state: Option<GroupDragState>,
}

impl GroupManager {
    /// Create a new group manager
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            node_to_group: HashMap::new(),
            drag_state: None,
        }
    }

    /// Create a new group with the given members
    pub fn create_group(&mut self, group_id: GroupId, members: HashSet<NodeId>) -> Result<()> {
        // Validate that group doesn't already exist
        if self.groups.contains_key(&group_id) {
            return Err(FlowError::invalid_operation(format!(
                "Group {} already exists",
                group_id
            )));
        }

        // Validate that group has members
        if members.is_empty() {
            return Err(FlowError::invalid_operation("Group must have at least one member"));
        }

        // Validate that all nodes are not already in groups
        for node_id in &members {
            if self.node_to_group.contains_key(node_id) {
                return Err(FlowError::invalid_operation(format!(
                    "Node {} is already in a group",
                    node_id
                )));
            }
        }

        // Create group and update mappings
        let group = Group::new(group_id.clone(), members.clone());
        self.groups.insert(group_id.clone(), group);

        for node_id in members {
            self.node_to_group.insert(node_id, group_id.clone());
        }

        Ok(())
    }

    /// Get a group by ID
    pub fn get_group(&self, group_id: &GroupId) -> Option<&Group> {
        self.groups.get(group_id)
    }

    /// Get a mutable reference to a group by ID
    pub fn get_group_mut(&mut self, group_id: &GroupId) -> Option<&mut Group> {
        self.groups.get_mut(group_id)
    }

    /// Get the group ID for a node
    pub fn get_node_group(&self, node_id: &NodeId) -> Option<&GroupId> {
        self.node_to_group.get(node_id)
    }

    /// Dissolve a group and return its members
    pub fn dissolve_group(&mut self, group_id: &GroupId) -> Result<HashSet<NodeId>> {
        let group = self
            .groups
            .remove(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        // Remove node-to-group mappings
        for node_id in &group.members {
            self.node_to_group.remove(node_id);
        }

        Ok(group.members)
    }

    /// Get all groups
    pub fn all_groups(&self) -> &HashMap<GroupId, Group> {
        &self.groups
    }

    /// Get all node-to-group mappings
    pub fn node_mappings(&self) -> &HashMap<NodeId, GroupId> {
        &self.node_to_group
    }

    /// Add a node to an existing group
    pub fn add_node_to_group(&mut self, group_id: &GroupId, node_id: NodeId) -> Result<()> {
        // Check if node is already in a group
        if self.node_to_group.contains_key(&node_id) {
            return Err(FlowError::invalid_operation(format!(
                "Node {} is already in a group",
                node_id
            )));
        }

        // Check if group exists
        let group = self
            .groups
            .get_mut(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        // Add node to group and update mapping
        group.add_member(node_id.clone());
        self.node_to_group.insert(node_id, group_id.clone());

        Ok(())
    }

    /// Remove a node from a group
    pub fn remove_node_from_group(&mut self, group_id: &GroupId, node_id: &NodeId) -> Result<()> {
        // Check if group exists
        let group = self
            .groups
            .get_mut(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        // Check if node is in this group
        if !group.contains_node(node_id) {
            return Err(FlowError::invalid_operation(format!(
                "Node {} is not in group {}",
                node_id, group_id
            )));
        }

        // Remove node from group and clear mapping
        group.remove_member(node_id);
        self.node_to_group.remove(node_id);

        // If group is now empty, dissolve it
        if group.member_count() == 0 {
            self.groups.remove(group_id);
        }

        Ok(())
    }

    /// Set the name of a group
    pub fn set_group_name(&mut self, group_id: &GroupId, name: Option<String>) -> Result<()> {
        let group = self
            .groups
            .get_mut(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        group.name = name;
        Ok(())
    }

    /// Calculate the bounding rectangle for a group based on its member positions
    pub fn calculate_group_bounds<N, E>(
        &mut self,
        group_id: &GroupId,
        graph: &Graph<N, E>,
    ) -> Result<()>
    where
        N: Clone,
        E: Clone,
    {
        let group = self
            .groups
            .get_mut(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        if group.members.is_empty() {
            return Ok(());
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for node_id in &group.members {
            if let Some(node) = graph.get_node(node_id) {
                let node_left = node.position.x;
                let node_right = node.position.x + node.size.width;
                let node_top = node.position.y;
                let node_bottom = node.position.y + node.size.height;

                min_x = min_x.min(node_left);
                min_y = min_y.min(node_top);
                max_x = max_x.max(node_right);
                max_y = max_y.max(node_bottom);
            }
        }

        group.position = Position::new(min_x, min_y);
        group.size = crate::types::Size::new(max_x - min_x, max_y - min_y);

        Ok(())
    }

    /// Update group bounds based on current member positions
    pub fn update_group_bounds<N, E>(
        &mut self,
        group_id: &GroupId,
        graph: &Graph<N, E>,
    ) -> Result<()>
    where
        N: Clone,
        E: Clone,
    {
        self.calculate_group_bounds(group_id, graph)
    }

    /// Get all group IDs
    pub fn group_ids(&self) -> Vec<GroupId> {
        self.groups.keys().cloned().collect()
    }

    /// Check if a group exists
    pub fn has_group(&self, group_id: &GroupId) -> bool {
        self.groups.contains_key(group_id)
    }

    /// Get the number of groups
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Check if a node is in any group
    pub fn is_node_grouped(&self, node_id: &NodeId) -> bool {
        self.node_to_group.contains_key(node_id)
    }

    // Group Drag Operations

    /// Start a group drag operation
    pub fn start_group_drag<N, E>(
        &mut self,
        group_id: &GroupId,
        start_position: Position,
        graph: &Graph<N, E>,
    ) -> Result<()>
    where
        N: Clone,
        E: Clone,
    {
        // Check if already dragging
        if self.drag_state.is_some() {
            return Err(FlowError::invalid_operation("Group drag already in progress"));
        }

        // Check if group exists
        let group = self
            .groups
            .get(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        // Create drag state
        let mut drag_state = GroupDragState::new(group_id.clone(), start_position);
        drag_state.store_original_positions(group, graph)?;

        self.drag_state = Some(drag_state);
        Ok(())
    }

    /// Update group drag position
    pub fn update_group_drag<N, E>(
        &mut self,
        new_position: Position,
        graph: &mut Graph<N, E>,
    ) -> Result<Position>
    where
        N: Clone,
        E: Clone,
    {
        let drag_state = self
            .drag_state
            .as_mut()
            .ok_or_else(|| FlowError::invalid_operation("No group drag in progress"))?;

        let delta = drag_state.update_position(new_position);

        // Move all nodes in the group
        if let Some(group) = self.groups.get(&drag_state.dragging_group) {
            for node_id in &group.members {
                if let Some(node) = graph.get_node_mut(node_id) {
                    if let Some(original_pos) = drag_state.original_node_positions.get(node_id) {
                        node.position = Position::new(
                            original_pos.x + delta.x,
                            original_pos.y + delta.y,
                        );
                    }
                }
            }
        }

        Ok(delta)
    }

    /// Complete group drag operation
    pub fn complete_group_drag(&mut self) -> Result<GroupId> {
        let drag_state = self
            .drag_state
            .take()
            .ok_or_else(|| FlowError::invalid_operation("No group drag in progress"))?;

        Ok(drag_state.dragging_group)
    }

    /// Cancel group drag operation and restore original positions
    pub fn cancel_group_drag<N, E>(
        &mut self,
        graph: &mut Graph<N, E>,
    ) -> Result<GroupId>
    where
        N: Clone,
        E: Clone,
    {
        let drag_state = self
            .drag_state
            .take()
            .ok_or_else(|| FlowError::invalid_operation("No group drag in progress"))?;

        // Restore original positions
        for (node_id, original_position) in &drag_state.original_node_positions {
            if let Some(node) = graph.get_node_mut(node_id) {
                node.position = *original_position;
            }
        }

        Ok(drag_state.dragging_group)
    }

    /// Check if a group is currently being dragged
    pub fn is_group_dragging(&self) -> bool {
        self.drag_state.is_some()
    }

    /// Get the current dragging group ID (if any)
    pub fn get_dragging_group(&self) -> Option<&GroupId> {
        self.drag_state.as_ref().map(|state| &state.dragging_group)
    }

    /// Get the current drag delta (if any)
    pub fn get_drag_delta(&self) -> Option<Position> {
        self.drag_state
            .as_ref()
            .map(|state| state.delta_from_start())
    }

    /// Check if there's an active drag operation
    pub fn has_active_drag(&self) -> bool {
        self.drag_state.is_some()
    }

    /// Move a group by the given delta
    pub fn move_group<N, E>(&mut self, group_id: &GroupId, delta: Position, graph: &mut Graph<N, E>) -> Result<()> 
    where
        N: Clone,
    {
        // Get the group
        let group = self.get_group(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        // Move all nodes in the group
        for node_id in &group.members {
            if let Some(node) = graph.get_node_mut(node_id) {
                let new_position = Position::new(
                    node.position.x + delta.x,
                    node.position.y + delta.y,
                );
                node.set_position(new_position);
            }
        }

        Ok(())
    }
}
