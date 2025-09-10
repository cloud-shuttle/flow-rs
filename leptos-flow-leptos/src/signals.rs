//! Reactive signals and state management for Leptos Flow

use leptos::*;
use serde::{Deserialize, Serialize};

use leptos_flow_core::{Graph, Node, Edge, NodeId, EdgeId, Position, Viewport};
use leptos_flow_renderer::{Renderer, RenderStats};

/// Main application state for the flow editor
#[derive(Clone, Debug)]
pub struct FlowState {
    pub renderer: Option<Box<dyn Renderer>>,
    pub render_stats: Option<RenderStats>,
    pub selected_nodes: Vec<NodeId>,
    pub selected_edges: Vec<EdgeId>,
    pub is_dragging: bool,
    pub drag_start: Option<Position>,
    pub last_mouse_pos: Option<Position>,
}

impl Default for FlowState {
    fn default() -> Self {
        Self {
            renderer: None,
            render_stats: None,
            selected_nodes: Vec::new(),
            selected_edges: Vec::new(),
            is_dragging: false,
            drag_start: None,
            last_mouse_pos: None,
        }
    }
}

impl FlowState {
    /// Create a new flow state
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
        self.selected_edges.clear();
    }

    /// Select a node (replacing current selection)
    pub fn select_node(&mut self, node_id: NodeId) {
        self.clear_selection();
        self.selected_nodes.push(node_id);
    }

    /// Add node to selection
    pub fn add_node_to_selection(&mut self, node_id: NodeId) {
        if !self.selected_nodes.contains(&node_id) {
            self.selected_nodes.push(node_id);
        }
    }

    /// Remove node from selection
    pub fn remove_node_from_selection(&mut self, node_id: &NodeId) {
        self.selected_nodes.retain(|id| id != node_id);
    }

    /// Check if node is selected
    pub fn is_node_selected(&self, node_id: &NodeId) -> bool {
        self.selected_nodes.contains(node_id)
    }

    /// Select an edge (replacing current selection)
    pub fn select_edge(&mut self, edge_id: EdgeId) {
        self.clear_selection();
        self.selected_edges.push(edge_id);
    }

    /// Start dragging operation
    pub fn start_drag(&mut self, mouse_pos: Position) {
        self.is_dragging = true;
        self.drag_start = Some(mouse_pos);
        self.last_mouse_pos = Some(mouse_pos);
    }

    /// Update drag position
    pub fn update_drag(&mut self, mouse_pos: Position) {
        if self.is_dragging {
            self.last_mouse_pos = Some(mouse_pos);
        }
    }

    /// End dragging operation
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.drag_start = None;
        self.last_mouse_pos = None;
    }

    /// Get drag delta since start
    pub fn drag_delta(&self) -> Option<Position> {
        if let (Some(start), Some(current)) = (self.drag_start, self.last_mouse_pos) {
            Some(Position::new(current.x - start.x, current.y - start.y))
        } else {
            None
        }
    }
}

/// Viewport state for pan and zoom operations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViewportState {
    pub viewport: Viewport,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub zoom_speed: f64,
    pub pan_speed: f64,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            viewport: Viewport::new(Position::new(0.0, 0.0), 1.0),
            min_zoom: 0.1,
            max_zoom: 5.0,
            zoom_speed: 0.1,
            pan_speed: 1.0,
        }
    }
}

impl ViewportState {
    /// Create a new viewport state
    pub fn new() -> Self {
        Self::default()
    }

    /// Pan the viewport by the given offset
    pub fn pan(&mut self, delta: Position) {
        let new_offset = Position::new(
            self.viewport.offset.x + delta.x * self.pan_speed,
            self.viewport.offset.y + delta.y * self.pan_speed,
        );
        self.viewport.offset = new_offset;
    }

    /// Zoom the viewport by the given factor at the given point
    pub fn zoom_at(&mut self, factor: f64, point: Position) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * factor).clamp(self.min_zoom, self.max_zoom);
        
        if (new_zoom - old_zoom).abs() < f64::EPSILON {
            return; // No zoom change
        }

        // Adjust offset to zoom at the given point
        let zoom_ratio = new_zoom / old_zoom - 1.0;
        let offset_delta = Position::new(
            -point.x * zoom_ratio / old_zoom,
            -point.y * zoom_ratio / old_zoom,
        );

        self.viewport.zoom = new_zoom;
        self.viewport.offset.x += offset_delta.x;
        self.viewport.offset.y += offset_delta.y;
    }

    /// Zoom in at the center
    pub fn zoom_in(&mut self, center: Position) {
        self.zoom_at(1.0 + self.zoom_speed, center);
    }

    /// Zoom out at the center
    pub fn zoom_out(&mut self, center: Position) {
        self.zoom_at(1.0 - self.zoom_speed, center);
    }

    /// Reset viewport to default
    pub fn reset(&mut self) {
        self.viewport = Viewport::new(Position::new(0.0, 0.0), 1.0);
    }

    /// Fit the given bounds in the viewport
    pub fn fit_bounds(&mut self, bounds: leptos_flow_core::Rect, canvas_size: (f64, f64)) {
        if bounds.width <= 0.0 || bounds.height <= 0.0 {
            return;
        }

        let padding = 50.0; // Padding around the content
        let available_width = canvas_size.0 - 2.0 * padding;
        let available_height = canvas_size.1 - 2.0 * padding;

        // Calculate zoom to fit
        let zoom_x = available_width / bounds.width;
        let zoom_y = available_height / bounds.height;
        let zoom = zoom_x.min(zoom_y).clamp(self.min_zoom, self.max_zoom);

        // Center the content
        let center_x = bounds.x + bounds.width / 2.0;
        let center_y = bounds.y + bounds.height / 2.0;
        let canvas_center_x = canvas_size.0 / 2.0;
        let canvas_center_y = canvas_size.1 / 2.0;

        self.viewport.zoom = zoom;
        self.viewport.offset = Position::new(
            canvas_center_x / zoom - center_x,
            canvas_center_y / zoom - center_y,
        );
    }
}

/// Create a reactive graph signal with helper methods
pub fn create_graph_signal<N, E>() -> (ReadSignal<Graph<N, E>>, WriteSignal<Graph<N, E>>)
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    create_signal(Graph::new())
}

/// Create a reactive flow state signal
pub fn create_flow_state() -> (ReadSignal<FlowState>, WriteSignal<FlowState>) {
    create_signal(FlowState::new())
}

/// Create a reactive viewport state signal
pub fn create_viewport_state() -> (ReadSignal<ViewportState>, WriteSignal<ViewportState>) {
    create_signal(ViewportState::new())
}

/// Graph operations helper
pub struct GraphOperations<N, E>
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    graph_signal: RwSignal<Graph<N, E>>,
}

impl<N, E> GraphOperations<N, E>
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    pub fn new(graph_signal: RwSignal<Graph<N, E>>) -> Self {
        Self { graph_signal }
    }

    /// Add a node to the graph
    pub fn add_node(&self, node: Node<N>) -> Result<(), leptos_flow_core::FlowError> {
        self.graph_signal.update(|graph| {
            let _ = graph.add_node(node);
        });
        Ok(())
    }

    /// Remove a node from the graph
    pub fn remove_node(&self, node_id: &NodeId) {
        self.graph_signal.update(|graph| {
            graph.remove_node(node_id);
        });
    }

    /// Add an edge to the graph
    pub fn add_edge(&self, edge: Edge<E>) -> Result<(), leptos_flow_core::FlowError> {
        self.graph_signal.update(|graph| {
            let _ = graph.add_edge(edge);
        });
        Ok(())
    }

    /// Remove an edge from the graph
    pub fn remove_edge(&self, edge_id: &EdgeId) {
        self.graph_signal.update(|graph| {
            graph.remove_edge(edge_id);
        });
    }

    /// Move a node to a new position
    pub fn move_node(&self, node_id: &NodeId, new_position: Position) {
        self.graph_signal.update(|graph| {
            if let Some(node) = graph.node_mut(node_id) {
                node.set_position(new_position);
            }
        });
    }

    /// Get a read-only view of the graph
    pub fn read_graph(&self) -> ReadSignal<Graph<N, E>> {
        self.graph_signal.read_only()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_flow_core::{Size, NodeBuilder};

    #[test]
    fn test_flow_state_selection() {
        let mut state = FlowState::new();
        let node_id = NodeId::new("test-node");
        
        assert!(!state.is_node_selected(&node_id));
        
        state.select_node(node_id.clone());
        assert!(state.is_node_selected(&node_id));
        assert_eq!(state.selected_nodes.len(), 1);
        
        state.clear_selection();
        assert!(!state.is_node_selected(&node_id));
        assert_eq!(state.selected_nodes.len(), 0);
    }

    #[test]
    fn test_viewport_operations() {
        let mut viewport = ViewportState::new();
        
        // Test pan
        viewport.pan(Position::new(10.0, 20.0));
        assert_eq!(viewport.viewport.offset.x, 10.0);
        assert_eq!(viewport.viewport.offset.y, 20.0);
        
        // Test zoom
        let center = Position::new(100.0, 100.0);
        let initial_zoom = viewport.viewport.zoom;
        viewport.zoom_in(center);
        assert!(viewport.viewport.zoom > initial_zoom);
        
        // Test reset
        viewport.reset();
        assert_eq!(viewport.viewport.offset.x, 0.0);
        assert_eq!(viewport.viewport.offset.y, 0.0);
        assert_eq!(viewport.viewport.zoom, 1.0);
    }

    #[test]
    fn test_drag_operations() {
        let mut state = FlowState::new();
        let start_pos = Position::new(10.0, 20.0);
        let current_pos = Position::new(30.0, 50.0);
        
        assert!(!state.is_dragging);
        
        state.start_drag(start_pos);
        assert!(state.is_dragging);
        assert_eq!(state.drag_start, Some(start_pos));
        
        state.update_drag(current_pos);
        let delta = state.drag_delta().unwrap();
        assert_eq!(delta.x, 20.0);
        assert_eq!(delta.y, 30.0);
        
        state.end_drag();
        assert!(!state.is_dragging);
        assert!(state.drag_start.is_none());
    }
}