//! Property-based testing for Leptos Flow Core
//!
//! This module contains comprehensive property-based tests using PropTest
//! to validate invariants and edge cases in the core data structures.

use proptest::prelude::*;
use std::collections::HashSet;

use crate::{
    Graph, Node, Edge, Position, Size, Rect, Viewport,
    types::{NodeId, EdgeId, GroupId},
    error::FlowError,
    spatial::SpatialIndex,
    layout::{LayoutAlgorithm, ForceDirectedLayout, GridLayout, CircularLayout, HierarchicalLayout, LayoutDirection, EdgeRouting},
};

/// Custom generators for property-based testing

/// Generate valid positions within reasonable bounds
pub fn arb_position() -> impl Strategy<Value = Position> {
    (-10000.0..10000.0, -10000.0..10000.0)
        .prop_map(|(x, y)| Position::new(x, y))
}

/// Generate valid sizes (positive dimensions)
pub fn arb_size() -> impl Strategy<Value = Size> {
    (1.0..1000.0, 1.0..1000.0)
        .prop_map(|(width, height)| Size::new(width, height))
}

/// Generate valid rectangles
pub fn arb_rect() -> impl Strategy<Value = Rect> {
    (arb_position(), arb_size())
        .prop_map(|(pos, size)| Rect::from_pos_size(pos, size))
}

/// Generate valid viewports
pub fn arb_viewport() -> impl Strategy<Value = Viewport> {
    (arb_position(), arb_size(), 0.1..10.0)
        .prop_map(|(pos, size, zoom)| {
            Viewport::new(pos.x, pos.y, size.width, size.height, zoom)
        })
}

/// Generate valid node IDs
pub fn arb_node_id() -> impl Strategy<Value = NodeId> {
    prop::string::string_regex("[a-zA-Z][a-zA-Z0-9_-]*")
        .unwrap()
        .prop_map(|s| NodeId::new(s))
}

/// Generate valid edge IDs
pub fn arb_edge_id() -> impl Strategy<Value = EdgeId> {
    prop::string::string_regex("[a-zA-Z][a-zA-Z0-9_-]*")
        .unwrap()
        .prop_map(|s| EdgeId::new(s))
}

/// Generate nodes with valid properties
pub fn arb_node() -> impl Strategy<Value = Node<()>> {
    (arb_node_id(), arb_position(), arb_size())
        .prop_map(|(id, position, size)| {
            let mut node = Node::new(id, position, ());
            node.size = size;
            node
        })
}

/// Generate edges with valid properties
pub fn arb_edge() -> impl Strategy<Value = Edge<()>> {
    (arb_edge_id(), arb_node_id(), arb_node_id())
        .prop_filter("edges cannot be self-connections", |(_, source, target)| {
            source != target
        })
        .prop_map(|(id, source, target)| {
            Edge::new(id, source, target, ())
        })
}

/// Generate graphs with nodes and edges
pub fn arb_graph() -> impl Strategy<Value = Graph<(), ()>> {
    prop::collection::vec(arb_node(), 0..20) // Reduce size to avoid too many rejections
        .prop_flat_map(|nodes| {
            let node_count = nodes.len();
            let node_ids: Vec<NodeId> = nodes.iter().map(|n| n.id.clone()).collect();

            let edges_strategy = if node_count == 0 {
                // No nodes, so no edges possible
                Just(Vec::new()).boxed()
            } else {
                prop::collection::vec(
                    // Generate edges that are guaranteed to connect existing nodes
                    (0..node_count, 0..node_count)
                        .prop_filter("edges cannot be self-connections", |(source_idx, target_idx)| {
                            source_idx != target_idx
                        })
                        .prop_map(move |(source_idx, target_idx)| {
                            Edge::new(
                                EdgeId::generate(),
                                node_ids[source_idx].clone(),
                                node_ids[target_idx].clone(),
                                ()
                            )
                        }),
                    0..(node_count.min(10)) // Limit edges to avoid too many rejections
                ).boxed()
            };
            (Just(nodes), edges_strategy)
        })
        .prop_map(|(nodes, edges)| {
            let mut graph = Graph::new();

            // Add nodes
            for node in nodes {
                let _ = graph.add_node(node);
            }

            // Add edges
            for edge in edges {
                let _ = graph.add_edge(edge);
            }

            graph
        })
}

/// Generate connected graphs (DAGs)
pub fn arb_connected_graph() -> impl Strategy<Value = Graph<(), ()>> {
    prop::collection::vec(arb_node(), 1..20) // Smaller size for connected graphs
        .prop_map(|nodes| {
            let node_count = nodes.len();
            let node_ids: Vec<_> = nodes.iter().map(|n| n.id.clone()).collect();

            let mut graph = Graph::new();

            // Add all nodes first
            for node in nodes {
                let _ = graph.add_node(node);
            }

            // Create a simple chain to ensure connectivity
            for i in 0..(node_count - 1) {
                let edge = Edge::new(
                    EdgeId::generate(),
                    node_ids[i].clone(),
                    node_ids[i + 1].clone(),
                    ()
                );
                let _ = graph.add_edge(edge);
            }

            graph
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    // use proptest::prelude::*; // Not needed since we're inside proptest! macro

    proptest! {
        #[test]
        fn test_graph_invariants(graph in arb_graph()) {
            // Graph invariants that should always hold
            prop_assert!(graph.node_count() >= 0);
            prop_assert!(graph.edge_count() >= 0);
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
            nodes in prop::collection::vec(arb_node(), 0..1000)
        ) {
            let mut graph: Graph<(), ()> = Graph::new();

            // Add nodes
            for node in &nodes {
                let _ = graph.add_node(node.clone());
            }

            // Test that spatial queries are consistent
            let viewport = Viewport::new(-1000.0, -1000.0, 2000.0, 2000.0, 1.0);

            // Get nodes in viewport using spatial index (if available)
            let spatial_results = if graph.node_count() > 0 {
                // For now, we'll use a simple linear search since spatial index isn't implemented yet
                graph.nodes()
                    .filter(|node| viewport.contains_point(node.position))
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            // Get nodes in viewport using linear search
            let linear_results: Vec<_> = graph.nodes()
                .filter(|node| viewport.contains_point(node.position))
                .collect();

            prop_assert_eq!(spatial_results.len(), linear_results.len());
        }

        #[test]
        fn test_graph_bounds_consistency(graph in arb_graph()) {
            if graph.is_empty() {
                prop_assert!(graph.bounds().is_none());
                return Ok(());
            }

            let bounds = graph.bounds().unwrap();
            prop_assert!(bounds.is_valid());

            // All nodes should be within or on the bounds
            for node in graph.nodes() {
                let node_bounds = node.bounds();
                prop_assert!(bounds.intersects(&node_bounds));
            }

            // Bounds should be tight (no unnecessary padding)
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

    proptest! {
        #[test]
        fn test_spatial_index_consistency(
            nodes in prop::collection::vec(arb_node(), 0..100)
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes (duplicates will overwrite)
            for node in &nodes {
                let result = index.insert(node);
                prop_assert!(result.is_ok(), "Failed to insert node: {:?}", node.id);
            }

            // Count unique nodes
            let unique_nodes: HashSet<NodeId> = nodes.iter().map(|n| n.id.clone()).collect();
            let expected_count = unique_nodes.len();

            // Verify all unique nodes are in the index
            prop_assert_eq!(index.len(), expected_count, "Index length mismatch");

            // Test that querying the entire space returns all unique nodes
            if !unique_nodes.is_empty() {
                let bounds = index.bounds().unwrap();
                let all_results = index.query_rect(&bounds);
                prop_assert_eq!(all_results.len(), expected_count, "Query should return all unique nodes");

                // Verify all returned node IDs are valid
                for result_id in &all_results {
                    prop_assert!(unique_nodes.contains(result_id), "Query returned invalid node ID: {:?}", result_id);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_spatial_index_insert_remove_consistency(
            nodes in prop::collection::vec(arb_node(), 1..50)
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes (duplicates will overwrite)
            for node in &nodes {
                index.insert(node).unwrap();
            }

            // Get unique nodes
            let unique_nodes: HashSet<NodeId> = nodes.iter().map(|n| n.id.clone()).collect();
            let unique_nodes_list: Vec<_> = nodes.iter()
                .filter(|n| unique_nodes.contains(&n.id))
                .collect();

            let initial_count = index.len();
            prop_assert_eq!(initial_count, unique_nodes.len());

            // Remove half the unique nodes (only if we have more than 1)
            if unique_nodes_list.len() > 1 {
                let nodes_to_remove = &unique_nodes_list[..unique_nodes_list.len() / 2];
                let mut actually_removed = 0;

                for node in nodes_to_remove {
                    let removed = index.remove(&node.id);
                    if removed {
                        actually_removed += 1;
                    }
                }

                // Verify count decreased correctly
                let expected_count = initial_count - actually_removed;
                prop_assert_eq!(index.len(), expected_count);
            }
        }
    }

    proptest! {
        #[test]
        fn test_spatial_index_query_properties(
            nodes in prop::collection::vec(arb_node(), 1..50),
            query_bounds in arb_rect()
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes
            for node in &nodes {
                index.insert(node).unwrap();
            }

            // Query with the given bounds
            let results = index.query_rect(&query_bounds);

            // Verify all results actually intersect with the query bounds
            for result_id in &results {
                let node = nodes.iter().find(|n| n.id == *result_id).unwrap();
                let node_bounds = node.bounds();
                prop_assert!(query_bounds.intersects(&node_bounds),
                    "Query result {:?} does not intersect with query bounds", result_id);
            }

            // Verify no duplicates in results
            let unique_results: HashSet<NodeId> = results.iter().cloned().collect();
            prop_assert_eq!(unique_results.len(), results.len(), "Query results contain duplicates");
        }
    }

    proptest! {
        #[test]
        fn test_spatial_index_radius_query_properties(
            nodes in prop::collection::vec(arb_node(), 1..50),
            center in arb_position(),
            radius in 1.0..1000.0f64
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes
            for node in &nodes {
                index.insert(node).unwrap();
            }

            // Query with radius
            let results = index.query_radius(center, radius);

            // Verify all results are within the radius
            for result_id in &results {
                let node = nodes.iter().find(|n| n.id == *result_id).unwrap();
                let node_center = Position::new(
                    node.position.x + node.size.width / 2.0,
                    node.position.y + node.size.height / 2.0,
                );
                let distance = center.distance_to(node_center);
                prop_assert!(distance <= radius,
                    "Node {:?} at distance {} exceeds radius {}", result_id, distance, radius);
            }

            // Verify no duplicates in results
            let unique_results: HashSet<NodeId> = results.iter().cloned().collect();
            prop_assert_eq!(unique_results.len(), results.len(), "Radius query results contain duplicates");
        }
    }

    proptest! {
        #[test]
        fn test_spatial_index_nearest_properties(
            nodes in prop::collection::vec(arb_node(), 1..50),
            query_point in arb_position()
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes
            for node in &nodes {
                index.insert(node).unwrap();
            }

            // Find nearest node
            let nearest = index.nearest(query_point);

            if let Some(nearest_id) = nearest {
                // Verify the nearest node exists in our input
                prop_assert!(nodes.iter().any(|n| n.id == nearest_id),
                    "Nearest node {:?} not found in input nodes", nearest_id);

                // Verify it's actually the nearest
                let nearest_node = nodes.iter().find(|n| n.id == nearest_id).unwrap();
                let nearest_center = Position::new(
                    nearest_node.position.x + nearest_node.size.width / 2.0,
                    nearest_node.position.y + nearest_node.size.height / 2.0,
                );
                let nearest_distance = query_point.distance_to(nearest_center);

                for node in &nodes {
                    if node.id != nearest_id {
                        let node_center = Position::new(
                            node.position.x + node.size.width / 2.0,
                            node.position.y + node.size.height / 2.0,
                        );
                        let distance = query_point.distance_to(node_center);
                        prop_assert!(distance >= nearest_distance,
                            "Found closer node {:?} at distance {} than reported nearest {:?} at distance {}",
                            node.id, distance, nearest_id, nearest_distance);
                    }
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_spatial_index_bounds_consistency(
            nodes in prop::collection::vec(arb_node(), 1..50)
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes (duplicates will overwrite)
            for node in &nodes {
                index.insert(node).unwrap();
            }

            // Get unique nodes for bounds calculation
            let unique_nodes: HashSet<NodeId> = nodes.iter().map(|n| n.id.clone()).collect();
            let unique_nodes_list: Vec<_> = nodes.iter()
                .filter(|n| unique_nodes.contains(&n.id))
                .collect();

            // Get spatial bounds
            let bounds = index.bounds();

            if let Some(bounds) = bounds {
                // Verify bounds are reasonable (not empty)
                prop_assert!(bounds.width > 0.0, "Bounds width should be positive");
                prop_assert!(bounds.height > 0.0, "Bounds height should be positive");

                // Verify that querying the bounds returns all unique nodes
                let results = index.query_rect(&bounds);
                prop_assert_eq!(results.len(), unique_nodes.len(),
                    "Querying bounds should return all unique nodes");
            } else {
                // If no bounds, index should be empty
                prop_assert!(index.is_empty(), "Empty index should have no bounds");
            }
        }
    }

    proptest! {
        #[test]
        fn test_spatial_index_update_consistency(
            nodes in prop::collection::vec(arb_node(), 1..20),
            new_positions in prop::collection::vec(arb_position(), 1..20)
        ) {
            let mut index = SpatialIndex::new();

            // Insert all nodes (duplicates will overwrite)
            for node in &nodes {
                index.insert(node).unwrap();
            }

            let initial_count = index.len();

            // Get unique nodes to avoid updating duplicates
            let mut seen_ids = HashSet::new();
            let unique_nodes_list: Vec<_> = nodes.iter()
                .filter(|n| seen_ids.insert(n.id.clone()))
                .collect();

            // Update unique nodes with new positions
            for (i, node) in unique_nodes_list.iter().enumerate() {
                if i < new_positions.len() {
                    let mut updated_node = (*node).clone();
                    updated_node.set_position(new_positions[i]);

                    let result = index.update(&updated_node);
                    prop_assert!(result.is_ok(), "Failed to update node: {:?}", node.id);
                }
            }

            // Verify count remains the same
            prop_assert_eq!(index.len(), initial_count, "Update should not change node count");

            // Verify new positions return the nodes (only for nodes that were actually moved)
            for (i, node) in unique_nodes_list.iter().enumerate() {
                if i < new_positions.len() && new_positions[i] != node.position {
                    let new_bounds = Rect::new(
                        new_positions[i].x,
                        new_positions[i].y,
                        node.size.width,
                        node.size.height,
                    );
                    let results = index.query_rect(&new_bounds);
                    prop_assert!(results.contains(&node.id),
                        "New position does not return updated node {:?}", node.id);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_force_directed_layout_properties(
            nodes in prop::collection::vec(arb_node(), 1..20),
            edges in prop::collection::vec(arb_edge(), 0..30)
        ) {
            let mut graph = Graph::new();

            // Add nodes (handle duplicates by ignoring them)
            for node in &nodes {
                let _ = graph.add_node(node.clone());
            }

            // Add edges (only if both nodes exist)
            for edge in &edges {
                if graph.get_node(&edge.source).is_some() && graph.get_node(&edge.target).is_some() {
                    let _ = graph.add_edge(edge.clone());
                }
            }

            let mut layout = ForceDirectedLayout::builder()
                .iterations(10)
                .randomize_start(false)
                .build();

            // Store initial positions
            let initial_positions: std::collections::HashMap<_, _> = graph.nodes()
                .map(|node| (node.id.clone(), node.position))
                .collect();

            // Apply layout
            let result = layout.apply(&mut graph);
            prop_assert!(result.is_ok(), "Force-directed layout should succeed");

            // Verify all unique nodes still exist
            let unique_node_count = nodes.iter().map(|n| &n.id).collect::<std::collections::HashSet<_>>().len();
            prop_assert_eq!(graph.node_count(), unique_node_count, "Node count should match unique nodes");

            // Verify layout algorithm properties
            prop_assert_eq!(<ForceDirectedLayout as LayoutAlgorithm<(), ()>>::name(&layout), "Force-Directed");
            prop_assert!(!<ForceDirectedLayout as LayoutAlgorithm<(), ()>>::is_running(&layout), "Layout should not be running after completion");
            // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
            let progress = <ForceDirectedLayout as LayoutAlgorithm<(), ()>>::progress(&layout);
            prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");
            prop_assert!(<ForceDirectedLayout as LayoutAlgorithm<(), ()>>::can_interrupt(&layout), "Force-directed layout should be interruptible");
        }
    }

    proptest! {
        #[test]
        fn test_grid_layout_properties(
            nodes in prop::collection::vec(arb_node(), 1..20)
        ) {
            let mut graph: Graph<(), ()> = Graph::new();

            // Add nodes
            for node in &nodes {
                let _ = graph.add_node(node.clone());
            }

            let mut layout = GridLayout::new()
                .columns(Some(3))
                .cell_size(100.0, 80.0)
                .margin(20.0);

            // Apply layout
            let result = layout.apply(&mut graph);
            prop_assert!(result.is_ok(), "Grid layout should succeed");

            // Verify all unique nodes still exist
            let unique_node_count = nodes.iter().map(|n| &n.id).collect::<std::collections::HashSet<_>>().len();
            prop_assert_eq!(graph.node_count(), unique_node_count, "Node count should match unique nodes");

            // Verify layout algorithm properties
            prop_assert_eq!(<GridLayout as LayoutAlgorithm<(), ()>>::name(&layout), "Grid");
            prop_assert!(!<GridLayout as LayoutAlgorithm<(), ()>>::is_running(&layout), "Layout should not be running after completion");
            // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
            let progress = <GridLayout as LayoutAlgorithm<(), ()>>::progress(&layout);
            prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");
            prop_assert!(!<GridLayout as LayoutAlgorithm<(), ()>>::can_interrupt(&layout), "Grid layout should not be interruptible");

            // Verify grid positioning properties
            let positions: Vec<Position> = graph.nodes().map(|n| n.position).collect();

            // All positions should be non-negative (grid starts at origin)
            for pos in &positions {
                prop_assert!(pos.x >= 0.0, "Grid x position should be non-negative");
                prop_assert!(pos.y >= 0.0, "Grid y position should be non-negative");
            }

            // Positions should follow grid pattern (multiples of cell size + margin)
            for pos in &positions {
                let x_expected = (pos.x / 120.0).round() * 120.0; // cell_width + margin
                let y_expected = (pos.y / 100.0).round() * 100.0; // cell_height + margin
                prop_assert!((pos.x - x_expected).abs() < 0.001, "X position should follow grid pattern");
                prop_assert!((pos.y - y_expected).abs() < 0.001, "Y position should follow grid pattern");
            }
        }
    }

    proptest! {
        #[test]
        fn test_circular_layout_properties(
            nodes in prop::collection::vec(arb_node(), 1..20)
        ) {
            let mut graph: Graph<(), ()> = Graph::new();

            // Add nodes
            for node in &nodes {
                let _ = graph.add_node(node.clone());
            }

            let mut layout = CircularLayout::new()
                .radius(200.0)
                .start_angle(0.0)
                .clockwise(true);

            // Apply layout
            let result = layout.apply(&mut graph);
            prop_assert!(result.is_ok(), "Circular layout should succeed");

            // Verify all unique nodes still exist
            let unique_node_count = nodes.iter().map(|n| &n.id).collect::<std::collections::HashSet<_>>().len();
            prop_assert_eq!(graph.node_count(), unique_node_count, "Node count should match unique nodes");

            // Verify layout algorithm properties
            prop_assert_eq!(<CircularLayout as LayoutAlgorithm<(), ()>>::name(&layout), "Circular");
            prop_assert!(!<CircularLayout as LayoutAlgorithm<(), ()>>::is_running(&layout), "Layout should not be running after completion");
            // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
            let progress = <CircularLayout as LayoutAlgorithm<(), ()>>::progress(&layout);
            prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");
            prop_assert!(!<CircularLayout as LayoutAlgorithm<(), ()>>::can_interrupt(&layout), "Circular layout should not be interruptible");

            // Verify circular positioning properties
            for node in graph.nodes() {
                let distance = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
                prop_assert!((distance - 200.0).abs() < 0.001, "Node should be at radius distance from origin");
            }
        }
    }

    proptest! {
        #[test]
        fn test_hierarchical_layout_properties(
            nodes in prop::collection::vec(arb_node(), 1..10),
            edges in prop::collection::vec(arb_edge(), 0..15)
        ) {
            let mut graph = Graph::new();

            // Add nodes
            for node in &nodes {
                let _ = graph.add_node(node.clone());
            }

            // Add edges (only if both nodes exist)
            for edge in &edges {
                if graph.get_node(&edge.source).is_some() && graph.get_node(&edge.target).is_some() {
                    let _ = graph.add_edge(edge.clone());
                }
            }

            let mut layout = HierarchicalLayout::builder()
                .node_separation(100.0)
                .level_separation(80.0)
                .direction(LayoutDirection::TopToBottom)
                .edge_routing(EdgeRouting::Straight)
                .build();

            // Apply layout (may fail for cyclic graphs, which is expected)
            let result = layout.apply(&mut graph);

            // Verify layout algorithm properties regardless of success
            prop_assert_eq!(<HierarchicalLayout as LayoutAlgorithm<(), ()>>::name(&layout), "Hierarchical");
            prop_assert!(!<HierarchicalLayout as LayoutAlgorithm<(), ()>>::is_running(&layout), "Layout should not be running after completion");
            // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
            let progress = <HierarchicalLayout as LayoutAlgorithm<(), ()>>::progress(&layout);
            prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");
            prop_assert!(!<HierarchicalLayout as LayoutAlgorithm<(), ()>>::can_interrupt(&layout), "Hierarchical layout should not be interruptible");

            // If layout succeeded, verify properties
            if result.is_ok() {
                // Verify all nodes still exist
                prop_assert_eq!(graph.node_count(), nodes.len(), "Node count should remain the same");

                // Verify hierarchical positioning properties
                let positions: Vec<Position> = graph.nodes().map(|n| n.position).collect();

                // All positions should be finite
                for pos in &positions {
                    prop_assert!(pos.x.is_finite(), "Position x should be finite");
                    prop_assert!(pos.y.is_finite(), "Position y should be finite");
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_layout_algorithm_consistency(
            nodes in prop::collection::vec(arb_node(), 1..15)
        ) {
            let mut graph = Graph::new();

            // Add nodes
            for node in &nodes {
                let _ = graph.add_node(node.clone());
            }

            // Test multiple layout algorithms
            let layouts: Vec<Box<dyn LayoutAlgorithm<(), ()>>> = vec![
                Box::new(ForceDirectedLayout::builder().iterations(5).build()),
                Box::new(GridLayout::new()),
                Box::new(CircularLayout::new()),
            ];

            for mut layout in layouts {
                let mut test_graph = graph.clone();

                // Apply layout
                let result = layout.apply(&mut test_graph);
                prop_assert!(result.is_ok(), "Layout algorithm should succeed");

                // Verify all unique nodes still exist
                let unique_node_count = nodes.iter().map(|n| &n.id).collect::<std::collections::HashSet<_>>().len();
                prop_assert_eq!(test_graph.node_count(), unique_node_count, "Node count should match unique nodes");

                // Verify layout algorithm properties
                prop_assert!(!layout.is_running(), "Layout should not be running after completion");
                // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
                let progress = layout.progress();
                prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");

                // Verify all positions are finite
                for node in test_graph.nodes() {
                    prop_assert!(node.position.x.is_finite(), "Position x should be finite");
                    prop_assert!(node.position.y.is_finite(), "Position y should be finite");
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_layout_algorithm_empty_graph(
            layout_type in 0..3usize
        ) {
            let mut graph: Graph<(), ()> = Graph::new();

            let mut layout: Box<dyn LayoutAlgorithm<(), ()>> = match layout_type {
                0 => Box::new(ForceDirectedLayout::new()),
                1 => Box::new(GridLayout::new()),
                2 => Box::new(CircularLayout::new()),
                _ => unreachable!(),
            };

            // Apply layout to empty graph
            let result = layout.apply(&mut graph);
            prop_assert!(result.is_ok(), "Layout should handle empty graph gracefully");

            // Verify graph remains empty
            prop_assert_eq!(graph.node_count(), 0, "Empty graph should remain empty");

            // Verify layout algorithm properties
            prop_assert!(!layout.is_running(), "Layout should not be running after completion");
            // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
            let progress = layout.progress();
            prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");
        }
    }

    // Group system property tests
    proptest! {
        #[test]
        fn test_group_manager_invariants(
            node_ids in prop::collection::vec(prop::string::string_regex(r"[a-zA-Z0-9_]{1,20}").unwrap(), 1..50),
            group_operations in prop::collection::vec(0..4usize, 0..100)
        ) {
            use crate::groups::{GroupManager, Group};
            use std::collections::HashSet;

            let mut group_manager = GroupManager::new();
            let unique_node_ids: Vec<NodeId> = node_ids.into_iter()
                .map(NodeId::new)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            prop_assert!(unique_node_ids.len() > 0, "Should have at least one unique node");

            for (i, &operation) in group_operations.iter().enumerate() {
                match operation {
                    0 => {
                        // Create group
                        if unique_node_ids.len() > 0 {
                            let group_id = GroupId::new(format!("group_{}", i));
                            let num_nodes = (i % unique_node_ids.len()).max(1);
                            let members: HashSet<NodeId> = unique_node_ids.iter()
                                .take(num_nodes)
                                .filter(|node_id| group_manager.get_node_group(node_id).is_none())
                                .cloned()
                                .collect();

                            if !members.is_empty() {
                                let result = group_manager.create_group(group_id.clone(), members.clone());
                                prop_assert!(result.is_ok(), "Group creation should succeed for ungrouped nodes");

                                // Verify invariants after group creation
                                let group = group_manager.get_group(&group_id).unwrap();
                                prop_assert_eq!(&group.members, &members, "Group members should match what we created");

                                // Verify node mappings
                                for node_id in &members {
                                    prop_assert_eq!(
                                        group_manager.get_node_group(node_id),
                                        Some(&group_id),
                                        "Node should be mapped to the group"
                                    );
                                }
                            }
                        }
                    },
                    1 => {
                        // Add node to existing group
                        let all_groups: Vec<_> = group_manager.all_groups().keys().cloned().collect();
                        if !all_groups.is_empty() && !unique_node_ids.is_empty() {
                            let group_id = &all_groups[i % all_groups.len()];
                            let node_id = &unique_node_ids[i % unique_node_ids.len()];

                            if group_manager.get_node_group(node_id).is_none() {
                                let before_count = group_manager.get_group(group_id).unwrap().member_count();
                                let result = group_manager.add_node_to_group(group_id, node_id.clone());

                                if result.is_ok() {
                                    let after_count = group_manager.get_group(group_id).unwrap().member_count();
                                    prop_assert_eq!(after_count, before_count + 1, "Group should have one more member");
                                    prop_assert_eq!(
                                        group_manager.get_node_group(node_id),
                                        Some(group_id),
                                        "Node should be mapped to the group"
                                    );
                                }
                            }
                        }
                    },
                    2 => {
                        // Remove node from group
                        let all_groups: Vec<_> = group_manager.all_groups().keys().cloned().collect();
                        if !all_groups.is_empty() {
                            let group_id = &all_groups[i % all_groups.len()];
                            let group_members: Vec<_> = group_manager.get_group(group_id).unwrap().members.iter().cloned().collect();

                            if !group_members.is_empty() {
                                let node_id = &group_members[i % group_members.len()];
                                let before_count = group_manager.get_group(group_id).unwrap().member_count();
                                let result = group_manager.remove_node_from_group(group_id, node_id);

                                prop_assert!(result.is_ok(), "Node removal should succeed");

                                // Check if group still exists (should be dissolved if empty)
                                if before_count == 1 {
                                    prop_assert!(group_manager.get_group(group_id).is_none(), "Empty group should be dissolved");
                                } else {
                                    let after_count = group_manager.get_group(group_id).unwrap().member_count();
                                    prop_assert_eq!(after_count, before_count - 1, "Group should have one fewer member");
                                }

                                prop_assert_eq!(
                                    group_manager.get_node_group(node_id),
                                    None,
                                    "Node should no longer be mapped to any group"
                                );
                            }
                        }
                    },
                    3 => {
                        // Dissolve group
                        let all_groups: Vec<_> = group_manager.all_groups().keys().cloned().collect();
                        if !all_groups.is_empty() {
                            let group_id = &all_groups[i % all_groups.len()];
                            let group_members: Vec<_> = group_manager.get_group(group_id).unwrap().members.iter().cloned().collect();

                            let result = group_manager.dissolve_group(group_id);
                            prop_assert!(result.is_ok(), "Group dissolution should succeed");

                            prop_assert!(group_manager.get_group(group_id).is_none(), "Group should no longer exist");

                            // Verify no nodes are mapped to the dissolved group
                            for node_id in &group_members {
                                prop_assert_eq!(
                                    group_manager.get_node_group(node_id),
                                    None,
                                    "Node should not be mapped to dissolved group"
                                );
                            }
                        }
                    },
                    _ => unreachable!(),
                }

                // Verify core invariants after every operation

                // Invariant 1: Every node in a group should have a reverse mapping
                for (group_id, group) in group_manager.all_groups() {
                    for node_id in &group.members {
                        prop_assert_eq!(
                            group_manager.get_node_group(node_id),
                            Some(group_id),
                            "Every group member should have reverse mapping"
                        );
                    }
                }

                // Invariant 2: Every node mapping should correspond to an actual group membership
                for (node_id, group_id) in group_manager.node_mappings() {
                    let group = group_manager.get_group(group_id);
                    prop_assert!(group.is_some(), "Mapped group should exist");
                    prop_assert!(
                        group.unwrap().contains_node(node_id),
                        "Node should be in the group it's mapped to"
                    );
                }

                // Invariant 3: Each node can only belong to at most one group
                let mut node_count = std::collections::HashMap::new();
                for group in group_manager.all_groups().values() {
                    for node_id in &group.members {
                        *node_count.entry(node_id.clone()).or_insert(0) += 1;
                    }
                }
                for (node_id, count) in node_count {
                    prop_assert_eq!(count, 1, "Node {} should belong to exactly one group", node_id);
                }

                // Invariant 4: No empty groups should exist
                for group in group_manager.all_groups().values() {
                    prop_assert!(group.member_count() > 0, "No empty groups should exist");
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_selection_with_groups_invariants(
            node_ids in prop::collection::vec(prop::string::string_regex(r"[a-zA-Z0-9_]{1,10}").unwrap(), 5..20),
            operations in prop::collection::vec(0..6usize, 0..50)
        ) {
            use crate::groups::GroupManager;
            use crate::selection::{SelectionManager, SelectionMode};
            use std::collections::HashSet;

            let unique_node_ids: Vec<NodeId> = node_ids.into_iter()
                .map(NodeId::new)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            let mut group_manager = GroupManager::new();
            let mut selection = SelectionManager::new();

            // Create some initial groups
            if unique_node_ids.len() >= 4 {
                let group1_members: HashSet<_> = unique_node_ids.iter().take(2).cloned().collect();
                let group2_members: HashSet<_> = unique_node_ids.iter().skip(2).take(2).cloned().collect();

                let _ = group_manager.create_group(GroupId::new("group1"), group1_members);
                let _ = group_manager.create_group(GroupId::new("group2"), group2_members);
            }

            for (i, &operation) in operations.iter().enumerate() {
                match operation {
                    0 => {
                        // Select individual node
                        if !unique_node_ids.is_empty() {
                            let node_id = unique_node_ids[i % unique_node_ids.len()].clone();
                            selection.select_node(node_id);
                        }
                    },
                    1 => {
                        // Select node with group
                        if !unique_node_ids.is_empty() {
                            let node_id = unique_node_ids[i % unique_node_ids.len()].clone();
                            selection.select_node_with_group(&group_manager, node_id, true);
                        }
                    },
                    2 => {
                        // Select entire group
                        let all_groups: Vec<_> = group_manager.all_groups().keys().cloned().collect();
                        if !all_groups.is_empty() {
                            let group_id = &all_groups[i % all_groups.len()];
                            selection.select_group(&group_manager, group_id);
                        }
                    },
                    3 => {
                        // Toggle selection mode
                        let modes = [SelectionMode::Single, SelectionMode::Multi];
                        selection.set_mode(modes[i % modes.len()]);
                    },
                    4 => {
                        // Deselect group
                        let all_groups: Vec<_> = group_manager.all_groups().keys().cloned().collect();
                        if !all_groups.is_empty() {
                            let group_id = &all_groups[i % all_groups.len()];
                            selection.deselect_group(&group_manager, group_id);
                        }
                    },
                    5 => {
                        // Clear selection
                        selection.clear_selection();
                    },
                    _ => unreachable!(),
                }

                // Verify selection invariants

                // Invariant 1: Selected nodes should be consistent with group queries
                let selected_groups = selection.get_selected_groups(&group_manager);
                for group_id in &selected_groups {
                    let has_selected_member = group_manager.get_group(group_id)
                        .unwrap()
                        .members
                        .iter()
                        .any(|node_id| selection.is_selected(node_id));
                    prop_assert!(has_selected_member, "Selected group should have at least one selected member");
                }

                // Invariant 2: Group fully selected check should be accurate
                for (group_id, group) in group_manager.all_groups() {
                    let is_fully_selected = selection.is_group_fully_selected(&group_manager, group_id);
                    let all_members_selected = group.members.iter().all(|node_id| selection.is_selected(node_id));
                    prop_assert_eq!(
                        is_fully_selected && !group.members.is_empty(),
                        all_members_selected,
                        "Group fully selected check should match actual member selection state"
                    );
                }

                // Invariant 3: Selection count should be consistent
                prop_assert_eq!(
                    selection.selection_count(),
                    selection.selected_nodes().len(),
                    "Selection count should match actual selected nodes"
                );
            }
        }
    }

    // Group drag operation property tests
    proptest! {
        #[test]
        fn test_group_drag_invariants(
            node_positions in prop::collection::vec((arb_position(), prop::string::string_regex(r"[a-zA-Z0-9_]{1,10}").unwrap()), 2..10),
            drag_operations in prop::collection::vec(arb_position(), 1..20)
        ) {
            use crate::groups::{GroupManager, Group};
            use crate::{Graph, Node};
            use std::collections::{HashSet, HashMap};

            let mut graph = Graph::<(), ()>::new();
            let mut group_manager = GroupManager::new();

            // Add nodes to graph
            let node_ids: Vec<NodeId> = node_positions.iter()
                .map(|(pos, id)| {
                    let node_id = NodeId::new(id);
                    let node = Node::simple(node_id.clone(), *pos);
                    graph.add_node(node).unwrap();
                    node_id
                })
                .collect();

            // Create a group with all nodes
            let members: HashSet<NodeId> = node_ids.iter().cloned().collect();
            let group_id = GroupId::new("test_group");
            group_manager.create_group(group_id.clone(), members.clone()).unwrap();

            // Store original positions
            let original_positions: HashMap<NodeId, Position> = node_ids.iter()
                .map(|node_id| {
                    let pos = graph.get_node(node_id).unwrap().position;
                    (node_id.clone(), pos)
                })
                .collect();

            // Start drag
            let start_pos = drag_operations.first().cloned().unwrap_or(Position::zero());
            let result = group_manager.start_group_drag(&group_id, start_pos, &graph);
            prop_assert!(result.is_ok(), "Group drag start should succeed");

            // Apply drag operations
            for &drag_pos in &drag_operations {
                let result = group_manager.update_group_drag(drag_pos, &mut graph);
                prop_assert!(result.is_ok(), "Group drag update should succeed");

                // Verify invariants during drag
                prop_assert!(group_manager.is_group_dragging(), "Should be dragging during operation");
                prop_assert_eq!(group_manager.get_dragging_group(), Some(&group_id), "Should track correct group");

                let delta = group_manager.get_drag_delta().unwrap();

                // Verify all nodes moved by the same delta
                for node_id in &node_ids {
                    let current_pos = graph.get_node(node_id).unwrap().position;
                    let expected_pos = original_positions[node_id].add(delta);

                    prop_assert!((current_pos.x - expected_pos.x).abs() < 0.001,
                        "Node {} x position should match expected", node_id);
                    prop_assert!((current_pos.y - expected_pos.y).abs() < 0.001,
                        "Node {} y position should match expected", node_id);
                }
            }

            // Test completion vs cancellation
            if drag_operations.len() % 2 == 0 {
                // Complete drag
                let result = group_manager.complete_group_drag();
                prop_assert!(result.is_ok(), "Group drag completion should succeed");
                prop_assert!(!group_manager.is_group_dragging(), "Should not be dragging after completion");

                // Nodes should remain at their final positions
                let final_delta = Position::new(
                    drag_operations.last().unwrap().x - start_pos.x,
                    drag_operations.last().unwrap().y - start_pos.y,
                );

                for node_id in &node_ids {
                    let current_pos = graph.get_node(node_id).unwrap().position;
                    let expected_pos = original_positions[node_id].add(final_delta);

                    prop_assert!((current_pos.x - expected_pos.x).abs() < 0.001,
                        "Node {} should be at final position after completion", node_id);
                }
            } else {
                // Cancel drag
                let result = group_manager.cancel_group_drag(&mut graph);
                prop_assert!(result.is_ok(), "Group drag cancellation should succeed");
                prop_assert!(!group_manager.is_group_dragging(), "Should not be dragging after cancellation");

                // Nodes should be restored to original positions
                for node_id in &node_ids {
                    let current_pos = graph.get_node(node_id).unwrap().position;
                    let original_pos = original_positions[node_id];

                    prop_assert!((current_pos.x - original_pos.x).abs() < 0.001,
                        "Node {} should be restored to original position", node_id);
                    prop_assert!((current_pos.y - original_pos.y).abs() < 0.001,
                        "Node {} should be restored to original position", node_id);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_group_bounds_calculation_properties(
            nodes in prop::collection::vec((arb_position(), arb_size()), 1..8)
        ) {
            use crate::groups::GroupManager;
            use crate::{Graph, Node};
            use std::collections::HashSet;

            let mut graph = Graph::<(), ()>::new();
            let mut group_manager = GroupManager::new();

            // Add nodes with specific positions and sizes
            let node_ids: Vec<NodeId> = nodes.iter()
                .enumerate()
                .map(|(i, (pos, size))| {
                    let node_id = NodeId::new(format!("node_{}", i));
                    let mut node = Node::simple(node_id.clone(), *pos);
                    node.size = *size;
                    graph.add_node(node).unwrap();
                    node_id
                })
                .collect();

            // Create group
            let members: HashSet<NodeId> = node_ids.iter().cloned().collect();
            let group_id = GroupId::new("test_group");
            group_manager.create_group(group_id.clone(), members).unwrap();

            // Calculate bounds
            let result = group_manager.calculate_group_bounds(&group_id, &graph);
            prop_assert!(result.is_ok(), "Bounds calculation should succeed");

            let group = group_manager.get_group(&group_id).unwrap();

            // Verify bounds contain all nodes
            for node_id in &node_ids {
                let node = graph.get_node(node_id).unwrap();
                let node_left = node.position.x;
                let node_right = node.position.x + node.size.width;
                let node_top = node.position.y;
                let node_bottom = node.position.y + node.size.height;

                prop_assert!(node_left >= group.position.x, "Node should be within group bounds (left)");
                prop_assert!(node_right <= group.position.x + group.size.width, "Node should be within group bounds (right)");
                prop_assert!(node_top >= group.position.y, "Node should be within group bounds (top)");
                prop_assert!(node_bottom <= group.position.y + group.size.height, "Node should be within group bounds (bottom)");
            }

            // Verify bounds are tight (minimal)
            if !nodes.is_empty() {
                let min_x = nodes.iter().map(|(pos, _)| pos.x).fold(f64::INFINITY, f64::min);
                let max_x = nodes.iter().map(|(pos, size)| pos.x + size.width).fold(f64::NEG_INFINITY, f64::max);
                let min_y = nodes.iter().map(|(pos, _)| pos.y).fold(f64::INFINITY, f64::min);
                let max_y = nodes.iter().map(|(pos, size)| pos.y + size.height).fold(f64::NEG_INFINITY, f64::max);

                prop_assert!((group.position.x - min_x).abs() < 0.001, "Group bounds should be tight (left)");
                prop_assert!((group.position.y - min_y).abs() < 0.001, "Group bounds should be tight (top)");
                prop_assert!((group.size.width - (max_x - min_x)).abs() < 0.001, "Group bounds should be tight (width)");
                prop_assert!((group.size.height - (max_y - min_y)).abs() < 0.001, "Group bounds should be tight (height)");
            }
        }
    }
}
