//! Core spatial indexing implementation

use std::collections::HashMap;

use crate::error::Result;
use crate::graph::Node;
use crate::types::{NodeId, Position, Rect, Viewport};

use super::grid::GridCell;

/// Spatial index for efficient viewport and proximity queries
pub struct SpatialIndex {
    entries: HashMap<NodeId, SpatialEntry>,
    cell_size: f64,
    grid: HashMap<GridCell, Vec<NodeId>>,
}

/// Entry in the spatial index
#[derive(Debug, Clone)]
struct SpatialEntry {
    #[allow(dead_code)]
    node_id: NodeId,
    bounds: Rect,
    grid_cells: Vec<GridCell>,
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialIndex {
    /// Create a new spatial index with default cell size
    pub fn new() -> Self {
        Self::with_cell_size(100.0) // Default 100x100 pixel cells
    }

    /// Create a new spatial index with specified cell size
    pub fn with_cell_size(cell_size: f64) -> Self {
        Self {
            entries: HashMap::new(),
            cell_size,
            grid: HashMap::new(),
        }
    }

    /// Insert a node into the spatial index
    pub fn insert<T>(&mut self, node: &Node<T>) -> Result<()>
    where
        T: Clone,
    {
        // Remove existing entry if present
        self.remove(&node.id);

        let bounds = node.bounds();
        let grid_cells = self.get_grid_cells_for_bounds(&bounds);

        let entry = SpatialEntry {
            node_id: node.id.clone(),
            bounds,
            grid_cells: grid_cells.clone(),
        };

        // Add to grid cells
        for cell in &grid_cells {
            self.grid.entry(*cell).or_default().push(node.id.clone());
        }

        // Store entry
        self.entries.insert(node.id.clone(), entry);

        Ok(())
    }

    /// Remove a node from the spatial index
    pub fn remove(&mut self, node_id: &NodeId) -> bool {
        if let Some(entry) = self.entries.remove(node_id) {
            // Remove from all grid cells
            for cell in &entry.grid_cells {
                if let Some(cell_nodes) = self.grid.get_mut(cell) {
                    cell_nodes.retain(|id| id != node_id);
                    if cell_nodes.is_empty() {
                        self.grid.remove(cell);
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// Update a node's position in the spatial index
    pub fn update<T>(&mut self, node: &Node<T>) -> Result<()>
    where
        T: Clone,
    {
        self.remove(&node.id);
        self.insert(node)
    }

    /// Clear all entries from the spatial index
    pub fn clear(&mut self) {
        self.entries.clear();
        self.grid.clear();
    }

    /// Query nodes within a rectangular area
    pub fn query_rect(&self, bounds: &Rect) -> Vec<NodeId> {
        let mut results = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let grid_cells = self.get_grid_cells_for_bounds(bounds);

        for cell in grid_cells {
            if let Some(cell_nodes) = self.grid.get(&cell) {
                for node_id in cell_nodes {
                    if !seen.contains(node_id) {
                        if let Some(entry) = self.entries.get(node_id) {
                            if bounds.intersects(&entry.bounds) {
                                results.push(node_id.clone());
                                seen.insert(node_id.clone());
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Query nodes within a viewport
    pub fn query_viewport(&self, viewport: &Viewport) -> Vec<NodeId> {
        let bounds = viewport.bounds();
        self.query_rect(&bounds)
    }

    /// Query nodes within a circular area
    pub fn query_radius(&self, center: Position, radius: f64) -> Vec<NodeId> {
        let mut results = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Create a bounding box for the circular area
        let bounds = Rect::new(
            center.x - radius,
            center.y - radius,
            radius * 2.0,
            radius * 2.0,
        );

        let grid_cells = self.get_grid_cells_for_bounds(&bounds);

        for cell in grid_cells {
            if let Some(cell_nodes) = self.grid.get(&cell) {
                for node_id in cell_nodes {
                    if !seen.contains(node_id) {
                        if let Some(entry) = self.entries.get(node_id) {
                            // Check if the node's center is within the radius
                            let node_center = entry.bounds.center();
                            if center.distance_to(node_center) <= radius {
                                results.push(node_id.clone());
                                seen.insert(node_id.clone());
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Find the nearest node to a given point
    pub fn nearest(&self, point: Position) -> Option<NodeId> {
        let mut nearest_id = None;
        let mut nearest_distance = f64::INFINITY;

        for (node_id, entry) in &self.entries {
            let node_center = entry.bounds.center();
            let distance = point.distance_to(node_center);

            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_id = Some(node_id.clone());
            }
        }

        nearest_id
    }

    /// Get all node IDs in the index
    pub fn node_ids(&self) -> Vec<NodeId> {
        self.entries.keys().cloned().collect()
    }

    /// Get the number of nodes in the index
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Bulk load multiple nodes into the index
    pub fn bulk_load<T>(&mut self, nodes: &[Node<T>]) -> Result<()>
    where
        T: Clone,
    {
        for node in nodes {
            self.insert(node)?;
        }
        Ok(())
    }

    /// Get the bounding rectangle of all nodes in the index
    pub fn bounds(&self) -> Option<Rect> {
        if self.entries.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for entry in self.entries.values() {
            min_x = min_x.min(entry.bounds.x);
            min_y = min_y.min(entry.bounds.y);
            max_x = max_x.max(entry.bounds.x + entry.bounds.width);
            max_y = max_y.max(entry.bounds.y + entry.bounds.height);
        }

        Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
    }

    /// Get the cell size of the spatial index
    pub fn cell_size(&self) -> f64 {
        self.cell_size
    }

    /// Get grid cells that a bounds rectangle intersects
    pub fn get_grid_cells_for_bounds(&self, bounds: &Rect) -> Vec<GridCell> {
        // Handle invalid bounds
        if !bounds.is_valid() || bounds.width < 0.0 || bounds.height < 0.0 {
            return Vec::new();
        }

        // Handle NaN or infinity values
        if bounds.x.is_nan() || bounds.y.is_nan() || bounds.width.is_nan() || bounds.height.is_nan() ||
           bounds.x.is_infinite() || bounds.y.is_infinite() || bounds.width.is_infinite() || bounds.height.is_infinite() {
            return Vec::new();
        }

        let min_cell_x = (bounds.x / self.cell_size).floor() as i32;
        let min_cell_y = (bounds.y / self.cell_size).floor() as i32;
        let max_cell_x = ((bounds.x + bounds.width) / self.cell_size).floor() as i32;
        let max_cell_y = ((bounds.y + bounds.height) / self.cell_size).floor() as i32;

        let mut cells = Vec::new();
        for x in min_cell_x..=max_cell_x {
            for y in min_cell_y..=max_cell_y {
                cells.push(GridCell::new(x, y));
            }
        }
        cells
    }
}
