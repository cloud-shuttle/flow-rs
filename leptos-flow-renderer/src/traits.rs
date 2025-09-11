//! Renderer trait definitions and shared types

use leptos_flow_core::{Graph, Node, Edge, Position, Viewport, Rect, NodeId};
use crate::error::Result;

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

    /// Render a complete graph
    fn render_graph<N, E>(&mut self, graph: &Graph<N, E>, viewport: &Viewport) -> Result<RenderStats>
    where
        N: Clone + 'static,
        E: Clone + 'static;

    /// Render a complete graph with selection state
    fn render_graph_with_selection<N, E>(
        &mut self,
        graph: &Graph<N, E>,
        viewport: &Viewport,
        selected_nodes: &[NodeId]
    ) -> Result<RenderStats>
    where
        N: Clone + 'static,
        E: Clone + 'static;

    /// Render nodes only
    fn render_nodes<N>(
        &mut self,
        nodes: &[Node<N>],
        viewport: &Viewport,
    ) -> Result<()>
    where
        N: Clone + 'static;

    /// Render edges only
    fn render_edges<E>(
        &mut self,
        edges: &[Edge<E>],
        viewport: &Viewport,
    ) -> Result<()>
    where
        E: Clone + 'static;

    /// Render selection indicators
    fn render_selection(
        &mut self,
        selected_bounds: &[Rect],
        style: &SelectionStyle,
    ) -> Result<()>;

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
    fn batch_edge<E>(&mut self, edge: &Edge<E>, source_pos: Position, target_pos: Position) -> Result<()>
    where
        E: Clone + 'static;

    /// Render and flush the current batch
    fn flush_batch(&mut self) -> Result<()>;
}

/// Utility functions for rendering
pub mod utils {
    use super::*;
    use leptos_flow_core::Position;

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

                return Some([
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    1.0,
                ]);
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
    pub fn is_visible(bounds: &leptos_flow_core::Rect, viewport: &Viewport) -> bool {
        viewport.intersects_rect(*bounds)
    }

    /// Calculate node bounds with padding
    pub fn node_bounds_with_padding(node: &Node<impl Clone>, padding: f64) -> leptos_flow_core::Rect {
        node.bounds().expand(padding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;

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
