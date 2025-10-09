//! Layout transition management and animation

use std::collections::HashMap;

use crate::graph::{Graph, Node};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Position};
    use std::collections::HashMap;

    #[test]
    fn test_transition_state_creation() {
        let mut from_positions = HashMap::new();
        let mut to_positions = HashMap::new();

        let node_id = NodeId::new("test");
        from_positions.insert(node_id.clone(), Position::new(0.0, 0.0));
        to_positions.insert(node_id, Position::new(100.0, 100.0));

        let state = TransitionState::new(1.0, from_positions, to_positions);

        assert_eq!(state.progress, 0.0);
        assert_eq!(state.duration, 1.0);
        assert_eq!(state.from_positions.len(), 1);
        assert_eq!(state.to_positions.len(), 1);
    }

    #[test]
    fn test_ease_in_out() {
        // Test boundary values
        assert_eq!(ease_in_out(0.0), 0.0);
        assert_eq!(ease_in_out(1.0), 1.0);

        // Test midpoint
        assert_eq!(ease_in_out(0.5), 0.5);

        // Test smooth curve properties
        let val1 = ease_in_out(0.25);
        let val2 = ease_in_out(0.75);
        assert!(val1 > 0.0 && val1 < 0.5); // First half should be smooth
        assert!(val2 > 0.5 && val2 < 1.0); // Second half should be smooth
    }

    #[test]
    fn test_interpolate_position() {
        let from = Position::new(0.0, 0.0);
        let to = Position::new(100.0, 100.0);

        // Test start of interpolation
        let result = interpolate_position(from, to, 0.0);
        assert_eq!(result, Position::new(0.0, 0.0));

        // Test end of interpolation
        let result = interpolate_position(from, to, 1.0);
        assert_eq!(result, Position::new(100.0, 100.0));

        // Test midpoint interpolation
        let result = interpolate_position(from, to, 0.5);
        assert_eq!(result, Position::new(50.0, 50.0));

        // Test arbitrary point
        let result = interpolate_position(from, to, 0.25);
        assert_eq!(result, Position::new(25.0, 25.0));
    }

    #[test]
    fn test_update_transition_no_state() {
        let mut graph = Graph::<(), ()>::new();
        let mut transition_state: Option<TransitionState> = None;

        let result = update_transition(&mut transition_state, &mut graph, 0.1);
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(transition_state.is_none());
    }

    #[test]
    fn test_update_transition_complete() {
        let mut graph = Graph::new();

        // Add a test node
        let node = Node::new("test", Position::new(0.0, 0.0), ());
        graph.add_node(node.clone()).unwrap();
        let node_id = node.id;

        // Create transition state
        let mut from_positions = HashMap::new();
        let mut to_positions = HashMap::new();
        from_positions.insert(node_id.clone(), Position::new(0.0, 0.0));
        to_positions.insert(node_id, Position::new(100.0, 100.0));

        let mut transition_state = Some(TransitionState::new(1.0, from_positions, to_positions));

        // Update with full duration
        let result = update_transition(&mut transition_state, &mut graph, 1.0);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true (transition completed)

        // Check final position
        let node = graph.node(node_id).unwrap();
        assert_eq!(node.position, Position::new(100.0, 100.0));

        // Transition state should be cleared
        assert!(transition_state.is_none());
    }

    #[test]
    fn test_update_transition_in_progress() {
        let mut graph = Graph::new();

        // Add a test node
        let node = Node::new("test", Position::new(0.0, 0.0), ());
        graph.add_node(node.clone()).unwrap();
        let node_id = node.id;

        // Create transition state
        let mut from_positions = HashMap::new();
        let mut to_positions = HashMap::new();
        from_positions.insert(node_id.clone(), Position::new(0.0, 0.0));
        to_positions.insert(node_id.clone(), Position::new(100.0, 100.0));

        let mut transition_state = Some(TransitionState::new(2.0, from_positions, to_positions));

        // Update with half duration
        let result = update_transition(&mut transition_state, &mut graph, 1.0);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false (transition in progress)

        // Check interpolated position (should be at t=0.5 with ease_in_out)
        let node = graph.node(node_id).unwrap();
        let expected_t = ease_in_out(0.5); // progress = 1.0/2.0 = 0.5
        let expected_pos = interpolate_position(Position::new(0.0, 0.0), Position::new(100.0, 100.0), expected_t);
        assert_eq!(node.position, expected_pos);

        // Transition state should still exist
        assert!(transition_state.is_some());
        assert_eq!(transition_state.as_ref().unwrap().progress, 0.5);
    }

    #[test]
    fn test_update_transition_multiple_nodes() {
        let mut graph = Graph::new();

        // Add multiple test nodes
        let node1 = Node::new("test1", Position::new(0.0, 0.0), ());
        let node2 = Node::new("test2", Position::new(10.0, 10.0), ());
        graph.add_node(node1.clone()).unwrap();
        graph.add_node(node2.clone()).unwrap();
        let node1_id = node1.id;
        let node2_id = node2.id;

        // Create transition state for both nodes
        let mut from_positions = HashMap::new();
        let mut to_positions = HashMap::new();
        from_positions.insert(node1_id.clone(), Position::new(0.0, 0.0));
        from_positions.insert(node2_id.clone(), Position::new(10.0, 10.0));
        to_positions.insert(node1_id.clone(), Position::new(100.0, 100.0));
        to_positions.insert(node2_id, Position::new(200.0, 200.0));

        let mut transition_state = Some(TransitionState::new(1.0, from_positions, to_positions));

        // Complete transition
        let result = update_transition(&mut transition_state, &mut graph, 1.0);
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Check final positions
        let node1 = graph.node(node1_id).unwrap();
        assert_eq!(node1.position, Position::new(100.0, 100.0));

        // Node2 should also be updated
        let node2 = graph.nodes().find(|n| n.id == node2_id).unwrap();
        assert_eq!(node2.position, Position::new(200.0, 200.0));
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
