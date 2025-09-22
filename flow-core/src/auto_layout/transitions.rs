//! Layout transition management and animation

use std::collections::HashMap;

use crate::graph::Graph;
use crate::types::{NodeId, Position};

/// State for managing layout transitions
#[derive(Debug)]
pub struct TransitionState {
    pub progress: f64,
    pub duration: f64,
    pub from_positions: HashMap<NodeId, Position>,
    pub to_positions: HashMap<NodeId, Position>,
}

impl TransitionState {
    /// Create a new transition state
    pub fn new(
        duration: f64,
        from_positions: HashMap<NodeId, Position>,
        to_positions: HashMap<NodeId, Position>,
    ) -> Self {
        Self {
            progress: 0.0,
            duration,
            from_positions,
            to_positions,
        }
    }
}

/// Helper function for smooth easing
pub fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Interpolate between two positions
pub fn interpolate_position(from: Position, to: Position, t: f64) -> Position {
    Position::new(
        from.x + (to.x - from.x) * t,
        from.y + (to.y - from.y) * t,
    )
}

/// Update transition state and apply interpolated positions
pub fn update_transition<N, E>(
    transition_state: &mut Option<TransitionState>,
    graph: &mut Graph<N, E>,
    delta_time: f64,
) -> Result<bool, crate::error::FlowError> {
    let should_remove_state = if let Some(ref mut state) = transition_state {
        state.progress += delta_time / state.duration;

        if state.progress >= 1.0 {
            // Transition complete - apply final positions
            for node in graph.nodes_mut() {
                if let Some(&target_pos) = state.to_positions.get(&node.id) {
                    node.position = target_pos;
                }
            }
            return Ok(true); // Transition completed
        } else {
            // Interpolate positions - collect data first to avoid borrowing issues
            let progress = state.progress;
            let transitions: Vec<_> = graph
                .nodes()
                .filter_map(|node| {
                    if let (Some(&from_pos), Some(&to_pos)) = (
                        state.from_positions.get(&node.id),
                        state.to_positions.get(&node.id),
                    ) {
                        Some((node.id.clone(), from_pos, to_pos))
                    } else {
                        None
                    }
                })
                .collect();

            // Apply interpolated positions
            let t = ease_in_out(progress);
            for (node_id, from_pos, to_pos) in transitions {
                if let Some(node) = graph.get_node_mut(&node_id) {
                    node.position = interpolate_position(from_pos, to_pos, t);
                }
            }
            return Ok(false); // Transition in progress
        }
    } else {
        false
    };

    if should_remove_state {
        *transition_state = None;
    }

    Ok(true) // No transition
}
