//! Property-based tests for layout algorithms

use proptest::prelude::*;

use crate::{
    layout::{
        CircularLayout, EdgeRouting, ForceDirectedLayout, GridLayout, HierarchicalLayout,
        LayoutAlgorithm, LayoutDirection,
    },
    Graph,
};

use super::generators::*;

proptest! {
    #[test]
    fn test_force_directed_layout_properties(
        nodes in prop::collection::vec(arb_node(), 1..20)
    ) {
        let mut graph: Graph<(), ()> = Graph::new();

        // Add nodes
        for node in &nodes {
            let _ = graph.add_node(node.clone());
        }

        let mut layout = ForceDirectedLayout::builder()
            .iterations(10) // Reduced for faster testing
            .spring_length(100.0)
            .spring_strength(0.1)
            .repulsion_strength(1000.0)
            .damping(0.9)
            .center_force(0.01)
            .randomize_start(false)
            .build();

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
        // Allow small floating point tolerance for progress calculation
        let tolerance = 1e-10;
        prop_assert!((progress - 1.0).abs() < tolerance || (progress - 0.0).abs() < tolerance,
                    "Progress should be approximately 0.0 or 1.0 after completion, got {}", progress);
        prop_assert!(<ForceDirectedLayout as LayoutAlgorithm<(), ()>>::can_interrupt(&layout), "Force-directed layout should be interruptible");
    }

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
        let positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

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
            let distance_from_center = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            // Allow some tolerance for the radius (nodes have size, so they might be slightly off)
            prop_assert!((distance_from_center - 200.0).abs() < 50.0, 
                "Node should be approximately at the specified radius, distance: {}", distance_from_center);
        }
    }

    #[test]
    fn test_hierarchical_layout_properties(
        nodes in prop::collection::vec(arb_node(), 1..20)
    ) {
        let mut graph: Graph<(), ()> = Graph::new();

        // Add nodes
        for node in &nodes {
            let _ = graph.add_node(node.clone());
        }

        // Test different hierarchical layout configurations
        let layout_configs = [
            (LayoutDirection::TopToBottom, EdgeRouting::Straight),
            (LayoutDirection::LeftToRight, EdgeRouting::Orthogonal),
            (LayoutDirection::BottomToTop, EdgeRouting::Curved),
        ];

        for (direction, edge_routing) in layout_configs {
            let mut test_graph = graph.clone();
            let mut layout = HierarchicalLayout::builder()
                .node_separation(100.0)
                .level_separation(150.0)
                .edge_routing(edge_routing)
                .direction(direction)
                .build();

            // Apply layout
            let result = layout.apply(&mut test_graph);
            prop_assert!(result.is_ok(), "Hierarchical layout should succeed with direction {:?} and routing {:?}", direction, edge_routing);

            // Verify all unique nodes still exist
            let unique_node_count = nodes.iter().map(|n| &n.id).collect::<std::collections::HashSet<_>>().len();
            prop_assert_eq!(test_graph.node_count(), unique_node_count, "Node count should match unique nodes");

            // Verify layout algorithm properties
            prop_assert_eq!(<HierarchicalLayout as LayoutAlgorithm<(), ()>>::name(&layout), "Hierarchical");
            prop_assert!(!<HierarchicalLayout as LayoutAlgorithm<(), ()>>::is_running(&layout), "Layout should not be running after completion");
            // Progress should be 1.0 after completion (or 0.0 if no iterations were run)
            let progress = <HierarchicalLayout as LayoutAlgorithm<(), ()>>::progress(&layout);
            prop_assert!(progress == 1.0 || progress == 0.0, "Progress should be 0.0 or 1.0 after completion");

            // Verify all positions are finite
            for node in test_graph.nodes() {
                prop_assert!(node.position.x.is_finite(), "Position x should be finite");
                prop_assert!(node.position.y.is_finite(), "Position y should be finite");
            }
        }
    }

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
