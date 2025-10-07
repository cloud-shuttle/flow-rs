//! Drag state management
//!
//! Manages drag-specific state tracking and history.

use flow_rs_core::{NodeId, Position};
use std::collections::VecDeque;

/// Current drag operation state
#[derive(Debug, Clone)]
pub struct DragState {
    /// The node being dragged
    pub node_id: NodeId,
    /// Initial position when drag started
    pub start_position: Position,
    /// Current position during drag
    pub current_position: Position,
    /// Drag velocity (for momentum)
    pub velocity: Velocity,
    /// Whether the drag is currently active
    pub is_active: bool,
    /// Whether snapping to grid is enabled
    pub snap_enabled: bool,
}

impl DragState {
    /// Create a new drag state
    pub fn new(node_id: NodeId, start_position: Position) -> Self {
        Self {
            node_id,
            start_position,
            current_position: start_position,
            velocity: Velocity::zero(),
            is_active: true,
            snap_enabled: false,
        }
    }

    /// Update the current position
    pub fn update_position(&mut self, new_position: Position) {
        self.current_position = new_position;
    }

    /// Calculate the drag delta from start position
    pub fn drag_delta(&self) -> Position {
        self.current_position - self.start_position
    }

    /// Check if drag has moved beyond threshold
    pub fn has_moved_beyond_threshold(&self, threshold: f64) -> bool {
        self.drag_delta().distance_to(Position::new(0.0, 0.0)) > threshold
    }

    /// End the drag operation
    pub fn end_drag(&mut self) {
        self.is_active = false;
    }

    /// Cancel the drag operation
    pub fn cancel_drag(&mut self) {
        self.is_active = false;
        self.current_position = self.start_position;
    }
}

/// Velocity tracking for drag operations
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Velocity {
    /// Zero velocity
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Create from position change over time
    pub fn from_delta(delta: Position, time_delta: f64) -> Self {
        if time_delta > 0.0 {
            Self {
                x: delta.x / time_delta,
                y: delta.y / time_delta,
            }
        } else {
            Self::zero()
        }
    }
}

/// History of drag positions for undo/redo or analysis
#[derive(Debug, Clone)]
pub struct DragHistory {
    positions: VecDeque<Position>,
    timestamps: VecDeque<f64>,
    max_history: usize,
}

impl DragHistory {
    /// Create a new drag history with maximum size
    pub fn new(max_history: usize) -> Self {
        Self {
            positions: VecDeque::with_capacity(max_history),
            timestamps: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Add a new position to the history
    pub fn record_position(&mut self, position: Position, timestamp: f64) {
        if self.positions.len() >= self.max_history {
            self.positions.pop_front();
            self.timestamps.pop_front();
        }

        self.positions.push_back(position);
        self.timestamps.push_back(timestamp);
    }

    /// Get the most recent position
    pub fn latest_position(&self) -> Option<&Position> {
        self.positions.back()
    }

    /// Get the position history
    pub fn positions(&self) -> &VecDeque<Position> {
        &self.positions
    }

    /// Calculate average velocity over recent history
    pub fn average_velocity(&self) -> Velocity {
        if self.positions.len() < 2 {
            return Velocity::zero();
        }

        let mut total_velocity = Velocity::zero();
        let mut count = 0;

        for i in 1..self.positions.len() {
            let delta = self.positions[i] - self.positions[i - 1];
            let time_delta = self.timestamps[i] - self.timestamps[i - 1];

            if time_delta > 0.0 {
                let velocity = Velocity::from_delta(delta, time_delta);
                total_velocity.x += velocity.x;
                total_velocity.y += velocity.y;
                count += 1;
            }
        }

        if count > 0 {
            Velocity {
                x: total_velocity.x / count as f64,
                y: total_velocity.y / count as f64,
            }
        } else {
            Velocity::zero()
        }
    }

    /// Clear the history
    pub fn clear(&mut self) {
        self.positions.clear();
        self.timestamps.clear();
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}

impl Default for DragHistory {
    fn default() -> Self {
        Self::new(50) // Default to 50 history entries
    }
}
