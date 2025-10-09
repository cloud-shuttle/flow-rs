//! # Flow-RS Core
//!
//! Core data structures and algorithms for reactive flow-based node editing.
//! This crate provides framework-agnostic foundations for building flow editors.

pub mod auto_layout;
pub mod drag_operations;
pub mod edge_creator;
pub mod error;
pub mod graph;
pub mod groups;
pub mod handle;
pub mod layout;
pub mod selection;
pub mod collaboration;
pub mod documentation;
pub mod framework_abstractions;
pub mod plugins;
pub mod spatial;
pub mod subflows;
pub mod types;

#[cfg(test)]
mod proptest;

#[cfg(test)]
mod performance_tests;

#[cfg(test)]
mod api_contracts;

#[cfg(test)]
mod documentation_tests;

#[cfg(test)]
mod api_reference_tests;

#[cfg(test)]
mod renaming_validation_tests;

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
pub use auto_layout::{
    AutoLayoutConfig, AutoLayoutConfigBuilder, AutoLayoutManager, AutoLayoutStrategy,
};
pub use drag_operations::DragOperation;
pub use edge_creator::{ConnectionFeedback, EdgeCreator, PreviewEdge};
pub use error::{FlowError, Result};
pub use graph::{Edge, Graph, Node};
pub use groups::{Group, GroupManager};
pub use handle::{Handle, HandleId, HandleManager, HandlePosition, HandleType};
pub use selection::{
    KeyboardShortcut, NavigationDirection, SelectionManager, SelectionMode, VisualFeedback,
};
pub use types::{EdgeId, GroupId, NodeId, Position, Rect, Size, Viewport};

/// Core prelude for convenient imports
pub mod prelude {
    pub use crate::error::{FlowError, Result};
    pub use crate::graph::{Edge, EdgeBuilder, Graph, Node, NodeBuilder};
    pub use crate::handle::{Handle, HandleId, HandleManager, HandlePosition, HandleType};
    pub use crate::selection::{
        KeyboardShortcut, NavigationDirection, SelectionManager, SelectionMode, VisualFeedback,
    };
    pub use crate::collaboration::{CollaborativeSession, OperationalTransform, Participant, Cursor};
    pub use crate::documentation::{DocumentationSystem, DocumentationGenerator};
    pub use crate::framework_abstractions::{FrameworkAdapter, FrameworkRegistry, FrameworkAgnosticFlow};
    pub use crate::plugins::{PluginManager, PluginRegistry, Plugin, PluginMetadata, PluginCapabilities};
    pub use crate::spatial::SpatialIndex;
    pub use crate::subflows::{HierarchicalGraph, NavigationState};
    pub use crate::types::{EdgeId, GroupId, NodeId, Position, Rect, Size, Viewport};
}
