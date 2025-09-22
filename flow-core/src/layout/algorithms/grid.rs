//! Grid layout algorithm implementation

use crate::error::Result;
use crate::graph::Graph;
use crate::layout::LayoutAlgorithm;
use crate::types::Position;

/// Simple grid layout
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub columns: Option<usize>,
    pub cell_width: f64,
    pub cell_height: f64,
    pub margin: f64,
}

impl Default for GridLayout {
    fn default() -> Self {
        Self {
            columns: None, // Auto-calculate based on sqrt
            cell_width: 120.0,
            cell_height: 80.0,
            margin: 20.0,
        }
    }
}

impl GridLayout {
    /// Create a new grid layout
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of columns (None for auto)
    pub fn columns(mut self, columns: Option<usize>) -> Self {
        self.columns = columns;
        self
    }

    /// Set cell dimensions
    pub fn cell_size(mut self, width: f64, height: f64) -> Self {
        self.cell_width = width;
        self.cell_height = height;
        self
    }

    /// Set margin between cells
    pub fn margin(mut self, margin: f64) -> Self {
        self.margin = margin;
        self
    }
}

impl<N, E> LayoutAlgorithm<N, E> for GridLayout {
    fn name(&self) -> &str {
        "Grid"
    }

    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        let nodes: Vec<_> = graph.nodes_mut().collect();
        let node_count = nodes.len();

        if node_count == 0 {
            return Ok(());
        }

        // Calculate columns
        let columns = self
            .columns
            .unwrap_or_else(|| (node_count as f64).sqrt().ceil() as usize);

        // Ensure columns is at least 1 to avoid division by zero
        let columns = columns.max(1);

        // Position nodes in grid
        for (i, node) in nodes.into_iter().enumerate() {
            let row = i / columns;
            let col = i % columns;

            let x = col as f64 * (self.cell_width + self.margin);
            let y = row as f64 * (self.cell_height + self.margin);

            node.position = Position::new(x, y);
        }

        Ok(())
    }
}
