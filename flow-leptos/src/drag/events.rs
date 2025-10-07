//! Mouse and touch event processing for drag operations
//!
//! Translates DOM events into drag operations and manages event state.

use web_sys::MouseEvent;
use flow_rs_core::{Graph, NodeId, Position};
use crate::signals::ViewportState;
use crate::drag::calculations::{screen_to_world_position, calculate_distance};

/// Result of processing a mouse or touch event
#[derive(Debug, Clone)]
pub enum DragEvent {
    /// Start a new drag operation
    Start {
        node_id: NodeId,
        position: Position,
        ctrl_held: bool,
    },
    /// Continue an existing drag operation
    Move {
        position: Position,
    },
    /// End the current drag operation
    End,
}

/// Process a mouse down event to potentially start a drag
pub fn process_mouse_down<N, E>(
    event: &MouseEvent,
    graph: &Graph<N, E>,
    viewport: &ViewportState,
) -> Option<DragEvent>
where
    N: Clone,
    E: Clone,
{
    let mouse_pos = screen_to_world_position(
        Position::new(event.client_x() as f64, event.client_y() as f64),
        viewport,
    );

    // Find node under cursor
    if let Some(node_id) = find_node_at_position(&mouse_pos, graph) {
        Some(DragEvent::Start {
            node_id,
            position: mouse_pos,
            ctrl_held: event.ctrl_key() || event.meta_key(),
        })
    } else {
        None
    }
}

/// Process a mouse move event during drag
pub fn process_mouse_move(
    event: &MouseEvent,
    viewport: &ViewportState,
    drag_threshold: f64,
    current_drag_start: Option<Position>,
) -> Option<DragEvent> {
    let mouse_pos = screen_to_world_position(
        Position::new(event.client_x() as f64, event.client_y() as f64),
        viewport,
    );

    // Check if we've moved enough to start dragging
    if let Some(start_pos) = current_drag_start {
        let distance = calculate_distance(start_pos, mouse_pos);
        if distance < drag_threshold {
            return None; // Not enough movement yet
        }
    }

    Some(DragEvent::Move { position: mouse_pos })
}

/// Process a mouse up event to end drag
pub fn process_mouse_up(_event: &MouseEvent) -> DragEvent {
    DragEvent::End
}

/// Find the node at a given position in the graph
pub fn find_node_at_position<N, E>(position: &Position, graph: &Graph<N, E>) -> Option<NodeId>
where
    N: Clone,
    E: Clone,
{
    // Check nodes (in insertion order, which approximates visual stack)
    // TODO: Add z-index sorting when implemented
    for node in graph.nodes() {
        // Simple bounding box check - in a real implementation,
        // this would check the actual node shape and size
        let node_bounds = calculate_node_bounds(&node.position, &node.size);

        if node_bounds.contains_point(*position) {
            return Some(node.id.clone());
        }
    }

    None
}

/// Calculate the bounding rectangle for a node
fn calculate_node_bounds(position: &Position, size: &flow_rs_core::Size) -> flow_rs_core::Rect {
    flow_rs_core::Rect::new(
        position.x - size.width / 2.0,
        position.y - size.height / 2.0,
        size.width,
        size.height,
    )
}

/// Process touch events for mobile drag support
pub fn process_touch_events(_events: &[web_sys::TouchEvent]) -> Option<DragEvent> {
    // TODO: Implement touch event processing
    // This is a placeholder for future mobile support
    // Touch events have a different API than mouse events
    None
}

/// Check if an event should be considered a drag gesture
pub fn is_drag_gesture(
    start_event: &MouseEvent,
    current_event: &MouseEvent,
    threshold: f64,
) -> bool {
    let start_pos = Position::new(
        start_event.client_x() as f64,
        start_event.client_y() as f64,
    );
    let current_pos = Position::new(
        current_event.client_x() as f64,
        current_event.client_y() as f64,
    );

    calculate_distance(start_pos, current_pos) >= threshold
}

/// Extract modifier keys from an event
#[derive(Debug, Clone)]
pub struct ModifierKeys {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl ModifierKeys {
    pub fn from_event(event: &MouseEvent) -> Self {
        Self {
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        }
    }
}

/// Determine drag behavior based on modifier keys
pub fn determine_drag_behavior(modifiers: &ModifierKeys) -> DragBehavior {
    if modifiers.ctrl || modifiers.meta {
        DragBehavior::AddToSelection
    } else if modifiers.shift {
        DragBehavior::ToggleSelection
    } else {
        DragBehavior::ReplaceSelection
    }
}

/// Drag behavior based on modifier keys
#[derive(Debug, Clone, Copy)]
pub enum DragBehavior {
    ReplaceSelection,
    AddToSelection,
    ToggleSelection,
}

/// Check if event should prevent default browser behavior
pub fn should_prevent_default(_event: &MouseEvent, is_dragging: bool) -> bool {
    // Prevent default during drag to avoid text selection, etc.
    is_dragging
}

/// Check if event should stop propagation
pub fn should_stop_propagation(_event: &MouseEvent, is_on_node: bool) -> bool {
    // Stop propagation if we're on a node to prevent canvas-level handling
    is_on_node
}

/// Get the appropriate cursor style for the current drag state
pub fn get_cursor_style(is_dragging: bool, can_drag: bool) -> &'static str {
    match (is_dragging, can_drag) {
        (true, _) => "grabbing",
        (false, true) => "grab",
        (false, false) => "default",
    }
}

/// Handle keyboard events that affect drag behavior
pub fn process_keyboard_event(
    event: &web_sys::KeyboardEvent,
    _current_drag: Option<&DragState>,
) -> Option<KeyboardDragAction> {
    match event.key().as_str() {
        "Escape" => Some(KeyboardDragAction::Cancel),
        "Enter" => Some(KeyboardDragAction::Confirm),
        " " if event.shift_key() => Some(KeyboardDragAction::Constrain),
        _ => None,
    }
}

/// Keyboard actions that affect drag
#[derive(Debug, Clone)]
pub enum KeyboardDragAction {
    Cancel,
    Confirm,
    Constrain, // e.g., Shift+drag for axis constraint
}

// Re-export for convenience
pub use crate::drag::state::DragState;
