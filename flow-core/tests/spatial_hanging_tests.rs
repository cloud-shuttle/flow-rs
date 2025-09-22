//! Tests to identify and fix hanging issues in spatial indexing
//!
//! These tests are designed to detect infinite loops and resource exhaustion
//! in the spatial indexing algorithms.

use flow_rs_core::graph::NodeBuilder;
use flow_rs_core::spatial::SpatialIndex;
use flow_rs_core::types::Rect;
use std::time::{Duration, Instant};

#[test]
fn test_spatial_index_handles_extreme_bounds() {
    let index = SpatialIndex::new();

    // Test with extreme bounds that could cause infinite loops
    let extreme_bounds = Rect::new(
        -1000000.0, // Very negative x
        -1000000.0, // Very negative y
        2000000.0,  // Very large width
        2000000.0,  // Very large height
    );

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // This should not hang
    let results = index.query_rect(&extreme_bounds);

    let elapsed = start_time.elapsed();
    assert!(elapsed < timeout, "Query took too long: {:?}", elapsed);
    assert_eq!(results.len(), 0); // Should be empty for new index
}

#[test]
fn test_spatial_index_handles_infinite_values() {
    let index = SpatialIndex::new();

    // Test with infinite values
    let infinite_bounds = Rect::new(
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::INFINITY,
    );

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // This should not hang or panic
    let results = index.query_rect(&infinite_bounds);

    let elapsed = start_time.elapsed();
    assert!(elapsed < timeout, "Query took too long: {:?}", elapsed);
    assert_eq!(results.len(), 0); // Should be empty for new index
}

#[test]
fn test_spatial_index_handles_nan_values() {
    let index = SpatialIndex::new();

    // Test with NaN values
    let nan_bounds = Rect::new(f64::NAN, f64::NAN, f64::NAN, f64::NAN);

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // This should not hang or panic
    let results = index.query_rect(&nan_bounds);

    let elapsed = start_time.elapsed();
    assert!(elapsed < timeout, "Query took too long: {:?}", elapsed);
    assert_eq!(results.len(), 0); // Should be empty for new index
}

#[test]
fn test_spatial_index_handles_very_small_cell_size() {
    // Create index with very small cell size that could cause many grid cells
    let index = SpatialIndex::with_cell_size(0.001); // 0.001 pixel cells

    let large_bounds = Rect::new(0.0, 0.0, 1000.0, 1000.0);

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // This should not hang even with many grid cells
    let results = index.query_rect(&large_bounds);

    let elapsed = start_time.elapsed();
    assert!(elapsed < timeout, "Query took too long: {:?}", elapsed);
    assert_eq!(results.len(), 0); // Should be empty for new index
}

#[test]
fn test_spatial_index_bounds_calculation_with_extreme_values() {
    let mut index = SpatialIndex::new();

    // Add a node with extreme position
    let node = NodeBuilder::<()>::new("extreme-node")
        .position(f64::MAX / 2.0, f64::MAX / 2.0)
        .size(1.0, 1.0)
        .build();

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // This should not hang
    let result = index.insert(&node);

    let elapsed = start_time.elapsed();
    assert!(elapsed < timeout, "Insert took too long: {:?}", elapsed);
    assert!(result.is_ok());

    // Test bounds calculation
    let bounds = index.bounds();
    assert!(bounds.is_some());
}

#[test]
fn test_spatial_index_maximum_grid_cells_limit() {
    let index = SpatialIndex::new();

    // Create bounds that would generate a reasonable number of grid cells
    let reasonable_bounds = Rect::new(0.0, 0.0, 10000.0, 10000.0); // 100x100 cells with default cell size

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // This should complete quickly
    let results = index.query_rect(&reasonable_bounds);

    let elapsed = start_time.elapsed();
    assert!(elapsed < timeout, "Query took too long: {:?}", elapsed);
    assert_eq!(results.len(), 0); // Should be empty for new index
}

#[test]
fn test_spatial_index_proptest_reproduction() {
    // This test reproduces the specific issue from the hanging proptest
    let mut index = SpatialIndex::new();

    // Create nodes with extreme positions that could cause issues
    let nodes = vec![
        NodeBuilder::<()>::new("node1")
            .position(392.7340132183363, 535.701616739003)
            .size(1.0, 1.0)
            .build(),
        NodeBuilder::<()>::new("node2")
            .position(296.6246075471901, 627.9501230077146)
            .size(1.0, 1.0)
            .build(),
        NodeBuilder::<()>::new("node3")
            .position(320.92346936459757, 599.6590195394173)
            .size(1.0, 1.0)
            .build(),
    ];

    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // Insert all nodes
    for node in &nodes {
        let result = index.insert(node);
        assert!(result.is_ok());
    }

    // Test spatial consistency query
    let viewport_bounds = Rect::new(-1000.0, -1000.0, 2000.0, 2000.0);
    let results = index.query_rect(&viewport_bounds);

    let elapsed = start_time.elapsed();
    assert!(
        elapsed < timeout,
        "Spatial consistency test took too long: {:?}",
        elapsed
    );
    assert_eq!(results.len(), 3); // Should find all 3 nodes
}
