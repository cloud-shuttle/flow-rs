use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos_flow_core::{
    Graph, Node, Edge, Position, Size,
    prelude::SpatialIndex,
    layout::{LayoutAlgorithm, ForceDirectedLayout, GridLayout, CircularLayout, HierarchicalLayout, LayoutDirection},
    types::{NodeId, EdgeId}
};
use std::collections::{HashMap, HashSet};

fn create_connected_graph(node_count: usize) -> Graph<(), ()> {
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

    // Create a connected graph (chain + some cross connections)
    for i in 0..node_count - 1 {
        let source = NodeId::new(format!("node_{}", i));
        let target = NodeId::new(format!("node_{}", i + 1));
        let edge = Edge::new(
            EdgeId::new(format!("edge_{}", i)),
            source,
            target,
            (),
        );
        let _ = graph.add_edge(edge);
    }

    // Add some cross connections for more interesting layouts
    for i in 0..node_count / 4 {
        let source = NodeId::new(format!("node_{}", i));
        let target = NodeId::new(format!("node_{}", i + node_count / 2));
        let edge = Edge::new(
            EdgeId::new(format!("cross_edge_{}", i)),
            source,
            target,
            (),
        );
        let _ = graph.add_edge(edge);
    }

    graph
}

fn create_dense_graph(node_count: usize, edge_probability: f64) -> Graph<(), ()> {
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

    // Add edges with given probability
    let mut edge_count = 0;
    for i in 0..node_count {
        for j in i + 1..node_count {
            if (edge_count as f64 / (node_count * (node_count - 1) / 2) as f64) < edge_probability {
                let source = NodeId::new(format!("node_{}", i));
                let target = NodeId::new(format!("node_{}", j));
                let edge = Edge::new(
                    EdgeId::new(format!("edge_{}_{}", i, j)),
                    source,
                    target,
                    (),
                );
                let _ = graph.add_edge(edge);
                edge_count += 1;
            }
        }
    }

    graph
}

fn benchmark_force_directed_convergence(c: &mut Criterion) {
    let mut group = c.benchmark_group("force_directed_convergence");

    for size in [20, 50, 100, 200].iter() {
        let graph = create_connected_graph(*size);

        for iterations in [10, 25, 50, 100, 200].iter() {
            group.bench_with_input(
                BenchmarkId::new("iterations", format!("{}_nodes_{}_iter", size, iterations)),
                &(*size, *iterations),
                |b, &(size, iterations)| {
                    b.iter(|| {
                        let mut test_graph = create_connected_graph(size);
                        let mut layout = ForceDirectedLayout::builder()
                            .iterations(iterations)
                            .spring_length(100.0)
                            .repulsion_strength(1000.0)
                            .damping(0.8)
                            .build();
                        layout.apply(&mut test_graph).unwrap();
                        black_box(test_graph)
                    })
                }
            );
        }
    }

    group.finish();
}

fn benchmark_layout_algorithm_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_algorithm_comparison");

    for size in [10, 25, 50, 100, 200].iter() {
        let graph = create_connected_graph(*size);

        // Force-directed layout
        group.bench_with_input(BenchmarkId::new("force_directed", size), &graph, |b, graph| {
            b.iter(|| {
                let mut test_graph = graph.clone();
                let mut layout = ForceDirectedLayout::builder()
                    .iterations(50)
                    .spring_length(100.0)
                    .repulsion_strength(1000.0)
                    .damping(0.8)
                    .build();
                layout.apply(&mut test_graph).unwrap();
                black_box(test_graph)
            })
        });

        // Grid layout
        group.bench_with_input(BenchmarkId::new("grid", size), &graph, |b, graph| {
            b.iter(|| {
                let mut test_graph = graph.clone();
                let mut layout = GridLayout::new()
                    .columns(Some((*size as f64).sqrt() as usize))
                    .cell_size(100.0, 80.0)
                    .margin(20.0);
                layout.apply(&mut test_graph).unwrap();
                black_box(test_graph)
            })
        });

        // Circular layout
        group.bench_with_input(BenchmarkId::new("circular", size), &graph, |b, graph| {
            b.iter(|| {
                let mut test_graph = graph.clone();
                let mut layout = CircularLayout::new()
                    .radius(200.0)
                    .start_angle(0.0)
                    .clockwise(true);
                layout.apply(&mut test_graph).unwrap();
                black_box(test_graph)
            })
        });

        // Hierarchical layout
        group.bench_with_input(BenchmarkId::new("hierarchical", size), &graph, |b, graph| {
            b.iter(|| {
                let mut test_graph = graph.clone();
                let mut layout = HierarchicalLayout::builder()
                    .node_separation(100.0)
                    .level_separation(150.0)
                    .direction(LayoutDirection::TopToBottom)
                    .build();
                let _ = layout.apply(&mut test_graph);
                black_box(test_graph)
            })
        });
    }

    group.finish();
}

fn benchmark_spatial_index_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index_scaling");

    for size in [100, 500, 1000, 5000, 10000, 50000].iter() {
        let mut index = SpatialIndex::new();

        // Populate index
        for i in 0..*size {
            let mut node = Node::new(
                NodeId::new(format!("node_{}", i)),
                Position::new(
                    (i as f64 * 137.5) % 10000.0, // Golden angle for good distribution
                    (i as f64 * 89.3) % 10000.0,
                ),
                (),
            );
            node.size = Size::new(50.0, 50.0);
            index.insert(&node).unwrap();
        }

        // Test different query sizes
        for query_size in [100.0, 500.0, 1000.0, 2000.0].iter() {
            group.bench_with_input(
                BenchmarkId::new("query_rect", format!("{}_nodes_{}_query", size, query_size)),
                &(*size, *query_size),
                |b, &(_, query_size)| {
                    b.iter(|| {
                        let rect = leptos_flow_core::Rect::new(0.0, 0.0, query_size, query_size);
                        black_box(index.query_rect(&rect))
                    })
                }
            );
        }

        // Test radius queries
        for radius in [50.0, 200.0, 500.0, 1000.0].iter() {
            group.bench_with_input(
                BenchmarkId::new("query_radius", format!("{}_nodes_{}_radius", size, radius)),
                &(*size, *radius),
                |b, &(_, radius)| {
                    b.iter(|| {
                        let center = Position::new(5000.0, 5000.0);
                        black_box(index.query_radius(center, radius))
                    })
                }
            );
        }

        // Test nearest neighbor
        group.bench_with_input(BenchmarkId::new("nearest", size), &size, |b, &size| {
            b.iter(|| {
                let center = Position::new(5000.0, 5000.0);
                black_box(index.nearest(center))
            })
        });
    }

    group.finish();
}

fn benchmark_graph_density_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_density_impact");

    let node_count = 100;

    for density in [0.1, 0.2, 0.3, 0.5, 0.7, 0.9].iter() {
        let graph = create_dense_graph(node_count, *density);

        group.bench_with_input(BenchmarkId::new("bounds_calculation", density), &graph, |b, graph| {
            b.iter(|| {
                black_box(graph.bounds())
            })
        });

        group.bench_with_input(BenchmarkId::new("edge_iteration", density), &graph, |b, graph| {
            b.iter(|| {
                let mut count = 0;
                for _edge in graph.edges() {
                    count += 1;
                }
                black_box(count)
            })
        });

        // Test layout performance with different densities
        group.bench_with_input(BenchmarkId::new("force_directed_layout", density), &graph, |b, graph| {
            b.iter(|| {
                let mut test_graph = graph.clone();
                let mut layout = ForceDirectedLayout::builder()
                    .iterations(30)
                    .spring_length(100.0)
                    .repulsion_strength(1000.0)
                    .damping(0.8)
                    .build();
                layout.apply(&mut test_graph).unwrap();
                black_box(test_graph)
            })
        });
    }

    group.finish();
}

fn benchmark_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");

    for size in [100, 1000, 5000, 10000].iter() {
        // Test graph memory usage
        group.bench_with_input(BenchmarkId::new("graph_creation", size), size, |b, &size| {
            b.iter(|| {
                let graph = create_connected_graph(size);
                black_box(graph)
            })
        });

        // Test spatial index memory usage
        group.bench_with_input(BenchmarkId::new("spatial_index_creation", size), size, |b, &size| {
            b.iter(|| {
                let mut index = SpatialIndex::new();
                for i in 0..size {
        let mut node = Node::new(
            NodeId::new(format!("node_{}", i)),
            Position::new(i as f64 * 100.0, i as f64 * 100.0),
            (),
        );
        node.size = Size::new(50.0, 50.0);
                    index.insert(&node).unwrap();
                }
                black_box(index)
            })
        });

        // Test combined memory usage
        group.bench_with_input(BenchmarkId::new("combined_structures", size), size, |b, &size| {
            b.iter(|| {
                let graph = create_connected_graph(size);
                let mut index = SpatialIndex::new();
                for node in graph.nodes() {
                    index.insert(node).unwrap();
                }
                black_box((graph, index))
            })
        });
    }

    group.finish();
}

fn benchmark_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    // Empty graph
    group.bench_function("empty_graph_bounds", |b| {
        b.iter(|| {
            let graph = Graph::<(), ()>::new();
            black_box(graph.bounds())
        })
    });

    // Single node
    group.bench_function("single_node_graph", |b| {
        b.iter(|| {
            let mut graph: Graph<(), ()> = Graph::new();
            let mut node = Node::new(
                NodeId::new("single"),
                Position::new(0.0, 0.0),
                (),
            );
            node.size = Size::new(50.0, 50.0);
            graph.add_node(node).unwrap();
            black_box(graph.bounds())
        })
    });

    // Very large coordinates
    group.bench_function("large_coordinates", |b| {
        b.iter(|| {
            let mut graph: Graph<(), ()> = Graph::new();
            for i in 0..100 {
                let mut node = Node::new(
                    NodeId::new(format!("node_{}", i)),
                    Position::new(i as f64 * 1000000.0, i as f64 * 1000000.0),
                    (),
                );
                node.size = Size::new(50.0, 50.0);
                graph.add_node(node).unwrap();
            }
            black_box(graph.bounds())
        })
    });

    // Very small coordinates
    group.bench_function("small_coordinates", |b| {
        b.iter(|| {
            let mut graph: Graph<(), ()> = Graph::new();
            for i in 0..100 {
                let mut node = Node::new(
                    NodeId::new(format!("node_{}", i)),
                    Position::new(i as f64 * 0.001, i as f64 * 0.001),
                    (),
                );
                node.size = Size::new(0.1, 0.1);
                graph.add_node(node).unwrap();
            }
            black_box(graph.bounds())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_force_directed_convergence,
    benchmark_layout_algorithm_comparison,
    benchmark_spatial_index_scaling,
    benchmark_graph_density_impact,
    benchmark_memory_efficiency,
    benchmark_edge_cases
);

criterion_main!(benches);
