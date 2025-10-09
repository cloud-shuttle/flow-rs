//! Conflict Resolution Strategies for Collaborative Editing
//!
//! Provides CRDT-inspired conflict resolution strategies for handling
//! concurrent operations in collaborative graph editing.

use crate::collaboration::operational_transform::{GraphOperation, Operation, OperationMetadata};
use crate::collaboration::collaborative_session::CollaborationError;

/// Conflict resolution strategies
#[derive(Debug)]
pub enum ConflictResolutionStrategy {
    /// Last writer wins - simple but may lose user intent
    LastWriterWins,
    /// Manual resolution required - pause collaboration for user decision
    Manual,
    /// Automatic merging when possible - intelligent conflict resolution
    AutoMerge,
    /// Client-side resolution with UI - show conflicts to users
    ClientSide,
}

impl Default for ConflictResolutionStrategy {
    fn default() -> Self {
        ConflictResolutionStrategy::AutoMerge
    }
}

/// Custom conflict resolver trait
pub trait ConflictResolver: Send + Sync {
    fn resolve_conflict(&self, operation1: &Operation, operation2: &Operation, graph_state: &crate::graph::Graph<String, String>) -> ConflictResolution;
}

/// Result of conflict resolution
#[derive(Clone, Debug)]
pub enum ConflictResolution {
    /// Apply first operation, discard second
    ApplyFirst,
    /// Apply second operation, discard first
    ApplySecond,
    /// Apply both operations (transformed)
    ApplyBoth(GraphOperation, GraphOperation),
    /// Merge operations into a single operation
    Merge(GraphOperation),
    /// Require manual resolution
    ManualResolutionNeeded(String),
    /// Cannot resolve automatically
    Unresolvable(String),
}

/// Conflict resolver for automatic merging
pub struct AutoMergeResolver;

impl AutoMergeResolver {
    pub fn new() -> Self {
        Self
    }

    /// Attempt to automatically resolve conflicts
    pub fn resolve(&self, op1: &Operation, op2: &Operation) -> ConflictResolution {
        match (&op1.operation, &op2.operation) {
            // Two nodes added with different IDs - both can coexist
            (GraphOperation::AddNode { node: n1, position: p1 },
             GraphOperation::AddNode { node: n2, position: p2 }) => {
                if n1.id != n2.id {
                    ConflictResolution::ApplyBoth(op1.operation.clone(), op2.operation.clone())
                } else {
                    // Same node ID - resolve by offsetting position
                    let offset_op2 = GraphOperation::AddNode {
                        node: n2.clone(),
                        position: crate::types::Position::new(p2.x + 20.0, p2.y + 20.0),
                    };
                    ConflictResolution::ApplyBoth(op1.operation.clone(), offset_op2)
                }
            }

            // Node moved by both users - use average position or last writer wins
            (GraphOperation::MoveNode { node_id: id1, to: to1, .. },
             GraphOperation::MoveNode { node_id: id2, to: to2, .. }) => {
                if id1 == id2 {
                    // Same node moved - use last writer wins for simplicity
                    if op1.metadata.timestamp > op2.metadata.timestamp {
                        ConflictResolution::ApplyFirst
                    } else {
                        ConflictResolution::ApplySecond
                    }
                } else {
                    // Different nodes - both can move
                    ConflictResolution::ApplyBoth(op1.operation.clone(), op2.operation.clone())
                }
            }

            // Conflicting node updates - merge if possible
            (GraphOperation::UpdateNode { node_id: id1, new_data: data1, .. },
             GraphOperation::UpdateNode { node_id: id2, new_data: data2, .. }) => {
                if id1 == id2 {
                    // Same node updated - last writer wins for now
                    // TODO: Implement more sophisticated merging (e.g., for structured data)
                    if op1.metadata.timestamp > op2.metadata.timestamp {
                        ConflictResolution::ApplyFirst
                    } else {
                        ConflictResolution::ApplySecond
                    }
                } else {
                    // Different nodes - both can be updated
                    ConflictResolution::ApplyBoth(op1.operation.clone(), op2.operation.clone())
                }
            }

            // Two edges added - check if they're different
            (GraphOperation::AddEdge { edge: e1, .. },
             GraphOperation::AddEdge { edge: e2, .. }) => {
                if e1.id != e2.id {
                    ConflictResolution::ApplyBoth(op1.operation.clone(), op2.operation.clone())
                } else {
                    // Same edge ID - this is a conflict
                    ConflictResolution::ManualResolutionNeeded(
                        format!("Duplicate edge ID: {}", e1.id)
                    )
                }
            }

            // One user deletes what another modifies - deletion takes precedence
            (GraphOperation::RemoveNode { node_id: remove_id },
             GraphOperation::MoveNode { node_id: move_id, .. }) => {
                if remove_id == move_id {
                    // Deletion takes precedence over modification
                    ConflictResolution::ApplyFirst
                } else {
                    ConflictResolution::ApplyBoth(op1.operation.clone(), op2.operation.clone())
                }
            }
            (GraphOperation::RemoveNode { node_id: remove_id },
             GraphOperation::UpdateNode { node_id: update_id, .. }) => {
                if remove_id == update_id {
                    // Deletion takes precedence over modification
                    ConflictResolution::ApplyFirst
                } else {
                    ConflictResolution::ApplyBoth(op1.operation.clone(), op2.operation.clone())
                }
            }

            // Symmetric cases
            (GraphOperation::MoveNode { .. }, GraphOperation::RemoveNode { .. }) |
            (GraphOperation::UpdateNode { .. }, GraphOperation::RemoveNode { .. }) => {
                self.resolve(op2, op1).swap_operations()
            }

            // Default: cannot resolve automatically
            _ => ConflictResolution::ManualResolutionNeeded(
                "Complex conflict requiring manual resolution".to_string()
            ),
        }
    }
}

impl ConflictResolver for AutoMergeResolver {
    fn resolve_conflict(&self, operation1: &Operation, operation2: &Operation, _graph_state: &crate::graph::Graph<String, String>) -> ConflictResolution {
        self.resolve(operation1, operation2)
    }
}

impl ConflictResolution {
    /// Check if resolution was successful
    pub fn is_successful(&self) -> bool {
        !matches!(self,
            ConflictResolution::ManualResolutionNeeded(_) |
            ConflictResolution::Unresolvable(_)
        )
    }

    /// Check if manual resolution is needed
    pub fn needs_manual_resolution(&self) -> bool {
        matches!(self, ConflictResolution::ManualResolutionNeeded(_))
    }

    /// Get resolution description
    pub fn description(&self) -> String {
        match self {
            ConflictResolution::ApplyFirst => "Applied first operation".to_string(),
            ConflictResolution::ApplySecond => "Applied second operation".to_string(),
            ConflictResolution::ApplyBoth(_, _) => "Applied both operations".to_string(),
            ConflictResolution::Merge(_) => "Merged operations".to_string(),
            ConflictResolution::ManualResolutionNeeded(reason) => format!("Manual resolution needed: {}", reason),
            ConflictResolution::Unresolvable(reason) => format!("Cannot resolve: {}", reason),
        }
    }

    /// Swap the operations in the resolution (for symmetric cases)
    fn swap_operations(self) -> Self {
        match self {
            ConflictResolution::ApplyFirst => ConflictResolution::ApplySecond,
            ConflictResolution::ApplySecond => ConflictResolution::ApplyFirst,
            ConflictResolution::ApplyBoth(op1, op2) => ConflictResolution::ApplyBoth(op2, op1),
            other => other, // Other cases are not affected by operation order
        }
    }
}

/// Conflict resolution manager
pub struct ConflictResolutionManager {
    strategy: ConflictResolutionStrategy,
    resolver: AutoMergeResolver,
}

impl ConflictResolutionManager {
    pub fn new(strategy: ConflictResolutionStrategy) -> Self {
        let resolver = AutoMergeResolver::new();
        Self { strategy, resolver }
    }

    /// Resolve a conflict between two operations
    pub fn resolve_conflict(&self, operation1: &Operation, operation2: &Operation, graph_state: &crate::graph::Graph<String, String>) -> Result<ConflictResolution, CollaborationError> {
        match &self.strategy {
            ConflictResolutionStrategy::LastWriterWins => {
                Ok(if operation1.metadata.timestamp > operation2.metadata.timestamp {
                    ConflictResolution::ApplyFirst
                } else {
                    ConflictResolution::ApplySecond
                })
            }
            ConflictResolutionStrategy::Manual => {
                Ok(ConflictResolution::ManualResolutionNeeded(
                    "Manual resolution requested".to_string()
                ))
            }
            ConflictResolutionStrategy::AutoMerge => {
                Ok(self.resolver.resolve_conflict(operation1, operation2, graph_state))
            }
            ConflictResolutionStrategy::ClientSide => {
                // In client-side resolution, we send the conflict to clients
                // For now, treat as manual
                Ok(ConflictResolution::ManualResolutionNeeded(
                    "Client-side resolution needed".to_string()
                ))
            }
        }
    }

    /// Get the current resolution strategy
    pub fn strategy(&self) -> &ConflictResolutionStrategy {
        &self.strategy
    }

    /// Change the resolution strategy
    pub fn set_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.strategy = strategy;
        // All strategies use the same resolver for now
    }

    /// Get conflict resolution statistics
    pub fn stats(&self) -> ConflictResolutionStats {
        ConflictResolutionStats {
            strategy: format!("{:?}", self.strategy),
            // In a real implementation, we'd track resolution statistics
            total_conflicts: 0,
            auto_resolved: 0,
            manual_required: 0,
        }
    }
}

/// Statistics about conflict resolution
#[derive(Clone, Debug)]
pub struct ConflictResolutionStats {
    pub strategy: String,
    pub total_conflicts: u64,
    pub auto_resolved: u64,
    pub manual_required: u64,
}

impl ConflictResolutionStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_conflicts == 0 {
            0.0
        } else {
            (self.auto_resolved as f64) / (self.total_conflicts as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Node;
    use crate::types::Position;

    fn create_test_operation(node_id: &str, timestamp: u64) -> Operation {
        Operation {
            operation: GraphOperation::AddNode {
                node: Node::new(node_id.to_string(), Position::new(100.0, 100.0), "test".to_string()),
                position: Position::new(100.0, 100.0),
            },
            metadata: crate::collaboration::operational_transform::OperationMetadata {
                id: format!("op-{}", node_id),
                client_id: "client-1".to_string(),
                timestamp,
                sequence_number: 0,
                parent_operations: vec![],
            },
        }
    }

    #[test]
    fn test_auto_merge_different_nodes() {
        let resolver = AutoMergeResolver::new();
        let op1 = create_test_operation("node-1", 1000);
        let op2 = create_test_operation("node-2", 1001);

        let resolution = resolver.resolve(&op1, &op2);
        assert!(resolution.is_successful());

        match resolution {
            ConflictResolution::ApplyBoth(_, _) => {} // Expected
            _ => panic!("Expected ApplyBoth"),
        }
    }

    #[test]
    fn test_auto_merge_same_node_id() {
        let resolver = AutoMergeResolver::new();
        let op1 = create_test_operation("node-1", 1000);
        let mut op2 = create_test_operation("node-1", 1001);
        op2.operation = GraphOperation::AddNode {
            node: Node::new("node-1".to_string(), Position::new(100.0, 100.0), "test".to_string()),
            position: Position::new(100.0, 100.0),
        };

        let resolution = resolver.resolve(&op1, &op2);
        assert!(resolution.is_successful());

        match resolution {
            ConflictResolution::ApplyBoth(_, GraphOperation::AddNode { position, .. }) => {
                // Second operation should be offset
                assert_eq!(position.x, 120.0);
                assert_eq!(position.y, 120.0);
            }
            _ => panic!("Expected ApplyBoth with offset"),
        }
    }

    #[test]
    fn test_conflict_resolution_manager() {
        let manager = ConflictResolutionManager::new(ConflictResolutionStrategy::AutoMerge);
        assert!(matches!(manager.strategy(), ConflictResolutionStrategy::AutoMerge));

        let graph = crate::graph::Graph::new();
        let op1 = create_test_operation("node-1", 1000);
        let op2 = create_test_operation("node-2", 1001);

        let resolution = manager.resolve_conflict(&op1, &op2, &graph).unwrap();
        assert!(resolution.is_successful());
    }

    #[test]
    fn test_last_writer_wins_strategy() {
        let manager = ConflictResolutionManager::new(ConflictResolutionStrategy::LastWriterWins);

        let graph = crate::graph::Graph::new();
        let op1 = create_test_operation("node-1", 1000); // Earlier
        let op2 = create_test_operation("node-2", 2000); // Later

        let resolution = manager.resolve_conflict(&op1, &op2, &graph).unwrap();
        match resolution {
            ConflictResolution::ApplySecond => {} // Should apply the later operation
            _ => panic!("Expected ApplySecond for LastWriterWins"),
        }
    }

    #[test]
    fn test_manual_resolution_strategy() {
        let manager = ConflictResolutionManager::new(ConflictResolutionStrategy::Manual);

        let graph = crate::graph::Graph::new();
        let op1 = create_test_operation("node-1", 1000);
        let op2 = create_test_operation("node-2", 1001);

        let resolution = manager.resolve_conflict(&op1, &op2, &graph).unwrap();
        assert!(resolution.needs_manual_resolution());
    }

    #[test]
    fn test_conflict_resolution_stats() {
        let manager = ConflictResolutionManager::new(ConflictResolutionStrategy::AutoMerge);
        let stats = manager.stats();

        assert_eq!(stats.total_conflicts, 0);
        assert_eq!(stats.auto_resolved, 0);
        assert_eq!(stats.manual_required, 0);
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_conflict_resolution_descriptions() {
        let apply_first = ConflictResolution::ApplyFirst;
        assert_eq!(apply_first.description(), "Applied first operation");

        let manual = ConflictResolution::ManualResolutionNeeded("test".to_string());
        assert!(manual.description().contains("Manual resolution needed"));
        assert!(manual.needs_manual_resolution());
        assert!(!manual.is_successful());
    }
}
