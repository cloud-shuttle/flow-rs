//! Circular layout algorithm implementation

use crate::error::Result;
use crate::graph::Graph;
use crate::layout::LayoutAlgorithm;
use crate::types::Position;

/// Circular layout algorithm
#[derive(Debug, Clone)]
pub struct CircularLayout {
    pub radius: f64,
    pub start_angle: f64,
    pub clockwise: bool,
}

impl Default for CircularLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl CircularLayout {
    /// Create a new circular layout
    pub fn new() -> Self {
        Self {
            radius: 200.0,
            start_angle: 0.0,
            clockwise: true,
        }
    }

    /// Set radius
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Set start angle in radians
    pub fn start_angle(mut self, angle: f64) -> Self {
        self.start_angle = angle;
        self
    }

    /// Set direction
    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = clockwise;
        self
    }
}

impl<N, E> LayoutAlgorithm<N, E> for CircularLayout {
    fn name(&self) -> &str {
        "Circular"
    }

    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        let nodes: Vec<_> = graph.nodes_mut().collect();
        let node_count = nodes.len();

        if node_count == 0 {
            return Ok(());
        }

        let angle_step = 2.0 * std::f64::consts::PI / node_count as f64;

        for (i, node) in nodes.into_iter().enumerate() {
            let angle = if self.clockwise {
                self.start_angle + i as f64 * angle_step
            } else {
                self.start_angle - i as f64 * angle_step
            };

            let x = self.radius * angle.cos();
            let y = self.radius * angle.sin();

            node.position = Position::new(x, y);
        }

        Ok(())
    }
}
