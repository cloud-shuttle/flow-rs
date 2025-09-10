//! Simple spatial indexing for efficient queries
//! 
//! This implementation uses a simple grid-based spatial partitioning scheme
//! for efficient viewport and proximity queries.

use std::collections::HashMap;

use crate::error::Result;
use crate::types::{Position, Rect, Viewport, NodeId};
use crate::graph::Node;

/// Spatial index for efficient viewport and proximity queries
pub struct SpatialIndex {
    entries: HashMap<NodeId, SpatialEntry>,
    cell_size: f64,
    grid: HashMap<GridCell, Vec<NodeId>>,
}

/// Entry in the spatial index
#[derive(Debug, Clone)]
struct SpatialEntry {
    node_id: NodeId,
    bounds: Rect,
    grid_cells: Vec<GridCell>,
}

/// Grid cell coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridCell {
    x: i32,
    y: i32,
}

impl GridCell {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_position(pos: Position, cell_size: f64) -> Self {
        Self {
            x: (pos.x / cell_size).floor() as i32,
            y: (pos.y / cell_size).floor() as i32,
        }
    }
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
            self.grid
                .entry(*cell)
                .or_insert_with(Vec::new)
                .push(node.id.clone());
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
        // Remove and re-insert to update position
        self.remove(&node.id);
        self.insert(node)
    }

    /// Clear all entries from the spatial index
    pub fn clear(&mut self) {
        self.entries.clear();
        self.grid.clear();
    }

    /// Query nodes that intersect with a rectangle
    pub fn query_rect(&self, bounds: &Rect) -> Vec<NodeId> {
        let grid_cells = self.get_grid_cells_for_bounds(bounds);
        let mut candidates = Vec::new();

        // Collect all nodes in overlapping grid cells
        for cell in &grid_cells {
            if let Some(cell_nodes) = self.grid.get(cell) {
                candidates.extend(cell_nodes.iter().cloned());
            }
        }

        // Remove duplicates and filter by actual intersection
        candidates.sort_unstable();
        candidates.dedup();

        candidates
            .into_iter()
            .filter(|node_id| {
                if let Some(entry) = self.entries.get(node_id) {
                    bounds.intersects(&entry.bounds)
                } else {
                    false
                }
            })
            .collect()
    }

    /// Query nodes within viewport bounds
    pub fn query_viewport(&self, viewport: &Viewport) -> Vec<NodeId> {
        let bounds = viewport.bounds();
        self.query_rect(&bounds)
    }

    /// Query nodes within a radius of a point
    pub fn query_radius(&self, center: Position, radius: f64) -> Vec<NodeId> {
        let bounds = Rect::new(
            center.x - radius,
            center.y - radius,
            radius * 2.0,
            radius * 2.0,
        );

        self.query_rect(&bounds)
            .into_iter()
            .filter(|node_id| {
                if let Some(entry) = self.entries.get(node_id) {
                    let node_center = Position::new(
                        entry.bounds.x + entry.bounds.width / 2.0,
                        entry.bounds.y + entry.bounds.height / 2.0,
                    );
                    center.distance_to(node_center) <= radius
                } else {
                    false
                }
            })
            .collect()
    }

    /// Find the nearest node to a point
    pub fn nearest(&self, point: Position) -> Option<NodeId> {
        let mut nearest_id: Option<NodeId> = None;
        let mut nearest_distance = f64::INFINITY;

        // Start with a small search radius and expand if needed
        let mut search_radius = self.cell_size;
        
        while search_radius <= 1000.0 && nearest_id.is_none() {
            let candidates = self.query_radius(point, search_radius);
            
            for node_id in candidates {
                if let Some(entry) = self.entries.get(&node_id) {
                    let node_center = Position::new(
                        entry.bounds.x + entry.bounds.width / 2.0,
                        entry.bounds.y + entry.bounds.height / 2.0,
                    );
                    let distance = point.distance_to(node_center);
                    
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_id = Some(node_id);
                    }
                }
            }
            
            if nearest_id.is_some() {
                break;
            }
            
            search_radius *= 2.0;
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

    /// Build the index from a collection of nodes
    pub fn bulk_load<T>(&mut self, nodes: &[Node<T>]) -> Result<()>
    where
        T: Clone,
    {
        self.clear();
        
        for node in nodes {
            self.insert(node)?;
        }

        Ok(())
    }

    /// Get spatial bounds of all entries
    pub fn bounds(&self) -> Option<Rect> {
        if self.is_empty() {
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

    /// Get grid cells that overlap with the given bounds
    fn get_grid_cells_for_bounds(&self, bounds: &Rect) -> Vec<GridCell> {
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

/// Query builder for spatial queries
pub struct SpatialQuery<'a> {
    index: &'a SpatialIndex,
    bounds: Option<Rect>,
    center: Option<Position>,
    radius: Option<f64>,
    max_results: Option<usize>,
}

impl<'a> SpatialQuery<'a> {
    /// Create a new spatial query
    pub fn new(index: &'a SpatialIndex) -> Self {
        Self {
            index,
            bounds: None,
            center: None,
            radius: None,
            max_results: None,
        }
    }

    /// Filter by rectangular bounds
    pub fn bounds(mut self, bounds: Rect) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Filter by circular area
    pub fn radius(mut self, center: Position, radius: f64) -> Self {
        self.center = Some(center);
        self.radius = Some(radius);
        self
    }

    /// Limit the number of results
    pub fn limit(mut self, max_results: usize) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// Execute the query and return node IDs
    pub fn execute(self) -> Vec<NodeId> {
        let mut results = if let Some(bounds) = self.bounds {
            self.index.query_rect(&bounds)
        } else if let (Some(center), Some(radius)) = (self.center, self.radius) {
            self.index.query_radius(center, radius)
        } else {
            self.index.node_ids()
        };

        if let Some(max_results) = self.max_results {
            results.truncate(max_results);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Size;
    use crate::graph::NodeBuilder;

    #[test]
    fn test_spatial_index_creation() {
        let index = SpatialIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_insert_and_query() {
        let mut index = SpatialIndex::new();
        
        let node = NodeBuilder::new("test-node")
            .position(Position::new(10.0, 10.0))
            .size(Size::new(50.0, 30.0))
            .build();

        index.insert(&node).unwrap();
        assert_eq!(index.len(), 1);

        // Query overlapping rectangle
        let query_bounds = Rect::new(0.0, 0.0, 50.0, 50.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], node.id().clone());

        // Query non-overlapping rectangle
        let query_bounds = Rect::new(100.0, 100.0, 50.0, 50.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_radius_query() {
        let mut index = SpatialIndex::new();
        
        let node = NodeBuilder::new("test-node")
            .position(Position::new(10.0, 10.0))
            .size(Size::new(20.0, 20.0))
            .build();

        index.insert(&node).unwrap();

        // Query within radius (center of node is at 20, 20)
        let results = index.query_radius(Position::new(25.0, 25.0), 10.0);
        assert_eq!(results.len(), 1);

        // Query outside radius
        let results = index.query_radius(Position::new(50.0, 50.0), 10.0);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_nearest_neighbor() {
        let mut index = SpatialIndex::new();
        
        let node1 = NodeBuilder::new("node1")
            .position(Position::new(10.0, 10.0))
            .size(Size::new(20.0, 20.0))
            .build();
            
        let node2 = NodeBuilder::new("node2")
            .position(Position::new(50.0, 50.0))
            .size(Size::new(20.0, 20.0))
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();

        // Find nearest to point closer to node1
        let nearest = index.nearest(Position::new(15.0, 15.0));
        assert_eq!(nearest, Some(node1.id().clone()));

        // Find nearest to point closer to node2
        let nearest = index.nearest(Position::new(55.0, 55.0));
        assert_eq!(nearest, Some(node2.id().clone()));
    }

    #[test]
    fn test_update_node() {
        let mut index = SpatialIndex::new();
        
        let mut node = NodeBuilder::new("test-node")
            .position(Position::new(10.0, 10.0))
            .size(Size::new(20.0, 20.0))
            .build();

        index.insert(&node).unwrap();

        // Move node to new position
        node.set_position(Position::new(100.0, 100.0));
        index.update(&node).unwrap();

        // Old position should return no results
        let old_query = Rect::new(0.0, 0.0, 50.0, 50.0);
        let results = index.query_rect(&old_query);
        assert_eq!(results.len(), 0);

        // New position should return the node
        let new_query = Rect::new(90.0, 90.0, 50.0, 50.0);
        let results = index.query_rect(&new_query);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_bulk_load() {
        let mut index = SpatialIndex::new();
        
        let nodes = vec![
            NodeBuilder::new("node1")
                .position(Position::new(10.0, 10.0))
                .size(Size::new(20.0, 20.0))
                .build(),
            NodeBuilder::new("node2")
                .position(Position::new(50.0, 50.0))
                .size(Size::new(20.0, 20.0))
                .build(),
            NodeBuilder::new("node3")
                .position(Position::new(90.0, 90.0))
                .size(Size::new(20.0, 20.0))
                .build(),
        ];

        index.bulk_load(&nodes).unwrap();
        assert_eq!(index.len(), 3);

        // Query all nodes
        let query_bounds = Rect::new(0.0, 0.0, 120.0, 120.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_spatial_query_builder() {
        let mut index = SpatialIndex::new();
        
        let nodes = vec![
            NodeBuilder::new("node1")
                .position(Position::new(10.0, 10.0))
                .size(Size::new(20.0, 20.0))
                .build(),
            NodeBuilder::new("node2")
                .position(Position::new(50.0, 50.0))
                .size(Size::new(20.0, 20.0))
                .build(),
            NodeBuilder::new("node3")
                .position(Position::new(90.0, 90.0))
                .size(Size::new(20.0, 20.0))
                .build(),
        ];

        index.bulk_load(&nodes).unwrap();

        // Query with bounds and limit
        let results = SpatialQuery::new(&index)
            .bounds(Rect::new(0.0, 0.0, 120.0, 120.0))
            .limit(2)
            .execute();
        
        assert_eq!(results.len(), 2);

        // Query with radius
        let results = SpatialQuery::new(&index)
            .radius(Position::new(60.0, 60.0), 30.0)
            .execute();
        
        assert!(!results.is_empty());
    }

    #[test]
    fn test_grid_cells() {
        let index = SpatialIndex::with_cell_size(50.0);
        
        // Test single cell
        let bounds = Rect::new(10.0, 10.0, 20.0, 20.0);
        let cells = index.get_grid_cells_for_bounds(&bounds);
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0], GridCell::new(0, 0));
        
        // Test spanning multiple cells
        let bounds = Rect::new(25.0, 25.0, 50.0, 50.0);
        let cells = index.get_grid_cells_for_bounds(&bounds);
        assert!(cells.len() >= 4); // Should span 2x2 cells at minimum
    }
}