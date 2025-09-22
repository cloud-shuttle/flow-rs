//! Tests for spatial indexing

#[cfg(test)]
mod tests {
    use crate::graph::NodeBuilder;
    use crate::types::{Position, Rect, Viewport};
    
    use crate::spatial::SpatialIndex;
    use crate::spatial::SpatialQuery;

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
    }

    #[test]
    fn test_remove_node() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test-node")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node).unwrap();
        assert_eq!(index.len(), 1);

        // Remove the node
        let removed = index.remove(&node.id);
        assert!(removed);
        assert_eq!(index.len(), 0);

        // Query should return no results
        let query_bounds = Rect::new(0.0, 0.0, 50.0, 50.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 0);
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

        // Test multiple cells
        let bounds = Rect::new(10.0, 10.0, 100.0, 100.0);
        let cells = index.get_grid_cells_for_bounds(&bounds);
        assert_eq!(cells.len(), 9); // 3x3 grid (from cell 0,0 to cell 2,2)
    }

    #[test]
    fn test_viewport_query() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test-node")
            .position(50.0, 50.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node).unwrap();

        // Create viewport that includes the node
        let viewport = Viewport::with_size(0.0, 0.0, 100.0, 100.0);
        let results = index.query_viewport(&viewport);
        assert_eq!(results.len(), 1);

        // Create viewport that doesn't include the node
        let viewport = Viewport::with_size(200.0, 200.0, 50.0, 50.0);
        let results = index.query_viewport(&viewport);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_radius_query() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test-node")
            .position(50.0, 50.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node).unwrap();

        // Query with radius that includes the node
        let results = index.query_radius(Position::new(60.0, 60.0), 20.0);
        assert_eq!(results.len(), 1);

        // Query with radius that doesn't include the node
        let results = index.query_radius(Position::new(100.0, 100.0), 10.0);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_nearest_query() {
        let mut index = SpatialIndex::new();

        let node1 = NodeBuilder::<()>::new("node1")
            .position(10.0, 10.0)
            .size(20.0, 20.0)
            .build();

        let node2 = NodeBuilder::<()>::new("node2")
            .position(100.0, 100.0)
            .size(20.0, 20.0)
            .build();

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();

        // Query nearest to node1
        let nearest = index.nearest(Position::new(20.0, 20.0));
        assert_eq!(nearest, Some(node1.id.clone()));

        // Query nearest to node2
        let nearest = index.nearest(Position::new(110.0, 110.0));
        assert_eq!(nearest, Some(node2.id.clone()));
    }

    #[test]
    fn test_bounds_calculation() {
        let mut index = SpatialIndex::new();

        // Empty index should have no bounds
        assert_eq!(index.bounds(), None);

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

        let bounds = index.bounds().unwrap();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 10.0);
        assert_eq!(bounds.width, 70.0); // 50 + 30 - 10
        assert_eq!(bounds.height, 70.0); // 50 + 30 - 10
    }

    #[test]
    fn test_custom_cell_size() {
        let index = SpatialIndex::with_cell_size(25.0);
        assert_eq!(index.cell_size(), 25.0);
    }

    #[test]
    fn test_default_implementation() {
        let index = SpatialIndex::default();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        assert_eq!(index.cell_size(), 100.0);
    }

    #[test]
    fn test_spatial_index_nan_infinity_handling() {
        let index = SpatialIndex::new();

        // Test bounds with NaN values
        let nan_bounds = Rect::new(f64::NAN, 10.0, 50.0, 50.0);
        let cells_nan = index.get_grid_cells_for_bounds(&nan_bounds);
        assert_eq!(cells_nan.len(), 0);

        // Test bounds with infinity values
        let inf_bounds = Rect::new(f64::INFINITY, 10.0, 50.0, 50.0);
        let cells_inf = index.get_grid_cells_for_bounds(&inf_bounds);
        assert_eq!(cells_inf.len(), 0);

        // Test bounds with negative infinity
        let neg_inf_bounds = Rect::new(f64::NEG_INFINITY, 10.0, 50.0, 50.0);
        let cells_neg_inf = index.get_grid_cells_for_bounds(&neg_inf_bounds);
        assert_eq!(cells_neg_inf.len(), 0);

        // Test query with NaN position
        let nan_pos = Position::new(f64::NAN, 10.0);
        let nan_results = index.query_radius(nan_pos, 50.0);
        assert_eq!(nan_results.len(), 0);
    }

    #[test]
    fn test_spatial_index_viewport_query_edge_cases() {
        let mut index = SpatialIndex::new();

        let node = NodeBuilder::<()>::new("test-node")
            .position(100.0, 100.0)
            .size(50.0, 50.0)
            .build();

        index.insert(&node).unwrap();

        // Test viewport query with various viewport configurations
        let viewport1 = Viewport::with_size(0.0, 0.0, 200.0, 200.0);
        let results1 = index.query_viewport(&viewport1);
        assert_eq!(results1.len(), 1);

        // Test viewport query that doesn't include the node
        let viewport2 = Viewport::with_size(200.0, 200.0, 100.0, 100.0);
        let results2 = index.query_viewport(&viewport2);
        assert_eq!(results2.len(), 0);

        // Test viewport query with very small viewport
        let viewport3 = Viewport::with_size(125.0, 125.0, 1.0, 1.0);
        let results3 = index.query_viewport(&viewport3);
        assert_eq!(results3.len(), 1);
    }

    #[test]
    fn test_spatial_query_builder_edge_cases() {
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

        // Test query with no constraints (should return all nodes)
        let results = SpatialQuery::new(&index).execute();
        assert_eq!(results.len(), 3);

        // Test query with limit larger than available results
        let results = SpatialQuery::new(&index).limit(10).execute();
        assert_eq!(results.len(), 3);

        // Test query with limit of 0
        let results = SpatialQuery::new(&index).limit(0).execute();
        assert_eq!(results.len(), 0);

        // Test query with both bounds and radius (bounds should take precedence)
        let results = SpatialQuery::new(&index)
            .bounds(Rect::new(0.0, 0.0, 40.0, 40.0))
            .radius(Position::new(60.0, 60.0), 100.0)
            .execute();
        assert_eq!(results.len(), 1); // Only node1 should be in bounds
    }

    #[test]
    fn test_spatial_index_zero_negative_size_bounds() {
        let index = SpatialIndex::new();

        // Test with negative width (should return 0 due to validation)
        let neg_width_bounds = Rect::new(50.0, 50.0, -20.0, 20.0);
        let cells_neg_width = index.get_grid_cells_for_bounds(&neg_width_bounds);
        assert_eq!(cells_neg_width.len(), 0);

        // Test with negative height (should return 0 due to validation)
        let neg_height_bounds = Rect::new(50.0, 50.0, 20.0, -20.0);
        let cells_neg_height = index.get_grid_cells_for_bounds(&neg_height_bounds);
        assert_eq!(cells_neg_height.len(), 0);

        // Test with both negative dimensions (should return 0 due to validation)
        let neg_both_bounds = Rect::new(50.0, 50.0, -20.0, -20.0);
        let cells_neg_both = index.get_grid_cells_for_bounds(&neg_both_bounds);
        assert_eq!(cells_neg_both.len(), 0);
    }

    #[test]
    fn test_spatial_index_clear_functionality() {
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
        assert_eq!(index.len(), 2);

        // Clear the index
        index.clear();
        assert_eq!(index.len(), 0);
        assert!(index.is_empty());

        // Verify queries return empty results after clear
        let query_bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        let results = index.query_rect(&query_bounds);
        assert_eq!(results.len(), 0);

        // Verify bounds are None after clear
        let bounds = index.bounds();
        assert_eq!(bounds, None);
    }
}
