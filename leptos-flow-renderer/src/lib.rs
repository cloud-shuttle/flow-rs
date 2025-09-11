//! # Leptos Flow Renderer
//!
//! Rendering abstractions and implementations for Leptos Flow.
//! Supports multiple rendering backends with automatic fallback.

pub mod error;
pub mod traits;

#[cfg(feature = "canvas2d")]
pub mod canvas2d;

pub mod performance;

#[cfg(all(test, feature = "canvas2d"))]
mod canvas2d_tests;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "webgl2")]
pub mod webgl2;

#[cfg(feature = "webgpu")]
pub mod webgpu;

// Re-export commonly used types
pub use error::{RendererError, Result};
pub use traits::{Renderer, RendererCapabilities, RendererType};

#[cfg(feature = "canvas2d")]
pub use canvas2d::Canvas2DRenderer;

/// Renderer selection and detection utilities
pub mod detection {
    use super::traits::RendererType;

    /// Detect available rendering backends
    pub fn detect_available_renderers() -> Vec<RendererType> {
        let mut available = Vec::new();

        #[cfg(feature = "canvas2d")]
        if is_canvas2d_available() {
            available.push(RendererType::Canvas2D);
        }

        #[cfg(feature = "webgl2")]
        if is_webgl2_available() {
            available.push(RendererType::WebGL2);
        }

        #[cfg(feature = "webgpu")]
        if is_webgpu_available() {
            available.push(RendererType::WebGPU);
        }

        available
    }

    /// Select the best available renderer
    pub fn select_best_renderer() -> Option<RendererType> {
        let available = detect_available_renderers();

        // Priority order: WebGPU > WebGL2 > Canvas2D
        #[cfg(feature = "webgpu")]
        if available.contains(&RendererType::WebGPU) {
            return Some(RendererType::WebGPU);
        }

        #[cfg(feature = "webgl2")]
        if available.contains(&RendererType::WebGL2) {
            return Some(RendererType::WebGL2);
        }

        #[cfg(feature = "canvas2d")]
        if available.contains(&RendererType::Canvas2D) {
            return Some(RendererType::Canvas2D);
        }

        None
    }

    #[cfg(feature = "canvas2d")]
    fn is_canvas2d_available() -> bool {
        // Canvas2D is available in all modern browsers
        true
    }

    #[cfg(feature = "webgl2")]
    fn is_webgl2_available() -> bool {
        use wasm_bindgen::JsCast;
        use web_sys::{window, HtmlCanvasElement};

        if let Some(window) = window() {
            if let Ok(document) = window.document().ok_or("No document") {
                if let Ok(canvas) = document
                    .create_element("canvas")
                    .map_err(|_| "Cannot create canvas")
                    .and_then(|canvas| {
                        canvas
                            .dyn_into::<HtmlCanvasElement>()
                            .map_err(|_| "Cannot cast to canvas")
                    })
                {
                    return canvas.get_context("webgl2").is_ok();
                }
            }
        }
        false
    }

    #[cfg(feature = "webgpu")]
    fn is_webgpu_available() -> bool {
        // Check if WebGPU is available
        use wasm_bindgen::JsCast;
        use web_sys::window;

        if let Some(window) = window() {
            if let Ok(navigator) = js_sys::Reflect::get(&window, &"navigator".into()) {
                return js_sys::Reflect::has(&navigator, &"gpu".into()).unwrap_or(false);
            }
        }
        false
    }
}

/// Prelude for convenient imports
pub mod prelude {
    pub use super::error::{RendererError, Result};
    pub use super::traits::{Renderer, RendererCapabilities, RendererType};
    pub use super::detection::{detect_available_renderers, select_best_renderer};

    #[cfg(feature = "canvas2d")]
    pub use super::canvas2d::Canvas2DRenderer;
}
