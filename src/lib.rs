//! # Leptos Flow
//!
//! High-performance reactive flow editor for Leptos applications.
//!
//! This is the main crate that re-exports functionality from the core
//! architecture components for convenient use.

// Re-export core functionality
pub use leptos_flow_core as core;
pub use leptos_flow_renderer as renderer;

#[cfg(feature = "leptos")]
pub use leptos_flow_leptos as leptos;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::prelude::*;
    pub use crate::renderer::{Renderer, RendererType, RendererCapabilities};

    #[cfg(feature = "canvas2d")]
    pub use crate::renderer::Canvas2DRenderer;

    #[cfg(feature = "leptos")]
    pub use crate::leptos::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_basic_graph_creation() {
        let mut graph = Graph::new();

        let node1 = Node::builder("node1")
            .position(Position::new(0.0, 0.0))
            .size(Size::new(100.0, 50.0))
            .build();

        let node2 = Node::builder("node2")
            .position(Position::new(200.0, 100.0))
            .size(Size::new(100.0, 50.0))
            .build();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }
}
