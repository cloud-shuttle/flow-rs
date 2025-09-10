//! Error types for Leptos Flow operations

use thiserror::Error;

/// Main error type for flow operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum FlowError {
    #[error("Node with ID '{id}' not found")]
    NodeNotFound { id: String },

    #[error("Edge with ID '{id}' not found")]
    EdgeNotFound { id: String },

    #[error("Duplicate node ID: '{id}'")]
    DuplicateNodeId { id: String },

    #[error("Duplicate edge ID: '{id}'")]
    DuplicateEdgeId { id: String },

    #[error("Invalid connection: {reason}")]
    InvalidConnection { reason: String },

    #[error("Self connection not allowed")]
    SelfConnection,

    #[error("Spatial index error: {message}")]
    SpatialIndex { message: String },

    #[error("Layout error: {message}")]
    Layout { message: String },

    #[error("Invalid position: x={x}, y={y}")]
    InvalidPosition { x: f64, y: f64 },

    #[error("Invalid size: width={width}, height={height}")]
    InvalidSize { width: f64, height: f64 },

    #[error("Serialization error: {message}")]
    Serialization { message: String },
}

impl FlowError {
    /// Create a node not found error
    pub fn node_not_found(id: impl Into<String>) -> Self {
        Self::NodeNotFound { id: id.into() }
    }

    /// Create an edge not found error
    pub fn edge_not_found(id: impl Into<String>) -> Self {
        Self::EdgeNotFound { id: id.into() }
    }

    /// Create a duplicate node ID error
    pub fn duplicate_node_id(id: impl Into<String>) -> Self {
        Self::DuplicateNodeId { id: id.into() }
    }

    /// Create a duplicate edge ID error
    pub fn duplicate_edge_id(id: impl Into<String>) -> Self {
        Self::DuplicateEdgeId { id: id.into() }
    }

    /// Create an invalid connection error
    pub fn invalid_connection(reason: impl Into<String>) -> Self {
        Self::InvalidConnection {
            reason: reason.into(),
        }
    }

    /// Create a spatial index error
    pub fn spatial_index(message: impl Into<String>) -> Self {
        Self::SpatialIndex {
            message: message.into(),
        }
    }

    /// Create a layout error
    pub fn layout(message: impl Into<String>) -> Self {
        Self::Layout {
            message: message.into(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for FlowError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Result type alias for flow operations
pub type Result<T> = std::result::Result<T, FlowError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = FlowError::node_not_found("test-id");
        assert_eq!(error.to_string(), "Node with ID 'test-id' not found");

        let error = FlowError::invalid_connection("Invalid handle");
        assert_eq!(error.to_string(), "Invalid connection: Invalid handle");
    }

    #[test]
    fn test_error_equality() {
        let error1 = FlowError::node_not_found("test");
        let error2 = FlowError::node_not_found("test");
        let error3 = FlowError::node_not_found("other");

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }
}
