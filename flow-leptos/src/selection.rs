//! Multi-selection and lasso selection for Flow-RS Leptos components
//!
//! Provides comprehensive selection management including:
//! - Rectangle (marquee) selection
//! - Lasso selection with free-form drawing
//! - Keyboard modifier support (Shift, Ctrl)
//! - Visual selection feedback
//! - Bulk operations on selected nodes

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MouseEvent, Element};
use std::collections::HashSet;
use std::ops::Deref;
use flow_rs_core::{NodeId, Position, Size, Graph, Rect};
use crate::signals::{FlowState, ViewportState};

/// Selection mode types
#[derive(Clone, Debug, PartialEq)]
pub enum SelectionMode {
    /// Replace current selection
    Replace,
    /// Add to current selection
    Add,
    /// Remove from current selection
    Remove,
    /// Toggle selection state
    Toggle,
}

/// Selection operation types
#[derive(Clone, Debug)]
pub enum SelectionOperation {
    /// Rectangle selection (marquee)
    Rectangle { start: Position, end: Position },
    /// Lasso selection (free-form path)
    Lasso { points: Vec<Position> },
    /// Click selection on specific node
    NodeClick { node_id: NodeId },
    /// Select all nodes
    SelectAll,
    /// Clear all selections
    ClearAll,
}

/// Selection state
#[derive(Clone, Debug)]
pub struct SelectionState {
    /// Currently selected node IDs
    pub selected_nodes: HashSet<NodeId>,
    /// Selection rectangle (for visual feedback)
    pub selection_rect: Option<Rect>,
    /// Lasso path points (for visual feedback)
    pub lasso_path: Vec<Position>,
    /// Current selection operation in progress
    pub active_operation: Option<SelectionOperation>,
    /// Selection mode
    pub mode: SelectionMode,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected_nodes: HashSet::new(),
            selection_rect: None,
            lasso_path: Vec::new(),
            active_operation: None,
            mode: SelectionMode::Replace,
        }
    }

    /// Check if a node is selected
    pub fn is_selected(&self, node_id: &NodeId) -> bool {
        self.selected_nodes.contains(node_id)
    }

    /// Get the number of selected nodes
    pub fn count(&self) -> usize {
        self.selected_nodes.len()
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.selected_nodes.clear();
        self.selection_rect = None;
        self.lasso_path.clear();
        self.active_operation = None;
    }

    /// Select all nodes
    pub fn select_all(&mut self, graph: &Graph<impl Clone, impl Clone>) {
        self.selected_nodes.clear();
        for node in graph.nodes() {
            self.selected_nodes.insert(node.id.clone());
        }
    }

    /// Add node to selection
    pub fn add_node(&mut self, node_id: NodeId) {
        self.selected_nodes.insert(node_id);
    }

    /// Remove node from selection
    pub fn remove_node(&mut self, node_id: &NodeId) {
        self.selected_nodes.remove(node_id);
    }

    /// Toggle node selection
    pub fn toggle_node(&mut self, node_id: NodeId) {
        if self.selected_nodes.contains(&node_id) {
            self.selected_nodes.remove(&node_id);
        } else {
            self.selected_nodes.insert(node_id);
        }
    }

    /// Apply rectangle selection
    pub fn apply_rectangle_selection(&mut self, rect: &Rect, graph: &Graph<impl Clone, impl Clone>) {
        let nodes_in_rect: HashSet<NodeId> = graph.nodes()
            .filter(|node| rect_utils::contains_point(&rect, &node.position))
            .map(|node| node.id.clone())
            .collect();

        match self.mode {
            SelectionMode::Replace => {
                self.selected_nodes = nodes_in_rect;
            }
            SelectionMode::Add => {
                self.selected_nodes.extend(nodes_in_rect);
            }
            SelectionMode::Remove => {
                for node_id in nodes_in_rect {
                    self.selected_nodes.remove(&node_id);
                }
            }
            SelectionMode::Toggle => {
                for node_id in nodes_in_rect {
                    if self.selected_nodes.contains(&node_id) {
                        self.selected_nodes.remove(&node_id);
                    } else {
                        self.selected_nodes.insert(node_id);
                    }
                }
            }
        }
    }

    /// Apply lasso selection
    pub fn apply_lasso_selection(&mut self, points: &[Position], graph: &Graph<impl Clone, impl Clone>) {
        if points.len() < 3 {
            return; // Need at least 3 points for a valid lasso
        }

        let nodes_in_lasso: HashSet<NodeId> = graph.nodes()
            .filter(|node| point_in_polygon(&node.position, points))
            .map(|node| node.id.clone())
            .collect();

        match self.mode {
            SelectionMode::Replace => {
                self.selected_nodes = nodes_in_lasso;
            }
            SelectionMode::Add => {
                self.selected_nodes.extend(nodes_in_lasso);
            }
            SelectionMode::Remove => {
                for node_id in nodes_in_lasso {
                    self.selected_nodes.remove(&node_id);
                }
            }
            SelectionMode::Toggle => {
                for node_id in nodes_in_lasso {
                    if self.selected_nodes.contains(&node_id) {
                        self.selected_nodes.remove(&node_id);
                    } else {
                        self.selected_nodes.insert(node_id);
                    }
                }
            }
        }
    }

    /// Set selection mode based on keyboard modifiers
    pub fn set_mode_from_modifiers(&mut self, shift_pressed: bool, ctrl_pressed: bool) {
        self.mode = if ctrl_pressed {
            SelectionMode::Toggle
        } else if shift_pressed {
            SelectionMode::Add
        } else {
            SelectionMode::Replace
        };
    }
}

/// Rectangle selection helper functions
pub mod rect_utils {
    use flow_rs_core::{Position, Rect};

    /// Check if a point is inside the rectangle
    pub fn contains_point(rect: &Rect, point: &Position) -> bool {
        point.x >= rect.x &&
        point.x <= rect.x + rect.width &&
        point.y >= rect.y &&
        point.y <= rect.y + rect.height
    }

    /// Create rectangle from two corner points
    pub fn from_points(p1: &Position, p2: &Position) -> Rect {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).abs();
        let height = (p1.y - p2.y).abs();

        Rect { x, y, width, height }
    }
}

/// Point-in-polygon test for lasso selection
fn point_in_polygon(point: &Position, polygon: &[Position]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let pi = &polygon[i];
        let pj = &polygon[j];

        if ((pi.y > point.y) != (pj.y > point.y)) &&
           (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// Selection manager
#[derive(Clone)]
pub struct SelectionManager {
    state: SelectionState,
    selection_threshold: f64, // Minimum distance to start selection
    is_selecting: bool,
}

impl SelectionManager {
    pub fn new() -> Self {
        Self {
            state: SelectionState::new(),
            selection_threshold: 5.0, // 5 pixels
            is_selecting: false,
        }
    }

    /// Get current selection state
    pub fn state(&self) -> &SelectionState {
        &self.state
    }

    /// Start rectangle selection
    pub fn start_rectangle_selection(&mut self, start_pos: Position) {
        self.state.active_operation = Some(SelectionOperation::Rectangle {
            start: start_pos.clone(),
            end: start_pos,
        });
        self.is_selecting = false; // Will be set to true after threshold
    }

    /// Update rectangle selection
    pub fn update_rectangle_selection(&mut self, current_pos: Position) {
        if let Some(SelectionOperation::Rectangle { start, .. }) = &mut self.state.active_operation {
            // Check if we've moved enough to start selection
            let distance = ((current_pos.x - start.x).powi(2) + (current_pos.y - start.y).powi(2)).sqrt();

            if !self.is_selecting && distance > self.selection_threshold {
                self.is_selecting = true;
            }

            if self.is_selecting {
                self.state.selection_rect = Some(rect_utils::from_points(start, &current_pos));

                if let Some(SelectionOperation::Rectangle { end, .. }) = &mut self.state.active_operation {
                    *end = current_pos;
                }
            }
        }
    }

    /// End rectangle selection
    pub fn end_rectangle_selection(&mut self, graph: &Graph<impl Clone, impl Clone>) -> bool {
        let mut selection_changed = false;

        if let Some(SelectionOperation::Rectangle { start, end }) = &self.state.active_operation {
            if self.is_selecting {
                let rect = rect_utils::from_points(start, end);
                self.state.apply_rectangle_selection(&rect, graph);
                selection_changed = true;
            }
        }

        self.state.active_operation = None;
        self.state.selection_rect = None;
        self.is_selecting = false;

        selection_changed
    }

    /// Start lasso selection
    pub fn start_lasso_selection(&mut self, start_pos: Position) {
        self.state.lasso_path = vec![start_pos];
        self.state.active_operation = Some(SelectionOperation::Lasso {
            points: vec![start_pos],
        });
        self.is_selecting = true;
    }

    /// Update lasso selection
    pub fn update_lasso_selection(&mut self, current_pos: Position) {
        if let Some(SelectionOperation::Lasso { points }) = &mut self.state.active_operation {
            // Add point if it's far enough from the last point
            if let Some(last_point) = points.last() {
                let distance = ((current_pos.x - last_point.x).powi(2) + (current_pos.y - last_point.y).powi(2)).sqrt();
                if distance > 3.0 { // Minimum distance between points
                    points.push(current_pos);
                    self.state.lasso_path = points.clone();
                }
            }
        }
    }

    /// End lasso selection
    pub fn end_lasso_selection(&mut self, graph: &Graph<impl Clone, impl Clone>) -> bool {
        let mut selection_changed = false;

        if let Some(SelectionOperation::Lasso { points }) = &self.state.active_operation {
            if points.len() >= 3 {
                let points_clone = points.clone();
                self.state.apply_lasso_selection(&points_clone, graph);
                selection_changed = true;
            }
        }

        self.state.active_operation = None;
        self.state.lasso_path.clear();
        self.is_selecting = false;

        selection_changed
    }

    /// Handle node click selection
    pub fn handle_node_click(&mut self, node_id: NodeId, shift_pressed: bool, ctrl_pressed: bool) {
        self.state.set_mode_from_modifiers(shift_pressed, ctrl_pressed);

        match self.state.mode {
            SelectionMode::Replace => {
                self.state.clear();
                self.state.add_node(node_id);
            }
            SelectionMode::Add => {
                self.state.add_node(node_id);
            }
            SelectionMode::Remove => {
                self.state.remove_node(&node_id);
            }
            SelectionMode::Toggle => {
                self.state.toggle_node(node_id);
            }
        }
    }

    /// Select all nodes
    pub fn select_all(&mut self, graph: &Graph<impl Clone, impl Clone>) {
        self.state.select_all(graph);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.state.clear();
    }

    /// Check if currently selecting
    pub fn is_selecting(&self) -> bool {
        self.is_selecting
    }

    /// Get selected node IDs
    pub fn selected_nodes(&self) -> &HashSet<NodeId> {
        &self.state.selected_nodes
    }
}

/// Selection renderer for visual feedback
pub struct SelectionRenderer;

impl SelectionRenderer {
    /// Render selection rectangle
    pub fn render_selection_rect(
        context: &web_sys::CanvasRenderingContext2d,
        rect: &Rect,
        viewport: &flow_rs_core::Viewport,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Convert world coordinates to screen coordinates
        // Convert world rectangle to screen coordinates
        let top_left = viewport.flow_to_screen(Position::new(rect.x, rect.y));
        let bottom_right = viewport.flow_to_screen(Position::new(rect.x + rect.width, rect.y + rect.height));
        let screen_rect = Rect {
            x: top_left.x,
            y: top_left.y,
            width: bottom_right.x - top_left.x,
            height: bottom_right.y - top_left.y,
        };

        context.set_stroke_style(&"#007bff".into());
        context.set_line_width(2.0);
        context.set_global_alpha(0.8);

        // Draw selection rectangle
        context.stroke_rect(
            screen_rect.x,
            screen_rect.y,
            screen_rect.width,
            screen_rect.height,
        );

        // Fill with semi-transparent color
        context.set_fill_style(&"#007bff".into());
        context.set_global_alpha(0.1);
        context.fill_rect(
            screen_rect.x,
            screen_rect.y,
            screen_rect.width,
            screen_rect.height,
        );

        context.set_global_alpha(1.0);
        Ok(())
    }

    /// Render lasso path
    pub fn render_lasso_path(
        context: &web_sys::CanvasRenderingContext2d,
        points: &[Position],
        viewport: &flow_rs_core::Viewport,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if points.len() < 2 {
            return Ok(());
        }

        context.set_stroke_style(&"#007bff".into());
        context.set_line_width(2.0);
        context.set_global_alpha(0.8);

        context.begin_path();

        // Convert first point to screen coordinates
        let screen_start = viewport.flow_to_screen(points[0]);
        context.move_to(screen_start.x, screen_start.y);

        // Draw lines to each subsequent point
        for point in points.iter().skip(1) {
            let screen_point = viewport.flow_to_screen(*point);
            context.line_to(screen_point.x, screen_point.y);
        }

        // Close the path back to start
        context.close_path();
        context.stroke();

        // Fill with semi-transparent color
        context.set_fill_style(&"#007bff".into());
        context.set_global_alpha(0.1);
        context.fill();

        context.set_global_alpha(1.0);
        Ok(())
    }

    /// Render selected node highlights
    pub fn render_selected_nodes(
        context: &web_sys::CanvasRenderingContext2d,
        selected_nodes: &HashSet<NodeId>,
        graph: &Graph<impl Clone, impl Clone>,
        viewport: &flow_rs_core::Viewport,
    ) -> Result<(), Box<dyn std::error::Error>> {
        context.set_stroke_style(&"#007bff".into());
        context.set_line_width(3.0);
        context.set_global_alpha(0.8);

        for node_id in selected_nodes {
            if let Some(node) = graph.nodes().find(|n| &n.id == node_id) {
                let screen_pos = viewport.flow_to_screen(node.position);
                // Calculate screen size based on zoom
                let screen_size = Size::new(
                    node.size.width * viewport.zoom,
                    node.size.height * viewport.zoom,
                );

                // Draw selection highlight
                context.stroke_rect(
                    screen_pos.x - screen_size.width / 2.0 - 4.0,
                    screen_pos.y - screen_size.height / 2.0 - 4.0,
                    screen_size.width + 8.0,
                    screen_size.height + 8.0,
                );
            }
        }

        context.set_global_alpha(1.0);
        Ok(())
    }
}

/// Selection hook for Leptos components
pub fn use_selection(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    graph: RwSignal<Graph<(), ()>>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
) -> (ReadSignal<HashSet<NodeId>>, impl Fn() + 'static)
{
    let selection_manager = RwSignal::new(SelectionManager::new());
    let selected_nodes = RwSignal::new(HashSet::new());

    // Sync selection state with flow state
    Effect::new(move |_| {
        let manager = selection_manager.get();
        selected_nodes.set(manager.selected_nodes().clone());

        // Update flow state with selected nodes
        flow_state.update(|state| {
            state.selected_nodes = manager.selected_nodes().iter().cloned().collect();
        });
    });

    // Mouse event handlers
    let mouse_down = move |event: MouseEvent| {
        if event.button() != 0 {
            return; // Only handle left mouse button
        }

        let shift_pressed = event.shift_key();
        let ctrl_pressed = event.ctrl_key();

        // Use client coordinates (simplified)
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let graph_val = graph.get();
        let viewport_val = viewport_state.get();

        // Convert to world coordinates
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        // Check if clicking on a node first
        let mut clicked_node = None;
        for node in graph_val.nodes() {
            let dx = world_pos.x - node.position.x;
            let dy = world_pos.y - node.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= 50.0 { // Assuming node radius, should be configurable
                clicked_node = Some(node.id.clone());
                break;
            }
        }

        if let Some(node_id) = clicked_node {
            // Node click - handle selection
            selection_manager.update(|manager| {
                manager.handle_node_click(node_id, shift_pressed, ctrl_pressed);
            });
        } else {
            // Start rectangle selection
            selection_manager.update(|manager| {
                manager.state.set_mode_from_modifiers(shift_pressed, ctrl_pressed);
                manager.start_rectangle_selection(world_pos);
            });
        }

        event.prevent_default();
    };

    let mouse_move = move |event: MouseEvent| {
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let viewport_val = viewport_state.get();
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        selection_manager.update(|manager| {
            manager.update_rectangle_selection(world_pos);
        });
    };

    let mouse_up = move |_event: MouseEvent| {
        let graph_val = graph.get();
        let mut selection_changed = false;
        selection_manager.update(|manager| {
            selection_changed = manager.end_rectangle_selection(&graph_val);
        });

        if selection_changed {
            // Trigger re-render if selection changed
            selected_nodes.update(|_| {});
        }
    };

    // Set up event listeners when canvas is available
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_element = canvas.deref().clone();

            // Add mouse event listeners for selection
            let _ = canvas_element.add_event_listener_with_callback(
                "mousedown",
                &Closure::wrap(Box::new(mouse_down.clone()) as Box<dyn FnMut(MouseEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );

            let _ = canvas_element.add_event_listener_with_callback(
                "mousemove",
                &Closure::wrap(Box::new(mouse_move.clone()) as Box<dyn FnMut(MouseEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );

            let _ = canvas_element.add_event_listener_with_callback(
                "mouseup",
                &Closure::wrap(Box::new(mouse_up.clone()) as Box<dyn FnMut(MouseEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );
        }
    });

    // Return selected nodes signal and cleanup function
    (
        selected_nodes.read_only(),
        move || {
            // Cleanup would go here if needed
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_rs_core::{Node, Size};

    #[test]
    fn test_selection_state() {
        let mut state = SelectionState::new();
        assert!(state.selected_nodes.is_empty());
        assert_eq!(state.count(), 0);

        let node_id = NodeId::new("test-node");
        state.add_node(node_id.clone());
        assert!(state.is_selected(&node_id));
        assert_eq!(state.count(), 1);

        state.remove_node(&node_id);
        assert!(!state.is_selected(&node_id));
        assert_eq!(state.count(), 0);
    }

    #[test]
    fn test_rectangle_contains_point() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);

        assert!(rect.contains_point(&Position::new(50.0, 40.0))); // Inside
        assert!(!rect.contains_point(&Position::new(5.0, 40.0))); // Left of rect
        assert!(!rect.contains_point(&Position::new(50.0, 15.0))); // Above rect
    }

    #[test]
    fn test_rect_from_points() {
        let p1 = Position::new(10.0, 20.0);
        let p2 = Position::new(110.0, 70.0);

        let rect = Rect::from_points(&p1, &p2);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    fn test_selection_manager() {
        let mut manager = SelectionManager::new();
        assert!(!manager.is_selecting());
        assert_eq!(manager.selected_nodes().len(), 0);

        let mut graph = Graph::new();
        let node = Node::new("node1".to_string(), Position::new(50.0, 50.0), "Test".to_string());
        graph.add_node(node).unwrap();

        // Start rectangle selection
        manager.start_rectangle_selection(Position::new(10.0, 10.0));
        manager.update_rectangle_selection(Position::new(100.0, 100.0));

        // End selection
        let changed = manager.end_rectangle_selection(&graph);
        assert!(changed);
        assert_eq!(manager.selected_nodes().len(), 1);
    }

    #[test]
    fn test_point_in_polygon() {
        // Simple triangle
        let polygon = vec![
            Position::new(0.0, 0.0),
            Position::new(10.0, 0.0),
            Position::new(5.0, 10.0),
        ];

        // Point inside triangle
        assert!(point_in_polygon(&Position::new(5.0, 3.0), &polygon));

        // Point outside triangle
        assert!(!point_in_polygon(&Position::new(15.0, 5.0), &polygon));

        // Point on edge (should be considered inside for selection purposes)
        assert!(point_in_polygon(&Position::new(5.0, 0.0), &polygon));
    }
}
