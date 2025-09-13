//! Renderer error types

use thiserror::Error;

/// Renderer-specific errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum RendererError {
    #[error("Initialization failed: {message}")]
    InitializationFailed { message: String },

    #[error("Context creation failed: {message}")]
    ContextCreationFailed { message: String },

    #[error("Rendering failed: {message}")]
    RenderingFailed { message: String },

    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature { feature: String },

    #[error("Resource creation failed: {resource}")]
    ResourceCreationFailed { resource: String },

    #[error("Invalid state: {message}")]
    InvalidState { message: String },

    #[error("Canvas not found or invalid")]
    InvalidCanvas,

    #[error("WebGL context lost")]
    ContextLost,

    #[error("Shader compilation failed: {message}")]
    ShaderCompilationFailed { message: String },

    #[error("Buffer creation failed: {message}")]
    BufferCreationFailed { message: String },

    #[error("Texture creation failed: {message}")]
    TextureCreationFailed { message: String },

    #[error("Program linking failed: {message}")]
    ProgramLinkingFailed { message: String },

    #[error("Out of memory")]
    OutOfMemory,

    #[error("Performance warning: {message}")]
    PerformanceWarning { message: String },
}

impl RendererError {
    /// Create an initialization error
    pub fn initialization_failed(message: impl Into<String>) -> Self {
        Self::InitializationFailed {
            message: message.into(),
        }
    }

    /// Create a context creation error
    pub fn context_creation_failed(message: impl Into<String>) -> Self {
        Self::ContextCreationFailed {
            message: message.into(),
        }
    }

    /// Create a rendering error
    pub fn rendering_failed(message: impl Into<String>) -> Self {
        Self::RenderingFailed {
            message: message.into(),
        }
    }

    /// Create an unsupported feature error
    pub fn unsupported_feature(feature: impl Into<String>) -> Self {
        Self::UnsupportedFeature {
            feature: feature.into(),
        }
    }

    /// Create a resource creation error
    pub fn resource_creation_failed(resource: impl Into<String>) -> Self {
        Self::ResourceCreationFailed {
            resource: resource.into(),
        }
    }

    /// Create an invalid state error
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState {
            message: message.into(),
        }
    }

    /// Create a shader compilation error
    pub fn shader_compilation_failed(message: impl Into<String>) -> Self {
        Self::ShaderCompilationFailed {
            message: message.into(),
        }
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ContextLost => true,
            Self::OutOfMemory => false,
            Self::InvalidCanvas => false,
            Self::InitializationFailed { .. } => false,
            Self::ContextCreationFailed { .. } => false,
            _ => true,
        }
    }

    /// Check if this is a performance-related warning
    pub fn is_performance_warning(&self) -> bool {
        matches!(self, Self::PerformanceWarning { .. })
    }
}

/// Result type for renderer operations
pub type Result<T> = std::result::Result<T, RendererError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = RendererError::initialization_failed("Test message");
        assert_eq!(error.to_string(), "Initialization failed: Test message");

        let error = RendererError::unsupported_feature("WebGPU");
        assert_eq!(error.to_string(), "Unsupported feature: WebGPU");
    }

    #[test]
    fn test_error_properties() {
        let recoverable = RendererError::ContextLost;
        assert!(recoverable.is_recoverable());

        let non_recoverable = RendererError::OutOfMemory;
        assert!(!non_recoverable.is_recoverable());

        let warning = RendererError::PerformanceWarning {
            message: "Low FPS".to_string(),
        };
        assert!(warning.is_performance_warning());
    }
}
