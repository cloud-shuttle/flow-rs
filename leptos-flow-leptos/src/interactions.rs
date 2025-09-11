//! Interaction handlers for flow editor events
//!
//! This module provides high-level interaction handlers that bridge DOM events
//! to flow graph events and state changes.

use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, WheelEvent, KeyboardEvent};

use leptos_flow_core::{Graph, Node, Edge, Position, NodeId, EdgeId};
use crate::events::{FlowEvent, NodeEvent, EdgeEvent, KeyboardModifiers, MouseButton, DragTarget};
use crate::signals::{FlowState, ViewportState};

/// Interaction manager for handling flow editor events
pub struct InteractionManager {
    /// Current drag state
    pub drag_state: Option<DragState>,
    /// Last mouse position
    pub last_mouse_pos: Position,
    /// Mouse down state
    pub mouse_down: bool,
    /// Current button pressed
    pub mouse_button: Option<MouseButton>,
}

/// Drag operation state
#[derive(Debug, Clone)]
pub struct DragState {
    pub target: DragTarget,
    pub start_position: Position,
    pub current_position: Position,
    pub initial_node_positions: Vec<(NodeId, Position)>,
}

impl Default for InteractionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractionManager {
    /// Create a new interaction manager
    pub fn new() -> Self {
        Self {
            drag_state: None,
            last_mouse_pos: Position::zero(),
            mouse_down: false,
            mouse_button: None,
        }
    }

    /// Handle mouse down event
    pub fn handle_mouse_down<N: Clone, E: Clone>(
        &mut self,
        event: &MouseEvent,
        canvas_pos: Position,
        world_pos: Position,
        graph: &Graph<N, E>,
        viewport: &ViewportState,
    ) -> Vec<FlowEvent> {
        let mut events = Vec::new();

        let modifiers = KeyboardModifiers {
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        };

        let button = match event.button() {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Back,
            4 => MouseButton::Forward,
            _ => MouseButton::Left,
        };

        self.mouse_down = true;
        self.mouse_button = Some(button);
        self.last_mouse_pos = canvas_pos;

        // Hit test for nodes and edges
        if let Some(node_id) = self.hit_test_nodes(world_pos, graph) {
            // Node clicked
            let target = if modifiers.ctrl && !graph.get_node(&node_id).unwrap().selected {
                // Multi-selection - add to existing selection
                DragTarget::Nodes(vec![node_id.clone()])
            } else if modifiers.ctrl {
                // Already selected, start multi-node drag
                DragTarget::Nodes(self.get_selected_nodes(graph))
            } else {
                // Single node selection/drag
                DragTarget::Node(node_id.clone())
            };

            // Start drag
            self.drag_state = Some(DragState {
                target: target.clone(),
                start_position: world_pos,
                current_position: world_pos,
                initial_node_positions: self.capture_node_positions(&target, graph),
            });

            events.push(FlowEvent::DragStart {
                position: world_pos,
                target,
            });
        } else if let Some(_edge_id) = self.hit_test_edges(world_pos, graph) {
            // Edge clicked - for now just register the click
            events.push(FlowEvent::CanvasClick {
                position: world_pos,
                modifiers,
            });
        } else {
            // Canvas clicked - start canvas pan or selection
            if modifiers.shift {
                // Start selection rectangle
                self.drag_state = Some(DragState {
                    target: DragTarget::Selection,
                    start_position: world_pos,
                    current_position: world_pos,
                    initial_node_positions: Vec::new(),
                });
            } else {
                // Start canvas pan
                self.drag_state = Some(DragState {
                    target: DragTarget::Canvas,
                    start_position: canvas_pos,
                    current_position: canvas_pos,
                    initial_node_positions: Vec::new(),
                });
            }

            events.push(FlowEvent::CanvasClick {
                position: world_pos,
                modifiers,
            });
        }

        events
    }

    /// Handle mouse move event
    pub fn handle_mouse_move<N: Clone, E: Clone>(
        &mut self,
        event: &MouseEvent,
        canvas_pos: Position,
        world_pos: Position,
        graph: &mut Graph<N, E>,
        viewport: &mut ViewportState,
    ) -> Vec<FlowEvent> {
        let mut events = Vec::new();

        let delta = canvas_pos - self.last_mouse_pos;
        self.last_mouse_pos = canvas_pos;

        if let Some(ref mut drag_state) = self.drag_state {
            drag_state.current_position = match drag_state.target {
                DragTarget::Canvas => canvas_pos,
                _ => world_pos,
            };

            match &drag_state.target {
                DragTarget::Node(node_id) => {
                    // Move single node
                    if let Some(node) = graph.get_node_mut(node_id) {
                        let world_delta = delta / viewport.viewport.zoom;
                        node.position += world_delta;
                        node.dragging = true;
                    }
                }
                DragTarget::Nodes(node_ids) => {
                    // Move multiple nodes
                    let world_delta = delta / viewport.viewport.zoom;
                    for node_id in node_ids {
                        if let Some(node) = graph.get_node_mut(node_id) {
                            node.position += world_delta;
                            node.dragging = true;
                        }
                    }
                }
                DragTarget::Canvas => {
                    // Pan canvas
                    viewport.viewport.offset -= delta / viewport.viewport.zoom;
                    events.push(FlowEvent::ViewportChanged {
                        offset: viewport.viewport.offset,
                        zoom: viewport.viewport.zoom,
                    });
                }
                DragTarget::Selection => {
                    // Update selection rectangle (would need additional state)
                }
            }

            events.push(FlowEvent::DragUpdate {
                position: drag_state.current_position,
                delta,
            });
        }

        events
    }

    /// Handle mouse up event
    pub fn handle_mouse_up<N: Clone, E: Clone>(
        &mut self,
        event: &MouseEvent,
        canvas_pos: Position,
        world_pos: Position,
        graph: &mut Graph<N, E>,
    ) -> Vec<FlowEvent> {
        let mut events = Vec::new();

        self.mouse_down = false;
        self.mouse_button = None;

        if let Some(drag_state) = self.drag_state.take() {
            // Clear dragging state from nodes
            match &drag_state.target {
                DragTarget::Node(node_id) => {
                    if let Some(node) = graph.get_node_mut(node_id) {
                        node.dragging = false;
                    }
                }
                DragTarget::Nodes(node_ids) => {
                    for node_id in node_ids {
                        if let Some(node) = graph.get_node_mut(node_id) {
                            node.dragging = false;
                        }
                    }
                }
                _ => {}
            }

            let final_position = match drag_state.target {
                DragTarget::Canvas => canvas_pos,
                _ => world_pos,
            };

            events.push(FlowEvent::DragEnd {
                position: final_position,
                target: drag_state.target,
            });
        }

        events
    }

    /// Handle wheel event for zooming
    pub fn handle_wheel(
        &mut self,
        event: &WheelEvent,
        canvas_pos: Position,
        viewport: &mut ViewportState,
    ) -> Vec<FlowEvent> {
        let mut events = Vec::new();

        let zoom_factor = if event.delta_y() > 0.0 { 0.9 } else { 1.1 };
        let old_zoom = viewport.viewport.zoom;
        let new_zoom = (viewport.viewport.zoom * zoom_factor).clamp(0.1, 5.0);

        if (new_zoom - old_zoom).abs() > f64::EPSILON {
            // Zoom towards mouse position
            let zoom_point = canvas_pos / old_zoom + viewport.viewport.offset;
            let new_offset = zoom_point - canvas_pos / new_zoom;

            viewport.viewport.zoom = new_zoom;
            viewport.viewport.offset = new_offset;

            events.push(FlowEvent::ViewportChanged {
                offset: viewport.viewport.offset,
                zoom: viewport.viewport.zoom,
            });
        }

        events
    }

    /// Hit test for nodes at world position
    fn hit_test_nodes<N: Clone, E>(&self, world_pos: Position, graph: &Graph<N, E>) -> Option<NodeId> {
        // Test nodes in reverse z-order (front to back)
        let mut nodes: Vec<_> = graph.nodes().collect();
        nodes.sort_by(|a, b| {
            let z_a = a.z_index.unwrap_or(0);
            let z_b = b.z_index.unwrap_or(0);
            z_b.cmp(&z_a) // Reverse order
        });

        for node in nodes {
            if !node.hidden && node.contains_point(world_pos) {
                return Some(node.id.clone());
            }
        }
        None
    }

    /// Hit test for edges at world position
    fn hit_test_edges<N, E>(&self, _world_pos: Position, _graph: &Graph<N, E>) -> Option<EdgeId> {
        // TODO: Implement edge hit testing
        // This would require edge path calculation and distance testing
        None
    }

    /// Get all currently selected node IDs
    fn get_selected_nodes<N: Clone, E>(&self, graph: &Graph<N, E>) -> Vec<NodeId> {
        graph.nodes()
            .filter(|node| node.selected)
            .map(|node| node.id.clone())
            .collect()
    }

    /// Capture initial positions for nodes involved in drag
    fn capture_node_positions<N: Clone, E>(&self, target: &DragTarget, graph: &Graph<N, E>) -> Vec<(NodeId, Position)> {
        match target {
            DragTarget::Node(node_id) => {
                if let Some(node) = graph.get_node(node_id) {
                    vec![(node_id.clone(), node.position)]
                } else {
                    Vec::new()
                }
            }
            DragTarget::Nodes(node_ids) => {
                node_ids.iter()
                    .filter_map(|id| graph.get_node(id).map(|node| (id.clone(), node.position)))
                    .collect()
            }
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_flow_core::{Graph, prelude::{NodeBuilder, EdgeBuilder}};

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(100.0, 100.0)
            .size(80.0, 40.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(200.0, 150.0)
            .size(80.0, 40.0)
            .build();

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        graph
    }

    #[test]
    fn test_interaction_manager_creation() {
        let manager = InteractionManager::new();
        assert!(!manager.mouse_down);
        assert!(manager.drag_state.is_none());
        assert_eq!(manager.last_mouse_pos, Position::zero());
    }

    #[test]
    fn test_hit_test_nodes() {
        let manager = InteractionManager::new();
        let graph = create_test_graph();

        // Hit node1 (100,100 + 80x40)
        let hit = manager.hit_test_nodes(Position::new(120.0, 110.0), &graph);
        assert_eq!(hit, Some("node1".into()));

        // Hit node2 (200,150 + 80x40)
        let hit = manager.hit_test_nodes(Position::new(220.0, 160.0), &graph);
        assert_eq!(hit, Some("node2".into()));

        // Miss both nodes
        let hit = manager.hit_test_nodes(Position::new(50.0, 50.0), &graph);
        assert_eq!(hit, None);
    }

    #[test]
    fn test_get_selected_nodes() {
        let manager = InteractionManager::new();
        let mut graph = create_test_graph();

        // No nodes selected initially
        let selected = manager.get_selected_nodes(&graph);
        assert!(selected.is_empty());

        // Select node1
        graph.get_node_mut(&"node1".into()).unwrap().selected = true;
        let selected = manager.get_selected_nodes(&graph);
        assert_eq!(selected, vec!["node1".into()]);

        // Select both nodes
        graph.get_node_mut(&"node2".into()).unwrap().selected = true;
        let selected = manager.get_selected_nodes(&graph);
        assert_eq!(selected.len(), 2);
        assert!(selected.contains(&"node1".into()));
        assert!(selected.contains(&"node2".into()));
    }

    #[test]
    fn test_capture_node_positions() {
        let manager = InteractionManager::new();
        let graph = create_test_graph();

        // Single node
        let target = DragTarget::Node("node1".into());
        let positions = manager.capture_node_positions(&target, &graph);
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].0, "node1".into());
        assert_eq!(positions[0].1, Position::new(100.0, 100.0));

        // Multiple nodes
        let target = DragTarget::Nodes(vec!["node1".into(), "node2".into()]);
        let positions = manager.capture_node_positions(&target, &graph);
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn test_canvas_drag_state() {
        let target = DragTarget::Canvas;
        let drag_state = DragState {
            target: target.clone(),
            start_position: Position::new(10.0, 20.0),
            current_position: Position::new(15.0, 25.0),
            initial_node_positions: Vec::new(),
        };

        assert!(matches!(drag_state.target, DragTarget::Canvas));
        assert_eq!(drag_state.start_position, Position::new(10.0, 20.0));
        assert_eq!(drag_state.current_position, Position::new(15.0, 25.0));
        assert!(drag_state.initial_node_positions.is_empty());
    }
}
