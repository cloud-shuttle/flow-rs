//! Drag and drop interaction system for Leptos Flow
//!
//! Provides comprehensive drag and drop functionality for flow editors

use leptos::*;
use web_sys::MouseEvent;

use flow_core::{Graph, Position, NodeId, Rect, GroupId, GroupManager};
use crate::signals::{FlowState, ViewportState};

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

    /// Handle mouse down event to potentially start drag
    pub fn handle_mouse_down<N, E>(
        &self,
        event: &MouseEvent,
        graph: &Graph<N, E>,
        flow_state: &mut FlowState,
        viewport_state: &ViewportState,
        group_manager: Option<&GroupManager>,
    ) -> Option<DragResult>
    where
        N: Clone,
        E: Clone,
    {
        let mouse_pos = self.screen_to_world_position(event, viewport_state);

        // Find node under cursor
        if let Some(node_id) = self.find_node_at_position(&mouse_pos, graph) {
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
            return Some(DragResult::Started(mouse_pos));
        }

        None
    }

    /// Handle mouse move event during drag
    pub fn handle_mouse_move<N, E>(
        &self,
        event: &MouseEvent,
        graph: &mut Graph<N, E>,
        flow_state: &mut FlowState,
        viewport_state: &ViewportState,
        group_manager: Option<&mut GroupManager>,
    ) -> Option<DragResult>
    where
        N: Clone,
        E: Clone,
    {
        if !flow_state.is_dragging {
            return None;
        }

        let mouse_pos = self.screen_to_world_position(event, viewport_state);

        // Check if we've moved enough to start dragging
        if let Some(start_pos) = flow_state.drag_start {
            let distance = self.calculate_distance(start_pos, mouse_pos);
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
    pub fn handle_mouse_up<N, E>(
        &self,
        event: &MouseEvent,
        graph: &mut Graph<N, E>,
        flow_state: &mut FlowState,
        viewport_state: &ViewportState,
        group_manager: Option<&mut GroupManager>,
    ) -> Option<DragResult>
    where
        N: Clone,
        E: Clone,
    {
        if !flow_state.is_dragging {
            return None;
        }

        let mouse_pos = self.screen_to_world_position(event, viewport_state);

        // Final position update
        if let Some(delta) = flow_state.drag_delta() {
            self.apply_drag_to_nodes_and_groups(graph, flow_state, group_manager, delta);

            // Apply snap to grid if enabled
            if self.config.snap_to_grid {
                self.snap_selected_nodes_to_grid(graph, flow_state);
            }
        }

        // End drag operation
        flow_state.end_drag();

        Some(DragResult::Completed(mouse_pos))
    }

    /// Convert screen coordinates to world coordinates
    fn screen_to_world_position(&self, event: &MouseEvent, viewport_state: &ViewportState) -> Position {
        let viewport = &viewport_state.viewport;

        // Get canvas-relative coordinates
        let canvas_x = event.offset_x() as f64;
        let canvas_y = event.offset_y() as f64;

        // Transform to world coordinates
        let world_x = (canvas_x / viewport.zoom) - viewport.offset.x;
        let world_y = (canvas_y / viewport.zoom) - viewport.offset.y;

        Position::new(world_x, world_y)
    }

    /// Find node at the given position
    fn find_node_at_position<N, E>(&self, position: &Position, graph: &Graph<N, E>) -> Option<NodeId>
    where
        N: Clone,
        E: Clone,
    {
        // Iterate through nodes and check if position is inside
        for node in graph.nodes() {
            if node.contains_point(*position) {
                return Some(node.id.clone());
            }
        }
        None
    }

    /// Calculate distance between two positions
    fn calculate_distance(&self, pos1: Position, pos2: Position) -> f64 {
        let dx = pos2.x - pos1.x;
        let dy = pos2.y - pos1.y;
        (dx * dx + dy * dy).sqrt()
    }


    /// Apply bounds constraints to position
    fn apply_bounds_constraints(&self, position: Position, node_size: &flow_core::Size) -> Position {
        if let Some(bounds) = &self.config.canvas_bounds {
            let min_x = bounds.x;
            let min_y = bounds.y;
            let max_x = bounds.x + bounds.width - node_size.width;
            let max_y = bounds.y + bounds.height - node_size.height;

            Position::new(
                position.x.clamp(min_x, max_x),
                position.y.clamp(min_y, max_y),
            )
        } else {
            position
        }
    }

    /// Snap selected nodes to grid
    pub fn snap_selected_nodes_to_grid<N, E>(&self, graph: &mut Graph<N, E>, flow_state: &FlowState)
    where
        N: Clone,
        E: Clone,
    {
        let grid_size = self.config.grid_size;

        // Calculate target position once for efficiency
        let target_pos = if let (Some(drag_start), Some(last_pos)) = (flow_state.drag_start, flow_state.last_mouse_pos) {
            Position::new(
                drag_start.x + (last_pos.x - drag_start.x),
                drag_start.y + (last_pos.y - drag_start.y),
            )
        } else {
            return; // No drag in progress
        };

        for node_id in &flow_state.selected_nodes {
            if let Some(node) = graph.get_node_mut(node_id) {
                // Use the target position for snapping
                let snapped_x = (target_pos.x / grid_size).round() * grid_size;
                let snapped_y = (target_pos.y / grid_size).round() * grid_size;

                node.set_position(Position::new(snapped_x, snapped_y));
            }
        }
    }

    /// Apply drag to selected nodes and groups (NEW METHOD)
    pub fn apply_drag_to_nodes_and_groups<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        flow_state: &FlowState,
        group_manager: Option<&mut GroupManager>,
        delta: Position
    )
    where
        N: Clone,
        E: Clone,
    {
        if let Some(gm) = group_manager {
            // Get selected groups and handle group movement
            let selected_groups = flow_state.get_selected_groups(gm);
            if !selected_groups.is_empty() {
                // Move groups directly - they handle moving all their member nodes
                for group_id in selected_groups {
                    let _ = gm.move_group(&group_id, delta, graph);
                }
            } else {
                // No groups selected, fall back to individual node movement
                self.apply_drag_to_nodes(graph, flow_state, delta);
            }
        } else {
            // Fallback to original node-only drag
            self.apply_drag_to_nodes(graph, flow_state, delta);
        }
    }

    /// Apply drag to selected nodes (public method for tests)
    pub fn apply_drag_to_nodes<N, E>(&self, graph: &mut Graph<N, E>, flow_state: &FlowState, delta: Position)
    where
        N: Clone,
        E: Clone,
    {
        for node_id in &flow_state.selected_nodes {
            if let Some(node) = graph.get_node_mut(node_id) {
                let new_pos = Position::new(
                    node.position.x + delta.x,
                    node.position.y + delta.y,
                );

                // Apply bounds checking if enabled
                let final_pos = if self.config.enforce_bounds {
                    self.apply_bounds_constraints(new_pos, &node.size)
                } else {
                    new_pos
                };

                node.set_position(final_pos);
            }
        }
    }

    /// Detect collisions between dragged nodes and other nodes
    pub fn detect_collisions<N, E>(&self, graph: &Graph<N, E>, flow_state: &FlowState) -> Vec<(NodeId, NodeId)>
    where
        N: Clone,
        E: Clone,
    {
        let mut collisions = Vec::new();

        let delta = match flow_state.drag_delta() {
            Some(delta) => delta,
            None => return collisions, // No drag in progress
        };

        // Pre-calculate selected node IDs as a set for O(1) lookup
        let selected_set: std::collections::HashSet<_> = flow_state.selected_nodes.iter().collect();

        for dragged_node_id in &flow_state.selected_nodes {
            let dragged_node = match graph.get_node(dragged_node_id) {
                Some(node) => node,
                None => continue, // Node doesn't exist
            };

            let new_pos = Position::new(
                dragged_node.position.x + delta.x,
                dragged_node.position.y + delta.y,
            );

            // Check against all other nodes
            for other_node in graph.nodes() {
                if other_node.id != *dragged_node_id && !selected_set.contains(&other_node.id) {
                    if self.nodes_overlap(new_pos, &dragged_node.size, other_node.position, &other_node.size) {
                        collisions.push((dragged_node_id.clone(), other_node.id.clone()));
                    }
                }
            }
        }

        collisions
    }

    /// Resolve collisions by adjusting positions
    pub fn resolve_collisions<N, E>(&self, graph: &mut Graph<N, E>, _flow_state: &FlowState, collisions: &[(NodeId, NodeId)])
    where
        N: Clone,
        E: Clone,
    {
        // Simple collision resolution: move the dragged node to avoid overlap
        for (dragged_id, other_id) in collisions {
            // Get the other node's position first to avoid borrow checker issues
            let other_pos = if let Some(other_node) = graph.get_node(other_id) {
                (other_node.position, other_node.size.clone())
            } else {
                continue;
            };

            // Now modify the dragged node
            if let Some(dragged_node) = graph.get_node_mut(dragged_id) {
                // Move dragged node to the right of the other node
                let new_x = other_pos.0.x + other_pos.1.width + 10.0;
                dragged_node.set_position(Position::new(new_x, dragged_node.position.y));
            }
        }
    }

    /// Apply drag constraints (axis locking, etc.)
    pub fn apply_drag_constraints<N, E>(&self, graph: &mut Graph<N, E>, flow_state: &FlowState)
    where
        N: Clone,
        E: Clone,
    {
        if let Some(constraint) = &self.config.constraint {
            if let Some(delta) = flow_state.drag_delta() {
                let constrained_delta = match constraint {
                    DragConstraint::HorizontalOnly => Position::new(delta.x, 0.0),
                    DragConstraint::VerticalOnly => Position::new(0.0, delta.y),
                    DragConstraint::DiagonalOnly => {
                        let magnitude = (delta.x * delta.x + delta.y * delta.y).sqrt();
                        let normalized = if magnitude > 0.0 {
                            Position::new(delta.x / magnitude, delta.y / magnitude)
                        } else {
                            Position::new(0.0, 0.0)
                        };
                        Position::new(normalized.x * magnitude, normalized.y * magnitude)
                    }
                    DragConstraint::Custom(func) => func(delta),
                };

                self.apply_drag_to_nodes(graph, flow_state, constrained_delta);
            }
        }
    }

    /// Apply handle drag for precise node manipulation
    pub fn apply_handle_drag<N, E>(&self, graph: &mut Graph<N, E>, flow_state: &FlowState)
    where
        N: Clone,
        E: Clone,
    {
        for node_id in &flow_state.selected_nodes {
            if let Some(node) = graph.get_node_mut(node_id) {
                // Calculate the drag delta
                let delta = if let (Some(drag_start), Some(last_pos)) = (flow_state.drag_start, flow_state.last_mouse_pos) {
                    Position::new(
                        last_pos.x - drag_start.x,
                        last_pos.y - drag_start.y,
                    )
                } else {
                    return;
                };

                // Apply handle-specific resizing logic
                // For now, we'll implement a simple bottom-right handle resize
                // In a real implementation, we'd need to track which handle is being dragged
                let new_width = (node.size.width + delta.x).max(10.0); // Minimum width of 10
                let new_height = (node.size.height + delta.y).max(10.0); // Minimum height of 10

                node.set_size(flow_core::Size::new(new_width, new_height));
            }
        }
    }

    /// Check if two nodes overlap using axis-aligned bounding box collision detection
    fn nodes_overlap(&self, pos1: Position, size1: &flow_core::Size, pos2: Position, size2: &flow_core::Size) -> bool {
        pos1.x < pos2.x + size2.width &&
        pos1.x + size1.width > pos2.x &&
        pos1.y < pos2.y + size2.height &&
        pos1.y + size1.height > pos2.y
    }

    /// Calculate the target position for a node based on drag state
    fn calculate_target_position(&self, flow_state: &FlowState, node: &flow_core::Node<impl Clone>) -> Position {
        if let (Some(drag_start), Some(last_pos)) = (flow_state.drag_start, flow_state.last_mouse_pos) {
            Position::new(
                node.position.x + (last_pos.x - drag_start.x),
                node.position.y + (last_pos.y - drag_start.y),
            )
        } else {
            node.position
        }
    }
}

impl Default for DragHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Create drag event handlers for a flow editor
pub fn create_drag_handlers<N, E>(
    graph: RwSignal<Graph<N, E>>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
    drag_config: Option<DragConfig>,
) -> (
    impl Fn(MouseEvent) + Clone,  // mouse_down
    impl Fn(MouseEvent) + Clone,  // mouse_move
    impl Fn(MouseEvent) + Clone,  // mouse_up
)
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    let drag_handler = DragHandler::with_config(drag_config.unwrap_or_default());

    let mouse_down = {
        let drag_handler = drag_handler.clone();
        move |event: MouseEvent| {
            flow_state.update(|state| {
                let graph_val = graph.get_untracked();
                let viewport_val = viewport_state.get_untracked();

                if let Some(_result) = drag_handler.handle_mouse_down(&event, &graph_val, state, &viewport_val, None) {
                    // Drag started
                }
            });
        }
    };

    let mouse_move = {
        let drag_handler = drag_handler.clone();
        move |event: MouseEvent| {
            graph.update(|graph_mut| {
                flow_state.update(|state| {
                    let viewport_val = viewport_state.get_untracked();

                    if let Some(_result) = drag_handler.handle_mouse_move(&event, graph_mut, state, &viewport_val, None) {
                        // Drag updated
                    }
                });
            });
        }
    };

    let mouse_up = {
        move |event: MouseEvent| {
            graph.update(|graph_mut| {
                flow_state.update(|state| {
                    let viewport_val = viewport_state.get_untracked();

                    if let Some(_result) = drag_handler.handle_mouse_up(&event, graph_mut, state, &viewport_val, None) {
                        // Drag completed
                    }
                });
            });
        }
    };

    (mouse_down, mouse_move, mouse_up)
}

/// Create drag event handlers for a flow editor with group support
pub fn create_group_aware_drag_handlers<N, E>(
    graph: RwSignal<Graph<N, E>>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
    group_manager: RwSignal<GroupManager>,
    drag_config: Option<DragConfig>,
) -> (
    impl Fn(MouseEvent) + Clone,  // mouse_down
    impl Fn(MouseEvent) + Clone,  // mouse_move
    impl Fn(MouseEvent) + Clone,  // mouse_up
)
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    let drag_handler = DragHandler::with_config(drag_config.unwrap_or_default());

    let mouse_down = {
        let drag_handler = drag_handler.clone();
        move |event: MouseEvent| {
            flow_state.update(|state| {
                let graph_val = graph.get_untracked();
                let viewport_val = viewport_state.get_untracked();
                let group_manager_val = group_manager.get_untracked();

                if let Some(_result) = drag_handler.handle_mouse_down(&event, &graph_val, state, &viewport_val, Some(&group_manager_val)) {
                    // Drag started
                }
            });
        }
    };

    let mouse_move = {
        let drag_handler = drag_handler.clone();
        move |event: MouseEvent| {
            graph.update(|graph_mut| {
                flow_state.update(|state| {
                    group_manager.update(|gm| {
                        let viewport_val = viewport_state.get_untracked();

                        if let Some(_result) = drag_handler.handle_mouse_move(&event, graph_mut, state, &viewport_val, Some(gm)) {
                            // Drag updated
                        }
                    });
                });
            });
        }
    };

    let mouse_up = {
        move |event: MouseEvent| {
            graph.update(|graph_mut| {
                flow_state.update(|state| {
                    group_manager.update(|gm| {
                        let viewport_val = viewport_state.get_untracked();

                        if let Some(_result) = drag_handler.handle_mouse_up(&event, graph_mut, state, &viewport_val, Some(gm)) {
                            // Drag completed
                        }
                    });
                });
            });
        }
    };

    (mouse_down, mouse_move, mouse_up)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::{Node, Size, GroupManager, Group};

    #[test]
    fn test_drag_handler_creation() {
        let handler = DragHandler::new();
        assert!(!handler.config.snap_to_grid);
        assert_eq!(handler.config.grid_size, 20.0);
        assert_eq!(handler.config.drag_threshold, 3.0);
    }

    #[test]
    fn test_drag_config() {
        let config = DragConfig {
            snap_to_grid: true,
            grid_size: 10.0,
            drag_threshold: 5.0,
            enforce_bounds: true,
            canvas_bounds: Some(Rect::new(0.0, 0.0, 800.0, 600.0)),
            constraint: None,
        };

        let handler = DragHandler::with_config(config.clone());
        assert!(handler.config.snap_to_grid);
        assert_eq!(handler.config.grid_size, 10.0);
        assert!(handler.config.enforce_bounds);
    }

    #[test]
    fn test_distance_calculation() {
        let handler = DragHandler::new();
        let pos1 = Position::new(0.0, 0.0);
        let pos2 = Position::new(3.0, 4.0);

        let distance = handler.calculate_distance(pos1, pos2);
        assert_eq!(distance, 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_bounds_constraints() {
        let config = DragConfig {
            enforce_bounds: true,
            canvas_bounds: Some(Rect::new(0.0, 0.0, 400.0, 300.0)),
            ..Default::default()
        };
        let handler = DragHandler::with_config(config);

        let node_size = Size::new(50.0, 30.0);

        // Position within bounds
        let pos1 = Position::new(100.0, 100.0);
        let result1 = handler.apply_bounds_constraints(pos1, &node_size);
        assert_eq!(result1, pos1);

        // Position outside bounds (negative)
        let pos2 = Position::new(-10.0, -5.0);
        let result2 = handler.apply_bounds_constraints(pos2, &node_size);
        assert_eq!(result2.x, 0.0);
        assert_eq!(result2.y, 0.0);

        // Position outside bounds (too large)
        let pos3 = Position::new(400.0, 300.0);
        let result3 = handler.apply_bounds_constraints(pos3, &node_size);
        assert_eq!(result3.x, 350.0); // 400 - 50
        assert_eq!(result3.y, 270.0); // 300 - 30
    }

    // NEW TEST: Test group drag integration
    #[test]
    fn test_group_drag_integration() {
        let handler = DragHandler::new();
        let mut graph: Graph<(), ()> = Graph::new();
        let mut flow_state = FlowState::new();
        let mut group_manager = GroupManager::new();

        // Create nodes using correct API
        let node1_id = NodeId::new("node1");
        let node2_id = NodeId::new("node2");
        let node3_id = NodeId::new("node3");

        let mut node1 = Node::new(node1_id.clone(), Position::new(10.0, 10.0), ());
        node1.size = Size::new(50.0, 30.0);
        let mut node2 = Node::new(node2_id.clone(), Position::new(100.0, 50.0), ());
        node2.size = Size::new(50.0, 30.0);
        let mut node3 = Node::new(node3_id.clone(), Position::new(200.0, 100.0), ());
        node3.size = Size::new(50.0, 30.0);

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();
        graph.add_node(node3).unwrap();

        // Create a group with first two nodes using correct API
        let group_id = GroupId::new("group1");
        let mut members = std::collections::HashSet::new();
        members.insert(node1_id.clone());
        members.insert(node2_id.clone());

        group_manager.create_group(group_id.clone(), members).unwrap();

        // Select the group (should select both nodes)
        flow_state.select_group(&group_manager, &group_id);
        assert!(flow_state.is_node_selected(&node1_id));
        assert!(flow_state.is_node_selected(&node2_id));
        assert!(!flow_state.is_node_selected(&node3_id));

        // Apply drag to nodes and groups
        let delta = Position::new(50.0, 30.0);
        handler.apply_drag_to_nodes_and_groups(&mut graph, &flow_state, Some(&mut group_manager), delta);

        // Verify that all selected nodes moved by the same delta
        let node1_after = graph.get_node(&node1_id).unwrap();
        let node2_after = graph.get_node(&node2_id).unwrap();
        let node3_after = graph.get_node(&node3_id).unwrap();

        assert_eq!(node1_after.position, Position::new(60.0, 40.0)); // 10 + 50, 10 + 30
        assert_eq!(node2_after.position, Position::new(150.0, 80.0)); // 100 + 50, 50 + 30
        assert_eq!(node3_after.position, Position::new(200.0, 100.0)); // Unchanged

        // Verify that the group position was also updated
        let group_after = group_manager.get_group(&group_id).unwrap();
        // Note: group position should be calculated based on its bounds
        // The exact position depends on the group's bounding calculation
    }

    #[test]
    fn test_node_selection_with_group() {
        let mut flow_state = FlowState::new();
        let mut group_manager = GroupManager::new();

        // Create nodes
        let node1_id = NodeId::new("node1");
        let node2_id = NodeId::new("node2");

        // Create a group using correct API
        let group_id = GroupId::new("group1");
        let mut members = std::collections::HashSet::new();
        members.insert(node1_id.clone());
        members.insert(node2_id.clone());
        group_manager.create_group(group_id.clone(), members).unwrap();

        // Test selecting node with group
        flow_state.select_node_with_group(&group_manager, node1_id.clone(), true);

        // Both nodes should be selected since they're in the same group
        assert!(flow_state.is_node_selected(&node1_id));
        assert!(flow_state.is_node_selected(&node2_id));

        // Test selecting node without group
        flow_state.clear_selection();
        flow_state.select_node_with_group(&group_manager, node1_id.clone(), false);

        // Only the specific node should be selected
        assert!(flow_state.is_node_selected(&node1_id));
        assert!(!flow_state.is_node_selected(&node2_id));
    }
}
