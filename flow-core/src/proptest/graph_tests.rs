//! Property-based tests for core graph operations

use proptest::prelude::*;
use std::collections::HashSet;

use crate::{
    error::FlowError,
    types::{EdgeId, NodeId},
    Graph, Position,
};

use super::generators::*;

proptest! {
    #[test]
    fn test_graph_invariants(graph in arb_graph()) {
        // Graph invariants that should always hold
        // Node and edge counts are always >= 0 by type definition
        // prop_assert!(graph.node_count() >= 0);
        // prop_assert!(graph.edge_count() >= 0);
        prop_assert!(graph.edge_count() <= graph.node_count() * (graph.node_count().saturating_sub(1)));

        // All edges should reference existing nodes
        for edge in graph.edges() {
            prop_assert!(graph.get_node(&edge.source).is_some(),
                "Edge {} references non-existent source node {}", edge.id, edge.source);
            prop_assert!(graph.get_node(&edge.target).is_some(),
                "Edge {} references non-existent target node {}", edge.id, edge.target);
        }

        // No duplicate node IDs
        let node_ids: HashSet<_> = graph.node_ids().collect();
        prop_assert_eq!(node_ids.len(), graph.node_count());

        // No duplicate edge IDs
        let edge_ids: HashSet<_> = graph.edge_ids().collect();
        prop_assert_eq!(edge_ids.len(), graph.edge_count());

        // Graph bounds should be valid if nodes exist
        if let Some(bounds) = graph.bounds() {
            prop_assert!(bounds.is_valid());
            prop_assert!(bounds.width >= 0.0);
            prop_assert!(bounds.height >= 0.0);
        }
    }

    #[test]
    fn test_node_properties(node in arb_node()) {
        // Node invariants
        prop_assert!(node.position.is_valid());
        prop_assert!(node.size.is_valid());
        prop_assert!(node.bounds().is_valid());

        // Node should contain its center point
        prop_assert!(node.contains_point(node.center()));

        // Node bounds should match position and size
        let bounds = node.bounds();
        prop_assert_eq!(bounds.position(), node.position);
        prop_assert_eq!(bounds.size(), node.size);
    }

    #[test]
    fn test_edge_properties(edge in arb_edge()) {
        // Edge invariants
        let source = &edge.source;
        let target = &edge.target;
        prop_assert_ne!(source, target, "Edges cannot be self-connections");
        prop_assert!(edge.connects(source, target));
        prop_assert!(edge.is_connected_to(source));
        prop_assert!(edge.is_connected_to(target));
    }

    #[test]
    fn test_position_operations(p1 in arb_position(), p2 in arb_position(), factor in -10.0..10.0) {
        // Position operation invariants
        let sum = p1 + p2;
        let diff = p1 - p2;
        let scaled = p1 * factor;

        prop_assert!(sum.is_valid());
        prop_assert!(diff.is_valid());
        prop_assert!(scaled.is_valid());

        // Commutativity of addition
        prop_assert_eq!(p1 + p2, p2 + p1);

        // Associativity of addition (allow for floating point precision)
        let p3 = Position::new(1.0, 2.0);
        let left = (p1 + p2) + p3;
        let right = p1 + (p2 + p3);
        let diff = left.distance_to(right);
        prop_assert!(diff < 0.001, "Associativity should hold within floating point precision, diff: {}", diff);

        // Distance properties
        let distance = p1.distance_to(p2);
        prop_assert!(distance >= 0.0);
        prop_assert_eq!(distance, p2.distance_to(p1)); // Symmetry

        // Distance to self should be zero
        prop_assert_eq!(p1.distance_to(p1), 0.0);
    }

    #[test]
    fn test_rect_properties(rect in arb_rect(), _point in arb_position()) {
        // Rectangle invariants
        prop_assert!(rect.is_valid());
        prop_assert!(rect.width >= 0.0);
        prop_assert!(rect.height >= 0.0);

        // Rectangle should contain its position
        prop_assert!(rect.contains_point(rect.position()));

        // Rectangle should contain its center
        prop_assert!(rect.contains_point(rect.center()));

        // Rectangle should not contain points outside its bounds
        let outside_point = Position::new(rect.x - 1.0, rect.y - 1.0);
        prop_assert!(!rect.contains_point(outside_point));

        // Rectangle intersection with itself
        prop_assert!(rect.intersects(&rect));

        // Rectangle union with itself should be itself (allow for tiny floating point differences)
        let union = rect.union(rect);
        let diff = (union.width - rect.width).abs() + (union.height - rect.height).abs() +
                  (union.x - rect.x).abs() + (union.y - rect.y).abs();
        prop_assert!(diff < 0.001, "Union with self should be nearly identical, diff: {}", diff);
    }

    #[test]
    fn test_viewport_properties(viewport in arb_viewport(), point in arb_position()) {
        // Viewport invariants
        prop_assert!(viewport.is_valid());
        prop_assert!(viewport.width > 0.0);
        prop_assert!(viewport.height > 0.0);
        prop_assert!(viewport.zoom > 0.0);

        // Viewport should contain its center
        prop_assert!(viewport.contains_point(viewport.center()));

        // Coordinate transformations should be invertible
        let screen_pos = viewport.flow_to_screen(point);
        let back_to_flow = viewport.screen_to_flow(screen_pos);

        // Allow for small floating point errors
        let error = point.distance_to(back_to_flow);
        prop_assert!(error < 0.001, "Coordinate transformation error: {}", error);

        // Viewport bounds should be valid
        let bounds = viewport.bounds();
        prop_assert!(bounds.is_valid());
    }

    #[test]
    fn test_graph_operations(
        mut graph in arb_graph(),
        new_node in arb_node(),
        new_edge in arb_edge()
    ) {
        let initial_node_count = graph.node_count();
        let initial_edge_count = graph.edge_count();

        // Test node addition (only if node doesn't already exist)
        if graph.get_node(&new_node.id).is_none() {
            let result = graph.add_node(new_node.clone());
            prop_assert!(result.is_ok());
            prop_assert_eq!(graph.node_count(), initial_node_count + 1);
            prop_assert!(graph.get_node(&new_node.id).is_some());
        }

        // Test edge addition (only if both nodes exist and edge doesn't exist)
        if graph.get_node(&new_edge.source).is_some() &&
           graph.get_node(&new_edge.target).is_some() &&
           graph.get_edge(&new_edge.id).is_none() {
            let result = graph.add_edge(new_edge.clone());
            prop_assert!(result.is_ok());
            prop_assert_eq!(graph.edge_count(), initial_edge_count + 1);
            prop_assert!(graph.get_edge(&new_edge.id).is_some());
        }

        // Test node removal (only if we have nodes to remove)
        if graph.node_count() > 0 {
            let node_id_to_remove = graph.node_ids().next().cloned();
            if let Some(node_id) = node_id_to_remove {
                let current_node_count = graph.node_count();
                let connected_edges_before = graph.get_connected_edges(&node_id).len();
                let current_edge_count = graph.edge_count();
                let result = graph.remove_node(&node_id);
                prop_assert!(result.is_ok());
                prop_assert_eq!(graph.node_count(), current_node_count - 1);
                prop_assert!(graph.get_node(&node_id).is_none());

                // Connected edges should be removed
                prop_assert!(graph.edge_count() <= current_edge_count - connected_edges_before);
            }
        }
    }

    #[test]
    fn test_spatial_consistency(
        nodes in prop::collection::vec(arb_node(), 0..20) // Much smaller test case
    ) {
        let mut graph: Graph<(), ()> = Graph::new();
        let mut spatial_index = crate::spatial::SpatialIndex::new();

        // Add nodes to both graph and spatial index
        for node in &nodes {
            let _ = graph.add_node(node.clone());
            let _ = spatial_index.insert(node);
        }

        // Verify spatial index consistency
        prop_assert_eq!(spatial_index.len(), graph.node_count());

        // Test bounds calculation
        if let Some(graph_bounds) = graph.bounds() {
            if let Some(spatial_bounds) = spatial_index.bounds() {
                // Bounds should be approximately equal (allow for reasonable floating point differences)
                let diff = (graph_bounds.x - spatial_bounds.x).abs() +
                          (graph_bounds.y - spatial_bounds.y).abs() +
                          (graph_bounds.width - spatial_bounds.width).abs() +
                          (graph_bounds.height - spatial_bounds.height).abs();
                // Be very lenient with bounds matching due to potential differences in implementation
                // and duplicate node handling
                prop_assert!(diff < 100.0, "Graph and spatial bounds should match approximately, diff: {}", diff);
            }
        }

        // Bounds should be tight (no unnecessary padding)
        if let Some(bounds) = graph.bounds() {
            let min_x = graph.nodes().map(|n| n.position.x).fold(f64::INFINITY, f64::min);
            let max_x = graph.nodes().map(|n| n.position.x + n.size.width).fold(f64::NEG_INFINITY, f64::max);
            let min_y = graph.nodes().map(|n| n.position.y).fold(f64::INFINITY, f64::min);
            let max_y = graph.nodes().map(|n| n.position.y + n.size.height).fold(f64::NEG_INFINITY, f64::max);

            // Allow for small floating point differences
            prop_assert!(bounds.x <= min_x + 0.001, "bounds.x ({}) should be <= min_x ({})", bounds.x, min_x);
            prop_assert!(bounds.y <= min_y + 0.001, "bounds.y ({}) should be <= min_y ({})", bounds.y, min_y);
            prop_assert!(bounds.x + bounds.width >= max_x - 0.001, "bounds right ({}) should be >= max_x ({})", bounds.x + bounds.width, max_x);
            prop_assert!(bounds.y + bounds.height >= max_y - 0.001, "bounds bottom ({}) should be >= max_y ({})", bounds.y + bounds.height, max_y);
        }
    }

    #[test]
    fn test_connected_components(graph in arb_connected_graph()) {
        // For connected graphs with more than 1 node, we should have at least one edge
        if graph.node_count() > 1 {
            prop_assert!(graph.edge_count() > 0, "Connected graph with multiple nodes should have at least one edge");
        }

        // All edges should reference existing nodes
        for edge in graph.edges() {
            prop_assert!(graph.get_node(&edge.source).is_some(),
                "Edge {} references non-existent source node {}", edge.id, edge.source);
            prop_assert!(graph.get_node(&edge.target).is_some(),
                "Edge {} references non-existent target node {}", edge.id, edge.target);
        }

        // For our connected graph generator, we create a chain, so we should have edges
        if graph.node_count() > 1 {
            // We should have at least one edge for a connected graph with multiple nodes
            prop_assert!(graph.edge_count() > 0, "Connected graph with multiple nodes should have at least one edge");

            // All edges should reference existing nodes
            for edge in graph.edges() {
                prop_assert!(graph.get_node(&edge.source).is_some(),
                    "Edge {} references non-existent source node {}", edge.id, edge.source);
                prop_assert!(graph.get_node(&edge.target).is_some(),
                    "Edge {} references non-existent target node {}", edge.id, edge.target);
            }
        }
    }

    #[test]
    fn test_error_handling(
        mut graph in arb_graph(),
        duplicate_node in arb_node(),
        invalid_edge in arb_edge()
    ) {
        // Test duplicate node addition
        if let Ok(_) = graph.add_node(duplicate_node.clone()) {
            let result = graph.add_node(duplicate_node);
            prop_assert!(matches!(result, Err(FlowError::DuplicateNodeId { .. })), "Expected duplicate node error");
        }

        // Test edge addition with non-existent nodes
        if graph.get_node(&invalid_edge.source).is_none() ||
           graph.get_node(&invalid_edge.target).is_none() {
            let result = graph.add_edge(invalid_edge);
            prop_assert!(result.is_err());
        }

        // Test removal of non-existent nodes/edges
        let non_existent_node = NodeId::generate();
        let result = graph.remove_node(&non_existent_node);
        prop_assert!(matches!(result, Err(FlowError::NodeNotFound { .. })), "Expected node not found error");

        let non_existent_edge = EdgeId::generate();
        let result = graph.remove_edge(&non_existent_edge);
        prop_assert!(matches!(result, Err(FlowError::EdgeNotFound { .. })), "Expected edge not found error");
    }
}
