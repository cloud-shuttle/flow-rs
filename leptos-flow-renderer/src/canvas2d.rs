//! Canvas2D renderer implementation

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use leptos_flow_core::{Graph, Node, Edge, Position, Viewport, Rect, NodeId};
use crate::traits::{
    Renderer, CustomNodeRenderer, CustomEdgeRenderer, BatchRenderer,
    RendererCapabilities, RenderStats, NodeStyle, EdgeStyle, SelectionStyle,
    AnimatedSelectionStyle, MultiSelectionStyle, SelectionHoverStyle,
    BackgroundConfig, BackgroundVariant
};
use crate::error::{RendererError, Result};
use crate::performance::{PerformanceManager, PerformanceSettings};

/// Canvas2D renderer implementation
pub struct Canvas2DRenderer {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    capabilities: RendererCapabilities,
    stats: RenderStats,
    batch_nodes: Vec<BatchedNode>,
    batch_edges: Vec<BatchedEdge>,
    width: u32,
    height: u32,
    performance_manager: PerformanceManager,
    // Animation state
    selection_count: usize,
    multi_selection_active: bool,
    hover_active_bounds: Option<Rect>,
}

#[derive(Debug, Clone)]
struct BatchedNode {
    id: String,
    position: Position,
    size: (f64, f64),
    style: NodeStyle,
}

#[derive(Debug, Clone)]
struct BatchedEdge {
    id: String,
    source_pos: Position,
    target_pos: Position,
    style: EdgeStyle,
}

impl Canvas2DRenderer {
    fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
        canvas
            .get_context("2d")
            .map_err(|_| RendererError::context_creation_failed("Failed to get 2D context"))?
            .ok_or_else(|| RendererError::context_creation_failed("2D context not available"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| RendererError::context_creation_failed("Failed to cast to 2D context"))
    }

    fn create_capabilities(_canvas: &HtmlCanvasElement) -> RendererCapabilities {
        let max_size = 8192u32; // Reasonable limit for Canvas2D

        RendererCapabilities {
            name: "Canvas2D".to_string(),
            max_texture_size: max_size,
            max_textures: 1, // Canvas2D doesn't use textures
            supports_instancing: false,
            supports_compute_shaders: false,
            supports_msaa: true, // Browser handles antialiasing
            max_msaa_samples: 4,
            max_viewport_size: (max_size, max_size),
            memory_budget: Some(100 * 1024 * 1024), // 100MB reasonable limit
        }
    }

    fn apply_node_style(&self, style: &NodeStyle) -> Result<()> {
        // Set fill style
        if let Some(bg_color) = &style.background_color {
            self.context.set_fill_style(&bg_color.clone().into());
        }

        // Set stroke style and width
        if let Some(border_color) = &style.border_color {
            self.context.set_stroke_style(&border_color.clone().into());
        }

        if let Some(border_width) = style.border_width {
            self.context.set_line_width(border_width);
        }

        // Set global alpha
        if let Some(opacity) = style.opacity {
            self.context.set_global_alpha(opacity);
        }

        Ok(())
    }

    fn apply_edge_style(&self, style: &EdgeStyle) -> Result<()> {
        // Set stroke style and width
        if let Some(stroke_color) = &style.stroke_color {
            self.context.set_stroke_style(&stroke_color.clone().into());
        }

        if let Some(stroke_width) = style.stroke_width {
            self.context.set_line_width(stroke_width);
        }

        // Set dash pattern
        if let Some(dasharray) = &style.stroke_dasharray {
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
        } else {
            let _ = self.context.set_line_dash(&js_sys::Array::new());
        }

        // Set global alpha
        if let Some(opacity) = style.opacity {
            self.context.set_global_alpha(opacity);
        }

        Ok(())
    }

    fn reset_style(&self) {
        self.context.set_global_alpha(1.0);
        self.context.set_line_width(1.0);
        let _ = self.context.set_line_dash(&js_sys::Array::new());
    }

    fn draw_rounded_rect(&self, x: f64, y: f64, width: f64, height: f64, radius: f64) -> Result<()> {
        if radius <= 0.0 {
            self.context.rect(x, y, width, height);
            return Ok(());
        }

        let r = radius.min(width / 2.0).min(height / 2.0);

        self.context.begin_path();
        self.context.move_to(x + r, y);
        self.context.line_to(x + width - r, y);
        self.context.arc(x + width - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0)
            .map_err(|_| RendererError::rendering_failed("Failed to draw arc"))?;
        self.context.line_to(x + width, y + height - r);
        self.context.arc(x + width - r, y + height - r, r, 0.0, std::f64::consts::FRAC_PI_2)
            .map_err(|_| RendererError::rendering_failed("Failed to draw arc"))?;
        self.context.line_to(x + r, y + height);
        self.context.arc(x + r, y + height - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI)
            .map_err(|_| RendererError::rendering_failed("Failed to draw arc"))?;
        self.context.line_to(x, y + r);
        self.context.arc(x + r, y + r, r, std::f64::consts::PI, -std::f64::consts::FRAC_PI_2)
            .map_err(|_| RendererError::rendering_failed("Failed to draw arc"))?;
        self.context.close_path();

        Ok(())
    }

    fn draw_bezier_edge(&self, start: Position, end: Position, style: &EdgeStyle) -> Result<()> {
        // Calculate control points for a smooth bezier curve
        let control_offset = ((end.x - start.x).abs() * 0.5).max(50.0);
        let control1 = Position::new(start.x + control_offset, start.y);
        let control2 = Position::new(end.x - control_offset, end.y);

        self.context.begin_path();
        self.context.move_to(start.x, start.y);
        self.context.bezier_curve_to(
            control1.x, control1.y,
            control2.x, control2.y,
            end.x, end.y
        );

        self.apply_edge_style(style)?;
        self.context.stroke();
        self.reset_style();

        // Draw markers if specified
        if style.marker_end.is_some() {
            self.draw_arrow_marker(control2, end)?;
        }

        Ok(())
    }

    fn draw_arrow_marker(&self, control: Position, end: Position) -> Result<()> {
        let dx = end.x - control.x;
        let dy = end.y - control.y;
        let angle = dy.atan2(dx);

        let arrow_length = 10.0;
        let arrow_angle = std::f64::consts::FRAC_PI_6; // 30 degrees

        let x1 = end.x - arrow_length * (angle - arrow_angle).cos();
        let y1 = end.y - arrow_length * (angle - arrow_angle).sin();
        let x2 = end.x - arrow_length * (angle + arrow_angle).cos();
        let y2 = end.y - arrow_length * (angle + arrow_angle).sin();

        self.context.begin_path();
        self.context.move_to(end.x, end.y);
        self.context.line_to(x1, y1);
        self.context.move_to(end.x, end.y);
        self.context.line_to(x2, y2);
        self.context.stroke();

        Ok(())
    }

    fn render_background_pattern(&self, config: &BackgroundConfig, viewport: &Viewport) -> Result<()> {
        match config.variant {
            BackgroundVariant::None => {
                self.context.set_fill_style(&config.color.clone().into());
                self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
            },
            BackgroundVariant::Dots => {
                self.render_dot_pattern(config, viewport)?;
            },
            BackgroundVariant::Lines => {
                self.render_line_pattern(config, viewport, false)?;
            },
            BackgroundVariant::Grid => {
                self.render_line_pattern(config, viewport, true)?;
            },
            BackgroundVariant::Cross => {
                self.render_cross_pattern(config, viewport)?;
            }
        }
        Ok(())
    }

    fn render_dot_pattern(&self, config: &BackgroundConfig, viewport: &Viewport) -> Result<()> {
        self.context.set_fill_style(&config.color.clone().into());
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        let size = config.size * viewport.zoom;
        let offset_x = (-viewport.offset.x * viewport.zoom) % size;
        let offset_y = (-viewport.offset.y * viewport.zoom) % size;

        self.context.set_global_alpha(config.opacity);
        self.context.set_fill_style(&config.pattern_color.clone().into());

        let mut x = offset_x;
        while x < self.width as f64 {
            let mut y = offset_y;
            while y < self.height as f64 {
                self.context.begin_path();
                self.context.arc(x, y, 1.0, 0.0, 2.0 * std::f64::consts::PI)
                    .map_err(|_| RendererError::rendering_failed("Failed to draw dot"))?;
                self.context.fill();
                y += size;
            }
            x += size;
        }

        self.context.set_global_alpha(1.0);
        Ok(())
    }

    fn render_line_pattern(&self, config: &BackgroundConfig, viewport: &Viewport, cross: bool) -> Result<()> {
        self.context.set_fill_style(&config.color.clone().into());
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        let size = config.size * viewport.zoom;
        let offset_x = (-viewport.offset.x * viewport.zoom) % size;
        let offset_y = (-viewport.offset.y * viewport.zoom) % size;

        self.context.set_global_alpha(config.opacity);
        self.context.set_stroke_style(&config.pattern_color.clone().into());
        self.context.set_line_width(1.0);

        // Vertical lines
        let mut x = offset_x;
        while x < self.width as f64 {
            self.context.begin_path();
            self.context.move_to(x, 0.0);
            self.context.line_to(x, self.height as f64);
            self.context.stroke();
            x += size;
        }

        // Horizontal lines (for grid)
        if cross {
            let mut y = offset_y;
            while y < self.height as f64 {
                self.context.begin_path();
                self.context.move_to(0.0, y);
                self.context.line_to(self.width as f64, y);
                self.context.stroke();
                y += size;
            }
        }

        self.context.set_global_alpha(1.0);
        Ok(())
    }

    fn render_cross_pattern(&self, config: &BackgroundConfig, viewport: &Viewport) -> Result<()> {
        self.context.set_fill_style(&config.color.clone().into());
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        let size = config.size * viewport.zoom;
        let offset_x = (-viewport.offset.x * viewport.zoom) % size;
        let offset_y = (-viewport.offset.y * viewport.zoom) % size;

        self.context.set_global_alpha(config.opacity);
        self.context.set_stroke_style(&config.pattern_color.clone().into());
        self.context.set_line_width(1.0);

        let cross_size = 4.0;

        let mut x = offset_x;
        while x < self.width as f64 {
            let mut y = offset_y;
            while y < self.height as f64 {
                // Draw cross
                self.context.begin_path();
                self.context.move_to(x - cross_size, y);
                self.context.line_to(x + cross_size, y);
                self.context.move_to(x, y - cross_size);
                self.context.line_to(x, y + cross_size);
                self.context.stroke();
                y += size;
            }
            x += size;
        }

        self.context.set_global_alpha(1.0);
        Ok(())
    }

    /// Helper function to parse color components and apply alpha
    fn parse_color_components(color: &str, _alpha: f64) -> String {
        // Simple color parsing for animation effects
        if color.starts_with('#') {
            let hex = color.trim_start_matches('#');
            match hex.len() {
                3 => {
                    let r = u8::from_str_radix(&hex[0..1], 16).unwrap_or(0) * 17;
                    let g = u8::from_str_radix(&hex[1..2], 16).unwrap_or(0) * 17;
                    let b = u8::from_str_radix(&hex[2..3], 16).unwrap_or(0) * 17;
                    format!("{}, {}, {}", r, g, b)
                }
                6 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                    format!("{}, {}, {}", r, g, b)
                }
                _ => "0, 0, 0".to_string(),
            }
        } else {
            // Fallback for named colors or other formats
            "0, 0, 0".to_string()
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
            batch_nodes: Vec::new(),
            batch_edges: Vec::new(),
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
        self.context.set_fill_style(&color.into());
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // Reset stats
        self.stats = RenderStats::default();

        Ok(())
    }

    fn set_viewport(&mut self, viewport: &Viewport) {
        // Apply viewport transformation
        self.context.save();
        self.context.set_transform(
            viewport.zoom, 0.0, 0.0, viewport.zoom,
            -viewport.offset.x * viewport.zoom,
            -viewport.offset.y * viewport.zoom
        ).unwrap_or_default();
    }

    fn render_graph<N, E>(&mut self, graph: &Graph<N, E>, viewport: &Viewport) -> Result<RenderStats>
    where
        N: Clone + 'static,
        E: Clone + 'static,
    {
        let start_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        self.set_viewport(viewport);

        // Render edges first (behind nodes)
        for edge in graph.edges() {
            if let (Some(source), Some(target)) = (graph.get_node(&edge.source), graph.get_node(&edge.target)) {
                let source_pos = source.position;
                let target_pos = target.position;

                // Simple visibility check
                let edge_bounds = Rect::from_points(source_pos, target_pos);
                if viewport.intersects_rect(edge_bounds) {
                    let style = EdgeStyle::default();
                    self.draw_bezier_edge(source_pos, target_pos, &style)?;
                    self.stats.edges_rendered += 1;
                    self.stats.draw_calls += 1;
                } else {
                    self.stats.edges_culled += 1;
                }
            }
        }

        // Render nodes
        for node in graph.nodes() {
            let node_bounds = node.bounds();
            if viewport.intersects_rect(node_bounds) {
                let style = NodeStyle::default();
                self.render_custom_node(node, &style, viewport)?;
                self.stats.nodes_rendered += 1;
            } else {
                self.stats.nodes_culled += 1;
            }
        }

        self.context.restore();

        // Calculate frame time
        let end_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        self.stats.frame_time_ms = end_time - start_time;

        Ok(self.stats.clone())
    }

    fn render_graph_with_selection<N, E>(
        &mut self,
        graph: &Graph<N, E>,
        viewport: &Viewport,
        selected_nodes: &[NodeId]
    ) -> Result<RenderStats>
    where
        N: Clone + 'static,
        E: Clone + 'static,
    {
        let start_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        self.set_viewport(viewport);

        // Create a set of selected node IDs for fast lookup
        let selected_set: std::collections::HashSet<NodeId> = selected_nodes.iter().cloned().collect();

        // Render edges first (behind nodes)
        for edge in graph.edges() {
            if let (Some(source), Some(target)) = (graph.get_node(&edge.source), graph.get_node(&edge.target)) {
                let source_pos = source.position;
                let target_pos = target.position;

                // Simple visibility check
                let edge_bounds = Rect::from_points(source_pos, target_pos);
                if viewport.intersects_rect(edge_bounds) {
                    let style = EdgeStyle::default();
                    self.draw_bezier_edge(source_pos, target_pos, &style)?;
                    self.stats.edges_rendered += 1;
                    self.stats.draw_calls += 1;
                } else {
                    self.stats.edges_culled += 1;
                }
            }
        }

        // Render nodes with selection styling
        for node in graph.nodes() {
            let node_bounds = node.bounds();
            if viewport.intersects_rect(node_bounds) {
                let is_selected = selected_set.contains(&node.id);
                let style = if is_selected {
                    NodeStyle {
                        background_color: Some("#e3f2fd".to_string()), // Light blue for selected
                        border_color: Some("#2196f3".to_string()),     // Blue border for selected
                        border_width: Some(2.0),                       // Thicker border for selected
                        border_radius: Some(4.0),
                        shadow_color: Some("#2196f3".to_string()),     // Blue shadow for selected
                        shadow_offset: Some(Position::new(0.0, 2.0)),
                        shadow_blur: Some(4.0),
                        opacity: Some(1.0),
                    }
                } else {
                    NodeStyle::default()
                };
                self.render_custom_node(node, &style, viewport)?;
                self.stats.nodes_rendered += 1;
            } else {
                self.stats.nodes_culled += 1;
            }
        }

        self.context.restore();

        // Calculate frame time
        let end_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        self.stats.frame_time_ms = end_time - start_time;

        Ok(self.stats.clone())
    }

    fn render_nodes<N>(&mut self, nodes: &[Node<N>], viewport: &Viewport) -> Result<()>
    where
        N: Clone + 'static,
    {
        for node in nodes {
            let node_bounds = node.bounds();
            if viewport.intersects_rect(node_bounds) {
                let style = NodeStyle::default();
                self.render_custom_node(node, &style, viewport)?;
                self.stats.nodes_rendered += 1;
            } else {
                self.stats.nodes_culled += 1;
            }
        }
        Ok(())
    }

    fn render_edges<E>(&mut self, edges: &[Edge<E>], _viewport: &Viewport) -> Result<()>
    where
        E: Clone + 'static,
    {
        // Note: This would require additional position information in a real implementation
        // For now, we'll skip edge rendering without source/target positions
        self.stats.edges_rendered = edges.len();
        Ok(())
    }

    fn render_selection(&mut self, selected_bounds: &[Rect], style: &SelectionStyle) -> Result<()> {
        self.context.save();

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
            self.context.stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);

            // Add glow effect if specified
            if let (Some(glow_color), Some(glow_blur)) = (&style.glow_color, style.glow_blur) {
                self.context.save();
                self.context.set_shadow_color(glow_color);
                self.context.set_shadow_blur(glow_blur);
                self.context.stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);
                self.context.restore();
            }
        }

        self.context.restore();
        Ok(())
    }

    fn render_animated_selection(&mut self, selected_bounds: &[Rect], style: &AnimatedSelectionStyle) -> Result<()> {
        if selected_bounds.is_empty() {
            return Ok(());
        }

        self.context.save();

        // Update animation progress
        let current_time = js_sys::Date::now();
        let elapsed = if let Some(start_time) = style.animation_start_time {
            current_time - start_time
        } else {
            0.0
        };

        // Calculate animation progress (0.0 to 1.0)
        let progress = (elapsed / style.animation_duration_ms).min(1.0);

        // Apply animation effects
        let mut animated_style = style.base_style.clone();

        if style.fade_in_enabled {
            // Fade in effect
            let alpha = progress;
            let color_components = Self::parse_color_components(&style.base_style.color, alpha);
            animated_style.color = format!("rgba({}, {})", color_components, alpha);
        }

        if style.pulse_enabled {
            // Pulse effect - scale the selection rectangle
            let pulse_scale = 1.0 + (progress * 0.1 * (1.0 - progress)); // Subtle pulse
            let pulse_width = style.base_style.width * pulse_scale;
            animated_style.width = pulse_width;
        }

        // Render with animated style
        self.context.set_stroke_style(&animated_style.color.clone().into());
        self.context.set_line_width(animated_style.width);

        if let Some(dasharray) = &animated_style.dasharray {
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
            self.context.stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);

            // Add animated glow effect
            if let (Some(glow_color), Some(glow_blur)) = (&animated_style.glow_color, animated_style.glow_blur) {
                let animated_glow_blur = glow_blur * progress; // Glow grows with animation
                self.context.save();
                self.context.set_shadow_color(glow_color);
                self.context.set_shadow_blur(animated_glow_blur);
                self.context.stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);
                self.context.restore();
            }
        }

        self.context.restore();
        Ok(())
    }

    fn render_multi_selection(&mut self, selected_bounds: &[Rect], style: &MultiSelectionStyle) -> Result<()> {
        if selected_bounds.len() < 2 {
            return Ok(());
        }

        self.context.save();

        // Update selection count
        self.selection_count = selected_bounds.len();
        self.multi_selection_active = true;

        // Render connection lines between selected items
        if style.connection_lines {
            self.context.set_stroke_style(&style.connection_color.clone().into());
            self.context.set_line_width(style.connection_width);
            self.context.set_global_alpha(0.5);

            for i in 0..selected_bounds.len() {
                for j in (i + 1)..selected_bounds.len() {
                    let bounds1 = &selected_bounds[i];
                    let bounds2 = &selected_bounds[j];

                    // Draw line between centers
                    let center1_x = bounds1.x + bounds1.width / 2.0;
                    let center1_y = bounds1.y + bounds1.height / 2.0;
                    let center2_x = bounds2.x + bounds2.width / 2.0;
                    let center2_y = bounds2.y + bounds2.height / 2.0;

                    self.context.begin_path();
                    self.context.move_to(center1_x, center1_y);
                    self.context.line_to(center2_x, center2_y);
                    self.context.stroke();
                }
            }
        }

        // Render selection count indicator
        if style.selection_count_indicator && selected_bounds.len() > 1 {
            // Find the center of all selected bounds
            let mut center_x = 0.0;
            let mut center_y = 0.0;
            for bounds in selected_bounds {
                center_x += bounds.x + bounds.width / 2.0;
                center_y += bounds.y + bounds.height / 2.0;
            }
            center_x /= selected_bounds.len() as f64;
            center_y /= selected_bounds.len() as f64;

            // Draw count indicator circle
            let radius = 20.0;
            self.context.set_fill_style(&style.count_background_color.clone().into());
            self.context.set_global_alpha(0.9);
            self.context.begin_path();
            self.context.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)
                .map_err(|_| RendererError::rendering_failed("Failed to draw count indicator"))?;
            self.context.fill();

            // Draw count text
            self.context.set_fill_style(&style.count_text_color.clone().into());
            self.context.set_font("14px Arial");
            self.context.set_text_align("center");
            self.context.set_text_baseline("middle");
            let count_text = selected_bounds.len().to_string();
            self.context.fill_text(&count_text, center_x, center_y)
                .map_err(|_| RendererError::rendering_failed("Failed to draw count text"))?;
        }

        self.context.restore();
        Ok(())
    }

    fn render_selection_hover(&mut self, bounds: &Rect, hover_position: &Position, style: &SelectionHoverStyle) -> Result<()> {
        self.context.save();

        // Check if hover position is within bounds
        if bounds.contains_point(*hover_position) {
            self.hover_active_bounds = Some(*bounds);

            // Apply hover effects
            if style.hover_scale_factor != 1.0 {
                // Scale the bounds for hover effect
                let scale = style.hover_scale_factor;
                let scaled_width = bounds.width * scale;
                let scaled_height = bounds.height * scale;
                let offset_x = (bounds.width - scaled_width) / 2.0;
                let offset_y = (bounds.height - scaled_height) / 2.0;

                self.context.set_stroke_style(&style.hover_highlight_color.clone().into());
                self.context.set_line_width(2.0);
                self.context.stroke_rect(
                    bounds.x + offset_x,
                    bounds.y + offset_y,
                    scaled_width,
                    scaled_height
                );
            }

            // Add hover glow effect
            if style.hover_glow_enabled {
                self.context.set_shadow_color(&style.hover_glow_color);
                self.context.set_shadow_blur(style.hover_glow_blur);
                self.context.set_stroke_style(&style.hover_highlight_color.clone().into());
                self.context.set_line_width(2.0);
                self.context.stroke_rect(bounds.x, bounds.y, bounds.width, bounds.height);
            }
        } else {
            self.hover_active_bounds = None;
        }

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

    fn render_background(&mut self, config: &BackgroundConfig, viewport: &Viewport) -> Result<()> {
        self.render_background_pattern(config, viewport)
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

impl CustomNodeRenderer for Canvas2DRenderer {
    fn render_custom_node<N>(&mut self, node: &Node<N>, style: &NodeStyle, _viewport: &Viewport) -> Result<()>
    where
        N: Clone + 'static,
    {
        let pos = node.position;
        let size = node.size;
        let radius = style.border_radius.unwrap_or(0.0);

        self.context.save();
        self.apply_node_style(style)?;

        // Draw node background
        self.draw_rounded_rect(pos.x, pos.y, size.width, size.height, radius)?;
        self.context.fill();

        // Draw border if specified
        if style.border_width.unwrap_or(0.0) > 0.0 {
            self.context.stroke();
        }

        // Draw shadow if specified
        if let (Some(shadow_color), Some(shadow_offset), Some(shadow_blur)) =
            (&style.shadow_color, &style.shadow_offset, style.shadow_blur) {
            self.context.save();
            self.context.set_shadow_color(shadow_color);
            self.context.set_shadow_offset_x(shadow_offset.x);
            self.context.set_shadow_offset_y(shadow_offset.y);
            self.context.set_shadow_blur(shadow_blur);
            self.draw_rounded_rect(pos.x, pos.y, size.width, size.height, radius)?;
            self.context.fill();
            self.context.restore();
        }

        self.context.restore();
        self.stats.draw_calls += 1;
        self.stats.triangles += 2; // Approximate for a rectangle

        Ok(())
    }
}

impl CustomEdgeRenderer for Canvas2DRenderer {
    fn render_custom_edge<E>(
        &mut self,
        _edge: &Edge<E>,
        style: &EdgeStyle,
        source_pos: Position,
        target_pos: Position,
        _viewport: &Viewport,
    ) -> Result<()>
    where
        E: Clone + 'static,
    {
        self.draw_bezier_edge(source_pos, target_pos, style)?;
        self.stats.draw_calls += 1;
        Ok(())
    }
}

impl BatchRenderer for Canvas2DRenderer {
    fn begin_batch(&mut self) -> Result<()> {
        self.batch_nodes.clear();
        self.batch_edges.clear();
        Ok(())
    }

    fn batch_node<N>(&mut self, node: &Node<N>) -> Result<()>
    where
        N: Clone + 'static,
    {
        self.batch_nodes.push(BatchedNode {
            id: node.id.as_str().to_string(),
            position: node.position,
            size: (node.size.width, node.size.height),
            style: NodeStyle::default(),
        });
        Ok(())
    }

    fn batch_edge<E>(&mut self, edge: &Edge<E>, source_pos: Position, target_pos: Position) -> Result<()>
    where
        E: Clone + 'static,
    {
        self.batch_edges.push(BatchedEdge {
            id: edge.id.as_str().to_string(),
            source_pos,
            target_pos,
            style: EdgeStyle::default(),
        });
        Ok(())
    }

    fn flush_batch(&mut self) -> Result<()> {
        // Render all batched edges first
        for edge in &self.batch_edges {
            self.draw_bezier_edge(edge.source_pos, edge.target_pos, &edge.style)?;
            self.stats.edges_rendered += 1;
            self.stats.draw_calls += 1;
        }

        // Render all batched nodes
        for node in &self.batch_nodes {
            self.context.save();
            self.apply_node_style(&node.style)?;

            let radius = node.style.border_radius.unwrap_or(0.0);
            self.draw_rounded_rect(node.position.x, node.position.y, node.size.0, node.size.1, radius)?;
            self.context.fill();

            if node.style.border_width.unwrap_or(0.0) > 0.0 {
                self.context.stroke();
            }

            self.context.restore();
            self.stats.nodes_rendered += 1;
            self.stats.draw_calls += 1;
            self.stats.triangles += 2;
        }

        self.batch_nodes.clear();
        self.batch_edges.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::HtmlCanvasElement;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    fn create_test_canvas() -> HtmlCanvasElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        canvas.dyn_into::<HtmlCanvasElement>().unwrap()
    }

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_creation() {
        let canvas = create_test_canvas();
        let renderer = Canvas2DRenderer::new(&canvas);
        assert!(renderer.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_capabilities() {
        let canvas = create_test_canvas();
        let renderer = Canvas2DRenderer::new(&canvas).unwrap();
        let caps = renderer.capabilities();

        assert_eq!(caps.name, "Canvas2D");
        assert_eq!(caps.supports_msaa, true);
        assert_eq!(caps.supports_compute_shaders, false);
    }

    #[wasm_bindgen_test]
    fn test_clear() {
        let canvas = create_test_canvas();
        let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

        assert!(renderer.clear(Some("#ff0000")).is_ok());
        assert!(renderer.clear(None).is_ok());
    }

    #[wasm_bindgen_test]
    fn test_resize() {
        let canvas = create_test_canvas();
        let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

        assert!(renderer.resize(800, 600).is_ok());
        assert_eq!(renderer.width, 800);
        assert_eq!(renderer.height, 600);
    }
}
