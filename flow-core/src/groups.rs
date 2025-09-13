//! Node Grouping System
//!
//! Manages hierarchical organization of nodes through grouping operations.

use crate::error::{FlowError, Result};
use crate::types::{GroupId, NodeId, Position, Rect, Size};
use std::collections::{HashMap, HashSet};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Node group with metadata and member management
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Group {
    pub id: GroupId,
    pub name: Option<String>,
    pub members: HashSet<NodeId>,
    pub position: Position,
    pub size: Size,
    pub collapsed: bool,
    pub color: Option<String>,
    pub parent_group: Option<GroupId>,
    pub child_groups: HashSet<GroupId>,
}

impl Group {
    /// Create a new group with the given members
    pub fn new(id: GroupId, members: HashSet<NodeId>) -> Self {
        Self {
            id,
            name: None,
            members,
            position: Position::zero(),
            size: Size::default(),
            collapsed: false,
            color: None,
            parent_group: None,
            child_groups: HashSet::new(),
        }
    }

    /// Check if the group contains a specific node
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.members.contains(node_id)
    }

    /// Get the number of members in this group
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Add a node to this group
    pub fn add_member(&mut self, node_id: NodeId) -> bool {
        self.members.insert(node_id)
    }

    /// Remove a node from this group
    pub fn remove_member(&mut self, node_id: &NodeId) -> bool {
        self.members.remove(node_id)
    }

    /// Get bounding rectangle for the group
    pub fn bounds(&self) -> Rect {
        Rect::from_pos_size(self.position, self.size)
    }
}

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

    /// Store original node positions before starting the drag
    pub fn store_original_positions<N, E>(
        &mut self,
        group: &Group,
        graph: &crate::graph::Graph<N, E>,
    ) where
        N: Clone,
        E: Clone,
    {
        self.original_node_positions.clear();
        for node_id in &group.members {
            if let Some(node) = graph.get_node(node_id) {
                self.original_node_positions
                    .insert(node_id.clone(), node.position);
            }
        }
    }
}

/// Manager for all group operations
#[derive(Debug, Clone, Default)]
pub struct GroupManager {
    groups: HashMap<GroupId, Group>,
    node_to_group: HashMap<NodeId, GroupId>,
    /// Current drag state (if any group is being dragged)
    drag_state: Option<GroupDragState>,
}

impl GroupManager {
    /// Create a new group manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a group from a set of nodes
    pub fn create_group(&mut self, group_id: GroupId, members: HashSet<NodeId>) -> Result<()> {
        // Check for existing group membership conflicts
        for node_id in &members {
            if self.node_to_group.contains_key(node_id) {
                return Err(FlowError::invalid_operation(format!(
                    "Node {} is already in a group",
                    node_id
                )));
            }
        }

        if members.is_empty() {
            return Err(FlowError::invalid_operation(
                "Cannot create group with no members",
            ));
        }

        let group = Group::new(group_id.clone(), members.clone());

        // Update mappings
        for node_id in members {
            self.node_to_group.insert(node_id, group_id.clone());
        }

        self.groups.insert(group_id, group);
        Ok(())
    }

    /// Get a group by ID
    pub fn get_group(&self, group_id: &GroupId) -> Option<&Group> {
        self.groups.get(group_id)
    }

    /// Get a mutable group by ID
    pub fn get_group_mut(&mut self, group_id: &GroupId) -> Option<&mut Group> {
        self.groups.get_mut(group_id)
    }

    /// Find which group contains a node
    pub fn get_node_group(&self, node_id: &NodeId) -> Option<&GroupId> {
        self.node_to_group.get(node_id)
    }

    /// Dissolve a group, removing all memberships
    pub fn dissolve_group(&mut self, group_id: &GroupId) -> Result<HashSet<NodeId>> {
        let group = self
            .groups
            .remove(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        // Remove node mappings
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
        graph: &crate::graph::Graph<N, E>,
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

        if min_x.is_finite() && min_y.is_finite() && max_x.is_finite() && max_y.is_finite() {
            group.position = Position::new(min_x, min_y);
            group.size = Size::new(max_x - min_x, max_y - min_y);
        }

        Ok(())
    }

    /// Move all nodes in a group by a delta offset
    pub fn move_group<N, E>(
        &self,
        group_id: &GroupId,
        delta: Position,
        graph: &mut crate::graph::Graph<N, E>,
    ) -> Result<()>
    where
        N: Clone,
        E: Clone,
    {
        let group = self
            .groups
            .get(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        for node_id in &group.members {
            if let Some(node) = graph.get_node_mut(node_id) {
                node.position = node.position.add(delta);
            }
        }

        Ok(())
    }

    /// Check if a position is within the bounds of a group
    pub fn is_point_in_group(&self, group_id: &GroupId, point: Position) -> bool {
        if let Some(group) = self.groups.get(group_id) {
            let bounds = group.bounds();
            point.x >= bounds.x
                && point.x <= bounds.x + bounds.width
                && point.y >= bounds.y
                && point.y <= bounds.y + bounds.height
        } else {
            false
        }
    }

    /// Start dragging a group
    pub fn start_group_drag<N, E>(
        &mut self,
        group_id: &GroupId,
        start_position: Position,
        graph: &crate::graph::Graph<N, E>,
    ) -> Result<()>
    where
        N: Clone,
        E: Clone,
    {
        let group = self
            .groups
            .get(group_id)
            .ok_or_else(|| FlowError::invalid_operation(format!("Group {} not found", group_id)))?;

        let mut drag_state = GroupDragState::new(group_id.clone(), start_position);
        drag_state.store_original_positions(group, graph);

        self.drag_state = Some(drag_state);
        Ok(())
    }

    /// Update group drag position and move all nodes
    pub fn update_group_drag<N, E>(
        &mut self,
        new_position: Position,
        graph: &mut crate::graph::Graph<N, E>,
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

        // Move all nodes in the group by the delta
        let group_id = drag_state.dragging_group.clone();
        self.move_group(&group_id, delta, graph)?;

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
        graph: &mut crate::graph::Graph<N, E>,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_nodes() -> Vec<NodeId> {
        vec![
            NodeId::new("node1"),
            NodeId::new("node2"),
            NodeId::new("node3"),
        ]
    }

    #[test]
    fn test_create_group_from_nodes() {
        // RED: This should fail initially
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.into_iter().take(2).collect();
        let group_id = GroupId::new("test_group");

        let result = manager.create_group(group_id.clone(), members.clone());

        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.members.len(), 2);
        assert!(group.contains_node(&NodeId::new("node1")));
        assert!(group.contains_node(&NodeId::new("node2")));
    }

    #[test]
    fn test_empty_group_creation_fails() {
        let mut manager = GroupManager::new();
        let empty_members = HashSet::new();
        let group_id = GroupId::new("empty_group");

        let result = manager.create_group(group_id, empty_members);

        assert!(result.is_err());
        assert!(matches!(result, Err(FlowError::InvalidOperation { .. })));
    }

    #[test]
    fn test_node_already_in_group_fails() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let node1 = nodes[0].clone();

        // Create first group with node1
        let group1_members: HashSet<NodeId> = [node1.clone()].into_iter().collect();
        manager
            .create_group(GroupId::new("group1"), group1_members)
            .unwrap();

        // Try to create second group with same node - should fail
        let group2_members: HashSet<NodeId> = [node1].into_iter().collect();
        let result = manager.create_group(GroupId::new("group2"), group2_members);

        assert!(result.is_err());
    }

    #[test]
    fn test_dissolve_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.into_iter().take(2).collect();
        let group_id = GroupId::new("test_group");

        // Create group
        manager
            .create_group(group_id.clone(), members.clone())
            .unwrap();
        assert!(manager.get_group(&group_id).is_some());

        // Dissolve group
        let dissolved_members = manager.dissolve_group(&group_id).unwrap();
        assert_eq!(dissolved_members, members);
        assert!(manager.get_group(&group_id).is_none());

        // Check node mappings are cleared
        for node_id in &dissolved_members {
            assert!(manager.get_node_group(node_id).is_none());
        }
    }

    #[test]
    fn test_node_to_group_mapping() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.into_iter().take(2).collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        assert_eq!(
            manager.get_node_group(&NodeId::new("node1")),
            Some(&group_id)
        );
        assert_eq!(
            manager.get_node_group(&NodeId::new("node2")),
            Some(&group_id)
        );
        assert_eq!(manager.get_node_group(&NodeId::new("node3")), None);
    }

    #[test]
    fn test_group_bounds() {
        let members: HashSet<NodeId> = [NodeId::new("node1")].into_iter().collect();
        let mut group = Group::new(GroupId::new("test"), members);

        group.position = Position::new(10.0, 20.0);
        group.size = Size::new(100.0, 50.0);

        let bounds = group.bounds();
        assert_eq!(bounds.position(), Position::new(10.0, 20.0));
        assert_eq!(bounds.size(), Size::new(100.0, 50.0));
    }

    #[test]
    fn test_add_node_to_group() {
        // RED: This should fail initially
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let initial_members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        // Create group with one node
        manager
            .create_group(group_id.clone(), initial_members)
            .unwrap();

        // Add another node to the group
        let result = manager.add_node_to_group(&group_id, nodes[1].clone());

        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.member_count(), 2);
        assert!(group.contains_node(&nodes[1]));

        // Check mapping is updated
        assert_eq!(manager.get_node_group(&nodes[1]), Some(&group_id));
    }

    #[test]
    fn test_remove_node_from_group() {
        // RED: This should fail initially
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.clone().into_iter().take(2).collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        // Remove a node from the group
        let result = manager.remove_node_from_group(&group_id, &nodes[0]);

        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.member_count(), 1);
        assert!(!group.contains_node(&nodes[0]));
        assert!(group.contains_node(&nodes[1]));

        // Check mapping is cleared for removed node
        assert_eq!(manager.get_node_group(&nodes[0]), None);
        assert_eq!(manager.get_node_group(&nodes[1]), Some(&group_id));
    }

    #[test]
    fn test_remove_last_node_dissolves_group() {
        // RED: This should fail initially
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        // Remove the last node - should dissolve the group
        let result = manager.remove_node_from_group(&group_id, &nodes[0]);

        assert!(result.is_ok());
        assert!(manager.get_group(&group_id).is_none()); // Group should be gone
        assert_eq!(manager.get_node_group(&nodes[0]), None);
    }

    #[test]
    fn test_add_node_already_in_another_group_fails() {
        // RED: This should fail initially
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();

        // Create two groups
        let group1_members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group2_members: HashSet<NodeId> = [nodes[1].clone()].into_iter().collect();
        let group1_id = GroupId::new("group1");
        let group2_id = GroupId::new("group2");

        manager
            .create_group(group1_id.clone(), group1_members)
            .unwrap();
        manager
            .create_group(group2_id.clone(), group2_members)
            .unwrap();

        // Try to add node from group1 to group2 - should fail
        let result = manager.add_node_to_group(&group2_id, nodes[0].clone());

        assert!(result.is_err());
        assert!(matches!(result, Err(FlowError::InvalidOperation { .. })));
    }

    #[test]
    fn test_rename_group() {
        // RED: This should fail initially
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        let result = manager.set_group_name(&group_id, Some("New Group Name".to_string()));

        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.name, Some("New Group Name".to_string()));
    }

    // Group drag and bounds tests
    #[test]
    fn test_calculate_group_bounds() {
        // RED: This should fail initially
        use crate::{Graph, Node, Size};

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Add nodes with different positions and sizes
        let mut node1 = Node::simple("node1", Position::new(100.0, 100.0));
        node1.set_size(Size::new(50.0, 30.0));

        let mut node2 = Node::simple("node2", Position::new(200.0, 150.0));
        node2.set_size(Size::new(40.0, 25.0));

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // Create group
        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();

        // Calculate bounds
        let result = group_manager.calculate_group_bounds(&group_id, &graph);
        assert!(result.is_ok());

        let group = group_manager.get_group(&group_id).unwrap();

        // Expected bounds: from (100, 100) to (240, 175)
        // Position should be (100, 100), Size should be (140, 75)
        assert_eq!(group.position, Position::new(100.0, 100.0));
        assert_eq!(group.size, Size::new(140.0, 75.0));
    }

    #[test]
    fn test_move_group() {
        // RED: This should fail initially
        use crate::{Graph, Node};

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Add nodes
        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let node2 = Node::simple("node2", Position::new(200.0, 150.0));

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // Create group
        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();

        // Move group by delta (50, 25)
        let delta = Position::new(50.0, 25.0);
        let result = group_manager.move_group(&group_id, delta, &mut graph);
        assert!(result.is_ok());

        // Check that all nodes moved by the delta
        let node1_after = graph.get_node(&NodeId::new("node1")).unwrap();
        let node2_after = graph.get_node(&NodeId::new("node2")).unwrap();

        assert_eq!(node1_after.position, Position::new(150.0, 125.0));
        assert_eq!(node2_after.position, Position::new(250.0, 175.0));
    }

    #[test]
    fn test_is_point_in_group() {
        // RED: This should fail initially
        let mut group_manager = GroupManager::new();

        // Create a group with known bounds
        let members: HashSet<NodeId> = [NodeId::new("node1")].into_iter().collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();

        // Set group bounds manually
        let group = group_manager.get_group_mut(&group_id).unwrap();
        group.position = Position::new(100.0, 100.0);
        group.size = Size::new(50.0, 30.0);

        // Test points
        assert!(group_manager.is_point_in_group(&group_id, Position::new(125.0, 115.0))); // Inside
        assert!(!group_manager.is_point_in_group(&group_id, Position::new(50.0, 50.0))); // Outside
        assert!(!group_manager.is_point_in_group(&group_id, Position::new(200.0, 200.0))); // Outside

        // Test boundary points
        assert!(group_manager.is_point_in_group(&group_id, Position::new(100.0, 100.0))); // Top-left corner
        assert!(group_manager.is_point_in_group(&group_id, Position::new(150.0, 130.0)));
        // Bottom-right corner
    }

    #[test]
    fn test_empty_group_bounds_calculation() {
        use crate::Graph;

        let _graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Create empty group (should not happen in practice, but test edge case)
        let empty_members: HashSet<NodeId> = HashSet::new();
        let group_id = GroupId::new("empty_group");

        // This should fail due to empty members check
        let result = group_manager.create_group(group_id.clone(), empty_members);
        assert!(result.is_err());
    }

    #[test]
    fn test_move_nonexistent_group() {
        use crate::Graph;

        let mut graph = Graph::<(), ()>::new();
        let group_manager = GroupManager::new();

        let fake_group_id = GroupId::new("fake_group");
        let delta = Position::new(10.0, 10.0);

        let result = group_manager.move_group(&fake_group_id, delta, &mut graph);
        assert!(result.is_err());
        assert!(matches!(result, Err(FlowError::InvalidOperation { .. })));
    }

    // Group drag state tests
    #[test]
    fn test_group_drag_state_creation() {
        // RED: This should fail initially
        let group_id = GroupId::new("test_group");
        let start_pos = Position::new(100.0, 100.0);

        let drag_state = GroupDragState::new(group_id.clone(), start_pos);

        assert_eq!(drag_state.dragging_group, group_id);
        assert_eq!(drag_state.start_position, start_pos);
        assert_eq!(drag_state.current_position, start_pos);
        assert!(drag_state.original_node_positions.is_empty());
        assert_eq!(drag_state.delta_from_start(), Position::zero());
    }

    #[test]
    fn test_group_drag_state_update_position() {
        let group_id = GroupId::new("test_group");
        let start_pos = Position::new(100.0, 100.0);
        let mut drag_state = GroupDragState::new(group_id, start_pos);

        let new_pos = Position::new(150.0, 125.0);
        let delta = drag_state.update_position(new_pos);

        assert_eq!(drag_state.current_position, new_pos);
        assert_eq!(delta, Position::new(50.0, 25.0));
        assert_eq!(drag_state.delta_from_start(), Position::new(50.0, 25.0));
    }

    #[test]
    fn test_start_group_drag() {
        use crate::{Graph, Node};

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Add nodes and create group
        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let node2 = Node::simple("node2", Position::new(200.0, 150.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();

        // Start drag
        let start_pos = Position::new(125.0, 110.0);
        let result = group_manager.start_group_drag(&group_id, start_pos, &graph);

        assert!(result.is_ok());
        assert!(group_manager.is_group_dragging());
        assert_eq!(group_manager.get_dragging_group(), Some(&group_id));
        assert_eq!(group_manager.get_drag_delta(), Some(Position::zero()));
    }

    #[test]
    fn test_update_group_drag() {
        use crate::{Graph, Node};

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Setup group
        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let node2 = Node::simple("node2", Position::new(200.0, 150.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();

        // Start and update drag
        let start_pos = Position::new(125.0, 110.0);
        group_manager
            .start_group_drag(&group_id, start_pos, &graph)
            .unwrap();

        let new_pos = Position::new(175.0, 135.0);
        let result = group_manager.update_group_drag(new_pos, &mut graph);

        assert!(result.is_ok());
        let delta = result.unwrap();
        assert_eq!(delta, Position::new(50.0, 25.0));
        assert_eq!(
            group_manager.get_drag_delta(),
            Some(Position::new(50.0, 25.0))
        );

        // Check nodes moved
        let node1_after = graph.get_node(&NodeId::new("node1")).unwrap();
        let node2_after = graph.get_node(&NodeId::new("node2")).unwrap();
        assert_eq!(node1_after.position, Position::new(150.0, 125.0));
        assert_eq!(node2_after.position, Position::new(250.0, 175.0));
    }

    #[test]
    fn test_complete_group_drag() {
        use crate::{Graph, Node};

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Setup and start drag
        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        graph.add_node(node1).unwrap();

        let members: HashSet<NodeId> = [NodeId::new("node1")].into_iter().collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();
        group_manager
            .start_group_drag(&group_id, Position::zero(), &graph)
            .unwrap();

        // Complete drag
        let result = group_manager.complete_group_drag();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), group_id);
        assert!(!group_manager.is_group_dragging());
        assert_eq!(group_manager.get_dragging_group(), None);
    }

    #[test]
    fn test_cancel_group_drag() {
        use crate::{Graph, Node};

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Setup group
        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let node2 = Node::simple("node2", Position::new(200.0, 150.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager
            .create_group(group_id.clone(), members)
            .unwrap();

        // Start drag and move
        group_manager
            .start_group_drag(&group_id, Position::zero(), &graph)
            .unwrap();
        group_manager
            .update_group_drag(Position::new(50.0, 25.0), &mut graph)
            .unwrap();

        // Verify nodes moved
        assert_eq!(
            graph.get_node(&NodeId::new("node1")).unwrap().position,
            Position::new(150.0, 125.0)
        );

        // Cancel drag
        let result = group_manager.cancel_group_drag(&mut graph);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), group_id);
        assert!(!group_manager.is_group_dragging());

        // Verify nodes restored to original positions
        assert_eq!(
            graph.get_node(&NodeId::new("node1")).unwrap().position,
            Position::new(100.0, 100.0)
        );
        assert_eq!(
            graph.get_node(&NodeId::new("node2")).unwrap().position,
            Position::new(200.0, 150.0)
        );
    }

    #[test]
    fn test_drag_operations_without_active_drag() {
        use crate::Graph;

        let mut graph = Graph::<(), ()>::new();
        let mut group_manager = GroupManager::new();

        // Try to update drag without starting
        let result = group_manager.update_group_drag(Position::zero(), &mut graph);
        assert!(result.is_err());
        assert!(matches!(result, Err(FlowError::InvalidOperation { .. })));

        // Try to complete drag without starting
        let result = group_manager.complete_group_drag();
        assert!(result.is_err());

        // Try to cancel drag without starting
        let result = group_manager.cancel_group_drag(&mut graph);
        assert!(result.is_err());
    }
}
