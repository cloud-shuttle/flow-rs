//! Group data structure and basic operations

use crate::types::{GroupId, NodeId, Position, Rect, Size};
use std::collections::HashSet;

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
