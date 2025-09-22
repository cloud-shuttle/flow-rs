//! Layout algorithms for automatic node positioning

use crate::error::Result;
use crate::graph::Graph;

/// Trait for layout algorithms
pub trait LayoutAlgorithm<N, E> {
    /// Name of the layout algorithm
    fn name(&self) -> &str;

    /// Apply the layout to the graph
    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()>;

    /// Check if the algorithm is running
    fn is_running(&self) -> bool {
        false
    }

    /// Stop the algorithm if it's running
    fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    /// Get the progress (0.0 to 1.0)
    fn progress(&self) -> f64 {
        1.0
    }

    /// Check if the algorithm can be interrupted
    fn can_interrupt(&self) -> bool {
        false
    }
}

// Re-export all algorithms
pub mod algorithms;
pub mod utils;

pub use algorithms::*;
pub use utils::LayoutUtils;

#[cfg(test)]
mod tests;
