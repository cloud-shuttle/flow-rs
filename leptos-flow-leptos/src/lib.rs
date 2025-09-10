//! # Leptos Flow Leptos Integration
//!
//! Leptos framework integration for Leptos Flow providing reactive components,
//! state management, and event handling for flow-based node editors.

pub mod components;
pub mod hooks;
pub mod signals;
pub mod events;

// Re-export core types for convenience
pub use leptos_flow_core as core;
pub use leptos_flow_renderer as renderer;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::components::*;
    pub use crate::hooks::*;
    pub use crate::signals::*;
    pub use crate::events::*;

    // Re-export commonly used Leptos types
    pub use leptos::{
        component, create_signal, create_memo, create_effect, create_resource,
        Signal, ReadSignal, WriteSignal, RwSignal, Memo,
        IntoView, ComponentProps, Children, ChildrenFn,
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
