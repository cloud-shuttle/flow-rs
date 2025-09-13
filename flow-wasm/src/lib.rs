//! # Leptos Flow WASM
//!
//! WASM bindings and utilities for Leptos Flow providing direct JavaScript
//! interoperability for non-Leptos applications.

use wasm_bindgen::prelude::*;

// Re-export core functionality
pub use flow_core as core;
pub use flow_renderer as renderer;

pub mod bindings;
pub mod utils;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    #[cfg(feature = "debug")]
    {
        use tracing_wasm::{WASMLayerConfigBuilder};
        use tracing::Level;

        let config = WASMLayerConfigBuilder::default()
            .set_max_level(Level::INFO)
            .build();

        tracing_wasm::set_as_global_default_with_config(config);

        tracing::info!("Leptos Flow WASM initialized");
    }
}

/// Export WASM-compatible types and functions
pub mod wasm_exports {
    use super::*;
    pub use crate::bindings::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_init() {
        init();
        // If we reach here, initialization was successful
    }
}
