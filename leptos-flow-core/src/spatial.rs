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

        let node = NodeBuilder::<()>::new("test-node")
            .position(10.0, 10.0)
            .size(50.0, 30.0)
            .build();

        index.insert(&node).unwrap();
        assert_eq!(index.len(), 1);

        // Query overlapping rectangle
        let query_bounds = Rect::new(0.0, 0.0, 50.0, 50.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], node.id.clone());

        // Query non-overlapping rectangle
        let query_bounds = Rect::new(100.0, 100.0, 50.0, 50.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_radius_query() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test-node")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
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

        let node1 = NodeBuilder::<()>::new("node1")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(50.0, 50.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();

        // Find nearest to point closer to node1
        let nearest = index.nearest(Position::new(15.0, 15.0));
        assert_eq!(nearest, Some(node1.id.clone()));

        // Find nearest to point closer to node2
        let nearest = index.nearest(Position::new(55.0, 55.0));
        assert_eq!(nearest, Some(node2.id.clone()));
    }

    #[test]
    fn test_update_node() {
        let mut index = SpatialIndex::new();

        let mut node = NodeBuilder::<()>::new("test-node")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
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
            NodeBuilder::<()>::new("node1")
                .position(10.0, 10.0)
                .size(20.0, 20.0)
                .build(),
            NodeBuilder::<()>::new("node2")
                .position(50.0, 50.0)
                .size(20.0, 20.0)
                .build(),
            NodeBuilder::<()>::new("node3")
                .position(90.0, 90.0)
                .size(20.0, 20.0)
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
            NodeBuilder::<()>::new("node1")
                .position(10.0, 10.0)
                .size(20.0, 20.0)
                .build(),
            NodeBuilder::<()>::new("node2")
                .position(50.0, 50.0)
                .size(20.0, 20.0)
                .build(),
            NodeBuilder::<()>::new("node3")
                .position(90.0, 90.0)
                .size(20.0, 20.0)
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

    // Targeted unit tests for mathematical operations (mutation testing coverage)

    #[test]
    fn test_spatial_index_bounds_calculation() {
        let mut index = SpatialIndex::new();

        // Test empty index bounds
        let bounds = index.bounds();
        assert_eq!(bounds, None);

        // Add nodes and test bounds calculation
        let node1 = NodeBuilder::<()>::new("node1")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(50.0, 50.0)
            .size(30.0, 30.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();

        let bounds = index.bounds();
        // Bounds should encompass both nodes: (10,10) to (80,80)
        assert_eq!(bounds, Some(Rect::new(10.0, 10.0, 70.0, 70.0)));
    }

    #[test]
    fn test_spatial_index_distance_calculations() {
        let mut index = SpatialIndex::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(0.0, 0.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(30.0, 40.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();

        // Test distance from center of node1 to center of node2
        let center1 = Position::new(10.0, 10.0); // Center of node1
        let center2 = Position::new(40.0, 50.0); // Center of node2
        let distance = center1.distance_to(center2);

        // Distance should be sqrt(30^2 + 40^2) = 50
        assert_eq!(distance, 50.0);

        // Test radius query with exact distance
        let results = index.query_radius(center1, 50.0);
        assert_eq!(results.len(), 2); // Should include both nodes

        // Test radius query with distance just under
        let results = index.query_radius(center1, 49.9);
        assert_eq!(results.len(), 1); // Should include only node1
    }

    #[test]
    fn test_spatial_index_grid_cell_calculations() {
        let index = SpatialIndex::with_cell_size(100.0);

        // Test grid cell calculation for different positions
        let bounds1 = Rect::new(50.0, 50.0, 20.0, 20.0);
        let cells1 = index.get_grid_cells_for_bounds(&bounds1);
        assert_eq!(cells1.len(), 1);
        assert_eq!(cells1[0], GridCell::new(0, 0));

        // Test grid cell calculation for position at cell boundary
        let bounds2 = Rect::new(100.0, 100.0, 20.0, 20.0);
        let cells2 = index.get_grid_cells_for_bounds(&bounds2);
        assert_eq!(cells2.len(), 1);
        assert_eq!(cells2[0], GridCell::new(1, 1));

        // Test grid cell calculation for negative coordinates
        let bounds3 = Rect::new(-50.0, -50.0, 20.0, 20.0);
        let cells3 = index.get_grid_cells_for_bounds(&bounds3);
        assert_eq!(cells3.len(), 1);
        assert_eq!(cells3[0], GridCell::new(-1, -1));

        // Test grid cell calculation spanning multiple cells
        let bounds4 = Rect::new(50.0, 50.0, 150.0, 150.0);
        let cells4 = index.get_grid_cells_for_bounds(&bounds4);
        assert!(cells4.len() >= 4); // Should span at least 2x2 cells
        assert!(cells4.contains(&GridCell::new(0, 0)));
        assert!(cells4.contains(&GridCell::new(1, 0)));
        assert!(cells4.contains(&GridCell::new(0, 1)));
        assert!(cells4.contains(&GridCell::new(1, 1)));
    }

    #[test]
    fn test_spatial_index_nearest_calculation() {
        let mut index = SpatialIndex::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(0.0, 0.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(100.0, 100.0)
            .size(20.0, 20.0)
            .build();

        let node3 = NodeBuilder::<()>::new("node3")
            .position(50.0, 50.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();
        index.insert(&node3).unwrap();

        // Test nearest to point closest to node1
        let nearest = index.nearest(Position::new(10.0, 10.0));
        assert_eq!(nearest, Some(node1.id.clone()));

        // Test nearest to point closest to node2
        let nearest = index.nearest(Position::new(110.0, 110.0));
        assert_eq!(nearest, Some(node2.id.clone()));

        // Test nearest to point closest to node3
        let nearest = index.nearest(Position::new(60.0, 60.0));
        assert_eq!(nearest, Some(node3.id.clone()));

        // Test nearest with empty index
        let empty_index = SpatialIndex::new();
        let nearest = empty_index.nearest(Position::new(10.0, 10.0));
        assert_eq!(nearest, None);
    }

    #[test]
    fn test_spatial_index_radius_query_calculations() {
        let mut index = SpatialIndex::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(0.0, 0.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(30.0, 40.0)
            .size(20.0, 20.0)
            .build();

        let node3 = NodeBuilder::<()>::new("node3")
            .position(100.0, 100.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();
        index.insert(&node3).unwrap();

        // Test radius query from center of node1
        let center1 = Position::new(10.0, 10.0);
        let results = index.query_radius(center1, 30.0);
        assert_eq!(results.len(), 1); // Only node1 should be within radius

        // Test radius query with larger radius
        let results = index.query_radius(center1, 60.0);
        assert_eq!(results.len(), 2); // node1 and node2 should be within radius

        // Test radius query with very large radius
        let results = index.query_radius(center1, 200.0);
        assert_eq!(results.len(), 3); // All nodes should be within radius

        // Test radius query with zero radius
        let results = index.query_radius(center1, 0.0);
        assert_eq!(results.len(), 1); // Only node1 should be within radius
    }

    #[test]
    fn test_spatial_index_rect_query_calculations() {
        let mut index = SpatialIndex::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(50.0, 50.0)
            .size(20.0, 20.0)
            .build();

        let node3 = NodeBuilder::<()>::new("node3")
            .position(100.0, 100.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();
        index.insert(&node3).unwrap();

        // Test rect query that includes only node1
        let query1 = Rect::new(0.0, 0.0, 40.0, 40.0);
        let results1 = index.query_rect(&query1);
        assert_eq!(results1.len(), 1);
        assert_eq!(results1[0], node1.id.clone());

        // Test rect query that includes node1 and node2
        let query2 = Rect::new(0.0, 0.0, 80.0, 80.0);
        let results2 = index.query_rect(&query2);
        assert_eq!(results2.len(), 2);
        assert!(results2.contains(&node1.id.clone()));
        assert!(results2.contains(&node2.id.clone()));

        // Test rect query that includes all nodes
        let query3 = Rect::new(0.0, 0.0, 130.0, 130.0);
        let results3 = index.query_rect(&query3);
        assert_eq!(results3.len(), 3);

        // Test rect query that includes no nodes
        let query4 = Rect::new(200.0, 200.0, 50.0, 50.0);
        let results4 = index.query_rect(&query4);
        assert_eq!(results4.len(), 0);
    }

    #[test]
    fn test_spatial_index_update_calculations() {
        let mut index = SpatialIndex::new();

        let mut node = NodeBuilder::<()>::new("test-node")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node).unwrap();

        // Verify initial position
        let query1 = Rect::new(0.0, 0.0, 40.0, 40.0);
        let results1 = index.query_rect(&query1);
        assert_eq!(results1.len(), 1);

        // Update node position
        node.set_position(Position::new(100.0, 100.0));
        index.update(&node).unwrap();

        // Verify old position returns no results
        let results2 = index.query_rect(&query1);
        assert_eq!(results2.len(), 0);

        // Verify new position returns the node
        let query2 = Rect::new(90.0, 90.0, 40.0, 40.0);
        let results3 = index.query_rect(&query2);
        assert_eq!(results3.len(), 1);
        assert_eq!(results3[0], node.id.clone());

        // Test update with same position (should not change anything)
        let results4 = index.query_rect(&query2);
        assert_eq!(results4.len(), 1);
    }

    #[test]
    fn test_spatial_index_remove_calculations() {
        let mut index = SpatialIndex::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(50.0, 50.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();

        // Verify both nodes are present
        let query = Rect::new(0.0, 0.0, 80.0, 80.0);
        let results = index.query_rect(&query);
        assert_eq!(results.len(), 2);

        // Remove node1
        let removed = index.remove(&node1.id);
        assert!(removed);

        // Verify only node2 remains
        let results = index.query_rect(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], node2.id.clone());

        // Test removing non-existent node
        let result = index.remove(&"non-existent".into());
        assert!(!result);
    }

    #[test]
    fn test_spatial_index_bulk_operations_calculations() {
        let mut index = SpatialIndex::new();

        let nodes = vec![
            NodeBuilder::<()>::new("node1")
                .position(0.0, 0.0)
                .size(20.0, 20.0)
                .build(),
            NodeBuilder::<()>::new("node2")
                .position(30.0, 30.0)
                .size(20.0, 20.0)
                .build(),
            NodeBuilder::<()>::new("node3")
                .position(60.0, 60.0)
                .size(20.0, 20.0)
                .build(),
        ];

        // Test bulk load
        index.bulk_load(&nodes).unwrap();
        assert_eq!(index.len(), 3);

        // Test individual remove operations
        for node in &nodes {
            index.remove(&node.id);
        }
        assert_eq!(index.len(), 0);

        // Test individual update operations
        index.bulk_load(&nodes).unwrap();
        let mut updated_nodes = nodes.clone();
        for node in &mut updated_nodes {
            node.set_position(Position::new(node.position.x + 100.0, node.position.y + 100.0));
            index.update(node).unwrap();
        }

        // Verify positions were updated
        let query = Rect::new(90.0, 90.0, 100.0, 100.0);
        let results = index.query_rect(&query);
        assert_eq!(results.len(), 3);
    }

    // Edge case tests for boundary conditions (mutation testing coverage)

    #[test]
    fn test_spatial_index_edge_cases() {
        let mut index = SpatialIndex::new();

        // Test with very small node
        let tiny_node = NodeBuilder::<()>::new("tiny")
            .position(0.0, 0.0)
            .size(1e-10, 1e-10)
            .build();

        index.insert(&tiny_node).unwrap();
        assert_eq!(index.len(), 1);

        // Test with very large node
        let huge_node = NodeBuilder::<()>::new("huge")
            .position(0.0, 0.0)
            .size(1e10, 1e10)
            .build();

        index.insert(&huge_node).unwrap();
        assert_eq!(index.len(), 2);

        // Test with negative position
        let neg_node = NodeBuilder::<()>::new("negative")
            .position(-1e10, -1e10)
            .size(100.0, 100.0)
            .build();

        index.insert(&neg_node).unwrap();
        assert_eq!(index.len(), 3);
    }

    #[test]
    fn test_spatial_index_query_edge_cases() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test")
            .position(100.0, 100.0)
            .size(50.0, 50.0)
            .build();

        index.insert(&node).unwrap();

        // Test query with zero radius
        let results_zero = index.query_radius(Position::new(125.0, 125.0), 0.0);
        assert_eq!(results_zero.len(), 1); // Should include the node

        // Test query with very large radius
        let results_huge = index.query_radius(Position::new(0.0, 0.0), 1e10);
        assert_eq!(results_huge.len(), 1);

        // Test query with negative radius
        let results_neg = index.query_radius(Position::new(125.0, 125.0), -100.0);
        assert_eq!(results_neg.len(), 0);

        // Test query with zero-size rectangle
        let zero_rect = Rect::new(125.0, 125.0, 0.0, 0.0);
        let results_zero_rect = index.query_rect(&zero_rect);
        assert_eq!(results_zero_rect.len(), 1);

        // Test query with very large rectangle
        let huge_rect = Rect::new(0.0, 0.0, 1e10, 1e10);
        let results_huge_rect = index.query_rect(&huge_rect);
        assert_eq!(results_huge_rect.len(), 1);

        // Test query with negative rectangle
        let neg_rect = Rect::new(-1e10, -1e10, 1e10, 1e10);
        let results_neg_rect = index.query_rect(&neg_rect);
        assert_eq!(results_neg_rect.len(), 1);
    }

    #[test]
    fn test_spatial_index_nearest_edge_cases() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test")
            .position(100.0, 100.0)
            .size(50.0, 50.0)
            .build();

        index.insert(&node).unwrap();

        // Test nearest with very far point
        let nearest_far = index.nearest(Position::new(1e10, 1e10));
        assert_eq!(nearest_far, Some(node.id.clone()));

        // Test nearest with very close point
        let nearest_close = index.nearest(Position::new(125.0, 125.0));
        assert_eq!(nearest_close, Some(node.id.clone()));

        // Test nearest with negative coordinates
        let nearest_neg = index.nearest(Position::new(-1e10, -1e10));
        assert_eq!(nearest_neg, Some(node.id.clone()));

        // Test nearest with zero coordinates
        let nearest_zero = index.nearest(Position::new(0.0, 0.0));
        assert_eq!(nearest_zero, Some(node.id.clone()));
    }

    #[test]
    fn test_spatial_index_update_edge_cases() {
        let mut index = SpatialIndex::new();

        let mut node = NodeBuilder::<()>::new("test")
            .position(100.0, 100.0)
            .size(50.0, 50.0)
            .build();

        index.insert(&node).unwrap();

        // Test update to same position
        index.update(&node).unwrap();
        assert_eq!(index.len(), 1);

        // Test update to very large position
        node.set_position(Position::new(1e10, 1e10));
        index.update(&node).unwrap();
        assert_eq!(index.len(), 1);

        // Test update to negative position
        node.set_position(Position::new(-1e10, -1e10));
        index.update(&node).unwrap();
        assert_eq!(index.len(), 1);

        // Test update to zero position
        node.set_position(Position::new(0.0, 0.0));
        index.update(&node).unwrap();
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn test_spatial_index_remove_edge_cases() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test")
            .position(100.0, 100.0)
            .size(50.0, 50.0)
            .build();

        index.insert(&node).unwrap();

        // Test remove non-existent node
        let removed = index.remove(&"non-existent".into());
        assert!(!removed);
        assert_eq!(index.len(), 1);

        // Test remove existing node
        let removed_existing = index.remove(&node.id);
        assert!(removed_existing);
        assert_eq!(index.len(), 0);

        // Test remove from empty index
        let removed_empty = index.remove(&node.id);
        assert!(!removed_empty);
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_spatial_index_bulk_operations_edge_cases() {
        let mut index = SpatialIndex::new();

        // Test bulk_load with empty vector
        let empty_nodes: Vec<Node<()>> = vec![];
        let result = index.bulk_load(&empty_nodes);
        assert!(result.is_ok());
        assert_eq!(index.len(), 0);

        // Test bulk_load with single node
        let single_node = NodeBuilder::<()>::new("single")
            .position(0.0, 0.0)
            .size(10.0, 10.0)
            .build();

        let result_single = index.bulk_load(&vec![single_node]);
        assert!(result_single.is_ok());
        assert_eq!(index.len(), 1);

        // Test bulk_load with very large number of nodes
        let mut many_nodes = Vec::new();
        for i in 0..1000 {
            many_nodes.push(NodeBuilder::<()>::new(format!("node_{}", i))
                .position(i as f64, i as f64)
                .size(10.0, 10.0)
                .build());
        }

        let result_many = index.bulk_load(&many_nodes);
        assert!(result_many.is_ok());
        assert_eq!(index.len(), 1001); // 1 from previous + 1000 new
    }

    #[test]
    fn test_grid_cell_edge_cases() {
        let index = SpatialIndex::with_cell_size(100.0);

        // Test with zero-size bounds
        let zero_bounds = Rect::new(50.0, 50.0, 0.0, 0.0);
        let cells_zero = index.get_grid_cells_for_bounds(&zero_bounds);
        assert_eq!(cells_zero.len(), 1);

        // Test with very small bounds
        let tiny_bounds = Rect::new(50.0, 50.0, 1e-10, 1e-10);
        let cells_tiny = index.get_grid_cells_for_bounds(&tiny_bounds);
        assert_eq!(cells_tiny.len(), 1);

        // Test with very large bounds
        let huge_bounds = Rect::new(0.0, 0.0, 1e10, 1e10);
        let cells_huge = index.get_grid_cells_for_bounds(&huge_bounds);
        assert!(cells_huge.len() > 1000); // Should span many cells

        // Test with negative bounds
        let neg_bounds = Rect::new(-1e10, -1e10, 1e10, 1e10);
        let cells_neg = index.get_grid_cells_for_bounds(&neg_bounds);
        assert!(cells_neg.len() > 1000); // Should span many cells

        // Test with very small cell size
        let index_tiny = SpatialIndex::with_cell_size(1e-10);
        let normal_bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        let cells_normal = index_tiny.get_grid_cells_for_bounds(&normal_bounds);
        assert!(cells_normal.len() > 1000); // Should span many tiny cells
    }
}
