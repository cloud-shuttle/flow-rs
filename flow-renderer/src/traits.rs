//! Renderer trait definitions and shared types

use crate::error::Result;
use flow_core::{Edge, EdgeId, Graph, Node, NodeId, Position, Rect, Size, Viewport};
use js_sys;
use serde_json;

/// Supported rendering backends
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RendererType {
    Canvas2D,
    WebGL2,
    WebGPU,
}

impl RendererType {
    /// Get the name of the renderer
    pub fn name(self) -> &'static str {
        match self {
            Self::Canvas2D => "Canvas2D",
            Self::WebGL2 => "WebGL2",
            Self::WebGPU => "WebGPU",
        }
    }

    /// Get all available renderer types
    pub fn all() -> &'static [Self] {
        &[Self::Canvas2D, Self::WebGL2, Self::WebGPU]
    }
}

/// Renderer capabilities and features
#[derive(Debug, Clone, PartialEq)]
pub struct RendererCapabilities {
    pub name: String,
    pub max_texture_size: u32,
    pub max_textures: u32,
    pub supports_instancing: bool,
    pub supports_compute_shaders: bool,
    pub supports_msaa: bool,
    pub max_msaa_samples: u32,
    pub max_viewport_size: (u32, u32),
    pub memory_budget: Option<u64>, // bytes
}

impl Default for RendererCapabilities {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            max_texture_size: 2048,
            max_textures: 16,
            supports_instancing: false,
            supports_compute_shaders: false,
            supports_msaa: false,
            max_msaa_samples: 1,
            max_viewport_size: (4096, 4096),
            memory_budget: None,
        }
    }
}

/// Render statistics for performance monitoring
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    pub frame_time_ms: f64,
    pub nodes_rendered: usize,
    pub edges_rendered: usize,
    pub nodes_culled: usize,
    pub edges_culled: usize,
    pub draw_calls: usize,
    pub triangles: usize,
    pub memory_used: u64, // bytes
}

/// Styling information for nodes
#[derive(Debug, Clone, PartialEq)]
pub struct NodeStyle {
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<f64>,
    pub border_radius: Option<f64>,
    pub shadow_color: Option<String>,
    pub shadow_offset: Option<Position>,
    pub shadow_blur: Option<f64>,
    pub opacity: Option<f64>,
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            background_color: Some("#ffffff".to_string()),
            border_color: Some("#cccccc".to_string()),
            border_width: Some(1.0),
            border_radius: Some(4.0),
            shadow_color: None,
            shadow_offset: None,
            shadow_blur: None,
            opacity: Some(1.0),
        }
    }
}

/// Styling information for edges
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeStyle {
    pub stroke_color: Option<String>,
    pub stroke_width: Option<f64>,
    pub stroke_dasharray: Option<String>,
    pub marker_start: Option<String>,
    pub marker_end: Option<String>,
    pub opacity: Option<f64>,
    pub animated: bool,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            stroke_color: Some("#999999".to_string()),
            stroke_width: Some(2.0),
            stroke_dasharray: None,
            marker_start: None,
            marker_end: None,
            opacity: Some(1.0),
            animated: false,
        }
    }
}

/// Selection visual state
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionStyle {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
    pub glow_color: Option<String>,
    pub glow_blur: Option<f64>,
}

impl Default for SelectionStyle {
    fn default() -> Self {
        Self {
            color: "#1a73e8".to_string(),
            width: 2.0,
            dasharray: None,
            glow_color: Some("#1a73e8".to_string()),
            glow_blur: Some(4.0),
        }
    }
}

/// Animated selection style with timing and effects
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedSelectionStyle {
    pub base_style: SelectionStyle,
    pub animation_duration_ms: f64,
    pub pulse_enabled: bool,
    pub fade_in_enabled: bool,
    pub performance_mode: bool,
    pub batch_rendering: bool,
    // Animation state
    pub is_animating: bool,
    pub animation_start_time: Option<f64>,
    pub animation_progress: f64,
}

impl AnimatedSelectionStyle {
    pub fn new() -> Self {
        Self {
            base_style: SelectionStyle::default(),
            animation_duration_ms: 300.0,
            pulse_enabled: false,
            fade_in_enabled: false,
            performance_mode: false,
            batch_rendering: false,
            is_animating: false,
            animation_start_time: None,
            animation_progress: 0.0,
        }
    }

    pub fn with_animation_duration(mut self, duration_ms: f64) -> Self {
        self.animation_duration_ms = duration_ms;
        self
    }

    pub fn with_pulse_enabled(mut self, enabled: bool) -> Self {
        self.pulse_enabled = enabled;
        self
    }

    pub fn with_fade_in_enabled(mut self, enabled: bool) -> Self {
        self.fade_in_enabled = enabled;
        self
    }

    pub fn with_performance_mode(mut self, enabled: bool) -> Self {
        self.performance_mode = enabled;
        self
    }

    pub fn with_batch_rendering(mut self, enabled: bool) -> Self {
        self.batch_rendering = enabled;
        self
    }

    pub fn animation_duration_ms(&self) -> f64 {
        self.animation_duration_ms
    }

    pub fn pulse_enabled(&self) -> bool {
        self.pulse_enabled
    }

    pub fn fade_in_enabled(&self) -> bool {
        self.fade_in_enabled
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn animation_progress(&self) -> f64 {
        self.animation_progress
    }

    pub fn start_animation(&mut self) {
        self.is_animating = true;
        self.animation_start_time = Some(js_sys::Date::now());
        self.animation_progress = 0.0;
    }

    pub fn update_animation(&mut self, elapsed_ms: f64) {
        if !self.is_animating {
            return;
        }

        self.animation_progress = (elapsed_ms / self.animation_duration_ms).min(1.0);

        if self.animation_progress >= 1.0 {
            self.is_animating = false;
            self.animation_progress = 1.0;
        }
    }
}

impl Default for AnimatedSelectionStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-selection indicators style
#[derive(Debug, Clone, PartialEq)]
pub struct MultiSelectionStyle {
    pub connection_lines: bool,
    pub selection_count_indicator: bool,
    pub connection_color: String,
    pub connection_width: f64,
    pub count_background_color: String,
    pub count_text_color: String,
}

impl MultiSelectionStyle {
    pub fn new() -> Self {
        Self {
            connection_lines: false,
            selection_count_indicator: false,
            connection_color: "#1a73e8".to_string(),
            connection_width: 1.0,
            count_background_color: "#1a73e8".to_string(),
            count_text_color: "#ffffff".to_string(),
        }
    }

    pub fn with_connection_lines(mut self, enabled: bool) -> Self {
        self.connection_lines = enabled;
        self
    }

    pub fn with_selection_count_indicator(mut self, enabled: bool) -> Self {
        self.selection_count_indicator = enabled;
        self
    }
}

impl Default for MultiSelectionStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection hover effects style
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionHoverStyle {
    pub hover_highlight_color: String,
    pub hover_scale_factor: f64,
    pub hover_glow_enabled: bool,
    pub hover_glow_color: String,
    pub hover_glow_blur: f64,
}

impl SelectionHoverStyle {
    pub fn new() -> Self {
        Self {
            hover_highlight_color: "#ffeb3b".to_string(),
            hover_scale_factor: 1.0,
            hover_glow_enabled: false,
            hover_glow_color: "#ffeb3b".to_string(),
            hover_glow_blur: 8.0,
        }
    }

    pub fn with_hover_highlight_color(mut self, color: String) -> Self {
        self.hover_highlight_color = color;
        self
    }

    pub fn with_hover_scale_factor(mut self, scale: f64) -> Self {
        self.hover_scale_factor = scale;
        self
    }

    pub fn with_hover_glow_enabled(mut self, enabled: bool) -> Self {
        self.hover_glow_enabled = enabled;
        self
    }
}

impl Default for SelectionHoverStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Background pattern configuration
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundConfig {
    pub variant: BackgroundVariant,
    pub color: String,
    pub pattern_color: String,
    pub size: f64,
    pub opacity: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundVariant {
    None,
    Dots,
    Lines,
    Grid,
    Cross,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            variant: BackgroundVariant::None,
            color: "#ffffff".to_string(),
            pattern_color: "#e1e5e9".to_string(),
            size: 20.0,
            opacity: 0.8,
        }
    }
}

/// Trait for type-erased graph rendering
pub trait GraphRenderer {
    /// Get all nodes in the graph
    fn get_nodes(&self) -> Vec<Box<dyn NodeRenderer>>;
    /// Get all edges in the graph
    fn get_edges(&self) -> Vec<Box<dyn EdgeRenderer>>;
    /// Get node by ID
    fn get_node(&self, id: &NodeId) -> Option<Box<dyn NodeRenderer>>;
    /// Get edge by ID
    fn get_edge(&self, id: &EdgeId) -> Option<Box<dyn EdgeRenderer>>;
}

/// Trait for type-erased node rendering
pub trait NodeRenderer {
    /// Get node ID
    fn get_id(&self) -> NodeId;
    /// Get node position
    fn get_position(&self) -> Position;
    /// Get node size
    fn get_size(&self) -> Size;
    /// Get node data as JSON string
    fn get_data_json(&self) -> String;
    /// Check if node is selected
    fn is_selected(&self) -> bool;
}

/// Trait for type-erased edge rendering
pub trait EdgeRenderer {
    /// Get edge ID
    fn get_id(&self) -> EdgeId;
    /// Get source node ID
    fn get_source(&self) -> NodeId;
    /// Get target node ID
    fn get_target(&self) -> NodeId;
    /// Get edge data as JSON string
    fn get_data_json(&self) -> String;
    /// Check if edge is selected
    fn is_selected(&self) -> bool;
}

/// Main renderer trait
pub trait Renderer {
    /// Initialize the renderer with a canvas element
    fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self>
    where
        Self: Sized;

    /// Get renderer capabilities
    fn capabilities(&self) -> &RendererCapabilities;

    /// Set the viewport size
    fn resize(&mut self, width: u32, height: u32) -> Result<()>;

    /// Clear the render target
    fn clear(&mut self, color: Option<&str>) -> Result<()>;

    /// Set the viewport for rendering
    fn set_viewport(&mut self, viewport: &Viewport);

    /// Render a complete graph (dyn-compatible version)
    fn render_graph_dyn(
        &mut self,
        graph: &dyn GraphRenderer,
        viewport: &Viewport,
    ) -> Result<RenderStats>;

    /// Render a complete graph with selection state (dyn-compatible version)
    fn render_graph_with_selection_dyn(
        &mut self,
        graph: &dyn GraphRenderer,
        viewport: &Viewport,
        selected_nodes: &[NodeId],
    ) -> Result<RenderStats>;

    /// Render nodes only (dyn-compatible version)
    fn render_nodes_dyn(&mut self, nodes: &dyn NodeRenderer, viewport: &Viewport) -> Result<()>;

    /// Render edges only (dyn-compatible version)
    fn render_edges_dyn(&mut self, edges: &dyn EdgeRenderer, viewport: &Viewport) -> Result<()>;

    /// Render selection indicators
    fn render_selection(&mut self, selected_bounds: &[Rect], style: &SelectionStyle) -> Result<()>;

    /// Render animated selection indicators
    fn render_animated_selection(
        &mut self,
        selected_bounds: &[Rect],
        style: &AnimatedSelectionStyle,
    ) -> Result<()>;

    /// Render multi-selection indicators
    fn render_multi_selection(
        &mut self,
        selected_bounds: &[Rect],
        style: &MultiSelectionStyle,
    ) -> Result<()>;

    /// Render selection hover effects
    fn render_selection_hover(
        &mut self,
        bounds: &Rect,
        hover_position: &Position,
        style: &SelectionHoverStyle,
    ) -> Result<()>;

    /// Check if hover is active for given bounds
    fn is_hover_active(&self, bounds: &Rect) -> bool;

    /// Get current selection count
    fn get_selection_count(&self) -> usize;

    /// Check if multi-selection is active
    fn is_multi_selection_active(&self) -> bool;

    /// Render background pattern
    fn render_background(&mut self, config: &BackgroundConfig, viewport: &Viewport) -> Result<()>;

    /// Present the rendered frame
    fn present(&mut self) -> Result<()>;

    /// Get current render statistics
    fn get_stats(&self) -> &RenderStats;

    /// Check if the renderer context is valid
    fn is_context_valid(&self) -> bool;

    /// Recover from context loss (if supported)
    fn recover_context(&mut self) -> Result<()>;
}

/// Trait for renderers that support node customization
pub trait CustomNodeRenderer: Renderer {
    /// Render a single node with custom style
    fn render_custom_node<N>(
        &mut self,
        node: &Node<N>,
        style: &NodeStyle,
        viewport: &Viewport,
    ) -> Result<()>
    where
        N: Clone + 'static;
}

/// Trait for renderers that support edge customization
pub trait CustomEdgeRenderer: Renderer {
    /// Render a single edge with custom style
    fn render_custom_edge<E>(
        &mut self,
        edge: &Edge<E>,
        style: &EdgeStyle,
        source_pos: Position,
        target_pos: Position,
        viewport: &Viewport,
    ) -> Result<()>
    where
        E: Clone + 'static;
}

/// Trait for high-performance batch rendering
pub trait BatchRenderer: Renderer {
    /// Begin a new render batch
    fn begin_batch(&mut self) -> Result<()>;

    /// Add a node to the current batch
    fn batch_node<N>(&mut self, node: &Node<N>) -> Result<()>
    where
        N: Clone + 'static;

    /// Add an edge to the current batch
    fn batch_edge<E>(
        &mut self,
        edge: &Edge<E>,
        source_pos: Position,
        target_pos: Position,
    ) -> Result<()>
    where
        E: Clone + 'static;

    /// Render and flush the current batch
    fn flush_batch(&mut self) -> Result<()>;
}

/// Utility functions for rendering
pub mod utils {
    use super::*;
    use flow_core::Position;

    /// Parse CSS color string to RGBA values
    pub fn parse_color(color: &str) -> Option<[f32; 4]> {
        if color.starts_with('#') {
            parse_hex_color(color)
        } else if color.starts_with("rgb") {
            parse_rgb_color(color)
        } else {
            parse_named_color(color)
        }
    }

    fn parse_hex_color(color: &str) -> Option<[f32; 4]> {
        let hex = color.trim_start_matches('#');

        let (r, g, b, a) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                (r, g, b, 255)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                (r, g, b, a)
            }
            _ => return None,
        };

        Some([
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        ])
    }

    fn parse_rgb_color(color: &str) -> Option<[f32; 4]> {
        // Simple RGB parsing - could be enhanced
        if color.starts_with("rgb(") {
            let values = color.trim_start_matches("rgb(").trim_end_matches(')');
            let parts: Vec<&str> = values.split(',').map(|s| s.trim()).collect();

            if parts.len() == 3 {
                let r = parts[0].parse::<u8>().ok()?;
                let g = parts[1].parse::<u8>().ok()?;
                let b = parts[2].parse::<u8>().ok()?;

                return Some([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]);
            }
        }
        None
    }

    fn parse_named_color(color: &str) -> Option<[f32; 4]> {
        // Basic named colors
        match color.to_lowercase().as_str() {
            "black" => Some([0.0, 0.0, 0.0, 1.0]),
            "white" => Some([1.0, 1.0, 1.0, 1.0]),
            "red" => Some([1.0, 0.0, 0.0, 1.0]),
            "green" => Some([0.0, 1.0, 0.0, 1.0]),
            "blue" => Some([0.0, 0.0, 1.0, 1.0]),
            "transparent" => Some([0.0, 0.0, 0.0, 0.0]),
            _ => None,
        }
    }

    /// Calculate bezier curve points for edge rendering
    pub fn calculate_bezier_curve(
        start: Position,
        end: Position,
        control1: Option<Position>,
        control2: Option<Position>,
        steps: usize,
    ) -> Vec<Position> {
        let c1 = control1.unwrap_or_else(|| {
            let mid_x = (start.x + end.x) / 2.0;
            Position::new(mid_x, start.y)
        });

        let c2 = control2.unwrap_or_else(|| {
            let mid_x = (start.x + end.x) / 2.0;
            Position::new(mid_x, end.y)
        });

        (0..=steps)
            .map(|i| {
                let t = i as f64 / steps as f64;
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;

                Position::new(
                    mt3 * start.x + 3.0 * mt2 * t * c1.x + 3.0 * mt * t2 * c2.x + t3 * end.x,
                    mt3 * start.y + 3.0 * mt2 * t * c1.y + 3.0 * mt * t2 * c2.y + t3 * end.y,
                )
            })
            .collect()
    }

    /// Check if a rectangle is visible in the viewport
    pub fn is_visible(bounds: &flow_core::Rect, viewport: &Viewport) -> bool {
        viewport.intersects_rect(*bounds)
    }

    /// Calculate node bounds with padding
    pub fn node_bounds_with_padding(node: &Node<impl Clone>, padding: f64) -> flow_core::Rect {
        node.bounds().expand(padding)
    }
}

// Trait implementations for type erasure
impl<N, E> GraphRenderer for Graph<N, E>
where
    N: Clone + serde::Serialize + 'static,
    E: Clone + serde::Serialize + 'static,
{
    fn get_nodes(&self) -> Vec<Box<dyn NodeRenderer>> {
        self.nodes()
            .map(|node| Box::new(ErasedNode::new(node.clone())) as Box<dyn NodeRenderer>)
            .collect()
    }

    fn get_edges(&self) -> Vec<Box<dyn EdgeRenderer>> {
        self.edges()
            .map(|edge| Box::new(ErasedEdge::new(edge.clone())) as Box<dyn EdgeRenderer>)
            .collect()
    }

    fn get_node(&self, id: &NodeId) -> Option<Box<dyn NodeRenderer>> {
        self.nodes()
            .find(|node| &node.id == id)
            .map(|node| Box::new(ErasedNode::new(node.clone())) as Box<dyn NodeRenderer>)
    }

    fn get_edge(&self, id: &EdgeId) -> Option<Box<dyn EdgeRenderer>> {
        self.edges()
            .find(|edge| &edge.id == id)
            .map(|edge| Box::new(ErasedEdge::new(edge.clone())) as Box<dyn EdgeRenderer>)
    }
}

// Type-erased wrapper for nodes
struct ErasedNode<N> {
    node: Node<N>,
}

impl<N> ErasedNode<N> {
    fn new(node: Node<N>) -> Self {
        Self { node }
    }
}

impl<N> NodeRenderer for ErasedNode<N>
where
    N: Clone + serde::Serialize + 'static,
{
    fn get_id(&self) -> NodeId {
        self.node.id.clone()
    }

    fn get_position(&self) -> Position {
        self.node.position
    }

    fn get_size(&self) -> Size {
        self.node.size
    }

    fn get_data_json(&self) -> String {
        serde_json::to_string(&self.node.data).unwrap_or_else(|_| "{}".to_string())
    }

    fn is_selected(&self) -> bool {
        self.node.selected
    }
}

// Type-erased wrapper for edges
struct ErasedEdge<E> {
    edge: Edge<E>,
}

impl<E> ErasedEdge<E> {
    fn new(edge: Edge<E>) -> Self {
        Self { edge }
    }
}

impl<E> EdgeRenderer for ErasedEdge<E>
where
    E: Clone + serde::Serialize + 'static,
{
    fn get_id(&self) -> EdgeId {
        self.edge.id.clone()
    }

    fn get_source(&self) -> NodeId {
        self.edge.source.clone()
    }

    fn get_target(&self) -> NodeId {
        self.edge.target.clone()
    }

    fn get_data_json(&self) -> String {
        serde_json::to_string(&self.edge.data).unwrap_or_else(|_| "{}".to_string())
    }

    fn is_selected(&self) -> bool {
        self.edge.selected
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_color_parsing() {
        // Test hex colors
        assert_eq!(parse_color("#ff0000"), Some([1.0, 0.0, 0.0, 1.0]));
        assert_eq!(parse_color("#00ff00"), Some([0.0, 1.0, 0.0, 1.0]));
        assert_eq!(parse_color("#f00"), Some([1.0, 0.0, 0.0, 1.0]));

        // Test named colors
        assert_eq!(parse_color("black"), Some([0.0, 0.0, 0.0, 1.0]));
        assert_eq!(parse_color("white"), Some([1.0, 1.0, 1.0, 1.0]));

        // Test RGB colors
        assert_eq!(parse_color("rgb(255, 0, 0)"), Some([1.0, 0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_bezier_curve_calculation() {
        let start = Position::new(0.0, 0.0);
        let end = Position::new(100.0, 100.0);

        let points = calculate_bezier_curve(start, end, None, None, 10);
        assert_eq!(points.len(), 11); // 0 to 10 inclusive
        assert_eq!(points[0], start);
        assert_eq!(points[10], end);
    }

    #[test]
    fn test_renderer_type_name() {
        assert_eq!(RendererType::Canvas2D.name(), "Canvas2D");
        assert_eq!(RendererType::WebGL2.name(), "WebGL2");
        assert_eq!(RendererType::WebGPU.name(), "WebGPU");
    }

    #[test]
    fn test_default_styles() {
        let node_style = NodeStyle::default();
        assert_eq!(node_style.background_color, Some("#ffffff".to_string()));
        assert_eq!(node_style.border_width, Some(1.0));

        let edge_style = EdgeStyle::default();
        assert_eq!(edge_style.stroke_color, Some("#999999".to_string()));
        assert_eq!(edge_style.stroke_width, Some(2.0));
    }
}
