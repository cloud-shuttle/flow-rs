//! Operational Transformation for Conflict-Free Collaborative Editing
//!
//! Implements operational transformation algorithms to ensure that concurrent
//! operations on the same graph converge to the same final state, regardless
//! of the order in which they are applied.

use crate::graph::{Graph, Node, Edge};
use crate::types::{NodeId, EdgeId, Position};
use std::collections::{HashMap, VecDeque};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Operational Transformation types for different graph operations
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GraphOperation {
    /// Add a node to the graph
    AddNode { node: Node<String>, position: Position },
    /// Remove a node from the graph
    RemoveNode { node_id: NodeId },
    /// Move a node to a new position
    MoveNode { node_id: NodeId, from: Position, to: Position },
    /// Update node data
    UpdateNode { node_id: NodeId, old_data: String, new_data: String },
    /// Add an edge between nodes
    AddEdge { edge: Edge<String>, source: NodeId, target: NodeId },
    /// Remove an edge
    RemoveEdge { edge_id: EdgeId },
    /// Update edge data
    UpdateEdge { edge_id: EdgeId, old_data: String, new_data: String },
}

/// Operation metadata for tracking and conflict resolution
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OperationMetadata {
    pub id: String,
    pub client_id: String,
    pub timestamp: u64,
    pub sequence_number: u64,
    pub parent_operations: Vec<String>,
}

/// Wrapped operation with metadata
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Operation {
    pub operation: GraphOperation,
    pub metadata: OperationMetadata,
}

/// Transformation function for operational transformation
pub type Transformation = Box<dyn Fn(&GraphOperation) -> TransformResult + Send + Sync>;

/// Result of a transformation operation
#[derive(Clone, Debug)]
pub enum TransformResult {
    /// Operation transformed successfully
    Transformed(GraphOperation),
    /// Operation should be discarded (no-op)
    NoOp,
    /// Transformation failed
    Conflict(String),
}

/// Operational Transformation manager
pub struct OperationalTransform {
    operations: VecDeque<Operation>,
    transformation_functions: HashMap<String, Transformation>,
    client_id: String,
    sequence_counter: u64,
}

impl std::fmt::Debug for OperationalTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperationalTransform")
            .field("operations", &self.operations)
            .field("client_id", &self.client_id)
            .field("sequence_counter", &self.sequence_counter)
            .field("transformation_functions_count", &self.transformation_functions.len())
            .finish()
    }
}

impl OperationalTransform {
    pub fn new(client_id: String) -> Self {
        let mut transform = Self {
            operations: VecDeque::new(),
            transformation_functions: HashMap::new(),
            client_id,
            sequence_counter: 0,
        };

        // Register built-in transformation functions
        transform.register_transformations();
        transform
    }

    /// Apply an operation with operational transformation
    pub fn apply_operation(&mut self, mut operation: Operation) -> Result<(), OTError> {
        // Set sequence number and client ID
        operation.metadata.sequence_number = self.sequence_counter;
        operation.metadata.client_id = self.client_id.clone();
        self.sequence_counter += 1;

        // Transform against concurrent operations
        for existing_op in &self.operations {
            if existing_op.metadata.client_id != operation.metadata.client_id {
                operation.operation = self.transform_operation(
                    &operation.operation,
                    &existing_op.operation,
                )?;
            }
        }

        // Add to operation history
        self.operations.push_back(operation);
        Ok(())
    }

    /// Transform one operation against another
    pub fn transform_operation(&self, op1: &GraphOperation, op2: &GraphOperation) -> Result<GraphOperation, OTError> {
        match (op1, op2) {
            // AddNode transformations
            (GraphOperation::AddNode { node: n1, position: p1 }, GraphOperation::AddNode { node: n2, position: p2 }) => {
                if n1.id == n2.id {
                    // Same node ID - resolve by offsetting position
                    Ok(GraphOperation::AddNode {
                        node: n1.clone(),
                        position: Position::new(p1.x + 10.0, p1.y + 10.0),
                    })
                } else {
                    Ok(op1.clone())
                }
            }
            (GraphOperation::AddNode { .. }, GraphOperation::RemoveNode { node_id }) => {
                // If we're adding a node that was removed, keep the add
                Ok(op1.clone())
            }

            // RemoveNode transformations
            (GraphOperation::RemoveNode { node_id: id1 }, GraphOperation::RemoveNode { node_id: id2 }) => {
                if id1 == id2 {
                    // Double remove - this is a no-op
                    return Err(OTError::NoOp);
                } else {
                    Ok(op1.clone())
                }
            }
            (GraphOperation::RemoveNode { .. }, GraphOperation::AddNode { .. }) => {
                // Add after remove - keep both
                Ok(op1.clone())
            }

            // MoveNode transformations
            (GraphOperation::MoveNode { node_id: id1, from: _, to: to1 }, GraphOperation::MoveNode { node_id: id2, from: _, to: to2 }) => {
                if id1 == id2 {
                    // Both moving the same node - use the later operation
                    Ok(GraphOperation::MoveNode {
                        node_id: id1.clone(),
                        from: to2.clone(),
                        to: to1.clone(),
                    })
                } else {
                    Ok(op1.clone())
                }
            }

            // Edge operations
            (GraphOperation::AddEdge { edge: e1, .. }, GraphOperation::AddEdge { edge: e2, .. }) => {
                if e1.id == e2.id {
                    // Same edge ID - this is a conflict
                    return Err(OTError::Conflict("Duplicate edge ID".to_string()));
                } else {
                    Ok(op1.clone())
                }
            }

            // Default: operations commute
            _ => Ok(op1.clone()),
        }
    }

    /// Get pending operations for synchronization
    pub fn get_pending_operations(&self, after_sequence: u64) -> Vec<Operation> {
        self.operations
            .iter()
            .filter(|op| op.metadata.sequence_number > after_sequence)
            .cloned()
            .collect()
    }

    /// Apply remote operations
    pub fn apply_remote_operations(&mut self, operations: Vec<Operation>) -> Result<(), OTError> {
        for operation in operations {
            self.apply_operation(operation)?;
        }
        Ok(())
    }

    /// Register transformation functions
    fn register_transformations(&mut self) {
        // Transformation functions would be registered here for custom operations
        // This is a simplified implementation
    }

    /// Get the client ID
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the current sequence number
    pub fn sequence_counter(&self) -> u64 {
        self.sequence_counter
    }

    /// Get the operation history
    pub fn operations(&self) -> &VecDeque<Operation> {
        &self.operations
    }
}

/// Operational Transformation errors
#[derive(Clone, Debug)]
pub enum OTError {
    Conflict(String),
    InvalidOperation(String),
    NoOp,
    SequenceError(String),
}

impl std::fmt::Display for OTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OTError::Conflict(msg) => write!(f, "Operation conflict: {}", msg),
            OTError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            OTError::NoOp => write!(f, "Operation is a no-op"),
            OTError::SequenceError(msg) => write!(f, "Sequence error: {}", msg),
        }
    }
}

impl std::error::Error for OTError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operational_transform_creation() {
        let ot = OperationalTransform::new("client-1".to_string());
        assert_eq!(ot.client_id(), "client-1");
        assert_eq!(ot.sequence_counter(), 0);
    }

    #[test]
    fn test_add_node_operation() {
        let mut ot = OperationalTransform::new("client-1".to_string());

        let operation = Operation {
            operation: GraphOperation::AddNode {
                node: Node::new("test-node".to_string(), Position::new(100.0, 100.0), "test".to_string()),
                position: Position::new(100.0, 100.0),
            },
            metadata: OperationMetadata {
                id: "op-1".to_string(),
                client_id: "client-1".to_string(),
                timestamp: 1234567890,
                sequence_number: 0,
                parent_operations: vec![],
            },
        };

        assert!(ot.apply_operation(operation).is_ok());
        assert_eq!(ot.operations().len(), 1);
        assert_eq!(ot.sequence_counter(), 1);
    }

    #[test]
    fn test_conflicting_add_node_operations() {
        let mut ot = OperationalTransform::new("client-1".to_string());

        let node = Node::new("test-node".to_string(), Position::new(100.0, 100.0), "test".to_string());

        let op1 = GraphOperation::AddNode {
            node: node.clone(),
            position: Position::new(100.0, 100.0),
        };

        let op2 = GraphOperation::AddNode {
            node: node.clone(),
            position: Position::new(100.0, 100.0),
        };

        let result = ot.transform_operation(&op1, &op2).unwrap();
        match result {
            GraphOperation::AddNode { position, .. } => {
                // Should be offset to resolve conflict
                assert_eq!(position.x, 110.0);
                assert_eq!(position.y, 110.0);
            }
            _ => panic!("Expected AddNode operation"),
        }
    }

    #[test]
    fn test_double_remove_noop() {
        let ot = OperationalTransform::new("client-1".to_string());

        let op1 = GraphOperation::RemoveNode {
            node_id: "node-1".into(),
        };

        let op2 = GraphOperation::RemoveNode {
            node_id: "node-1".into(),
        };

        let result = ot.transform_operation(&op1, &op2);
        assert!(matches!(result, Err(OTError::NoOp)));
    }
}
