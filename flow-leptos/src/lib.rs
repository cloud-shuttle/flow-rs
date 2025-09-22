//! # Leptos Flow Leptos Integration
//!
//! Leptos framework integration for Leptos Flow providing reactive components,
//! state management, and event handling for flow-based node editors.

#![allow(clippy::all)]

pub mod components;
pub mod dom_rect;
pub mod drag;
pub mod edge_connection;
pub mod events;
pub mod hooks;
pub mod interactions;
pub mod keyboard;
pub mod minimap;
pub mod mouse_integration;
pub mod signals;

// Re-export core types for convenience
pub use flow_rs_core as core;
pub use flow_rs_renderer as renderer;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::components::controls::*;
    pub use crate::components::minimap::*;
    pub use crate::components::*;
    pub use crate::dom_rect::*;
    pub use crate::drag::*;
    pub use crate::edge_connection::*;
    pub use crate::events::*;
    pub use crate::hooks::*;
    pub use crate::interactions::*;
    pub use crate::keyboard::*;
    pub use crate::minimap::*;
    pub use crate::mouse_integration::*;
    pub use crate::signals::*;

    // Re-export commonly used Leptos types
    pub use leptos::prelude::*;

    // Re-export core types
    pub use crate::core::prelude::*;
    pub use crate::renderer::{Renderer, RendererType};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_module_exists() {
        // Basic smoke test to ensure module compiles
        assert!(true);
    }
}

#[cfg(test)]
mod mouse_interactions_tests {
    include!("tests/mouse_interactions.rs");
}

#[cfg(test)]
mod edge_connection_integration_tests {
    include!("tests/edge_connection_integration.rs");
}
