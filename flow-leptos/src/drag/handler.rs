//! Core drag handler implementation
//!
//! Contains the main DragHandler struct and core drag lifecycle methods.

use crate::signals::{FlowState, ViewportState};
use flow_rs_core::{Graph, GroupManager, NodeId, Position, Rect};
use std::collections::HashSet;

/// Drag configuration and constraints
#[derive(Debug, Clone)]
pub struct DragConfig {
    /// Enable snap to grid functionality
    pub snap_to_grid: bool,
    /// Grid size for snapping
    pub grid_size: f64,
    /// Minimum drag distance to start dragging
    pub drag_threshold: f64,
    /// Enable bounds checking
    pub enforce_bounds: bool,
    /// Canvas bounds (optional)
    pub canvas_bounds: Option<Rect>,
    /// Drag constraint (axis locking, etc.)
    pub constraint: Option<DragConstraint>,
}

/// Drag constraint types
#[derive(Debug, Clone)]
pub enum DragConstraint {
    HorizontalOnly,
    VerticalOnly,
    DiagonalOnly,
    Custom(fn(Position) -> Position),
}

/// Drag handle types for precise manipulation
#[derive(Debug, Clone, Copy)]
pub enum DragHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for DragConfig {
    fn default() -> Self {
        Self {
            snap_to_grid: false,
            grid_size: 20.0,
            drag_threshold: 3.0,
            enforce_bounds: false,
            canvas_bounds: None,
            constraint: None,
        }
    }
}

/// Drag operation result
#[derive(Debug, Clone)]
pub enum DragResult {
    /// Drag operation started
    Started(Position),
    /// Drag operation updated with new position
    Updated(Position),
    /// Drag operation completed
    Completed(Position),
    /// Drag operation cancelled
    Cancelled,
}


/// Main drag handler for flow editors
#[derive(Clone)]
pub struct DragHandler {
    config: DragConfig,
}

impl DragHandler {
    /// Create a new drag handler with default config
    pub fn new() -> Self {
        Self {
            config: DragConfig::default(),
        }
    }

    /// Create a new drag handler with custom config
    pub fn with_config(config: DragConfig) -> Self {
        Self { config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &DragConfig {
        &self.config
    }

    /// Handle mouse down event to potentially start drag
    pub fn handle_mouse_down<N, E>(
        &self,
        event: &web_sys::MouseEvent,
        graph: &Graph<N, E>,
        flow_state: &mut FlowState,
        viewport_state: &ViewportState,
        group_manager: Option<&GroupManager>,
    ) -> Option<DragResult>
    where
        N: Clone,
        E: Clone,
    {
        use crate::drag::calculations::screen_to_world_position;

        let mouse_pos = screen_to_world_position(
            Position::new(event.client_x() as f64, event.client_y() as f64),
            viewport_state,
        );

        // Find node under cursor
        if let Some(node_id) = crate::drag::events::find_node_at_position(&mouse_pos, graph) {
            // If node is not selected, select it first
            if !flow_state.is_node_selected(&node_id) {
                // If we have a group manager, check if this node is part of a group
                if let Some(gm) = group_manager {
                    if event.ctrl_key() {
                        // Ctrl+Click: select just the node
                        flow_state.select_node(node_id);
                    } else {
                        // Regular click: select node with its group if applicable
                        flow_state.select_node_with_group(gm, node_id, true);
                    }
                } else {
                    flow_state.select_node(node_id);
                }
            }

            // Start drag operation
            flow_state.start_drag(mouse_pos);

            Some(DragResult::Started(mouse_pos))
        } else {
            None
        }
    }

    /// Handle mouse move event during drag
    pub fn handle_mouse_move<N, E>(
        &self,
        event: &web_sys::MouseEvent,
        graph: &mut Graph<N, E>,
        flow_state: &mut FlowState,
        viewport_state: &ViewportState,
        group_manager: Option<&mut GroupManager>,
    ) -> Option<DragResult>
    where
        N: Clone,
        E: Clone,
    {
        use crate::drag::calculations::screen_to_world_position;

        let mouse_pos = screen_to_world_position(
            Position::new(event.client_x() as f64, event.client_y() as f64),
            viewport_state,
        );

        // Check if we've moved enough to start dragging
        if let Some(start_pos) = flow_state.drag_start {
            let distance = crate::drag::calculations::calculate_distance(start_pos, mouse_pos);
            if distance < self.config.drag_threshold {
                return None; // Not enough movement yet
            }
        }

        // Update drag state
        flow_state.update_drag(mouse_pos);

        // Apply drag to selected nodes and groups
        if let Some(delta) = flow_state.drag_delta() {
            self.apply_drag_to_nodes_and_groups(graph, flow_state, group_manager, delta);
        }

        Some(DragResult::Updated(mouse_pos))
    }

    /// Handle mouse up event to end drag
    pub fn handle_mouse_up(
        &self,
        _event: &web_sys::MouseEvent,
        flow_state: &mut crate::signals::FlowState,
    ) -> Option<DragResult> {
        // Get the final position before ending drag
        let final_pos = flow_state.last_mouse_pos;
        flow_state.end_drag();

        if let Some(pos) = final_pos {
            Some(DragResult::Completed(pos))
        } else {
            None
        }
    }

    /// Apply drag delta to selected nodes and groups
    fn apply_drag_to_nodes_and_groups<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        flow_state: &FlowState,
        group_manager: Option<&mut GroupManager>,
        delta: Position,
    ) where
        N: Clone,
        E: Clone,
    {
        // Apply to selected nodes
        for node_id in flow_state.selection_manager().selected_nodes() {
            if let Some(node) = find_node_mut(graph, node_id) {
                let new_position = node.position + delta;
                node.position = new_position;
            }
        }

        // Apply to selected groups if group manager exists
        if let Some(gm) = group_manager {
            let selected_groups = flow_state.selection_manager().get_selected_groups(gm);
            for group_id in selected_groups {
                if let Some(group) = gm.get_group_mut(&group_id) {
                    let new_position = group.position + delta;
                    group.position = new_position;

                    // Update all nodes in the group
                    for node_id in &group.members {
                        if let Some(node) = find_node_mut(graph, node_id) {
                            node.position = node.position + delta;
                        }
                    }
                }
            }
        }
    }

    /// Snap selected nodes to grid
    pub fn snap_selected_nodes_to_grid<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        flow_state: &FlowState,
    ) where
        N: Clone,
        E: Clone,
    {
        use crate::drag::calculations::apply_snap_to_grid;

        for node_id in flow_state.selection_manager().selected_nodes() {
            if let Some(node) = find_node_mut(graph, node_id) {
                node.position = apply_snap_to_grid(node.position, self.config.grid_size);
            }
        }
    }

    /// Apply drag constraints to current drag operation
    pub fn apply_drag_constraints<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        flow_state: &FlowState,
    ) where
        N: Clone,
        E: Clone,
    {
        // Apply bounds checking if enabled
        if self.config.enforce_bounds {
            if let Some(bounds) = self.config.canvas_bounds {
                for node_id in flow_state.selection_manager().selected_nodes() {
                    if let Some(node) = find_node_mut(graph, node_id) {
                        node.position = crate::drag::calculations::constrain_to_bounds(
                            node.position,
                            bounds,
                        );
                    }
                }
            }
        }

        // Apply grid snapping if enabled
        if self.config.snap_to_grid {
            self.snap_selected_nodes_to_grid(graph, flow_state);
        }
    }

    /// Apply drag to specific nodes
    pub fn apply_drag_to_nodes<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        flow_state: &FlowState,
        delta: Position,
    ) where
        N: Clone,
        E: Clone,
    {
        for node_id in flow_state.selection_manager().selected_nodes() {
            if let Some(node) = find_node_mut(graph, node_id) {
                node.position = node.position + delta;
            }
        }
    }

    /// Detect collisions between dragged nodes and other nodes
    pub fn detect_collisions<N, E>(
        &self,
        graph: &Graph<N, E>,
        flow_state: &FlowState,
    ) -> Vec<Collision>
    where
        N: Clone,
        E: Clone,
    {
        let mut collisions = Vec::new();
        let selected_ids: HashSet<_> = flow_state.selection_manager().selected_nodes().clone();

        for node_id in flow_state.selection_manager().selected_nodes() {
            if let Some(node) = find_node(graph, node_id) {
                // Check collision with all other non-selected nodes
                for other_node in graph.nodes() {
                    if selected_ids.contains(&other_node.id) {
                        continue; // Skip other selected nodes
                    }

                    if self.nodes_collide(node, other_node) {
                        collisions.push(Collision {
                            node_id: node.id.clone(),
                            other_id: other_node.id.clone(),
                            overlap: self.calculate_overlap(node, other_node),
                        });
                    }
                }
            }
        }

        collisions
    }

    /// Resolve detected collisions
    pub fn resolve_collisions<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        flow_state: &FlowState,
        collisions: &[Collision],
    ) where
        N: Clone,
        E: Clone,
    {
        // Simple collision resolution: separate overlapping nodes
        for collision in collisions {
            if let Some(node) = find_node_mut(graph, &collision.node_id) {
                // Move node away from collision
                let separation = Position::new(collision.overlap.x * 0.5, collision.overlap.y * 0.5);
                node.position = node.position + separation;
            }
        }
    }

    /// Handle drag for resize handles
    pub fn apply_handle_drag<N, E>(
        &self,
        _graph: &mut Graph<N, E>,
        flow_state: &FlowState,
    ) where
        N: Clone,
        E: Clone,
    {
        // TODO: Implement handle-based resizing
        // This would handle resizing nodes by dragging corner/edge handles
        // For now, this is a placeholder
    }

    /// Check if two nodes collide
    fn nodes_collide<N>(&self, node1: &flow_rs_core::Node<N>, node2: &flow_rs_core::Node<N>) -> bool {
        let bounds1 = Rect::new(
            node1.position.x - node1.size.width / 2.0,
            node1.position.y - node1.size.height / 2.0,
            node1.size.width,
            node1.size.height,
        );

        let bounds2 = Rect::new(
            node2.position.x - node2.size.width / 2.0,
            node2.position.y - node2.size.height / 2.0,
            node2.size.width,
            node2.size.height,
        );

        bounds1.intersects(&bounds2)
    }

    /// Calculate overlap between two colliding nodes
    fn calculate_overlap<N>(&self, node1: &flow_rs_core::Node<N>, node2: &flow_rs_core::Node<N>) -> Position {
        let dx = (node1.position.x - node2.position.x).abs() - (node1.size.width + node2.size.width) / 2.0;
        let dy = (node1.position.y - node2.position.y).abs() - (node1.size.height + node2.size.height) / 2.0;

        Position::new(dx.max(0.0), dy.max(0.0))
    }
}

impl Default for DragHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Collision between two nodes
#[derive(Debug, Clone)]
pub struct Collision {
    pub node_id: NodeId,
    pub other_id: NodeId,
    pub overlap: Position,
}

/// Helper function to find and modify a node by ID in the graph
fn find_node_mut<'a, N, E>(
    graph: &'a mut Graph<N, E>,
    node_id: &NodeId,
) -> Option<&'a mut flow_rs_core::Node<N>> {
    graph.nodes_mut().find(|node| &node.id == node_id)
}

/// Helper function to find a node by ID in the graph
fn find_node<'a, N, E>(
    graph: &'a Graph<N, E>,
    node_id: &NodeId,
) -> Option<&'a flow_rs_core::Node<N>> {
    graph.nodes().find(|node| &node.id == node_id)
}
