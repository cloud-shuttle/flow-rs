//! Minimap component for Leptos Flow
//!
//! Provides a minimap overview of the flow graph with navigation capabilities,
//! viewport synchronization, and configurable rendering options.

use flow_core::{Edge, EdgeId, Graph, Node, NodeId, Position, Viewport};

/// Configuration for the minimap component
#[derive(Debug, Clone)]
pub struct MinimapConfig {
    pub width: f64,
    pub height: f64,
    pub background_color: String,
    pub node_color: String,
    pub edge_color: String,
    pub viewport_color: String,
    pub show_grid: bool,
    pub grid_size: f64,
}

impl Default for MinimapConfig {
    fn default() -> Self {
        Self {
            width: 200.0,
            height: 150.0,
            background_color: "rgba(240, 240, 240, 0.8)".to_string(),
            node_color: "#4A90E2".to_string(),
            edge_color: "#666666".to_string(),
            viewport_color: "rgba(255, 0, 0, 0.3)".to_string(),
            show_grid: false,
            grid_size: 20.0,
        }
    }
}

/// Bounds of the graph content
#[derive(Debug, Clone)]
pub struct GraphBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl GraphBounds {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }
}

/// Rectangle representing the viewport in minimap coordinates
#[derive(Debug, Clone)]
pub struct ViewportRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl ViewportRectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Rendered node information for minimap
#[derive(Debug, Clone)]
pub struct RenderedNode {
    pub id: NodeId,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Rendered edge information for minimap
#[derive(Debug, Clone)]
pub struct RenderedEdge {
    pub id: EdgeId,
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

/// Minimap renderer for rendering the minimap
///
/// Provides comprehensive minimap functionality including:
/// - Graph overview rendering with nodes and edges
/// - Viewport synchronization and navigation
/// - Coordinate conversion between world and minimap space
/// - Performance optimization for large graphs
pub struct MinimapRenderer {
    config: MinimapConfig,
    current_viewport: Viewport,
    rendered_nodes: Vec<RenderedNode>,
    rendered_edges: Vec<RenderedEdge>,
    rendered_viewport: Viewport,
    viewport_rectangle: ViewportRectangle,
}

impl MinimapRenderer {
    pub fn new(config: MinimapConfig) -> Self {
        Self {
            config,
            current_viewport: Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0),
            rendered_nodes: Vec::new(),
            rendered_edges: Vec::new(),
            rendered_viewport: Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0),
            viewport_rectangle: ViewportRectangle::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Render the minimap with the given graph and viewport
    pub fn render_minimap<N, E>(
        &mut self,
        graph: &Graph<N, E>,
        viewport: &Viewport,
    ) -> Result<(), String>
    where
        N: Clone,
        E: Clone,
    {
        // Update current viewport
        self.current_viewport = *viewport;
        self.rendered_viewport = *viewport;

        // Clear previous render data
        self.rendered_nodes.clear();
        self.rendered_edges.clear();

        // Calculate graph bounds
        let bounds = self.calculate_graph_bounds(graph);

        // Calculate scale to fit graph in minimap
        let scale = self.calculate_scale_from_bounds(&bounds);

        // Render nodes
        for node in graph.nodes() {
            if !node.hidden {
                let rendered_node = self.render_node(node, &bounds, scale);
                self.rendered_nodes.push(rendered_node);
            }
        }

        // Render edges
        for edge in graph.edges() {
            if !edge.hidden {
                if let (Some(source_node), Some(target_node)) =
                    (graph.get_node(&edge.source), graph.get_node(&edge.target))
                {
                    let rendered_edge =
                        self.render_edge(edge, source_node, target_node, &bounds, scale);
                    self.rendered_edges.push(rendered_edge);
                }
            }
        }

        // Calculate viewport rectangle
        self.viewport_rectangle = self.calculate_viewport_rectangle(&bounds, scale);

        Ok(())
    }

    /// Update the viewport
    pub fn update_viewport(&mut self, viewport: &Viewport) {
        self.current_viewport = *viewport;
    }

    /// Get the current viewport
    pub fn get_current_viewport(&self) -> Viewport {
        self.current_viewport
    }

    /// Get the rendered viewport
    pub fn get_rendered_viewport(&self) -> Viewport {
        self.rendered_viewport
    }

    /// Get rendered nodes
    pub fn get_rendered_nodes(&self) -> &[RenderedNode] {
        &self.rendered_nodes
    }

    /// Get rendered edges
    pub fn get_rendered_edges(&self) -> &[RenderedEdge] {
        &self.rendered_edges
    }

    /// Get the viewport rectangle
    pub fn get_viewport_rectangle(&self) -> ViewportRectangle {
        self.viewport_rectangle.clone()
    }

    /// Get the configuration
    pub fn get_config(&self) -> &MinimapConfig {
        &self.config
    }

    /// Resize the minimap
    pub fn resize(&mut self, new_config: MinimapConfig) {
        self.config = new_config;
    }

    /// Calculate bounds of the graph
    pub fn calculate_graph_bounds<N, E>(&self, graph: &Graph<N, E>) -> GraphBounds
    where
        N: Clone,
        E: Clone,
    {
        if graph.node_count() == 0 {
            return GraphBounds::new(0.0, 0.0, 100.0, 100.0);
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for node in graph.nodes() {
            if !node.hidden {
                min_x = min_x.min(node.position.x);
                min_y = min_y.min(node.position.y);
                max_x = max_x.max(node.position.x + node.size.width);
                max_y = max_y.max(node.position.y + node.size.height);
            }
        }

        // Add some padding
        let padding = 50.0;
        GraphBounds::new(
            min_x - padding,
            min_y - padding,
            max_x + padding,
            max_y + padding,
        )
    }

    /// Calculate scale to fit graph in minimap
    pub fn calculate_scale<N, E>(&self, graph: &Graph<N, E>) -> f64
    where
        N: Clone,
        E: Clone,
    {
        let bounds = self.calculate_graph_bounds(graph);
        self.calculate_scale_from_bounds(&bounds)
    }

    /// Calculate scale from bounds
    fn calculate_scale_from_bounds(&self, bounds: &GraphBounds) -> f64 {
        let scale_x = self.config.width / bounds.width();
        let scale_y = self.config.height / bounds.height();
        scale_x.min(scale_y).min(1.0) // Don't scale up beyond 1:1
    }

    /// Render a node for the minimap
    fn render_node(
        &self,
        node: &Node<impl Clone>,
        bounds: &GraphBounds,
        scale: f64,
    ) -> RenderedNode {
        let x = (node.position.x - bounds.min_x) * scale;
        let y = (node.position.y - bounds.min_y) * scale;
        let width = node.size.width * scale;
        let height = node.size.height * scale;

        RenderedNode {
            id: node.id.clone(),
            x,
            y,
            width,
            height,
        }
    }

    /// Render an edge for the minimap
    fn render_edge(
        &self,
        edge: &Edge<impl Clone>,
        source_node: &Node<impl Clone>,
        target_node: &Node<impl Clone>,
        bounds: &GraphBounds,
        scale: f64,
    ) -> RenderedEdge {
        let start_x = (source_node.position.x + source_node.size.width - bounds.min_x) * scale;
        let start_y =
            (source_node.position.y + source_node.size.height / 2.0 - bounds.min_y) * scale;
        let end_x = (target_node.position.x - bounds.min_x) * scale;
        let end_y = (target_node.position.y + target_node.size.height / 2.0 - bounds.min_y) * scale;

        RenderedEdge {
            id: edge.id.clone(),
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    /// Calculate viewport rectangle in minimap coordinates
    fn calculate_viewport_rectangle(&self, bounds: &GraphBounds, scale: f64) -> ViewportRectangle {
        let viewport_x = (self.current_viewport.x - bounds.min_x) * scale;
        let viewport_y = (self.current_viewport.y - bounds.min_y) * scale;
        let viewport_width = self.config.width / self.current_viewport.zoom;
        let viewport_height = self.config.height / self.current_viewport.zoom;

        ViewportRectangle::new(viewport_x, viewport_y, viewport_width, viewport_height)
    }

    /// Convert world coordinates to minimap coordinates
    pub fn world_to_minimap<N, E>(&self, world_pos: Position, graph: &Graph<N, E>) -> Position
    where
        N: Clone,
        E: Clone,
    {
        let bounds = self.calculate_graph_bounds(graph);
        let scale = self.calculate_scale_from_bounds(&bounds);

        Position::new(
            (world_pos.x - bounds.min_x) * scale,
            (world_pos.y - bounds.min_y) * scale,
        )
    }

    /// Convert minimap coordinates to world coordinates
    pub fn minimap_to_world<N, E>(&self, minimap_pos: Position, graph: &Graph<N, E>) -> Position
    where
        N: Clone,
        E: Clone,
    {
        let bounds = self.calculate_graph_bounds(graph);
        let scale = self.calculate_scale_from_bounds(&bounds);

        Position::new(
            minimap_pos.x / scale + bounds.min_x,
            minimap_pos.y / scale + bounds.min_y,
        )
    }
}

/// Minimap interaction handler
///
/// Handles user interactions with the minimap including:
/// - Click navigation to different parts of the graph
/// - Viewport updates based on user input
/// - Coordinate mapping for interaction events
pub struct MinimapInteraction;

impl MinimapInteraction {
    pub fn new() -> Self {
        Self
    }

    /// Handle click on minimap
    pub fn handle_click(
        &self,
        renderer: &MinimapRenderer,
        click_pos: Position,
    ) -> Result<Viewport, String> {
        // For now, just return a simple navigation result
        // In a real implementation, this would calculate the new viewport position
        let new_position = Position::new(click_pos.x * 2.0, click_pos.y * 2.0);
        Ok(Viewport::new(
            new_position.x,
            new_position.y,
            800.0,
            600.0,
            1.0,
        ))
    }
}

impl Default for MinimapInteraction {
    fn default() -> Self {
        Self::new()
    }
}
