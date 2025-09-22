//! Property-based testing for Leptos Flow Core
//!
//! This module contains comprehensive property-based tests using PropTest
//! to validate invariants and edge cases in the core data structures.

pub mod generators;
pub mod graph_tests;
pub mod group_tests;
pub mod layout_tests;
pub mod spatial_tests;

// Re-export generators for external use
// pub use generators::*; // Commented out to avoid unused import warning
