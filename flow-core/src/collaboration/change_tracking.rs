//! Change Tracking for Undo/Redo Across Users
//!
//! Manages operation history and provides undo/redo functionality
//! that works across multiple collaborative users.

use std::collections::VecDeque;
use crate::collaboration::operational_transform::{Operation, GraphOperation, OperationMetadata};

/// Tracked change for undo/redo
#[derive(Clone, Debug)]
pub struct TrackedChange {
    pub operation: Operation,
    pub inverse_operation: GraphOperation,
    pub timestamp: u64,
    pub participant_id: String,
}

impl TrackedChange {
    pub fn new(operation: Operation, inverse_operation: GraphOperation, participant_id: String) -> Self {
        Self {
            operation,
            inverse_operation,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            participant_id,
        }
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// Change operation types
#[derive(Clone, Debug)]
pub enum ChangeOperation {
    Execute(Operation),
    Undo(TrackedChange),
    Redo(TrackedChange),
}

/// Change tracking for undo/redo across users
pub struct ChangeTracker {
    changes: VecDeque<TrackedChange>,
    current_index: usize,
    max_history_size: usize,
}

impl ChangeTracker {
    pub fn new() -> Self {
        Self {
            changes: VecDeque::new(),
            current_index: 0,
            max_history_size: 100, // Default max history
        }
    }

    pub fn with_max_history_size(mut self, max_size: usize) -> Self {
        self.max_history_size = max_size;
        self
    }

    /// Track a change
    pub fn track_change(&mut self, change: TrackedChange) {
        // Remove any changes after current index (when user made new changes after undo)
        while self.changes.len() > self.current_index {
            self.changes.pop_back();
        }

        // Add new change
        self.changes.push_back(change);
        self.current_index = self.changes.len();

        // Enforce max history size
        while self.changes.len() > self.max_history_size {
            self.changes.pop_front();
            self.current_index = self.current_index.saturating_sub(1);
        }
    }

    /// Undo the last change
    pub fn undo(&mut self) -> Option<&TrackedChange> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.changes[self.current_index])
        } else {
            None
        }
    }

    /// Redo the next change
    pub fn redo(&mut self) -> Option<&TrackedChange> {
        if self.current_index < self.changes.len() {
            let change = &self.changes[self.current_index];
            self.current_index += 1;
            Some(change)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.current_index < self.changes.len()
    }

    /// Get the current undo operation (preview)
    pub fn peek_undo(&self) -> Option<&TrackedChange> {
        if self.current_index > 0 {
            Some(&self.changes[self.current_index - 1])
        } else {
            None
        }
    }

    /// Get the current redo operation (preview)
    pub fn peek_redo(&self) -> Option<&TrackedChange> {
        if self.current_index < self.changes.len() {
            Some(&self.changes[self.current_index])
        } else {
            None
        }
    }

    /// Clear all change history
    pub fn clear(&mut self) {
        self.changes.clear();
        self.current_index = 0;
    }

    /// Get change history (for debugging/serialization)
    pub fn history(&self) -> &VecDeque<TrackedChange> {
        &self.changes
    }

    /// Get current history index
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Get total number of changes in history
    pub fn history_size(&self) -> usize {
        self.changes.len()
    }

    /// Get maximum history size
    pub fn max_history_size(&self) -> usize {
        self.max_history_size
    }

    /// Set maximum history size
    pub fn set_max_history_size(&mut self, max_size: usize) {
        self.max_history_size = max_size;

        // Enforce new limit
        while self.changes.len() > self.max_history_size {
            self.changes.pop_front();
            self.current_index = self.current_index.saturating_sub(1);
        }
    }

    /// Create inverse operation for a given operation
    pub fn create_inverse_operation(operation: &Operation) -> Option<GraphOperation> {
        match &operation.operation {
            GraphOperation::AddNode { node, .. } => {
                Some(GraphOperation::RemoveNode { node_id: node.id.clone() })
            }
            GraphOperation::RemoveNode { node_id } => {
                // Note: We can't create a perfect inverse for RemoveNode without knowing the original node data
                // In a real implementation, we'd store the original node data
                Some(GraphOperation::AddNode {
                    node: crate::graph::Node::new(
                        node_id.to_string(),
                        crate::types::Position::new(0.0, 0.0),
                        String::new()
                    ),
                    position: crate::types::Position::new(0.0, 0.0),
                })
            }
            GraphOperation::MoveNode { node_id, from, to } => {
                Some(GraphOperation::MoveNode {
                    node_id: node_id.clone(),
                    from: *to,
                    to: *from,
                })
            }
            GraphOperation::UpdateNode { node_id, old_data, new_data } => {
                Some(GraphOperation::UpdateNode {
                    node_id: node_id.clone(),
                    old_data: new_data.clone(),
                    new_data: old_data.clone(),
                })
            }
            GraphOperation::AddEdge { edge, .. } => {
                Some(GraphOperation::RemoveEdge { edge_id: edge.id.clone() })
            }
            GraphOperation::RemoveEdge { edge_id } => {
                // Note: Similar to RemoveNode, we can't create a perfect inverse
                Some(GraphOperation::AddEdge {
                    edge: crate::graph::Edge::new(
                        edge_id.to_string(),
                        String::new(),
                        String::new(),
                        String::new()
                    ),
                    source: crate::types::NodeId::new("unknown-source"),
                    target: crate::types::NodeId::new("unknown-target"),
                })
            }
            GraphOperation::UpdateEdge { edge_id, old_data, new_data } => {
                Some(GraphOperation::UpdateEdge {
                    edge_id: edge_id.clone(),
                    old_data: new_data.clone(),
                    new_data: old_data.clone(),
                })
            }
        }
    }

    /// Get statistics about change tracking
    pub fn stats(&self) -> ChangeTrackerStats {
        ChangeTrackerStats {
            total_changes: self.changes.len(),
            current_index: self.current_index,
            max_history_size: self.max_history_size,
            can_undo: self.can_undo(),
            can_redo: self.can_redo(),
            memory_usage_bytes: self.estimate_memory_usage(),
        }
    }

    /// Estimate memory usage of the change tracker
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimation: each TrackedChange has some overhead
        // This is a simplified calculation
        self.changes.len() * std::mem::size_of::<TrackedChange>()
    }
}

/// Statistics about change tracking
#[derive(Clone, Debug)]
pub struct ChangeTrackerStats {
    pub total_changes: usize,
    pub current_index: usize,
    pub max_history_size: usize,
    pub can_undo: bool,
    pub can_redo: bool,
    pub memory_usage_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Node;
    use crate::types::Position;

    fn create_test_operation() -> Operation {
        Operation {
            operation: GraphOperation::AddNode {
                node: Node::new("test-node".to_string(), Position::new(100.0, 100.0), "test".to_string()),
                position: Position::new(100.0, 100.0),
            },
            metadata: crate::collaboration::operational_transform::OperationMetadata {
                id: "test-op".to_string(),
                client_id: "test-client".to_string(),
                timestamp: 1234567890,
                sequence_number: 0,
                parent_operations: vec![],
            },
        }
    }

    fn create_test_inverse() -> GraphOperation {
        GraphOperation::RemoveNode {
            node_id: "test-node".into(),
        }
    }

    #[test]
    fn test_change_tracker_creation() {
        let tracker = ChangeTracker::new();
        assert_eq!(tracker.history_size(), 0);
        assert_eq!(tracker.current_index(), 0);
        assert!(!tracker.can_undo());
        assert!(!tracker.can_redo());
    }

    #[test]
    fn test_track_change() {
        let mut tracker = ChangeTracker::new();
        let operation = create_test_operation();
        let inverse = create_test_inverse();

        let change = TrackedChange::new(operation, inverse, "user-1".to_string());
        tracker.track_change(change);

        assert_eq!(tracker.history_size(), 1);
        assert_eq!(tracker.current_index(), 1);
        assert!(tracker.can_undo());
        assert!(!tracker.can_redo());
    }

    #[test]
    fn test_undo_redo() {
        let mut tracker = ChangeTracker::new();

        // Add two changes
        let change1 = TrackedChange::new(create_test_operation(), create_test_inverse(), "user-1".to_string());
        let change2 = TrackedChange::new(create_test_operation(), create_test_inverse(), "user-1".to_string());

        tracker.track_change(change1);
        tracker.track_change(change2);

        assert_eq!(tracker.current_index(), 2);
        assert!(tracker.can_undo());
        assert!(!tracker.can_redo());

        // Undo
        let undone = tracker.undo();
        assert!(undone.is_some());
        assert_eq!(tracker.current_index(), 1);
        assert!(tracker.can_undo());
        assert!(tracker.can_redo());

        // Redo
        let redone = tracker.redo();
        assert!(redone.is_some());
        assert_eq!(tracker.current_index(), 2);
        assert!(tracker.can_undo());
        assert!(!tracker.can_redo());
    }

    #[test]
    fn test_max_history_size() {
        let mut tracker = ChangeTracker::new().with_max_history_size(2);

        // Add three changes
        for i in 0..3 {
            let operation = create_test_operation();
            let inverse = create_test_inverse();
            let change = TrackedChange::new(operation, inverse, format!("user-{}", i));
            tracker.track_change(change);
        }

        // Should only keep 2 changes
        assert_eq!(tracker.history_size(), 2);
        assert_eq!(tracker.current_index(), 2);
    }

    #[test]
    fn test_clear_history() {
        let mut tracker = ChangeTracker::new();
        let change = TrackedChange::new(create_test_operation(), create_test_inverse(), "user-1".to_string());
        tracker.track_change(change);

        assert_eq!(tracker.history_size(), 1);

        tracker.clear();
        assert_eq!(tracker.history_size(), 0);
        assert_eq!(tracker.current_index(), 0);
    }

    #[test]
    fn test_create_inverse_operation() {
        let add_node_op = Operation {
            operation: GraphOperation::AddNode {
                node: Node::new("test".to_string(), Position::new(0.0, 0.0), "data".to_string()),
                position: Position::new(0.0, 0.0),
            },
            metadata: crate::collaboration::operational_transform::OperationMetadata {
                id: "test".to_string(),
                client_id: "client".to_string(),
                timestamp: 0,
                sequence_number: 0,
                parent_operations: vec![],
            },
        };

        let inverse = ChangeTracker::create_inverse_operation(&add_node_op);
        assert!(inverse.is_some());

        match inverse.unwrap() {
            GraphOperation::RemoveNode { node_id } => {
                assert_eq!(node_id.to_string(), "test");
            }
            _ => panic!("Expected RemoveNode operation"),
        }
    }

    #[test]
    fn test_change_tracker_stats() {
        let mut tracker = ChangeTracker::new().with_max_history_size(10);
        let change = TrackedChange::new(create_test_operation(), create_test_inverse(), "user-1".to_string());
        tracker.track_change(change);

        let stats = tracker.stats();
        assert_eq!(stats.total_changes, 1);
        assert_eq!(stats.current_index, 1);
        assert_eq!(stats.max_history_size, 10);
        assert!(stats.can_undo);
        assert!(!stats.can_redo);
        assert!(stats.memory_usage_bytes > 0);
    }
}
