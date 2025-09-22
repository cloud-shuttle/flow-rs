//! Tests for layout algorithms

#[cfg(test)]
mod tests {
    use crate::graph::{Edge, Node};
    use crate::types::Position;
    use std::collections::HashMap;
    
    // Import layout types
    use crate::layout::{
        CircularLayout, ForceDirectedLayout, GridLayout, HierarchicalLayout, 
        LayoutAlgorithm, LayoutUtils, LayoutDirection, EdgeRouting
    };

    fn create_test_graph() -> crate::graph::Graph<(), ()> {
        let mut graph = crate::graph::Graph::new();

        // Add nodes
        graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("2").position(100.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("3").position(50.0, 100.0).build())
            .unwrap();

        // Add edges
        graph
            .add_edge(Edge::builder().connect("1", "2").build().unwrap())
            .unwrap();
        graph
            .add_edge(Edge::builder().connect("2", "3").build().unwrap())
            .unwrap();
        graph
            .add_edge(Edge::builder().connect("3", "1").build().unwrap())
            .unwrap();

        graph
    }

    fn create_hierarchical_test_graph() -> crate::graph::Graph<(), ()> {
        let mut graph = crate::graph::Graph::new();

        // Create a simple tree structure:
        //     root
        //    /    \
        // child1  child2
        //   |
        // grandchild

        graph
            .add_node(Node::builder("root").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("child1").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("child2").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("grandchild").position(0.0, 0.0).build())
            .unwrap();

        graph
            .add_edge(Edge::builder().connect("root", "child1").build().unwrap())
            .unwrap();
        graph
            .add_edge(Edge::builder().connect("root", "child2").build().unwrap())
            .unwrap();
        graph
            .add_edge(
                Edge::builder()
                    .connect("child1", "grandchild")
                    .build()
                    .unwrap(),
            )
            .unwrap();

        graph
    }

    fn create_cyclic_graph() -> crate::graph::Graph<(), ()> {
        let mut graph = crate::graph::Graph::new();

        graph
            .add_node(Node::builder("a").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("b").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("c").position(0.0, 0.0).build())
            .unwrap();

        // Create a cycle: a -> b -> c -> a
        graph
            .add_edge(Edge::builder().connect("a", "b").build().unwrap())
            .unwrap();
        graph
            .add_edge(Edge::builder().connect("b", "c").build().unwrap())
            .unwrap();
        graph
            .add_edge(Edge::builder().connect("c", "a").build().unwrap())
            .unwrap();

        graph
    }

    #[test]
    fn test_force_directed_layout() {
        let mut graph = create_test_graph();
        let mut layout = ForceDirectedLayout::builder()
            .iterations(10)
            .randomize_start(false)
            .build();

        // Store initial positions
        let initial_positions: HashMap<_, _> = graph
            .nodes()
            .map(|node| (node.id.clone(), node.position))
            .collect();

        // Apply layout
        layout.apply(&mut graph).unwrap();

        // Check that positions changed
        let mut positions_changed = false;
        for node in graph.nodes() {
            if let Some(&initial_pos) = initial_positions.get(&node.id) {
                if node.position != initial_pos {
                    positions_changed = true;
                    break;
                }
            }
        }

        assert!(positions_changed, "Layout should change node positions");
        assert_eq!(LayoutAlgorithm::<(), ()>::progress(&layout), 1.0);
        assert!(!LayoutAlgorithm::<(), ()>::is_running(&layout));
    }

    #[test]
    fn test_grid_layout() {
        let mut graph = create_test_graph();
        let mut layout = GridLayout::new().columns(Some(2)).cell_size(100.0, 80.0);

        layout.apply(&mut graph).unwrap();

        // Check that nodes are positioned in a grid
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 3);

        // Collect all positions and verify they follow grid pattern
        let mut positions: Vec<Position> = graph.nodes().map(|n| n.position).collect();
        positions.sort_by(|a, b| {
            if (a.y - b.y).abs() < 1e-10 {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        // Verify grid layout: 2 columns, with cell size 100x80 and margin 20
        // Expected positions: (0,0), (120,0), (0,100)
        assert_eq!(positions[0], Position::new(0.0, 0.0));
        assert_eq!(positions[1], Position::new(120.0, 0.0));
        assert_eq!(positions[2], Position::new(0.0, 100.0));
    }

    #[test]
    fn test_layout_utils_center() {
        let mut graph = create_test_graph();
        LayoutUtils::center_graph(&mut graph);

        if let Some(bounds) = graph.bounds() {
            let center = bounds.center();
            // Center should be close to origin (allowing for floating point errors)
            assert!((center.x.abs() < 1e-10) && (center.y.abs() < 1e-10));
        }
    }

    #[test]
    fn test_layout_utils_scale() {
        let mut graph = create_test_graph();
        LayoutUtils::scale_to_fit(&mut graph, 200.0, 200.0);

        if let Some(bounds) = graph.bounds() {
            assert!(bounds.width <= 200.0);
            assert!(bounds.height <= 200.0);
        }
    }

    #[test]
    fn test_force_directed_builder() {
        let layout = ForceDirectedLayout::builder()
            .iterations(50)
            .spring_strength(0.8)
            .repulsion_strength(1500.0)
            .damping(0.95)
            .randomize_start(false)
            .build();

        assert_eq!(layout.iterations, 50);
        assert_eq!(layout.spring_strength, 0.8);
        assert_eq!(layout.repulsion_strength, 1500.0);
        assert_eq!(layout.damping, 0.95);
        assert!(!layout.randomize_start);
    }

    #[test]
    fn test_layout_interruption() {
        let _graph = create_test_graph();
        let mut layout = ForceDirectedLayout::builder().iterations(1000).build();

        assert!(LayoutAlgorithm::<(), ()>::can_interrupt(&layout));

        // Start layout in a separate context (simulated)
        // In real usage, this would be in a separate thread or async context
        layout.running = true;
        layout.current_iteration = 10;

        assert!(LayoutAlgorithm::<(), ()>::is_running(&layout));
        assert!(LayoutAlgorithm::<(), ()>::progress(&layout) < 1.0);

        LayoutAlgorithm::<(), ()>::stop(&mut layout).unwrap();
        assert!(!LayoutAlgorithm::<(), ()>::is_running(&layout));
    }

    #[test]
    fn test_hierarchical_layout() {
        let mut graph = create_hierarchical_test_graph();
        let mut layout = HierarchicalLayout::builder()
            .root_node("root")
            .node_separation(100.0)
            .level_separation(80.0)
            .direction(LayoutDirection::TopToBottom)
            .build();

        // Store initial positions
        let initial_positions: HashMap<_, _> = graph
            .nodes()
            .map(|node| (node.id.clone(), node.position))
            .collect();

        // Apply layout
        layout.apply(&mut graph).unwrap();

        // Verify positions changed
        let mut positions_changed = false;
        for node in graph.nodes() {
            if let Some(&initial_pos) = initial_positions.get(&node.id) {
                if node.position != initial_pos {
                    positions_changed = true;
                    break;
                }
            }
        }
        assert!(
            positions_changed,
            "Hierarchical layout should change node positions"
        );

        // Verify hierarchical structure
        let root = graph.get_node(&"root".into()).unwrap();
        let child1 = graph.get_node(&"child1".into()).unwrap();
        let child2 = graph.get_node(&"child2".into()).unwrap();
        let grandchild = graph.get_node(&"grandchild".into()).unwrap();

        // Root should be at top level
        assert_eq!(root.position.y, 0.0);

        // Children should be at second level
        assert_eq!(child1.position.y, 80.0);
        assert_eq!(child2.position.y, 80.0);

        // Grandchild should be at third level
        assert_eq!(grandchild.position.y, 160.0);

        // Children should be separated horizontally
        assert!((child1.position.x - child2.position.x).abs() >= 100.0);
    }

    #[test]
    fn test_hierarchical_layout_builder() {
        let layout = HierarchicalLayout::builder()
            .node_separation(150.0)
            .level_separation(120.0)
            .direction(LayoutDirection::LeftToRight)
            .edge_routing(EdgeRouting::Orthogonal)
            .root_node("custom-root")
            .build();

        assert_eq!(layout.node_separation, 150.0);
        assert_eq!(layout.level_separation, 120.0);
        assert_eq!(layout.direction, LayoutDirection::LeftToRight);
        assert_eq!(layout.edge_routing, EdgeRouting::Orthogonal);
        assert_eq!(layout.root_node, Some("custom-root".into()));
    }

    #[test]
    fn test_circular_layout() {
        let mut graph = create_test_graph();
        let mut layout = CircularLayout::new()
            .radius(150.0)
            .start_angle(std::f64::consts::PI / 4.0)
            .clockwise(false);

        // Store initial positions
        let initial_positions: HashMap<_, _> = graph
            .nodes()
            .map(|node| (node.id.clone(), node.position))
            .collect();

        // Apply layout
        layout.apply(&mut graph).unwrap();

        // Verify positions changed
        let mut positions_changed = false;
        for node in graph.nodes() {
            if let Some(&initial_pos) = initial_positions.get(&node.id) {
                if node.position != initial_pos {
                    positions_changed = true;
                    break;
                }
            }
        }
        assert!(
            positions_changed,
            "Circular layout should change node positions"
        );

        // Verify nodes are arranged in a circle
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 3);

        for node in &nodes {
            // Distance from origin should be approximately the radius
            let distance =
                (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            assert!(
                (distance - 150.0).abs() < 1e-10,
                "Node should be at radius distance from origin"
            );
        }
    }

    #[test]
    fn test_hierarchical_layout_cycle_detection() {
        let mut graph = create_cyclic_graph();
        let mut layout = HierarchicalLayout::new();

        // Should detect cycle and return error
        let result = layout.apply(&mut graph);
        assert!(result.is_err());

        if let Err(crate::error::FlowError::Layout { message: msg }) = result {
            assert!(msg.contains("Cycle detected") || msg.contains("No root nodes found"));
        } else {
            panic!("Expected LayoutError for cyclic graph");
        }
    }

    #[test]
    fn test_circular_layout_empty_graph() {
        let mut graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        let mut layout = CircularLayout::new();

        // Should handle empty graph gracefully
        let result = layout.apply(&mut graph);
        assert!(result.is_ok());
    }

    // Additional edge case tests for comprehensive coverage
    #[test]
    fn test_force_directed_energy_calculation() {
        let mut graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("2").position(100.0, 0.0).build())
            .unwrap();
        graph
            .add_edge(Edge::builder().connect("1", "2").build().unwrap())
            .unwrap();

        let layout = ForceDirectedLayout::builder()
            .spring_length(100.0)
            .spring_strength(1.0)
            .repulsion_strength(1000.0)
            .build();

        let energy = layout.calculate_energy(&graph);

        // Energy should be positive (spring energy + repulsion energy)
        assert!(energy >= 0.0, "Energy should be non-negative");
    }

    #[test]
    fn test_grid_layout_cell_calculations() {
        let mut graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        for i in 0..6 {
            graph
                .add_node(Node::builder(format!("{}", i)).position(0.0, 0.0).build())
                .unwrap();
        }

        let mut layout = GridLayout::new()
            .columns(Some(3))
            .cell_size(100.0, 80.0)
            .margin(20.0);

        layout.apply(&mut graph).unwrap();

        // Verify grid positioning
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 6);

        // Check that nodes are positioned in a 3x2 grid
        let mut positions: Vec<_> = nodes.iter().map(|n| n.position).collect();
        positions.sort_by(|a, b| {
            if (a.y - b.y).abs() < 1e-10 {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        // Expected positions with 3 columns, cell size 100x80, margin 20:
        // Row 0: (0,0), (120,0), (240,0)
        // Row 1: (0,100), (120,100), (240,100)
        assert_eq!(positions[0], Position::new(0.0, 0.0));
        assert_eq!(positions[1], Position::new(120.0, 0.0));
        assert_eq!(positions[2], Position::new(240.0, 0.0));
        assert_eq!(positions[3], Position::new(0.0, 100.0));
        assert_eq!(positions[4], Position::new(120.0, 100.0));
        assert_eq!(positions[5], Position::new(240.0, 100.0));
    }

    #[test]
    fn test_circular_layout_angle_calculations() {
        let mut graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        for i in 0..4 {
            graph
                .add_node(Node::builder(format!("{}", i)).position(0.0, 0.0).build())
                .unwrap();
        }

        let mut layout = CircularLayout::new()
            .radius(100.0)
            .start_angle(0.0)
            .clockwise(true);

        layout.apply(&mut graph).unwrap();

        // Verify circular positioning
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 4);

        for node in &nodes {
            // Distance from origin should be approximately the radius
            let distance =
                (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            assert!(
                (distance - 100.0).abs() < 1e-10,
                "Node should be at radius distance from origin"
            );
        }
    }

    #[test]
    fn test_layout_utils_padding_calculations() {
        let mut graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("2").position(100.0, 100.0).build())
            .unwrap();

        let initial_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

        // Apply padding of 50 units
        LayoutUtils::apply_padding(&mut graph, 50.0);

        let new_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

        // All positions should be shifted by (50, 50)
        for (old, new) in initial_positions.iter().zip(new_positions.iter()) {
            assert_eq!(new.x, old.x + 50.0);
            assert_eq!(new.y, old.y + 50.0);
        }
    }

    #[test]
    fn test_force_directed_edge_cases() {
        // Test with single node
        let mut single_node_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        single_node_graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();

        let mut layout = ForceDirectedLayout::builder().iterations(1).build();

        // Should complete without error
        let result = layout.apply(&mut single_node_graph);
        assert!(result.is_ok());
    }

    #[test]
    fn test_grid_layout_edge_cases() {
        // Test with zero nodes
        let mut empty_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        let mut layout = GridLayout::new();

        let result = layout.apply(&mut empty_graph);
        assert!(result.is_ok());

        // Test with single node
        let mut single_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        single_graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();

        let result_single = layout.apply(&mut single_graph);
        assert!(result_single.is_ok());
    }

    #[test]
    fn test_circular_layout_edge_cases() {
        // Test with zero nodes
        let mut empty_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        let mut layout = CircularLayout::new();

        let result = layout.apply(&mut empty_graph);
        assert!(result.is_ok());

        // Test with single node
        let mut single_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        single_graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();

        let result_single = layout.apply(&mut single_graph);
        assert!(result_single.is_ok());
    }

    #[test]
    fn test_hierarchical_layout_edge_cases() {
        // Test with empty graph
        let mut empty_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        let mut layout = HierarchicalLayout::new();

        let result = layout.apply(&mut empty_graph);
        assert!(result.is_ok());

        // Test with single node
        let mut single_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        single_graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();

        let result_single = layout.apply(&mut single_graph);
        assert!(result_single.is_ok());
    }

    #[test]
    fn test_layout_utils_edge_cases() {
        // Test center_graph with empty graph
        let mut empty_graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        LayoutUtils::center_graph(&mut empty_graph);
        // Should complete without error

        // Test scale_to_fit with zero target dimensions
        let mut graph: crate::graph::Graph<(), ()> = crate::graph::Graph::new();
        graph
            .add_node(Node::builder("1").position(0.0, 0.0).build())
            .unwrap();
        graph
            .add_node(Node::builder("2").position(100.0, 100.0).build())
            .unwrap();

        LayoutUtils::scale_to_fit(&mut graph, 0.0, 0.0);
        // Should complete without error

        // Test apply_padding with zero padding
        LayoutUtils::apply_padding(&mut graph, 0.0);
        // Should complete without error
    }
}
