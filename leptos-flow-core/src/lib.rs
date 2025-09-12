//! # Leptos Flow Core
//!
//! Core data structures and algorithms for reactive flow-based node editing.
//! This crate provides framework-agnostic foundations for building flow editors.

pub mod error;
pub mod graph;
pub mod spatial;
pub mod layout;
pub mod auto_layout;
pub mod types;
pub mod selection;
pub mod groups;
pub mod handle;
pub mod drag_operations;
pub mod edge_creator;

#[cfg(test)]
mod proptest;

#[cfg(test)]
mod performance_tests;

#[cfg(test)]
mod api_contracts;

#[cfg(test)]
mod graph_handle_integration;

#[cfg(test)]
mod drag_node_integration;

#[cfg(test)]
mod interactive_edge_creation;

#[cfg(test)]
mod handle_connection_counting;

#[cfg(test)]
mod graph_cycle_detection;

// Re-export commonly used types
pub use error::{FlowError, Result};
pub use graph::{Graph, Node, Edge};
pub use types::{Position, Size, Rect, Viewport, NodeId, EdgeId, GroupId};
pub use selection::{SelectionManager, SelectionMode, NavigationDirection, KeyboardShortcut, VisualFeedback};
pub use groups::{Group, GroupManager};
pub use handle::{Handle, HandleId, HandleType, HandlePosition, HandleManager};
pub use drag_operations::DragOperation;
pub use edge_creator::{EdgeCreator, PreviewEdge, ConnectionFeedback};
pub use auto_layout::{AutoLayoutManager, AutoLayoutStrategy, AutoLayoutConfig, AutoLayoutConfigBuilder};

/// Core prelude for convenient imports
pub mod prelude {
    pub use crate::error::{FlowError, Result};
    pub use crate::graph::{Graph, Node, Edge, NodeBuilder, EdgeBuilder};
    pub use crate::types::{Position, Size, Rect, Viewport, NodeId, EdgeId, GroupId};
    pub use crate::spatial::SpatialIndex;
    pub use crate::handle::{Handle, HandleId, HandleType, HandlePosition, HandleManager};
    pub use crate::selection::{SelectionManager, SelectionMode, NavigationDirection, KeyboardShortcut, VisualFeedback};
}
