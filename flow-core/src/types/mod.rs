//! Core geometric and utility types

pub mod geometry;
pub mod identifiers;
pub mod viewport;
pub mod tests;

// Re-export main types
pub use geometry::{Position, Rect, Size};
pub use identifiers::{EdgeId, GroupId, NodeId};
pub use viewport::Viewport;
