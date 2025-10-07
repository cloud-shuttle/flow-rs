//! # Leptos Flow Leptos Integration
//!
//! Leptos framework integration for Leptos Flow providing reactive components,
//! state management, and event handling for flow-based node editors.

#![allow(clippy::all)]

pub mod accessibility;
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
pub mod node_resizing;
pub mod selection;
pub mod signals;
pub mod touch;
pub mod context_menu;
pub mod history;
pub mod keyboard_shortcuts;
pub mod subflow_integration;

// Re-export core types for convenience
pub use flow_rs_core as core;
pub use flow_rs_renderer as renderer;

/// Prelude module for convenient imports
pub mod prelude {
    // Accessibility exports
    pub use crate::accessibility::*;

    // Touch and mobile support
    pub use crate::touch::*;

    // Node resizing and advanced interactions
    pub use crate::node_resizing::*;
    pub use crate::selection::*;
    pub use crate::context_menu::*;
    pub use crate::history::*;
    pub use crate::keyboard_shortcuts::*;
    pub use crate::subflow_integration::*;

    // Component exports (avoiding conflicts)
    pub use crate::components::controls::*;
    pub use crate::components::minimap::*;

    // Specific exports to avoid conflicts
    pub use crate::dom_rect::{ElementRect, DomRectError};
    pub use crate::drag::{DragHandler, DragConfig, DragResult, DragHandle};
    pub use crate::edge_connection::*;
    pub use crate::events::{FlowEvent, MouseButton};
    pub use crate::hooks::*;
    pub use crate::interactions::InteractionManager;
    pub use crate::keyboard::*;
    pub use crate::minimap::*;
    pub use crate::mouse_integration::MouseEventConverter;
    pub use crate::signals::*;
    pub use crate::components::{FlowEditor, FlowCanvas, FlowStats};

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
