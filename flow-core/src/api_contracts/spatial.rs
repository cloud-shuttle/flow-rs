//! Spatial Index API Contracts

use crate::graph::Node;
use crate::spatial::SpatialIndex;
use crate::types::{NodeId, Position, Rect};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_index_api_contract() {
        let mut index = SpatialIndex::new();

        // Test empty index
        assert_eq!(index.len(), 0);
        assert!(index.is_empty());

        // Test insertion
        let node = Node::new("test_node", Position::new(10.0, 20.0), ());
        index.insert(&node).unwrap();

        assert_eq!(index.len(), 1);
        assert!(!index.is_empty());

        // Test querying
        let query_rect = Rect::new(50.0, 30.0, 20.0, 20.0);
        let results = index.query_rect(&query_rect);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], node.id);

        // Test removal
        index.remove(&node.id);
        assert_eq!(index.len(), 0);
        assert!(index.is_empty());

        // Test query after removal
        let results = index.query_rect(&query_rect);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_spatial_index_multiple_items_api_contract() {
        let mut index = SpatialIndex::new();

        // Insert multiple items
        let node1 = Node::new("node1", Position::new(0.0, 0.0), ());
        let node2 = Node::new("node2", Position::new(100.0, 100.0), ());
        let node3 = Node::new("node3", Position::new(25.0, 25.0), ());

        index.insert(&node1).unwrap();
        index.insert(&node2).unwrap();
        index.insert(&node3).unwrap();

        assert_eq!(index.len(), 3);

        // Test query that intersects with multiple items
        let query_rect = Rect::new(30.0, 30.0, 20.0, 20.0);
        let results = index.query_rect(&query_rect);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&node1.id));
        assert!(results.contains(&node3.id));

        // Test query that intersects with one item
        let query_rect2 = Rect::new(10.0, 10.0, 20.0, 20.0);
        let results2 = index.query_rect(&query_rect2);
        // This query should intersect with both node1 and node3
        assert_eq!(results2.len(), 2);
        assert!(results2.contains(&node1.id));
        assert!(results2.contains(&node3.id));

        // Test query that intersects with no items
        let query_rect3 = Rect::new(200.0, 200.0, 20.0, 20.0);
        let results3 = index.query_rect(&query_rect3);
        assert_eq!(results3.len(), 0);
    }

    #[test]
    fn test_spatial_index_update_api_contract() {
        let mut index = SpatialIndex::new();

        let mut node = Node::new("test_node", Position::new(10.0, 20.0), ());
        index.insert(&node).unwrap();

        // Test update
        node.set_position(Position::new(200.0, 300.0));
        index.update(&node).unwrap();

        // Test query with old rect (should not find)
        let old_query = Rect::new(50.0, 30.0, 20.0, 20.0);
        let old_results = index.query_rect(&old_query);
        assert_eq!(old_results.len(), 0);

        // Test query with new rect (should find)
        let new_query = Rect::new(220.0, 310.0, 20.0, 20.0);
        let new_results = index.query_rect(&new_query);
        assert_eq!(new_results.len(), 1);
        assert_eq!(new_results[0], node.id);
    }
}
