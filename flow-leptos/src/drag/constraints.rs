//! Drag constraints and boundaries
//!
//! Handles movement limitations, bounds checking, and constraint validation.

use flow_rs_core::{Position, Rect};
use crate::drag::calculations::direction_vector;

/// Drag constraint types (re-exported from handler for convenience)
pub type DragConstraint = super::handler::DragConstraint;

/// Comprehensive drag constraints configuration
#[derive(Debug, Clone)]
pub struct DragConstraints {
    /// Optional rectangular bounds
    pub bounds: Option<Rect>,
    /// Grid snapping configuration
    pub grid_snap: Option<GridSnap>,
    /// Axis locking
    pub axis_lock: Option<Axis>,
    /// Minimum drag distance
    pub min_distance: f64,
    /// Maximum drag distance from start position
    pub max_distance: Option<f64>,
    /// Custom constraint function
    pub custom_constraint: Option<fn(Position, Position) -> Position>,
}

impl DragConstraints {
    /// Create a new constraints configuration
    pub fn new() -> Self {
        Self {
            bounds: None,
            grid_snap: None,
            axis_lock: None,
            min_distance: 0.0,
            max_distance: None,
            custom_constraint: None,
        }
    }

    /// Set rectangular bounds
    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Set grid snapping
    pub fn with_grid_snap(mut self, size: f64) -> Self {
        self.grid_snap = Some(GridSnap { size });
        self
    }

    /// Lock to horizontal axis only
    pub fn horizontal_only(mut self) -> Self {
        self.axis_lock = Some(Axis::Horizontal);
        self
    }

    /// Lock to vertical axis only
    pub fn vertical_only(mut self) -> Self {
        self.axis_lock = Some(Axis::Vertical);
        self
    }

    /// Set minimum drag distance
    pub fn with_min_distance(mut self, distance: f64) -> Self {
        self.min_distance = distance;
        self
    }

    /// Set maximum drag distance
    pub fn with_max_distance(mut self, distance: f64) -> Self {
        self.max_distance = Some(distance);
        self
    }

    /// Set custom constraint function
    pub fn with_custom_constraint(mut self, constraint: fn(Position, Position) -> Position) -> Self {
        self.custom_constraint = Some(constraint);
        self
    }
}

impl Default for DragConstraints {
    fn default() -> Self {
        Self::new()
    }
}

/// Grid snapping configuration
#[derive(Debug, Clone, Copy)]
pub struct GridSnap {
    /// Grid cell size
    pub size: f64,
}

impl GridSnap {
    /// Snap a position to the grid
    pub fn snap(&self, position: Position) -> Position {
        Position::new(
            (position.x / self.size).round() * self.size,
            (position.y / self.size).round() * self.size,
        )
    }
}

/// Axis locking options
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// Apply all constraints to a position
pub fn apply_constraints(
    start_position: Position,
    current_position: Position,
    constraints: &DragConstraints,
) -> Position {
    let mut constrained_pos = current_position;

    // Apply axis locking first
    if let Some(axis) = constraints.axis_lock {
        constrained_pos = apply_axis_lock(start_position, constrained_pos, axis);
    }

    // Apply distance constraints
    constrained_pos = apply_distance_constraints(start_position, constrained_pos, constraints);

    // Apply bounds checking
    if let Some(bounds) = constraints.bounds {
        constrained_pos = constrain_to_bounds(constrained_pos, bounds);
    }

    // Apply grid snapping
    if let Some(grid_snap) = &constraints.grid_snap {
        constrained_pos = grid_snap.snap(constrained_pos);
    }

    // Apply custom constraints
    if let Some(custom_constraint) = constraints.custom_constraint {
        constrained_pos = custom_constraint(start_position, constrained_pos);
    }

    constrained_pos
}

/// Apply axis locking constraint
fn apply_axis_lock(start: Position, current: Position, axis: Axis) -> Position {
    match axis {
        Axis::Horizontal => Position::new(current.x, start.y),
        Axis::Vertical => Position::new(start.x, current.y),
    }
}

/// Apply distance-based constraints
fn apply_distance_constraints(start: Position, current: Position, constraints: &DragConstraints) -> Position {
    let delta = current - start;
    let distance = delta.distance_to(Position::new(0.0, 0.0));

    // Check minimum distance (don't apply until threshold met)
    if distance < constraints.min_distance {
        return start;
    }

    // Check maximum distance
    if let Some(max_dist) = constraints.max_distance {
        if distance > max_dist {
            let direction = direction_vector(start, current);
            return start + direction * max_dist;
        }
    }

    current
}

/// Constrain position to rectangular bounds
pub fn constrain_to_bounds(position: Position, bounds: Rect) -> Position {
    Position::new(
        position.x.max(bounds.x).min(bounds.x + bounds.width),
        position.y.max(bounds.y).min(bounds.y + bounds.height),
    )
}

/// Validate that a position meets all constraints
pub fn validate_constraints(
    start_position: Position,
    current_position: Position,
    constraints: &DragConstraints,
) -> Result<Position, ConstraintViolation> {
    let delta = current_position - start_position;
    let distance = delta.distance_to(Position::new(0.0, 0.0));

    // Check minimum distance
    if distance < constraints.min_distance {
        return Err(ConstraintViolation::MinDistance {
            current: distance,
            required: constraints.min_distance,
        });
    }

    // Check maximum distance
    if let Some(max_dist) = constraints.max_distance {
        if distance > max_dist {
            return Err(ConstraintViolation::MaxDistance {
                current: distance,
                allowed: max_dist,
            });
        }
    }

    // Check bounds
    if let Some(bounds) = constraints.bounds {
        let constrained = constrain_to_bounds(current_position, bounds);
        if constrained != current_position {
            return Err(ConstraintViolation::Bounds {
                attempted: current_position,
                constrained_to: constrained,
                bounds,
            });
        }
    }

    Ok(current_position)
}

/// Constraint violation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintViolation {
    /// Drag distance below minimum threshold
    MinDistance { current: f64, required: f64 },
    /// Drag distance exceeds maximum allowed
    MaxDistance { current: f64, allowed: f64 },
    /// Position outside allowed bounds
    Bounds {
        attempted: Position,
        constrained_to: Position,
        bounds: Rect,
    },
}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintViolation::MinDistance { current, required } => {
                write!(f, "Drag distance {} below minimum required {}", current, required)
            }
            ConstraintViolation::MaxDistance { current, allowed } => {
                write!(f, "Drag distance {} exceeds maximum allowed {}", current, allowed)
            }
            ConstraintViolation::Bounds { attempted, constrained_to, bounds } => {
                write!(f, "Position {:?} outside bounds {:?}, constrained to {:?}", attempted, bounds, constrained_to)
            }
        }
    }
}

impl std::error::Error for ConstraintViolation {}

/// Check if a drag operation would violate constraints before applying it
pub fn check_constraint_violation(
    start_position: Position,
    proposed_position: Position,
    constraints: &DragConstraints,
) -> Option<ConstraintViolation> {
    validate_constraints(start_position, proposed_position, constraints).err()
}

/// Calculate the maximum allowed position within constraints
pub fn calculate_max_allowed_position(
    start_position: Position,
    direction: Position,
    constraints: &DragConstraints,
) -> Position {
    let direction = direction_vector(Position::new(0.0, 0.0), direction);

    // Find the maximum distance we can travel
    let max_distance = constraints.max_distance.unwrap_or(f64::INFINITY);

    // Check bounds constraints
    let bounds_limited_pos = if let Some(bounds) = constraints.bounds {
        // Find intersection with bounds in the direction of movement
        let mut test_pos = start_position;

        // Test moving in the direction until we hit bounds
        for _ in 0..100 { // Prevent infinite loop
            test_pos = test_pos + direction * 10.0; // Move in small increments
            if !bounds.contains_point(test_pos) {
                // We've gone outside bounds, back up one step
                test_pos = test_pos - direction * 10.0;
                break;
            }
        }

        test_pos
    } else {
        start_position + direction * max_distance
    };

    // Apply distance limits
    let distance_limited_pos = start_position + direction * max_distance;

    // Return the more restrictive of the two
    let bounds_distance = (bounds_limited_pos - start_position).distance_to(Position::new(0.0, 0.0));
    let distance_limit = (distance_limited_pos - start_position).distance_to(Position::new(0.0, 0.0));

    if bounds_distance < distance_limit {
        bounds_limited_pos
    } else {
        distance_limited_pos
    }
}
