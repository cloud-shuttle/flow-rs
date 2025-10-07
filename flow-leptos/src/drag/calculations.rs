//! Mathematical calculations for drag operations
//!
//! Position calculations, coordinate transformations, and drag mathematics.

use flow_rs_core::{Position, Rect};
use crate::signals::ViewportState;

/// Calculate the Euclidean distance between two positions
pub fn calculate_distance(pos1: Position, pos2: Position) -> f64 {
    let dx = pos2.x - pos1.x;
    let dy = pos2.y - pos1.y;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate the drag delta (change in position)
pub fn calculate_drag_delta(start_position: Position, current_position: Position) -> Position {
    current_position - start_position
}

/// Apply grid snapping to a position
pub fn apply_snap_to_grid(position: Position, grid_size: f64) -> Position {
    Position::new(
        (position.x / grid_size).round() * grid_size,
        (position.y / grid_size).round() * grid_size,
    )
}

/// Constrain position to rectangular bounds
pub fn constrain_to_bounds(position: Position, bounds: Rect) -> Position {
    Position::new(
        position.x.max(bounds.x).min(bounds.x + bounds.width),
        position.y.max(bounds.y).min(bounds.y + bounds.height),
    )
}

/// Convert screen coordinates to world (flow) coordinates
pub fn screen_to_world_position(screen_pos: Position, viewport: &ViewportState) -> Position {
    Position::new(
        (screen_pos.x - viewport.viewport.x) / viewport.viewport.zoom,
        (screen_pos.y - viewport.viewport.y) / viewport.viewport.zoom,
    )
}

/// Convert world (flow) coordinates to screen coordinates
pub fn world_to_screen_position(world_pos: Position, viewport: &ViewportState) -> Position {
    Position::new(
        world_pos.x * viewport.viewport.zoom + viewport.viewport.x,
        world_pos.y * viewport.viewport.zoom + viewport.viewport.y,
    )
}

/// Calculate the center position of multiple positions
pub fn calculate_center(positions: &[Position]) -> Option<Position> {
    if positions.is_empty() {
        return None;
    }

    let sum = positions.iter().fold(Position::zero(), |acc, pos| acc + *pos);
    Some(sum / positions.len() as f64)
}

/// Calculate the bounding rectangle of multiple positions
pub fn calculate_bounds(positions: &[Position]) -> Option<Rect> {
    if positions.is_empty() {
        return None;
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for pos in positions {
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
    }

    Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
}

/// Calculate the average velocity from a series of positions and timestamps
pub fn calculate_average_velocity(
    positions: &[Position],
    timestamps: &[f64],
) -> Option<Position> {
    if positions.len() < 2 || timestamps.len() < 2 || positions.len() != timestamps.len() {
        return None;
    }

    let mut total_velocity = Position::zero();
    let mut count = 0;

    for i in 1..positions.len() {
        let delta_pos = positions[i] - positions[i - 1];
        let delta_time = timestamps[i] - timestamps[i - 1];

        if delta_time > 0.0 {
            let velocity = delta_pos / delta_time;
            total_velocity = total_velocity + velocity;
            count += 1;
        }
    }

    if count > 0 {
        Some(total_velocity / count as f64)
    } else {
        None
    }
}

/// Apply momentum to a position based on velocity and time delta
pub fn apply_momentum(
    position: Position,
    velocity: Position,
    time_delta: f64,
    friction: f64,
) -> (Position, Position) {
    let damped_velocity = velocity * friction;
    let new_position = position + damped_velocity * time_delta;

    (new_position, damped_velocity)
}

/// Calculate the intersection point of a line with a rectangle
pub fn calculate_line_bounds_intersection(
    start: Position,
    end: Position,
    bounds: Rect,
) -> Option<Position> {
    let line = Line { start, end };

    // Check intersection with each edge of the bounds
    let edges = [
        Line { start: Position::new(bounds.x, bounds.y), end: Position::new(bounds.x + bounds.width, bounds.y) }, // Top
        Line { start: Position::new(bounds.x + bounds.width, bounds.y), end: Position::new(bounds.x + bounds.width, bounds.y + bounds.height) }, // Right
        Line { start: Position::new(bounds.x + bounds.width, bounds.y + bounds.height), end: Position::new(bounds.x, bounds.y + bounds.height) }, // Bottom
        Line { start: Position::new(bounds.x, bounds.y + bounds.height), end: Position::new(bounds.x, bounds.y) }, // Left
    ];

    for edge in &edges {
        if let Some(intersection) = line.intersection_with(edge) {
            if bounds.contains_point(intersection) {
                return Some(intersection);
            }
        }
    }

    None
}

/// Simple line segment representation
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    /// Calculate intersection point with another line segment
    fn intersection_with(&self, other: &Line) -> Option<Position> {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;

        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator.abs() < f64::EPSILON {
            return None; // Lines are parallel
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            Some(Position::new(
                x1 + t * (x2 - x1),
                y1 + t * (y2 - y1),
            ))
        } else {
            None
        }
    }
}

/// Calculate the angle of a position relative to the origin
pub fn calculate_angle(position: Position) -> f64 {
    position.y.atan2(position.x)
}

/// Calculate a position at a given angle and distance from origin
pub fn position_from_angle_distance(angle: f64, distance: f64) -> Position {
    Position::new(
        angle.cos() * distance,
        angle.sin() * distance,
    )
}

/// Smooth a position value using exponential smoothing
pub fn exponential_smooth(current: f64, target: f64, smoothing_factor: f64) -> f64 {
    current + (target - current) * smoothing_factor
}

/// Smooth a position using exponential smoothing
pub fn smooth_position(current: Position, target: Position, smoothing_factor: f64) -> Position {
    Position::new(
        exponential_smooth(current.x, target.x, smoothing_factor),
        exponential_smooth(current.y, target.y, smoothing_factor),
    )
}

/// Check if a position is within a certain tolerance of another position
pub fn positions_equal_within_tolerance(pos1: Position, pos2: Position, tolerance: f64) -> bool {
    calculate_distance(pos1, pos2) <= tolerance
}

/// Calculate the direction vector from one position to another
pub fn direction_vector(from: Position, to: Position) -> Position {
    let delta = to - from;
    let magnitude = delta.distance_to(Position::new(0.0, 0.0));
    if magnitude > 0.0 {
        delta / magnitude
    } else {
        Position::new(0.0, 0.0)
    }
}

/// Project a position onto a line defined by two points
pub fn project_onto_line(point: Position, line_start: Position, line_end: Position) -> Position {
    let line_vector = line_end - line_start;
    let point_vector = point - line_start;

    let line_length_squared = line_vector.distance_to(Position::new(0.0, 0.0)).powi(2);
    if line_length_squared == 0.0 {
        return line_start;
    }

    // Simple dot product calculation
    let dot_product = (point_vector.x * line_vector.x) + (point_vector.y * line_vector.y);
    let t = dot_product / line_length_squared;
    let t_clamped = t.max(0.0).min(1.0); // Clamp to line segment

    line_start + line_vector * t_clamped
}
