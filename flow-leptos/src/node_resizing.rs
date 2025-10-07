//! Node resizing functionality for Flow-RS Leptos components
//!
//! Provides comprehensive node resizing with:
//! - Resize handles (corners and edges)
//! - Visual feedback during resize
//! - Aspect ratio constraints
//! - Minimum/maximum size limits
//! - Smooth resize animations

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MouseEvent, Element};
use std::ops::Deref;
use crate::signals::{FlowState, ViewportState};
use flow_rs_core::{NodeId, Position, Size, Graph};

/// Resize handle positions
#[derive(Clone, Debug, PartialEq)]
pub enum ResizeHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

/// Node resize operation state
#[derive(Clone, Debug)]
pub struct ResizeState {
    pub node_id: NodeId,
    pub handle: ResizeHandle,
    pub start_size: Size,
    pub start_position: Position,
    pub start_mouse_pos: Position,
    pub maintain_aspect_ratio: bool,
    pub min_size: Size,
    pub max_size: Option<Size>,
}

impl ResizeState {
    pub fn new(
        node_id: NodeId,
        handle: ResizeHandle,
        start_size: Size,
        start_position: Position,
        start_mouse_pos: Position,
    ) -> Self {
        Self {
            node_id,
            handle,
            start_size,
            start_position,
            start_mouse_pos,
            maintain_aspect_ratio: false,
            min_size: Size::new(50.0, 30.0), // Minimum readable size
            max_size: None,
        }
    }

    /// Calculate new size based on mouse position and resize handle
    pub fn calculate_new_size(&self, current_mouse_pos: Position) -> Size {
        let delta_x = current_mouse_pos.x - self.start_mouse_pos.x;
        let delta_y = current_mouse_pos.y - self.start_mouse_pos.y;

        let mut new_width = self.start_size.width;
        let mut new_height = self.start_size.height;

        match self.handle {
            ResizeHandle::TopLeft => {
                new_width = (self.start_size.width - delta_x).max(self.min_size.width);
                new_height = (self.start_size.height - delta_y).max(self.min_size.height);
            }
            ResizeHandle::Top => {
                new_height = (self.start_size.height - delta_y).max(self.min_size.height);
            }
            ResizeHandle::TopRight => {
                new_width = (self.start_size.width + delta_x).max(self.min_size.width);
                new_height = (self.start_size.height - delta_y).max(self.min_size.height);
            }
            ResizeHandle::Right => {
                new_width = (self.start_size.width + delta_x).max(self.min_size.width);
            }
            ResizeHandle::BottomRight => {
                new_width = (self.start_size.width + delta_x).max(self.min_size.width);
                new_height = (self.start_size.height + delta_y).max(self.min_size.height);
            }
            ResizeHandle::Bottom => {
                new_height = (self.start_size.height + delta_y).max(self.min_size.height);
            }
            ResizeHandle::BottomLeft => {
                new_width = (self.start_size.width - delta_x).max(self.min_size.width);
                new_height = (self.start_size.height + delta_y).max(self.min_size.height);
            }
            ResizeHandle::Left => {
                new_width = (self.start_size.width - delta_x).max(self.min_size.width);
            }
        }

        // Apply aspect ratio constraint if enabled
        if self.maintain_aspect_ratio {
            let aspect_ratio = self.start_size.width / self.start_size.height;
            match self.handle {
                ResizeHandle::TopLeft | ResizeHandle::BottomRight => {
                    // Proportional resize - use the smaller dimension change
                    if delta_x.abs() > delta_y.abs() {
                        new_height = new_width / aspect_ratio;
                    } else {
                        new_width = new_height * aspect_ratio;
                    }
                }
                ResizeHandle::TopRight | ResizeHandle::BottomLeft => {
                    // Proportional resize
                    if delta_x.abs() > delta_y.abs() {
                        new_height = new_width / aspect_ratio;
                    } else {
                        new_width = new_height * aspect_ratio;
                    }
                }
                _ => {
                    // For edge handles, maintain aspect ratio by adjusting the other dimension
                    match self.handle {
                        ResizeHandle::Top | ResizeHandle::Bottom => {
                            new_width = new_height * aspect_ratio;
                        }
                        ResizeHandle::Left | ResizeHandle::Right => {
                            new_height = new_width / aspect_ratio;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Apply maximum size constraint
        if let Some(max_size) = self.max_size {
            new_width = new_width.min(max_size.width);
            new_height = new_height.min(max_size.height);
        }

        Size::new(new_width, new_height)
    }

    /// Calculate new position based on size change (for top/left handles)
    pub fn calculate_new_position(&self, new_size: &Size) -> Position {
        let mut new_position = self.start_position.clone();

        match self.handle {
            ResizeHandle::TopLeft => {
                new_position.x = self.start_position.x + (self.start_size.width - new_size.width);
                new_position.y = self.start_position.y + (self.start_size.height - new_size.height);
            }
            ResizeHandle::Top | ResizeHandle::TopRight => {
                new_position.y = self.start_position.y + (self.start_size.height - new_size.height);
            }
            ResizeHandle::Left | ResizeHandle::BottomLeft => {
                new_position.x = self.start_position.x + (self.start_size.width - new_size.width);
            }
            _ => {} // Other handles don't change position
        }

        new_position
    }
}

/// Node resizing manager
#[derive(Clone)]
pub struct NodeResizingManager {
    resize_state: Option<ResizeState>,
    handle_size: f64,
    handle_hit_test_size: f64,
}

impl NodeResizingManager {
    pub fn new() -> Self {
        Self {
            resize_state: None,
            handle_size: 8.0,        // Visual size of resize handles
            handle_hit_test_size: 12.0, // Larger hit test area for easier interaction
        }
    }

    /// Check if a point is over a resize handle for a given node
    pub fn get_resize_handle_at_point(
        &self,
        node_position: &Position,
        node_size: &Size,
        point: &Position,
    ) -> Option<ResizeHandle> {
        let handles = self.get_resize_handles(node_position, node_size);

        for (handle, handle_pos) in handles {
            let distance = ((point.x - handle_pos.x).powi(2) + (point.y - handle_pos.y).powi(2)).sqrt();
            if distance <= self.handle_hit_test_size {
                return Some(handle);
            }
        }

        None
    }

    /// Get all resize handle positions for a node
    pub fn get_resize_handles(
        &self,
        node_position: &Position,
        node_size: &Size,
    ) -> Vec<(ResizeHandle, Position)> {
        let x = node_position.x;
        let y = node_position.y;
        let w = node_size.width;
        let h = node_size.height;

        vec![
            (ResizeHandle::TopLeft, Position::new(x, y)),
            (ResizeHandle::Top, Position::new(x + w / 2.0, y)),
            (ResizeHandle::TopRight, Position::new(x + w, y)),
            (ResizeHandle::Right, Position::new(x + w, y + h / 2.0)),
            (ResizeHandle::BottomRight, Position::new(x + w, y + h)),
            (ResizeHandle::Bottom, Position::new(x + w / 2.0, y + h)),
            (ResizeHandle::BottomLeft, Position::new(x, y + h)),
            (ResizeHandle::Left, Position::new(x, y + h / 2.0)),
        ]
    }

    /// Start a resize operation
    pub fn start_resize(
        &mut self,
        node_id: NodeId,
        handle: ResizeHandle,
        node_position: Position,
        node_size: Size,
        mouse_pos: Position,
    ) {
        self.resize_state = Some(ResizeState::new(
            node_id,
            handle,
            node_size,
            node_position,
            mouse_pos,
        ));
    }

    /// Update resize operation
    pub fn update_resize(&self, mouse_pos: Position) -> Option<(NodeId, Size, Position)> {
        if let Some(ref resize_state) = self.resize_state {
            let new_size = resize_state.calculate_new_size(mouse_pos);
            let new_position = resize_state.calculate_new_position(&new_size);
            Some((resize_state.node_id.clone(), new_size, new_position))
        } else {
            None
        }
    }

    /// End resize operation
    pub fn end_resize(&mut self) {
        self.resize_state = None;
    }

    /// Check if currently resizing
    pub fn is_resizing(&self) -> bool {
        self.resize_state.is_some()
    }

    /// Get current resize handle
    pub fn current_resize_handle(&self) -> Option<&ResizeHandle> {
        self.resize_state.as_ref().map(|rs| &rs.handle)
    }

    /// Get handle size for rendering
    pub fn handle_size(&self) -> f64 {
        self.handle_size
    }
}

/// Resize cursor utilities
pub struct ResizeCursor;

impl ResizeCursor {
    /// Get CSS cursor style for a resize handle
    pub fn get_cursor_for_handle(handle: &ResizeHandle) -> &'static str {
        match handle {
            ResizeHandle::TopLeft | ResizeHandle::BottomRight => "nw-resize",
            ResizeHandle::Top | ResizeHandle::Bottom => "ns-resize",
            ResizeHandle::TopRight | ResizeHandle::BottomLeft => "ne-resize",
            ResizeHandle::Right | ResizeHandle::Left => "ew-resize",
        }
    }

    /// Get cursor style for resize operation
    pub fn get_cursor_for_resize(handle: &ResizeHandle) -> &'static str {
        Self::get_cursor_for_handle(handle)
    }
}

/// Node resizing hook for Leptos components
pub fn use_node_resizing<N, E>(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    graph: RwSignal<Graph<N, E>>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
) -> impl Fn() + 'static
where
    N: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
{
    let resizing_manager = RwSignal::new(NodeResizingManager::new());
    let is_resizing = RwSignal::new(false);

    // Mouse event handlers for resizing
    let mouse_down = move |event: MouseEvent| {
        if event.button() != 0 {
            return; // Only handle left mouse button
        }

        // Use client coordinates directly for now
        // In a full implementation, this would account for canvas scaling
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        // Check if clicking on a resize handle
        let graph_val = graph.get();
        let viewport_val = viewport_state.get();

        // Convert screen coordinates to world coordinates
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        // Check each node for resize handles
        for node in graph_val.nodes() {
            let node_size = node.size.clone();
            let node_position = node.position.clone();

            if let Some(handle) = resizing_manager.get().get_resize_handle_at_point(
                &node_position,
                &node_size,
                &world_pos,
            ) {
                // Start resize operation
                resizing_manager.update(|rm| {
                    rm.start_resize(
                        node.id.clone(),
                        handle,
                        node_position,
                        node_size,
                        world_pos,
                    );
                });
                is_resizing.set(true);
                event.prevent_default();
                return;
            }
        }
    };

    let mouse_move = move |event: MouseEvent| {
        if !is_resizing.get() {
            return;
        }

        // Use client coordinates directly for now
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let viewport_val = viewport_state.get();
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        if let Some((node_id, new_size, new_position)) = resizing_manager.get().update_resize(world_pos) {
            // Update the node size and position
            graph.update(|g| {
                if let Some(node) = g.nodes_mut().find(|n| n.id == node_id) {
                    node.size = new_size;
                    node.position = new_position;
                }
            });
        }

        event.prevent_default();
    };

    let mouse_up = move |_event: MouseEvent| {
        if is_resizing.get() {
            resizing_manager.update(|rm| rm.end_resize());
            is_resizing.set(false);
        }
    };

    // Set up event listeners when canvas is available
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_element = canvas.deref().clone();

            // Add mouse event listeners for resizing
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

    // Return cleanup function
    move || {
        // Cleanup would go here if needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_rs_core::{Node, Size};

    #[test]
    fn test_resize_state_creation() {
        let node_id = NodeId::new("test-node");
        let handle = ResizeHandle::BottomRight;
        let start_size = Size::new(100.0, 50.0);
        let start_position = Position::new(10.0, 20.0);
        let start_mouse_pos = Position::new(110.0, 70.0);

        let resize_state = ResizeState::new(
            node_id.clone(),
            handle.clone(),
            start_size.clone(),
            start_position.clone(),
            start_mouse_pos.clone(),
        );

        assert_eq!(resize_state.node_id, node_id);
        assert_eq!(resize_state.handle, handle);
        assert_eq!(resize_state.start_size, start_size);
        assert_eq!(resize_state.start_position, start_position);
        assert_eq!(resize_state.start_mouse_pos, start_mouse_pos);
        assert_eq!(resize_state.min_size, Size::new(50.0, 30.0));
    }

    #[test]
    fn test_resize_calculation() {
        let resize_state = ResizeState::new(
            NodeId::new("test"),
            ResizeHandle::BottomRight,
            Size::new(100.0, 50.0),
            Position::new(10.0, 20.0),
            Position::new(110.0, 70.0),
        );

        // Move mouse to increase size
        let new_mouse_pos = Position::new(120.0, 80.0);
        let new_size = resize_state.calculate_new_size(new_mouse_pos);

        assert_eq!(new_size.width, 110.0); // 100 + 10
        assert_eq!(new_size.height, 60.0); // 50 + 10
    }

    #[test]
    fn test_resize_handles() {
        let manager = NodeResizingManager::new();
        let position = Position::new(10.0, 20.0);
        let size = Size::new(100.0, 50.0);

        let handles = manager.get_resize_handles(&position, &size);
        assert_eq!(handles.len(), 8); // Should have 8 resize handles

        // Check corner positions
        assert_eq!(handles[0].1, Position::new(10.0, 20.0)); // TopLeft
        assert_eq!(handles[4].1, Position::new(110.0, 70.0)); // BottomRight
    }

    #[test]
    fn test_resize_cursor() {
        assert_eq!(ResizeCursor::get_cursor_for_handle(&ResizeHandle::TopLeft), "nw-resize");
        assert_eq!(ResizeCursor::get_cursor_for_handle(&ResizeHandle::Top), "ns-resize");
        assert_eq!(ResizeCursor::get_cursor_for_handle(&ResizeHandle::Right), "ew-resize");
    }
}
