//! Simple spatial indexing for efficient queries
//!
//! This implementation uses a simple grid-based spatial partitioning scheme
//! for efficient viewport and proximity queries.

pub mod grid;
pub mod index;
pub mod query;
pub mod tests;

// Re-export main types
pub use index::SpatialIndex;
pub use query::SpatialQuery;
