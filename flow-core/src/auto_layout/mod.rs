//! Auto Layout Manager - Intelligent layout algorithm selection and management
//!
//! This module provides automatic layout algorithm selection based on graph characteristics,
//! dynamic layout switching, and smooth transitions between different layout styles.

pub mod analysis;
pub mod config;
pub mod manager;
pub mod transitions;
pub mod tests;

// Re-export main types
pub use config::{AutoLayoutConfig, AutoLayoutConfigBuilder, AutoLayoutStrategy};
pub use manager::AutoLayoutManager;
