//! Canvas2D renderer implementation

use crate::error::{RendererError, Result};
use crate::performance::{PerformanceManager, PerformanceSettings};
use crate::traits::{
    AnimatedSelectionStyle, BackgroundConfig, BackgroundVariant, MultiSelectionStyle, RenderStats,
    Renderer, RendererCapabilities, SelectionHoverStyle, SelectionStyle,
};
use flow_core::{NodeId, Position, Rect, Viewport};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Canvas2D renderer implementation
pub struct Canvas2DRenderer {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    capabilities: RendererCapabilities,
    stats: RenderStats,
    width: u32,
    height: u32,
    #[allow(dead_code)]
    performance_manager: PerformanceManager,
    // Animation state
    selection_count: usize,
    multi_selection_active: bool,
    hover_active_bounds: Option<Rect>,
}

impl Canvas2DRenderer {
    fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
        canvas
            .get_context("2d")
            .map_err(|_| RendererError::context_creation_failed("Failed to get 2D context"))?
            .ok_or_else(|| RendererError::context_creation_failed("No 2D context available"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| {
                RendererError::context_creation_failed("Failed to cast to CanvasRenderingContext2d")
            })
    }

    fn create_capabilities(canvas: &HtmlCanvasElement) -> RendererCapabilities {
        RendererCapabilities {
            name: "Canvas2D".to_string(),
            max_texture_size: 4096,
            max_textures: 16,
            supports_instancing: false,
            supports_compute_shaders: false,
            supports_msaa: false,
            max_msaa_samples: 0,
            max_viewport_size: (canvas.width(), canvas.height()),
            memory_budget: None,
        }
    }
}

impl Renderer for Canvas2DRenderer {
    fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
        let context = Self::get_context(canvas)?;
        let capabilities = Self::create_capabilities(canvas);

        let width = canvas.width();
        let height = canvas.height();

        Ok(Self {
            context,
            canvas: canvas.clone(),
            capabilities,
            stats: RenderStats::default(),
            width,
            height,
            performance_manager: PerformanceManager::new(PerformanceSettings::balanced()),
            // Animation state
            selection_count: 0,
            multi_selection_active: false,
            hover_active_bounds: None,
        })
    }

    fn capabilities(&self) -> &RendererCapabilities {
        &self.capabilities
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.width = width;
        self.height = height;
        Ok(())
    }

    fn clear(&mut self, color: Option<&str>) -> Result<()> {
        let color = color.unwrap_or("#ffffff");
        #[allow(deprecated)]
        self.context.set_fill_style(&color.into());
        self.context
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // Reset stats
        self.stats = RenderStats::default();

        Ok(())
    }

    fn set_viewport(&mut self, viewport: &Viewport) {
        // Apply viewport transformation
        self.context.save();
        self.context
            .set_transform(
                viewport.zoom,
                0.0,
                0.0,
                viewport.zoom,
                -viewport.offset.x * viewport.zoom,
                -viewport.offset.y * viewport.zoom,
            )
            .unwrap_or_default();
    }

    // Dyn-compatible rendering methods
    fn render_graph_dyn(
        &mut self,
        _graph: &dyn crate::traits::GraphRenderer,
        _viewport: &Viewport,
    ) -> Result<RenderStats> {
        // Minimal implementation - just return stats
        Ok(self.stats.clone())
    }

    fn render_graph_with_selection_dyn(
        &mut self,
        _graph: &dyn crate::traits::GraphRenderer,
        _viewport: &Viewport,
        _selected_nodes: &[NodeId],
    ) -> Result<RenderStats> {
        // Minimal implementation - just return stats
        Ok(self.stats.clone())
    }

    fn render_nodes_dyn(
        &mut self,
        _nodes: &dyn crate::traits::NodeRenderer,
        _viewport: &Viewport,
    ) -> Result<()> {
        // Minimal implementation
        Ok(())
    }

    fn render_edges_dyn(
        &mut self,
        _edges: &dyn crate::traits::EdgeRenderer,
        _viewport: &Viewport,
    ) -> Result<()> {
        // Minimal implementation
        Ok(())
    }

    fn render_selection(&mut self, selected_bounds: &[Rect], style: &SelectionStyle) -> Result<()> {
        self.context.save();

        #[allow(deprecated)]
        self.context.set_stroke_style(&style.color.clone().into());
        self.context.set_line_width(style.width);

        if let Some(dasharray) = &style.dasharray {
            let dashes: Vec<f64> = dasharray
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if !dashes.is_empty() {
                let js_array = js_sys::Array::new();
                for dash in dashes {
                    js_array.push(&dash.into());
                }
                let _ = self.context.set_line_dash(&js_array);
            }
        }

        for bounds in selected_bounds {
            self.context
                .stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);

            // Add glow effect if specified
            if let (Some(glow_color), Some(glow_blur)) = (&style.glow_color, style.glow_blur) {
                self.context.save();
                self.context.set_shadow_color(glow_color);
                self.context.set_shadow_blur(glow_blur);
                self.context
                    .stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);
                self.context.restore();
            }
        }

        self.context.restore();
        Ok(())
    }

    fn render_animated_selection(
        &mut self,
        selected_bounds: &[Rect],
        style: &AnimatedSelectionStyle,
    ) -> Result<()> {
        // Simple implementation - just render as regular selection for now
        let selection_style = SelectionStyle {
            color: style.base_style.color.clone(),
            width: style.base_style.width,
            dasharray: style.base_style.dasharray.clone(),
            glow_color: None,
            glow_blur: None,
        };
        self.render_selection(selected_bounds, &selection_style)
    }

    fn render_multi_selection(
        &mut self,
        selected_bounds: &[Rect],
        style: &MultiSelectionStyle,
    ) -> Result<()> {
        // Simple implementation - render each selection with the style
        for bounds in selected_bounds {
            self.context.save();
            #[allow(deprecated)]
            self.context
                .set_stroke_style(&style.connection_color.clone().into());
            self.context.set_line_width(style.connection_width);

            self.context
                .stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);
            self.context.restore();
        }
        Ok(())
    }

    fn render_selection_hover(
        &mut self,
        bounds: &Rect,
        _hover_position: &Position,
        style: &SelectionHoverStyle,
    ) -> Result<()> {
        self.context.save();
        #[allow(deprecated)]
        self.context
            .set_fill_style(&style.hover_highlight_color.clone().into());
        self.context.set_global_alpha(0.5); // Use a fixed opacity

        self.context
            .fill_rect(bounds.x, bounds.y, bounds.width, bounds.height);
        self.context.restore();
        Ok(())
    }

    fn is_hover_active(&self, bounds: &Rect) -> bool {
        self.hover_active_bounds.as_ref() == Some(bounds)
    }

    fn get_selection_count(&self) -> usize {
        self.selection_count
    }

    fn is_multi_selection_active(&self) -> bool {
        self.multi_selection_active
    }

    fn render_background(&mut self, config: &BackgroundConfig, _viewport: &Viewport) -> Result<()> {
        match config.variant {
            BackgroundVariant::None => Ok(()),
            _ => {
                // Simple background rendering
                self.context.save();
                #[allow(deprecated)]
                self.context.set_fill_style(&config.color.clone().into());
                self.context
                    .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
                self.context.restore();
                Ok(())
            }
        }
    }

    fn present(&mut self) -> Result<()> {
        // Canvas2D automatically presents, no explicit present needed
        Ok(())
    }

    fn get_stats(&self) -> &RenderStats {
        &self.stats
    }

    fn is_context_valid(&self) -> bool {
        // Canvas2D context is always valid unless the canvas is removed from DOM
        true
    }

    fn recover_context(&mut self) -> Result<()> {
        // Canvas2D doesn't need context recovery
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::HtmlCanvasElement;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_creation() {
        // Create a canvas element for testing
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(800);
        canvas.set_height(600);

        let renderer = Canvas2DRenderer::new(&canvas);
        assert!(renderer.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_capabilities() {
        // Create a canvas element for testing
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(800);
        canvas.set_height(600);

        let renderer = Canvas2DRenderer::new(&canvas).unwrap();
        let capabilities = renderer.capabilities();

        assert_eq!(capabilities.name, "Canvas2D");
        assert!(!capabilities.supports_instancing);
        assert!(!capabilities.supports_compute_shaders);
        assert!(!capabilities.supports_msaa);
    }

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_resize() {
        // Create a canvas element for testing
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(800);
        canvas.set_height(600);

        let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
        let result = renderer.resize(1024, 768);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_clear() {
        // Create a canvas element for testing
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(800);
        canvas.set_height(600);

        let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
        let result = renderer.clear(Some("#ff0000"));
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_stats() {
        // Create a canvas element for testing
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(800);
        canvas.set_height(600);

        let renderer = Canvas2DRenderer::new(&canvas).unwrap();
        let stats = renderer.get_stats();

        assert_eq!(stats.draw_calls, 0);
        assert_eq!(stats.triangles, 0);
        assert_eq!(stats.nodes_rendered, 0);
        assert_eq!(stats.edges_rendered, 0);
    }
}
