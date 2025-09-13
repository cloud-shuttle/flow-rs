//! # Leptos Flow Leptos Integration
//!
//! Leptos framework integration for Leptos Flow providing reactive components,
//! state management, and event handling for flow-based node editors.

pub mod components;
pub mod hooks;
pub mod signals;
pub mod interactions;
pub mod events;
pub mod keyboard;
pub mod drag;
pub mod edge_connection;
pub mod minimap;
pub mod dom_rect;
pub mod mouse_integration;

// Re-export core types for convenience
pub use flow_core as core;
pub use flow_renderer as renderer;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::components::*;
    pub use crate::components::minimap::*;
    pub use crate::components::controls::*;
    pub use crate::hooks::*;
    pub use crate::signals::*;
    pub use crate::events::*;
    pub use crate::interactions::*;
    pub use crate::keyboard::*;
    pub use crate::drag::*;
    pub use crate::edge_connection::*;
    pub use crate::minimap::*;
    pub use crate::dom_rect::*;
    pub use crate::mouse_integration::*;

    // Re-export commonly used Leptos types
    pub use leptos::{
        component, create_signal, create_memo, create_effect, create_resource,
        Signal, ReadSignal, WriteSignal, RwSignal, Memo,
        IntoView, Children, ChildrenFn,
    };

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
