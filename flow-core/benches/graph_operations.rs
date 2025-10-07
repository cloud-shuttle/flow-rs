use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use flow_rs_core::{
    layout::{
        CircularLayout, ForceDirectedLayout, GridLayout, HierarchicalLayout, LayoutAlgorithm,
        LayoutDirection,
    },
    prelude::SpatialIndex,
    types::{EdgeId, NodeId},
    Edge, Graph, Node, Position, Size,
};
use std::collections::HashMap;

fn create_test_graph(node_count: usize, edge_count: usize) -> Graph<(), ()> {
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

    // Add edges (create a connected graph)
    for i in 0..edge_count {
        let source = NodeId::new(format!("node_{}", i % node_count));
        let target = NodeId::new(format!("node_{}", (i + 1) % node_count));
        let edge = Edge::new(EdgeId::new(format!("edge_{}", i)), source, target, ());
        let _ = graph.add_edge(edge);
    }

    graph
}

fn create_test_spatial_index(node_count: usize) -> SpatialIndex {
    let mut index = SpatialIndex::new();

    for i in 0..node_count {
        let mut node = Node::new(
            NodeId::new(format!("node_{}", i)),
            Position::new(i as f64 * 100.0, i as f64 * 100.0),
            (),
        );
        node.size = Size::new(50.0, 50.0);
        index.insert(&node).unwrap();
    }

    index
}

fn benchmark_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");

    for size in [10, 100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::new("nodes", size), size, |b, &size| {
            b.iter(|| {
                let mut graph: Graph<(), ()> = Graph::new();
                for i in 0..size {
                    let mut node = Node::new(
                        NodeId::new(format!("node_{}", i)),
                        Position::new(i as f64, i as f64),
                        (),
                    );
                    node.size = Size::new(50.0, 50.0);
                    graph.add_node(node).unwrap();
                }
                black_box(graph)
            })
        });
    }

    group.finish();
}

fn benchmark_graph_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_operations");

    for size in [100, 1000, 5000].iter() {
        let graph = create_test_graph(*size, *size / 2);

        group.bench_with_input(BenchmarkId::new("get_node", size), &graph, |b, graph| {
            b.iter(|| {
                for i in 0..100 {
                    let node_id = NodeId::new(format!("node_{}", i % *size));
                    black_box(graph.get_node(&node_id));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("get_edge", size), &graph, |b, graph| {
            b.iter(|| {
                for i in 0..100 {
                    let edge_id = EdgeId::new(format!("edge_{}", i % (*size / 2)));
                    black_box(graph.get_edge(&edge_id));
                }
            })
        });

        group.bench_with_input(
            BenchmarkId::new("bounds_calculation", size),
            &graph,
            |b, graph| b.iter(|| black_box(graph.bounds())),
        );

        group.bench_with_input(
            BenchmarkId::new("node_iteration", size),
            &graph,
            |b, graph| {
                b.iter(|| {
                    for node in graph.nodes() {
                        black_box(node);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("edge_iteration", size),
            &graph,
            |b, graph| {
                b.iter(|| {
                    for edge in graph.edges() {
                        black_box(edge);
                    }
                })
            },
        );
    }

    group.finish();
}

fn benchmark_spatial_index_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index_operations");

    for size in [100, 1000, 5000, 10000].iter() {
        let index = create_test_spatial_index(*size);

        group.bench_with_input(
            BenchmarkId::new("query_rect", size),
            &index,
            |b, index: &SpatialIndex| {
                b.iter(|| {
                    let rect = flow_rs_core::Rect::new(0.0, 0.0, 1000.0, 1000.0);
                    black_box(index.query_rect(&rect))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("query_radius", size),
            &index,
            |b, index: &SpatialIndex| {
                b.iter(|| {
                    let center = Position::new(500.0, 500.0);
                    black_box(index.query_radius(center, 200.0))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("nearest", size),
            &index,
            |b, index: &SpatialIndex| {
                b.iter(|| {
                    let center = Position::new(500.0, 500.0);
                    black_box(index.nearest(center))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bounds", size),
            &index,
            |b, index: &SpatialIndex| b.iter(|| black_box(index.bounds())),
        );
    }

    group.finish();
}

fn benchmark_spatial_index_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index_updates");

    for size in [100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
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

        group.bench_with_input(BenchmarkId::new("update", size), size, |b, &size| {
            b.iter(|| {
                let mut index = create_test_spatial_index(size);
                for i in 0..100 {
                    let mut node = Node::new(
                        NodeId::new(format!("node_{}", i % size)),
                        Position::new(i as f64 * 10.0, i as f64 * 10.0),
                        (),
                    );
                    node.size = Size::new(50.0, 50.0);
                    index.update(&node).unwrap();
                }
                black_box(index)
            })
        });

        group.bench_with_input(BenchmarkId::new("remove", size), size, |b, &size| {
            b.iter(|| {
                let mut index = create_test_spatial_index(size);
                for i in 0..100 {
                    let node_id = NodeId::new(format!("node_{}", i % size));
                    index.remove(&node_id);
                }
                black_box(index)
            })
        });
    }

    group.finish();
}

fn benchmark_layout_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_algorithms");

    for size in [10, 50, 100, 500].iter() {
        let graph = create_test_graph(*size, *size / 2);

        group.bench_with_input(
            BenchmarkId::new("force_directed", size),
            &graph,
            |b, graph| {
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
            },
        );

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

        group.bench_with_input(
            BenchmarkId::new("hierarchical", size),
            &graph,
            |b, graph| {
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
            },
        );
    }

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    for size in [100, 1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("graph_memory", size), size, |b, &size| {
            b.iter(|| {
                let graph = create_test_graph(size, size / 2);
                black_box(graph)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("spatial_index_memory", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let index = create_test_spatial_index(size);
                    black_box(index)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_scaling_characteristics(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_characteristics");

    // Test how performance scales with different graph densities
    for density in [0.1, 0.3, 0.5, 0.8].iter() {
        let node_count = 1000;
        let edge_count = (node_count as f64 * density) as usize;
        let graph = create_test_graph(node_count, edge_count);

        group.bench_with_input(
            BenchmarkId::new("bounds_with_density", density),
            &graph,
            |b, graph| b.iter(|| black_box(graph.bounds())),
        );

        let index = create_test_spatial_index(node_count);
        group.bench_with_input(
            BenchmarkId::new("query_with_density", density),
            &index,
            |b, index: &SpatialIndex| {
                b.iter(|| {
                    let rect = flow_rs_core::Rect::new(0.0, 0.0, 1000.0, 1000.0);
                    black_box(index.query_rect(&rect))
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_graph_creation,
    benchmark_graph_operations,
    benchmark_spatial_index_operations,
    benchmark_spatial_index_updates,
    benchmark_layout_algorithms,
    benchmark_memory_usage,
    benchmark_scaling_characteristics
);

criterion_main!(benches);
