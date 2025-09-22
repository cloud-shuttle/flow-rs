//! Node Selection System
//!
//! Manages multi-node selection, keyboard navigation, and selection state.

pub mod manager;
pub mod modes;
pub mod tests;
pub mod visual_feedback;

// Re-export main types
pub use manager::SelectionManager;
pub use modes::{KeyboardShortcut, NavigationDirection, SelectionMode};
pub use visual_feedback::VisualFeedback;
