//! Drag and drop interaction system for Leptos Flow
//!
//! Provides comprehensive drag and drop functionality for flow editors,
//! split into focused modules for maintainability.

pub mod handler;
pub mod events;
pub mod calculations;
pub mod state;
pub mod constraints;

pub use handler::DragHandler;
pub use events::{process_mouse_down, process_mouse_move, process_mouse_up, DragEvent};
pub use calculations::{calculate_drag_delta, apply_snap_to_grid, constrain_to_bounds};
pub use state::{DragState, DragHistory};
pub use constraints::{DragConstraints, DragConstraint};

// Re-export core types
pub use handler::{DragConfig, DragResult, DragHandle};
