//! # Leptos Flow Core
//!
//! Core data structures and algorithms for reactive flow-based node editing.
//! This crate provides framework-agnostic foundations for building flow editors.

pub mod error;
pub mod graph;
pub mod spatial;
pub mod layout;
pub mod types;
pub mod selection;
pub mod groups;

#[cfg(test)]
mod proptest;

// Re-export commonly used types
pub use error::{FlowError, Result};
pub use graph::{Graph, Node, Edge};
pub use types::{Position, Size, Rect, Viewport, NodeId, EdgeId, GroupId};
pub use selection::{SelectionManager, SelectionMode, NavigationDirection};
pub use groups::{Group, GroupManager};

/// Core prelude for convenient imports
pub mod prelude {
    pub use crate::error::{FlowError, Result};
    pub use crate::graph::{Graph, Node, Edge, NodeBuilder, EdgeBuilder};
    pub use crate::types::{Position, Size, Rect, Viewport, NodeId, EdgeId, GroupId};
    pub use crate::spatial::SpatialIndex;
}
