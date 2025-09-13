// Performance validation tests for leptos-flow-core
// Tests performance with 1000+ node graphs

use std::time::Instant;
use crate::{
    Graph, Node, Edge, Position, Size,
    prelude::SpatialIndex,
    layout::{LayoutAlgorithm, ForceDirectedLayout, GridLayout, CircularLayout},
    types::{NodeId, EdgeId, Rect}
};

fn create_large_graph(node_count: usize, edge_density: f64) -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Add nodes
    for i in 0..node_count {
        let mut node = Node::new(
            NodeId::new(format!("node_{}", i)),
            Position::new(i as f64 * 100.0, i as f64 * 100.0),
            (),
        );
        node.size = Size::new(50.0, 50.0);
        graph.add_node(node).unwrap();
    }

    // Add edges based on density
    let max_edges = node_count * (node_count - 1) / 2;
    let target_edges = (max_edges as f64 * edge_density) as usize;

    let mut edge_count = 0;
    for i in 0..node_count {
        for j in i + 1..node_count {
            if edge_count >= target_edges {
                break;
            }

            // Create edges in a pattern (chain + some random connections)
            if j == i + 1 || (i % 10 == 0 && j % 10 == 0) {
                let edge = Edge::new(
                    EdgeId::new(format!("edge_{}_{}", i, j)),
                    NodeId::new(format!("node_{}", i)),
                    NodeId::new(format!("node_{}", j)),
                    (),
                );
                let _ = graph.add_edge(edge);
                edge_count += 1;
            }
        }
        if edge_count >= target_edges {
            break;
        }
    }

    graph
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_1000_node_graph_creation_performance() {
        let start = Instant::now();
        let graph = create_large_graph(1000, 0.1);
        let creation_time = start.elapsed();

        println!("1000 node graph creation: {:?}", creation_time);
        println!("Nodes: {}, Edges: {}", graph.node_count(), graph.edge_count());

        // Should complete in reasonable time (less than 1 second)
        assert!(creation_time.as_millis() < 1000, "Graph creation took too long: {:?}", creation_time);
        assert_eq!(graph.node_count(), 1000);
    }

    #[test]
    fn test_1000_node_spatial_index_performance() {
        let graph = create_large_graph(1000, 0.1);

        let start = Instant::now();
        let mut index = SpatialIndex::new();

        // Insert all nodes
        for node in graph.nodes() {
            index.insert(node).unwrap();
        }

        let insertion_time = start.elapsed();
        println!("1000 node spatial index creation: {:?}", insertion_time);

        // Test queries
        let query_rect = Rect::new(0.0, 0.0, 1000.0, 1000.0);

        let start = Instant::now();
        for _ in 0..100 {
            let _results = index.query_rect(&query_rect);
        }
        let query_time = start.elapsed();
        println!("100 rect queries: {:?} ({:.2} μs/query)", query_time, query_time.as_micros() as f64 / 100.0);

        // Should complete in reasonable time
        assert!(insertion_time.as_millis() < 100, "Spatial index creation took too long: {:?}", insertion_time);
        assert!(query_time.as_millis() < 100, "Queries took too long: {:?}", query_time);
    }

    #[test]
    fn test_1000_node_layout_algorithms_performance() {
        let mut graph = create_large_graph(1000, 0.1);

        // Grid layout (should be fastest)
        let start = Instant::now();
        let mut layout = GridLayout::new();
        let result = layout.apply(&mut graph);
        let grid_time = start.elapsed();
        println!("1000 node grid layout: {:?} - {}", grid_time, if result.is_ok() { "SUCCESS" } else { "FAILED" });

        // Circular layout
        let start = Instant::now();
        let mut layout = CircularLayout::new();
        let result = layout.apply(&mut graph);
        let circular_time = start.elapsed();
        println!("1000 node circular layout: {:?} - {}", circular_time, if result.is_ok() { "SUCCESS" } else { "FAILED" });

        // Force-directed layout (with fewer iterations for performance)
        let start = Instant::now();
        let mut layout = ForceDirectedLayout::builder()
            .iterations(10) // Further reduced iterations for performance test
            .build();
        let result = layout.apply(&mut graph);
        let force_time = start.elapsed();
        println!("1000 node force-directed (10 iter): {:?} - {}", force_time, if result.is_ok() { "SUCCESS" } else { "FAILED" });

        // Should complete in reasonable time
        assert!(grid_time.as_millis() < 100, "Grid layout took too long: {:?}", grid_time);
        assert!(circular_time.as_millis() < 100, "Circular layout took too long: {:?}", circular_time);
        assert!(force_time.as_millis() < 3000, "Force-directed layout took too long: {:?}", force_time);
    }

    #[test]
    fn test_2000_node_graph_performance() {
        let start = Instant::now();
        let graph = create_large_graph(2000, 0.05); // Lower density for 2000 nodes
        let creation_time = start.elapsed();

        println!("2000 node graph creation: {:?}", creation_time);
        println!("Nodes: {}, Edges: {}", graph.node_count(), graph.edge_count());

        // Should complete in reasonable time
        assert!(creation_time.as_millis() < 2000, "2000 node graph creation took too long: {:?}", creation_time);
        assert_eq!(graph.node_count(), 2000);
    }

    #[test]
    fn test_5000_node_graph_performance() {
        let start = Instant::now();
        let graph = create_large_graph(5000, 0.02); // Even lower density for 5000 nodes
        let creation_time = start.elapsed();

        println!("5000 node graph creation: {:?}", creation_time);
        println!("Nodes: {}, Edges: {}", graph.node_count(), graph.edge_count());

        // Should complete in reasonable time
        assert!(creation_time.as_millis() < 5000, "5000 node graph creation took too long: {:?}", creation_time);
        assert_eq!(graph.node_count(), 5000);
    }

    #[test]
    fn test_large_graph_operations_performance() {
        let graph = create_large_graph(1000, 0.1);

        // Node retrieval performance
        let start = Instant::now();
        for i in 0..1000 {
            let node_id = NodeId::new(format!("node_{}", i % graph.node_count()));
            let _node = graph.get_node(&node_id);
        }
        let retrieval_time = start.elapsed();
        println!("1000 node retrievals: {:?} ({:.2} μs/retrieval)", retrieval_time, retrieval_time.as_micros() as f64 / 1000.0);

        // Edge iteration performance
        let start = Instant::now();
        let mut edge_count = 0;
        for _edge in graph.edges() {
            edge_count += 1;
        }
        let iteration_time = start.elapsed();
        println!("Edge iteration ({} edges): {:?}", edge_count, iteration_time);

        // Bounds calculation performance
        let start = Instant::now();
        let _bounds = graph.bounds();
        let bounds_time = start.elapsed();
        println!("Bounds calculation: {:?}", bounds_time);

        // Should complete in reasonable time
        assert!(retrieval_time.as_micros() < 10000, "Node retrievals took too long: {:?}", retrieval_time);
        assert!(iteration_time.as_millis() < 10, "Edge iteration took too long: {:?}", iteration_time);
        assert!(bounds_time.as_micros() < 1000, "Bounds calculation took too long: {:?}", bounds_time);
    }
}
