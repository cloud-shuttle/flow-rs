//! Node Grouping System
//!
//! Manages hierarchical organization of nodes through grouping operations.

pub mod drag_state;
pub mod group;
pub mod manager;
pub mod tests;

// Re-export main types
pub use drag_state::GroupDragState;
pub use group::Group;
pub use manager::GroupManager;
