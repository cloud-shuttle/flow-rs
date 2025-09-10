//! # Leptos Flow Core
//!
//! Core data structures and algorithms for reactive flow-based node editing.
//! This crate provides framework-agnostic foundations for building flow editors.

pub mod error;
pub mod graph;
pub mod spatial;
pub mod layout;
pub mod types;

// Re-export commonly used types
pub use error::{FlowError, Result};
pub use graph::{Graph, Node, Edge};
pub use types::{Position, Size, Rect, Viewport};

/// Core prelude for convenient imports
pub mod prelude {
    pub use crate::error::{FlowError, Result};
    pub use crate::graph::{Graph, Node, Edge, NodeBuilder, EdgeBuilder};
    pub use crate::types::{Position, Size, Rect, Viewport};
    pub use crate::spatial::SpatialIndex;
}