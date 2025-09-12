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

    #[error("Invalid connection: {message}")]
    InvalidConnection { message: String },

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

    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    #[error("Handle not found: {handle_id}")]
    HandleNotFound { handle_id: String },

    #[error("Connection limit exceeded for handle '{handle_id}': {current}/{limit}")]
    ConnectionLimitExceeded {
        handle_id: String,
        current: usize,
        limit: usize,
    },
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
    pub fn invalid_connection(message: impl Into<String>) -> Self {
        Self::InvalidConnection {
            message: message.into(),
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

    /// Create an invalid operation error
    pub fn invalid_operation(message: impl Into<String>) -> Self {
        Self::InvalidOperation {
            message: message.into(),
        }
    }

    /// Create a handle not found error
    pub fn handle_not_found(handle_id: impl Into<String>) -> Self {
        Self::HandleNotFound {
            handle_id: handle_id.into(),
        }
    }

    /// Create a connection limit exceeded error
    pub fn connection_limit_exceeded(handle_id: impl Into<String>, current: usize, limit: usize) -> Self {
        Self::ConnectionLimitExceeded {
            handle_id: handle_id.into(),
            current,
            limit,
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
    use std::error::Error;

    // === Constructor Method Tests ===

    #[test]
    fn test_node_not_found_constructor() {
        let error = FlowError::node_not_found("node-123");
        match &error {
            FlowError::NodeNotFound { id } => {
                assert_eq!(id, "node-123");
            }
            _ => panic!("Expected NodeNotFound variant"),
        }
        assert_eq!(error.to_string(), "Node with ID 'node-123' not found");
    }

    #[test]
    fn test_edge_not_found_constructor() {
        let error = FlowError::edge_not_found("edge-456");
        match &error {
            FlowError::EdgeNotFound { id } => {
                assert_eq!(id, "edge-456");
            }
            _ => panic!("Expected EdgeNotFound variant"),
        }
        assert_eq!(error.to_string(), "Edge with ID 'edge-456' not found");
    }

    #[test]
    fn test_duplicate_node_id_constructor() {
        let error = FlowError::duplicate_node_id("dup-node");
        match &error {
            FlowError::DuplicateNodeId { id } => {
                assert_eq!(id, "dup-node");
            }
            _ => panic!("Expected DuplicateNodeId variant"),
        }
        assert_eq!(error.to_string(), "Duplicate node ID: 'dup-node'");
    }

    #[test]
    fn test_duplicate_edge_id_constructor() {
        let error = FlowError::duplicate_edge_id("dup-edge");
        match &error {
            FlowError::DuplicateEdgeId { id } => {
                assert_eq!(id, "dup-edge");
            }
            _ => panic!("Expected DuplicateEdgeId variant"),
        }
        assert_eq!(error.to_string(), "Duplicate edge ID: 'dup-edge'");
    }

    #[test]
    fn test_invalid_connection_constructor() {
        let error = FlowError::invalid_connection("Type mismatch");
        match &error {
            FlowError::InvalidConnection { message } => {
                assert_eq!(message, "Type mismatch");
            }
            _ => panic!("Expected InvalidConnection variant"),
        }
        assert_eq!(error.to_string(), "Invalid connection: Type mismatch");
    }

    #[test]
    fn test_spatial_index_constructor() {
        let error = FlowError::spatial_index("Out of bounds");
        match &error {
            FlowError::SpatialIndex { message } => {
                assert_eq!(message, "Out of bounds");
            }
            _ => panic!("Expected SpatialIndex variant"),
        }
        assert_eq!(error.to_string(), "Spatial index error: Out of bounds");
    }

    #[test]
    fn test_layout_constructor() {
        let error = FlowError::layout("Circular dependency");
        match &error {
            FlowError::Layout { message } => {
                assert_eq!(message, "Circular dependency");
            }
            _ => panic!("Expected Layout variant"),
        }
        assert_eq!(error.to_string(), "Layout error: Circular dependency");
    }

    #[test]
    fn test_invalid_operation_constructor() {
        let error = FlowError::invalid_operation("Operation not supported");
        match &error {
            FlowError::InvalidOperation { message } => {
                assert_eq!(message, "Operation not supported");
            }
            _ => panic!("Expected InvalidOperation variant"),
        }
        assert_eq!(error.to_string(), "Invalid operation: Operation not supported");
    }

    #[test]
    fn test_handle_not_found_constructor() {
        let error = FlowError::handle_not_found("handle-789");
        match &error {
            FlowError::HandleNotFound { handle_id } => {
                assert_eq!(handle_id, "handle-789");
            }
            _ => panic!("Expected HandleNotFound variant"),
        }
        assert_eq!(error.to_string(), "Handle not found: handle-789");
    }

    #[test]
    fn test_connection_limit_exceeded_constructor() {
        let error = FlowError::connection_limit_exceeded("output-handle", 3, 2);
        match &error {
            FlowError::ConnectionLimitExceeded { handle_id, current, limit } => {
                assert_eq!(handle_id, "output-handle");
                assert_eq!(*current, 3);
                assert_eq!(*limit, 2);
            }
            _ => panic!("Expected ConnectionLimitExceeded variant"),
        }
        assert_eq!(error.to_string(), "Connection limit exceeded for handle 'output-handle': 3/2");
    }

    // === Direct Variant Construction Tests ===

    #[test]
    fn test_self_connection_variant() {
        let error = FlowError::SelfConnection;
        assert_eq!(error.to_string(), "Self connection not allowed");
    }

    #[test]
    fn test_invalid_position_variant() {
        let error = FlowError::InvalidPosition { x: f64::NAN, y: f64::INFINITY };
        assert_eq!(error.to_string(), "Invalid position: x=NaN, y=inf");
    }

    #[test]
    fn test_invalid_size_variant() {
        let error = FlowError::InvalidSize { width: -10.0, height: 0.0 };
        assert_eq!(error.to_string(), "Invalid size: width=-10, height=0");
    }

    #[test]
    fn test_serialization_variant() {
        let error = FlowError::Serialization { message: "JSON parse error".to_string() };
        assert_eq!(error.to_string(), "Serialization error: JSON parse error");
    }

    // === Error Equality and Comparison Tests ===

    #[test]
    fn test_error_equality_comprehensive() {
        // Same error types with same data should be equal
        let error1 = FlowError::node_not_found("test");
        let error2 = FlowError::node_not_found("test");
        assert_eq!(error1, error2);

        // Same error types with different data should not be equal
        let error3 = FlowError::node_not_found("other");
        assert_ne!(error1, error3);

        // Different error types should not be equal
        let error4 = FlowError::edge_not_found("test");
        assert_ne!(error1, error4);

        // Complex error types equality
        let error5 = FlowError::connection_limit_exceeded("handle", 5, 3);
        let error6 = FlowError::connection_limit_exceeded("handle", 5, 3);
        let error7 = FlowError::connection_limit_exceeded("handle", 4, 3);

        assert_eq!(error5, error6);
        assert_ne!(error5, error7);
    }

    // === Error Cloning Tests ===

    #[test]
    fn test_error_cloning() {
        let original = FlowError::invalid_connection("Test message");
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Ensure they're independent objects
        match (&original, &cloned) {
            (FlowError::InvalidConnection { message: msg1 },
             FlowError::InvalidConnection { message: msg2 }) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("Cloning changed error variant"),
        }
    }

    // === Edge Cases and Boundary Conditions ===

    #[test]
    fn test_empty_string_parameters() {
        let error1 = FlowError::node_not_found("");
        assert_eq!(error1.to_string(), "Node with ID '' not found");

        let error2 = FlowError::invalid_connection("");
        assert_eq!(error2.to_string(), "Invalid connection: ");
    }

    #[test]
    fn test_special_characters_in_ids() {
        let special_id = "node-123_with.special@chars#$%";
        let error = FlowError::duplicate_node_id(special_id);
        assert!(error.to_string().contains(special_id));
    }

    #[test]
    fn test_unicode_characters() {
        let unicode_id = "节点-123-ñoño";
        let error = FlowError::edge_not_found(unicode_id);
        assert!(error.to_string().contains(unicode_id));
    }

    #[test]
    fn test_very_long_strings() {
        let long_id = "a".repeat(1000);
        let error = FlowError::handle_not_found(&long_id);
        assert!(error.to_string().contains(&long_id));
    }

    #[test]
    fn test_connection_limit_boundary_values() {
        // Zero limit
        let error1 = FlowError::connection_limit_exceeded("handle", 1, 0);
        assert_eq!(error1.to_string(), "Connection limit exceeded for handle 'handle': 1/0");

        // Maximum values
        let error2 = FlowError::connection_limit_exceeded("handle", usize::MAX, usize::MAX - 1);
        assert!(error2.to_string().contains(&format!("{}/{}", usize::MAX, usize::MAX - 1)));
    }

    // === Result Type Integration Tests ===

    #[test]
    fn test_result_type_usage() {
        fn returns_error() -> Result<String> {
            Err(FlowError::node_not_found("missing"))
        }

        fn returns_success() -> Result<String> {
            Ok("success".to_string())
        }

        // Test error case
        match returns_error() {
            Ok(_) => panic!("Expected error"),
            Err(error) => {
                assert_eq!(error, FlowError::node_not_found("missing"));
            }
        }

        // Test success case
        match returns_success() {
            Ok(value) => assert_eq!(value, "success"),
            Err(_) => panic!("Expected success"),
        }
    }

    // === Debug Formatting Tests ===

    #[test]
    fn test_debug_formatting() {
        let error = FlowError::node_not_found("debug-test");
        let debug_str = format!("{:?}", error);

        // Debug format should contain variant name and data
        assert!(debug_str.contains("NodeNotFound"));
        assert!(debug_str.contains("debug-test"));
    }

    // === Error Categorization Tests ===

    #[test]
    fn test_error_categorization_by_domain() {
        // Graph structure errors
        let graph_errors = vec![
            FlowError::node_not_found("test"),
            FlowError::edge_not_found("test"),
            FlowError::duplicate_node_id("test"),
            FlowError::duplicate_edge_id("test"),
        ];

        for error in graph_errors {
            assert!(is_graph_structure_error(&error));
        }

        // Connection errors
        let connection_errors = vec![
            FlowError::invalid_connection("test"),
            FlowError::SelfConnection,
            FlowError::connection_limit_exceeded("handle", 1, 0),
        ];

        for error in connection_errors {
            assert!(is_connection_error(&error));
        }

        // Validation errors
        let validation_errors = vec![
            FlowError::InvalidPosition { x: f64::NAN, y: 0.0 },
            FlowError::InvalidSize { width: -1.0, height: 0.0 },
        ];

        for error in validation_errors {
            assert!(is_validation_error(&error));
        }
    }

    // Helper functions for error categorization
    fn is_graph_structure_error(error: &FlowError) -> bool {
        matches!(error,
            FlowError::NodeNotFound { .. } |
            FlowError::EdgeNotFound { .. } |
            FlowError::DuplicateNodeId { .. } |
            FlowError::DuplicateEdgeId { .. }
        )
    }

    fn is_connection_error(error: &FlowError) -> bool {
        matches!(error,
            FlowError::InvalidConnection { .. } |
            FlowError::SelfConnection |
            FlowError::ConnectionLimitExceeded { .. }
        )
    }

    fn is_validation_error(error: &FlowError) -> bool {
        matches!(error,
            FlowError::InvalidPosition { .. } |
            FlowError::InvalidSize { .. }
        )
    }

    // === Serde Integration Tests ===

    #[cfg(feature = "serde")]
    #[test]
    fn test_error_serialization() {
        let error = FlowError::node_not_found("serialize-test");
        let serialized = serde_json::to_string(&error).expect("Serialization should work");

        // Should serialize as the error message string
        assert_eq!(serialized, "\"Node with ID 'serialize-test' not found\"");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_complex_error_serialization() {
        let error = FlowError::connection_limit_exceeded("output", 5, 3);
        let serialized = serde_json::to_string(&error).expect("Serialization should work");

        assert_eq!(serialized, "\"Connection limit exceeded for handle 'output': 5/3\"");
    }

    // === From Trait Integration Tests ===

    #[test]
    fn test_from_string_conversion() {
        let error1 = FlowError::node_not_found(String::from("owned-string"));
        assert_eq!(error1.to_string(), "Node with ID 'owned-string' not found");

        let error2 = FlowError::invalid_connection("string-slice");
        assert_eq!(error2.to_string(), "Invalid connection: string-slice");
    }

    // === Integration with std::error::Error Tests ===

    #[test]
    fn test_std_error_trait_integration() {
        let error = FlowError::layout("test error");

        // Should implement std::error::Error
        let _: &dyn std::error::Error = &error;

        // Test error source (should be None for our simple errors)
        assert!(error.source().is_none());
    }
}
