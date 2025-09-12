//! Handle system for precise node connections
//!
//! Handles are connection points on nodes that allow for precise edge attachment.
//! They support input/output semantics, type validation, and positioning.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::{Position, Size, NodeId};
use crate::error::{FlowError, Result};

/// Handle identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HandleId(pub String);

impl HandleId {
    /// Create a new handle ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get handle ID as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for HandleId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for HandleId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

/// Handle type indicating connection direction
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HandleType {
    Source,
    Target,
}

impl HandleType {
    /// Check if this is a source handle
    pub fn is_source(&self) -> bool {
        matches!(self, HandleType::Source)
    }

    /// Check if this is a target handle
    pub fn is_target(&self) -> bool {
        matches!(self, HandleType::Target)
    }
}

/// Handle position relative to node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HandlePosition {
    Top,
    Right,
    Bottom,
    Left,
    Custom(Position),
}

impl HandlePosition {
    /// Convert handle position to absolute position relative to node
    pub fn to_position(&self) -> Position {
        match self {
            HandlePosition::Top => Position::new(0.0, -10.0),
            HandlePosition::Right => Position::new(80.0, 30.0),
            HandlePosition::Bottom => Position::new(40.0, 60.0),
            HandlePosition::Left => Position::new(0.0, 30.0),
            HandlePosition::Custom(pos) => *pos,
        }
    }
}

/// Connection handle on a node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Handle {
    pub id: HandleId,
    pub handle_type: HandleType,
    pub position: HandlePosition,
    pub connection_limit: Option<usize>,
    pub valid_connection_types: Option<Vec<String>>,
    pub style: Option<String>,
}

impl Handle {
    /// Create a new handle
    pub fn new(
        id: impl Into<HandleId>,
        handle_type: HandleType,
        position: HandlePosition,
    ) -> Self {
        Self {
            id: id.into(),
            handle_type,
            position,
            connection_limit: None,
            valid_connection_types: None,
            style: None,
        }
    }

    /// Create a source handle
    pub fn source(id: impl Into<HandleId>, position: HandlePosition) -> Self {
        Self::new(id, HandleType::Source, position)
    }

    /// Create a target handle
    pub fn target(id: impl Into<HandleId>, position: HandlePosition) -> Self {
        Self::new(id, HandleType::Target, position)
    }

    /// Set connection limit
    pub fn with_connection_limit(mut self, limit: usize) -> Self {
        self.connection_limit = Some(limit);
        self
    }

    /// Set valid connection types
    pub fn with_connection_types(mut self, types: Vec<String>) -> Self {
        self.valid_connection_types = Some(types);
        self
    }

    /// Set style
    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Calculate absolute position of handle given node position and size
    pub fn absolute_position(&self, node_pos: Position, node_size: Size) -> Position {
        match &self.position {
            HandlePosition::Top => Position::new(
                node_pos.x + node_size.width / 2.0,
                node_pos.y,
            ),
            HandlePosition::Right => Position::new(
                node_pos.x + node_size.width,
                node_pos.y + node_size.height / 2.0,
            ),
            HandlePosition::Bottom => Position::new(
                node_pos.x + node_size.width / 2.0,
                node_pos.y + node_size.height,
            ),
            HandlePosition::Left => Position::new(
                node_pos.x,
                node_pos.y + node_size.height / 2.0,
            ),
            HandlePosition::Custom(pos) => Position::new(
                node_pos.x + pos.x,
                node_pos.y + pos.y,
            ),
        }
    }

    /// Check if a point is within the handle's bounds
    pub fn contains_point(&self, point: Position, node_pos: Position, node_size: Size, handle_size: f64) -> bool {
        let handle_pos = self.absolute_position(node_pos, node_size);
        let half_size = handle_size / 2.0;

        point.x >= handle_pos.x - half_size &&
        point.x <= handle_pos.x + half_size &&
        point.y >= handle_pos.y - half_size &&
        point.y <= handle_pos.y + half_size
    }

    /// Check if this handle can connect to another handle
    pub fn can_connect_to(&self, other: &Handle) -> bool {
        // Can't connect to same type (source to source, target to target)
        if self.handle_type == other.handle_type {
            return false;
        }

        // Check connection type compatibility
        if let (Some(self_types), Some(other_types)) = (&self.valid_connection_types, &other.valid_connection_types) {
            // At least one matching type required
            return self_types.iter().any(|t| other_types.contains(t));
        }

        true // No type restrictions
    }
}

/// Handle manager for a node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HandleManager {
    handles: Vec<Handle>,
    node_id: NodeId,
}

impl Default for HandleManager {
    fn default() -> Self {
        Self {
            handles: Vec::new(),
            node_id: "default".into(),
        }
    }
}

impl HandleManager {
    /// Create a new handle manager
    pub fn new(node_id: NodeId) -> Self {
        Self {
            handles: Vec::new(),
            node_id,
        }
    }

    /// Add a handle
    pub fn add_handle(&mut self, handle: Handle) -> Result<()> {
        // Check for duplicate handle IDs
        if self.handles.iter().any(|h| h.id == handle.id) {
            return Err(FlowError::invalid_operation(
                &format!("Handle '{}' already exists", handle.id.as_str())
            ));
        }

        self.handles.push(handle);
        Ok(())
    }

    /// Remove a handle
    pub fn remove_handle(&mut self, handle_id: &HandleId) -> Result<Handle> {
        let index = self.handles.iter().position(|h| &h.id == handle_id)
            .ok_or_else(|| FlowError::invalid_operation(
                &format!("Handle '{}' not found", handle_id.as_str())
            ))?;

        Ok(self.handles.remove(index))
    }

    /// Get a handle by ID
    pub fn get_handle(&self, handle_id: &HandleId) -> Option<&Handle> {
        self.handles.iter().find(|h| &h.id == handle_id)
    }

    /// Get all handles
    pub fn handles(&self) -> &[Handle] {
        &self.handles
    }

    /// Find handle at position
    pub fn handle_at_position(&self, point: Position, node_pos: Position, node_size: Size, handle_size: f64) -> Option<&Handle> {
        self.handles.iter().find(|handle| {
            handle.contains_point(point, node_pos, node_size, handle_size)
        })
    }

    /// Get source handles
    pub fn source_handles(&self) -> impl Iterator<Item = &Handle> {
        self.handles.iter().filter(|h| h.handle_type == HandleType::Source)
    }

    /// Get target handles
    pub fn target_handles(&self) -> impl Iterator<Item = &Handle> {
        self.handles.iter().filter(|h| h.handle_type == HandleType::Target)
    }

    /// Count connections for a handle
    ///
    /// Note: This method requires graph integration to work properly.
    /// Use `Graph::get_handle_connections()` directly for accurate counting.
    pub fn connection_count(&self, _handle_id: &HandleId) -> usize {
        // This method is deprecated in favor of Graph-level connection counting
        // because it needs access to the graph's edge collection to count connections accurately.
        // Use graph.get_handle_connections(node_id, handle_id).len() instead.
        0
    }

    /// Check if handle can accept new connections
    ///
    /// Note: This method requires graph integration to work properly.
    /// Use `Graph::can_handle_accept_connection()` for accurate validation.
    pub fn can_accept_connection(&self, handle_id: &HandleId) -> bool {
        if let Some(handle) = self.get_handle(handle_id) {
            if let Some(_limit) = handle.connection_limit {
                // This method is deprecated in favor of Graph-level validation
                // because it needs access to the graph's edge collection.
                // Use graph.can_handle_accept_connection(node_id, handle_id) instead.
                return true; // Conservative default
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Position, Size};

    #[test]
    fn test_handle_creation() {
        let handle = Handle::new("output", HandleType::Source, HandlePosition::Right);

        assert_eq!(handle.id.as_str(), "output");
        assert_eq!(handle.handle_type, HandleType::Source);
        assert_eq!(handle.position, HandlePosition::Right);
        assert!(handle.connection_limit.is_none());
    }

    #[test]
    fn test_handle_builder_methods() {
        let handle = Handle::source("out", HandlePosition::Right)
            .with_connection_limit(1)
            .with_connection_types(vec!["data".to_string()])
            .with_style("custom");

        assert_eq!(handle.connection_limit, Some(1));
        assert_eq!(handle.valid_connection_types, Some(vec!["data".to_string()]));
        assert_eq!(handle.style, Some("custom".to_string()));
    }

    #[test]
    fn test_absolute_position_calculation() {
        let node_pos = Position::new(100.0, 200.0);
        let node_size = Size::new(80.0, 60.0);

        // Test standard positions
        let top_handle = Handle::new("top", HandleType::Source, HandlePosition::Top);
        assert_eq!(
            top_handle.absolute_position(node_pos, node_size),
            Position::new(140.0, 200.0) // x: 100 + 40, y: 200
        );

        let right_handle = Handle::new("right", HandleType::Source, HandlePosition::Right);
        assert_eq!(
            right_handle.absolute_position(node_pos, node_size),
            Position::new(180.0, 230.0) // x: 100 + 80, y: 200 + 30
        );

        let custom_handle = Handle::new("custom", HandleType::Source, HandlePosition::Custom(Position::new(10.0, 20.0)));
        assert_eq!(
            custom_handle.absolute_position(node_pos, node_size),
            Position::new(110.0, 220.0) // x: 100 + 10, y: 200 + 20
        );
    }

    #[test]
    fn test_point_inside_handle() {
        let handle = Handle::new("test", HandleType::Source, HandlePosition::Right);
        let node_pos = Position::new(0.0, 0.0);
        let node_size = Size::new(100.0, 50.0);
        let handle_size = 10.0;

        // Handle is at (100, 25) with size 10x10
        assert!(handle.contains_point(Position::new(100.0, 25.0), node_pos, node_size, handle_size));
        assert!(handle.contains_point(Position::new(95.0, 25.0), node_pos, node_size, handle_size));
        assert!(handle.contains_point(Position::new(105.0, 25.0), node_pos, node_size, handle_size));
        assert!(!handle.contains_point(Position::new(90.0, 25.0), node_pos, node_size, handle_size));
        assert!(!handle.contains_point(Position::new(100.0, 35.0), node_pos, node_size, handle_size));
    }

    #[test]
    fn test_handle_connection_compatibility() {
        let source_handle = Handle::source("out", HandlePosition::Right);
        let target_handle = Handle::target("in", HandlePosition::Left);
        let another_source = Handle::source("out2", HandlePosition::Bottom);

        // Source can connect to target
        assert!(source_handle.can_connect_to(&target_handle));
        assert!(target_handle.can_connect_to(&source_handle));

        // Source cannot connect to source
        assert!(!source_handle.can_connect_to(&another_source));
    }

    #[test]
    fn test_handle_connection_type_validation() {
        let data_source = Handle::source("data_out", HandlePosition::Right)
            .with_connection_types(vec!["data".to_string()]);
        let data_target = Handle::target("data_in", HandlePosition::Left)
            .with_connection_types(vec!["data".to_string()]);
        let control_target = Handle::target("control_in", HandlePosition::Left)
            .with_connection_types(vec!["control".to_string()]);

        // Compatible types
        assert!(data_source.can_connect_to(&data_target));

        // Incompatible types
        assert!(!data_source.can_connect_to(&control_target));
    }

    #[test]
    fn test_handle_manager_operations() {
        let mut manager = HandleManager::new("node1".into());

        // Add handles
        let handle1 = Handle::source("out", HandlePosition::Right);
        let handle2 = Handle::target("in", HandlePosition::Left);

        manager.add_handle(handle1.clone()).unwrap();
        manager.add_handle(handle2.clone()).unwrap();

        assert_eq!(manager.handles().len(), 2);

        // Test duplicate prevention
        assert!(manager.add_handle(handle1.clone()).is_err());

        // Test retrieval
        assert!(manager.get_handle(&"out".into()).is_some());
        assert!(manager.get_handle(&"nonexistent".into()).is_none());

        // Test removal
        let removed = manager.remove_handle(&"out".into()).unwrap();
        assert_eq!(removed.id.as_str(), "out");
        assert_eq!(manager.handles().len(), 1);
    }

    #[test]
    fn test_handle_manager_find_at_position() {
        let mut manager = HandleManager::new("node1".into());
        let handle = Handle::new("right", HandleType::Source, HandlePosition::Right);
        manager.add_handle(handle).unwrap();

        let node_pos = Position::new(0.0, 0.0);
        let node_size = Size::new(100.0, 50.0);
        let handle_size = 10.0;

        // Should find handle at right position (100, 25)
        let found = manager.handle_at_position(Position::new(100.0, 25.0), node_pos, node_size, handle_size);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id.as_str(), "right");

        // Should not find handle at wrong position
        let not_found = manager.handle_at_position(Position::new(50.0, 25.0), node_pos, node_size, handle_size);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_handle_type_filtering() {
        let mut manager = HandleManager::new("node1".into());

        manager.add_handle(Handle::source("out1", HandlePosition::Right)).unwrap();
        manager.add_handle(Handle::source("out2", HandlePosition::Top)).unwrap();
        manager.add_handle(Handle::target("in1", HandlePosition::Left)).unwrap();

        let sources: Vec<_> = manager.source_handles().collect();
        let targets: Vec<_> = manager.target_handles().collect();

        assert_eq!(sources.len(), 2);
        assert_eq!(targets.len(), 1);
    }
}
