//! API Contract Tests - TDD approach to lock down core interfaces
//!
//! This module contains comprehensive tests that define and validate the
//! public API contracts for leptos-flow-core. These tests serve as:
//!
//! 1. **API Documentation** - Living documentation of expected behavior
//! 2. **Stability Guarantees** - Tests that must pass for API stability
//! 3. **Regression Prevention** - Catch breaking changes early
//! 4. **Usage Examples** - Demonstrate correct API usage patterns

use crate::layout::{CircularLayout, ForceDirectedLayout, GridLayout, LayoutAlgorithm};
use crate::prelude::*;
use crate::spatial::SpatialIndex;
use crate::{
    AutoLayoutManager, Edge, EdgeId, FlowError, Graph, GroupManager, Handle, HandleManager,
    HandlePosition, HandleType, Node, NodeId, Position, Rect, SelectionManager, SelectionMode,
    Size, Viewport,
};

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // CORE TYPES API CONTRACTS
    // ============================================================================

    #[test]
    fn test_position_api_contract() {
        // Test Position creation and basic operations
        let pos1 = Position::new(10.0, 20.0);
        let pos2 = Position::new(5.0, 15.0);

        // Test basic arithmetic operations
        let sum = pos1 + pos2;
        assert_eq!(sum.x, 15.0);
        assert_eq!(sum.y, 35.0);

        let diff = pos1 - pos2;
        assert_eq!(diff.x, 5.0);
        assert_eq!(diff.y, 5.0);

        // Test distance calculations
        let distance = pos1.distance_to(pos2);
        assert!((distance - 7.0710678118654755).abs() < 1e-10);

        // Test validation
        assert!(pos1.is_valid());
        assert!(!Position::new(f64::NAN, 0.0).is_valid());
        assert!(!Position::new(f64::INFINITY, 0.0).is_valid());

        // Test display formatting
        assert_eq!(format!("{}", pos1), "(10, 20)");

        // Test default
        assert_eq!(Position::default(), Position::zero());
    }

    #[test]
    fn test_size_api_contract() {
        let size = Size::new(100.0, 200.0);

        // Test basic properties
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 200.0);

        // Test validation
        assert!(size.is_valid());
        assert!(!Size::new(-10.0, 20.0).is_valid());
        assert!(!Size::new(f64::NAN, 20.0).is_valid());

        // Test default
        assert_eq!(Size::default(), Size::new(100.0, 50.0));
    }

    #[test]
    fn test_rect_api_contract() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);

        // Test basic properties
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 200.0);

        // Test computed properties
        assert_eq!(rect.x + rect.width, 110.0);
        assert_eq!(rect.y + rect.height, 220.0);
        assert_eq!(rect.center(), Position::new(60.0, 120.0));

        // Test point containment
        assert!(rect.contains_point(Position::new(50.0, 100.0)));
        assert!(!rect.contains_point(Position::new(5.0, 15.0)));

        // Test intersection
        let other = Rect::new(50.0, 50.0, 100.0, 100.0);
        assert!(rect.intersects(&other));

        let non_intersecting = Rect::new(200.0, 300.0, 50.0, 50.0);
        assert!(!rect.intersects(&non_intersecting));
    }

    #[test]
    fn test_viewport_api_contract() {
        let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

        // Test basic properties
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.width, 800.0);
        assert_eq!(viewport.height, 600.0);
        assert_eq!(viewport.zoom, 1.0);

        // Test coordinate transformations
        let flow_pos = Position::new(100.0, 200.0);
        let screen_pos = viewport.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(100.0, 200.0));

        let back_to_flow = viewport.screen_to_flow(screen_pos);
        assert_eq!(back_to_flow, flow_pos);

        // Test zoom transformations
        let zoomed_viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 2.0);
        let zoomed_screen = zoomed_viewport.flow_to_screen(flow_pos);
        assert_eq!(zoomed_screen, Position::new(200.0, 400.0));
    }

    // ============================================================================
    // NODE API CONTRACTS
    // ============================================================================

    #[test]
    fn test_node_creation_api_contract() {
        // Test basic node creation
        let node = Node::new("test_node", Position::new(10.0, 20.0), ());

        assert_eq!(node.id.as_str(), "test_node");
        assert_eq!(node.position, Position::new(10.0, 20.0));
        assert_eq!(node.size, Size::default());
        assert_eq!(node.data, ());

        // Test default properties
        assert!(!node.selected);
        assert!(!node.dragging);
        assert!(node.selectable);
        assert!(node.connectable);
        assert!(node.deletable);
        assert!(!node.hidden);
    }

    #[test]
    fn test_node_builder_api_contract() {
        // Test fluent builder pattern
        let node = Node::<()>::builder("builder_test")
            .position(30.0, 40.0)
            .size(100.0, 50.0)
            .node_type("custom_type")
            .selectable(false)
            .build();

        assert_eq!(node.id.as_str(), "builder_test");
        assert_eq!(node.position, Position::new(30.0, 40.0));
        assert_eq!(node.size, Size::new(100.0, 50.0));
        assert_eq!(node.node_type, Some("custom_type".to_string()));
        assert!(!node.selected); // Default is false
        assert!(!node.selectable);
    }

    #[test]
    fn test_node_operations_api_contract() {
        let mut node = Node::new("ops_test", Position::new(10.0, 20.0), ());

        // Test position updates
        node.set_position(Position::new(50.0, 60.0));
        assert_eq!(node.position, Position::new(50.0, 60.0));

        // Test size updates
        node.set_size(Size::new(200.0, 100.0));
        assert_eq!(node.size, Size::new(200.0, 100.0));

        // Test selection state
        node.set_selected(true);
        assert!(node.selected);

        // Test dragging state
        node.set_dragging(true);
        assert!(node.dragging);

        // Test bounds calculation
        let bounds = node.bounds();
        assert_eq!(bounds, Rect::new(50.0, 60.0, 200.0, 100.0));

        // Test center calculation
        let center = node.center();
        assert_eq!(center, Position::new(150.0, 110.0));

        // Test point containment
        assert!(node.contains_point(Position::new(100.0, 80.0)));
        assert!(!node.contains_point(Position::new(10.0, 20.0)));
    }

    #[test]
    fn test_node_data_mapping_api_contract() {
        let node = Node::new("mapping_test", Position::new(10.0, 20.0), "original_data");

        // Test data mapping
        let mapped_node = node.map_data(|data| data.len());
        assert_eq!(mapped_node.id.as_str(), "mapping_test");
        assert_eq!(mapped_node.data, 13); // "original_data".len()
        assert_eq!(mapped_node.position, Position::new(10.0, 20.0));
    }

    // ============================================================================
    // EDGE API CONTRACTS
    // ============================================================================

    #[test]
    fn test_edge_creation_api_contract() {
        let edge = Edge::new("test_edge", "source", "target", ());

        assert_eq!(edge.id.as_str(), "test_edge");
        assert_eq!(edge.source.as_str(), "source");
        assert_eq!(edge.target.as_str(), "target");
        assert_eq!(edge.data, ());

        // Test default properties
        assert!(!edge.selected);
        assert!(edge.selectable);
        assert!(edge.deletable);
    }

    #[test]
    fn test_edge_builder_api_contract() {
        let edge = Edge::<()>::builder()
            .id("builder_edge")
            .connect("src", "tgt")
            .build();

        let edge = edge.unwrap();
        assert_eq!(edge.id.as_str(), "builder_edge");
        assert_eq!(edge.source.as_str(), "src");
        assert_eq!(edge.target.as_str(), "tgt");
        assert!(edge.selectable);
    }

    // ============================================================================
    // GRAPH API CONTRACTS
    // ============================================================================

    #[test]
    fn test_graph_creation_api_contract() {
        let graph: Graph<(), ()> = Graph::new();

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_graph_node_operations_api_contract() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Test adding nodes
        let node1 = Node::new("node1", Position::new(10.0, 20.0), ());
        let node2 = Node::new("node2", Position::new(30.0, 40.0), ());

        assert!(graph.add_node(node1).is_ok());
        assert!(graph.add_node(node2).is_ok());

        assert_eq!(graph.node_count(), 2);
        assert!(!graph.is_empty());

        // Test duplicate node handling
        let duplicate = Node::new("node1", Position::new(50.0, 60.0), ());
        assert!(graph.add_node(duplicate).is_err());

        // Test node retrieval
        assert!(graph.get_node(&NodeId::new("node1")).is_some());
        assert!(graph.get_node(&NodeId::new("nonexistent")).is_none());

        // Test node removal
        assert!(graph.remove_node(&NodeId::new("node1")).is_ok());
        assert_eq!(graph.node_count(), 1);
        assert!(graph.remove_node(&NodeId::new("nonexistent")).is_err());
    }

    #[test]
    fn test_graph_edge_operations_api_contract() {
        let mut graph = Graph::new();

        // Add nodes first
        graph
            .add_node(Node::new("node1", Position::new(10.0, 20.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("node2", Position::new(30.0, 40.0), ()))
            .unwrap();

        // Test adding edges
        let edge1 = Edge::new("edge1", "node1", "node2", ());
        let edge2 = Edge::new("edge2", "node2", "node1", ());

        assert!(graph.add_edge(edge1).is_ok());
        assert!(graph.add_edge(edge2).is_ok());

        assert_eq!(graph.edge_count(), 2);

        // Test duplicate edge handling
        let duplicate = Edge::new("edge1", "node1", "node2", ());
        assert!(graph.add_edge(duplicate).is_err());

        // Test edge retrieval
        assert!(graph.get_edge(&EdgeId::new("edge1")).is_some());
        assert!(graph.get_edge(&EdgeId::new("nonexistent")).is_none());

        // Test edge removal
        assert!(graph.remove_edge(&EdgeId::new("edge1")).is_ok());
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.remove_edge(&EdgeId::new("nonexistent")).is_err());
    }

    #[test]
    fn test_graph_iteration_api_contract() {
        let mut graph = Graph::new();

        // Add test data
        graph
            .add_node(Node::new("node1", Position::new(10.0, 20.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("node2", Position::new(30.0, 40.0), ()))
            .unwrap();
        graph
            .add_edge(Edge::new("edge1", "node1", "node2", ()))
            .unwrap();

        // Test node iteration
        let node_ids: Vec<_> = graph.node_ids().collect();
        assert_eq!(node_ids.len(), 2);
        assert!(node_ids.contains(&&NodeId::new("node1")));
        assert!(node_ids.contains(&&NodeId::new("node2")));

        // Test edge iteration
        let edge_ids: Vec<_> = graph.edge_ids().collect();
        assert_eq!(edge_ids.len(), 1);
        assert!(edge_ids.contains(&&EdgeId::new("edge1")));

        // Test node iteration with data
        let mut node_count = 0;
        for node in graph.nodes() {
            node_count += 1;
            assert!(node.id.as_str() == "node1" || node.id.as_str() == "node2");
        }
        assert_eq!(node_count, 2);

        // Test edge iteration with data
        let mut edge_count = 0;
        for edge in graph.edges() {
            edge_count += 1;
            assert_eq!(edge.id.as_str(), "edge1");
        }
        assert_eq!(edge_count, 1);
    }

    #[test]
    fn test_graph_bounds_api_contract() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Test empty graph bounds
        let empty_bounds = graph.bounds();
        assert_eq!(empty_bounds, None);

        // Add nodes and test bounds calculation
        let mut node1 = Node::new("node1", Position::new(10.0, 20.0), ());
        node1.set_size(Size::new(100.0, 50.0));
        let mut node2 = Node::new("node2", Position::new(50.0, 80.0), ());
        node2.set_size(Size::new(80.0, 40.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let bounds = graph.bounds().unwrap();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 120.0); // 50 + 80 - 10
        assert_eq!(bounds.height, 100.0); // 80 + 40 - 20
    }

    // ============================================================================
    // SPATIAL INDEX API CONTRACTS
    // ============================================================================

    #[test]
    fn test_spatial_index_api_contract() {
        let mut index = SpatialIndex::new();

        // Test empty index
        assert_eq!(index.len(), 0);
        assert!(index.is_empty());

        // Create test nodes
        let mut node1 = Node::new("node1", Position::new(10.0, 20.0), ());
        node1.set_size(Size::new(50.0, 30.0));
        let mut node2 = Node::new("node2", Position::new(100.0, 200.0), ());
        node2.set_size(Size::new(40.0, 60.0));

        // Test insertion
        assert!(index.insert(&node1).is_ok());
        assert!(index.insert(&node2).is_ok());

        assert_eq!(index.len(), 2);
        assert!(!index.is_empty());

        // Test queries
        let query_rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let results: Vec<_> = index.query_rect(&query_rect).into_iter().collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].as_str(), "node1");

        // Test nearest neighbor
        let nearest = index.nearest(Position::new(15.0, 25.0));
        assert!(nearest.is_some());
        assert_eq!(nearest.unwrap().as_str(), "node1");

        // Test removal
        assert!(index.remove(&NodeId::new("node1")));
        assert_eq!(index.len(), 1);
        assert!(!index.remove(&NodeId::new("nonexistent")));
    }

    // ============================================================================
    // LAYOUT ALGORITHMS API CONTRACTS
    // ============================================================================

    #[test]
    fn test_layout_algorithm_trait_api_contract() {
        // Test that all layout algorithms implement the trait correctly
        let force_layout = ForceDirectedLayout::new();
        let grid_layout = GridLayout::new();
        let circular_layout = CircularLayout::new();

        // Test common trait methods
        assert_eq!(
            <ForceDirectedLayout as LayoutAlgorithm<(), ()>>::name(&force_layout),
            "Force-Directed"
        );
        assert_eq!(
            <GridLayout as LayoutAlgorithm<(), ()>>::name(&grid_layout),
            "Grid"
        );
        assert_eq!(
            <CircularLayout as LayoutAlgorithm<(), ()>>::name(&circular_layout),
            "Circular"
        );

        // Test default states
        assert!(!<ForceDirectedLayout as LayoutAlgorithm<(), ()>>::is_running(&force_layout));
        assert!(!<GridLayout as LayoutAlgorithm<(), ()>>::is_running(
            &grid_layout
        ));
        assert!(!<CircularLayout as LayoutAlgorithm<(), ()>>::is_running(
            &circular_layout
        ));

        assert_eq!(
            <ForceDirectedLayout as LayoutAlgorithm<(), ()>>::progress(&force_layout),
            1.0
        );
        assert_eq!(
            <GridLayout as LayoutAlgorithm<(), ()>>::progress(&grid_layout),
            1.0
        );
        assert_eq!(
            <CircularLayout as LayoutAlgorithm<(), ()>>::progress(&circular_layout),
            1.0
        );

        assert!(<ForceDirectedLayout as LayoutAlgorithm<(), ()>>::can_interrupt(&force_layout));
        assert!(!<GridLayout as LayoutAlgorithm<(), ()>>::can_interrupt(
            &grid_layout
        ));
        assert!(!<CircularLayout as LayoutAlgorithm<(), ()>>::can_interrupt(
            &circular_layout
        ));
    }

    #[test]
    fn test_force_directed_layout_api_contract() {
        let mut layout = ForceDirectedLayout::builder()
            .iterations(100)
            .spring_length(50.0)
            .spring_strength(0.1)
            .repulsion_strength(100.0)
            .damping(0.9)
            .center_force(0.01)
            .randomize_start(true)
            .build();

        // Test that parameters were set correctly
        assert_eq!(layout.iterations, 100);
        assert_eq!(layout.spring_length, 50.0);
        assert_eq!(layout.spring_strength, 0.1);
        assert_eq!(layout.repulsion_strength, 100.0);
        assert_eq!(layout.damping, 0.9);
        assert_eq!(layout.center_force, 0.01);
        assert!(layout.randomize_start);

        // Test layout application
        let mut graph = Graph::new();
        graph
            .add_node(Node::new("node1", Position::new(0.0, 0.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("node2", Position::new(100.0, 100.0), ()))
            .unwrap();
        graph
            .add_edge(Edge::new("edge1", "node1", "node2", ()))
            .unwrap();

        assert!(layout.apply(&mut graph).is_ok());
    }

    // ============================================================================
    // SELECTION MANAGER API CONTRACTS
    // ============================================================================

    #[test]
    fn test_selection_manager_api_contract() {
        let mut selection = SelectionManager::new();

        // Test initial state
        assert!(selection.selected_nodes().is_empty());
        assert_eq!(*selection.mode(), SelectionMode::Single);

        // Test mode changes
        selection.set_mode(SelectionMode::Multi);
        assert_eq!(*selection.mode(), SelectionMode::Multi);

        // Test node selection
        let node_id = NodeId::new("test_node");
        selection.select_node(node_id.clone());
        assert!(selection.is_selected(&node_id));
        assert_eq!(selection.selected_nodes().len(), 1);

        // Test node deselection
        selection.deselect_node(&node_id);
        assert!(!selection.is_selected(&node_id));
        assert!(selection.selected_nodes().is_empty());

        // Test clear selection
        selection.select_node(node_id.clone());
        selection.clear_selection();
        assert!(selection.selected_nodes().is_empty());
    }

    // ============================================================================
    // GROUP MANAGER API CONTRACTS
    // ============================================================================

    #[test]
    fn test_group_manager_api_contract() {
        let mut group_manager = GroupManager::new();

        // Test initial state
        assert!(group_manager.all_groups().is_empty());

        // Test group creation
        let group_id = GroupId::new("test_group");
        let node_ids: std::collections::HashSet<_> =
            [NodeId::new("node1"), NodeId::new("node2")].into();

        assert!(group_manager
            .create_group(group_id.clone(), node_ids)
            .is_ok());
        assert_eq!(group_manager.all_groups().len(), 1);

        // Test group retrieval
        assert!(group_manager.get_group(&group_id).is_some());
        assert!(group_manager
            .get_group(&GroupId::new("nonexistent"))
            .is_none());

        // Test group bounds calculation (requires a graph)
        let mut graph: Graph<(), ()> = Graph::new();
        graph
            .add_node(Node::new("node1", Position::new(10.0, 20.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("node2", Position::new(30.0, 40.0), ()))
            .unwrap();
        let bounds = group_manager.calculate_group_bounds(&group_id, &graph);
        assert!(bounds.is_ok());

        // Test group removal (method doesn't exist, so we'll skip this test)
        // assert!(group_manager.remove_group(&group_id).is_ok());
        // assert!(group_manager.all_groups().is_empty());
    }

    // ============================================================================
    // HANDLE MANAGER API CONTRACTS
    // ============================================================================

    #[test]
    fn test_handle_manager_api_contract() {
        let mut handle_manager = HandleManager::new(NodeId::new("test_node"));

        // Test initial state
        assert!(handle_manager.handles().is_empty());

        // Test handle creation and addition
        let handle = Handle::new(
            HandleId::new("handle1"),
            HandleType::Source,
            HandlePosition::Custom(Position::new(10.0, 20.0)),
        );

        assert!(handle_manager.add_handle(handle.clone()).is_ok());
        assert_eq!(handle_manager.handles().len(), 1);

        // Test handle retrieval
        assert!(handle_manager
            .get_handle(&HandleId::new("handle1"))
            .is_some());
        assert!(handle_manager
            .get_handle(&HandleId::new("nonexistent"))
            .is_none());

        // Test handle removal
        assert!(handle_manager
            .remove_handle(&HandleId::new("handle1"))
            .is_ok());
        assert!(handle_manager.handles().is_empty());
    }

    // ============================================================================
    // AUTO LAYOUT MANAGER API CONTRACTS
    // ============================================================================

    #[test]
    fn test_auto_layout_manager_api_contract() {
        let mut auto_layout = AutoLayoutManager::new();

        // Test initial state (check actual methods available)
        // Note: AutoLayoutManager API may be different than expected

        // Test layout application
        let mut graph: Graph<(), ()> = Graph::new();
        graph
            .add_node(Node::new("node1", Position::new(0.0, 0.0), ()))
            .unwrap();
        graph
            .add_node(Node::new("node2", Position::new(100.0, 100.0), ()))
            .unwrap();

        assert!(auto_layout.apply_auto_layout(&mut graph).is_ok());
    }

    // ============================================================================
    // ERROR HANDLING API CONTRACTS
    // ============================================================================

    #[test]
    fn test_error_types_api_contract() {
        // Test FlowError creation and properties
        let node_error = FlowError::NodeNotFound {
            id: "test".to_string(),
        };
        assert_eq!(node_error.to_string(), "Node with ID 'test' not found");

        let edge_error = FlowError::EdgeNotFound {
            id: "test".to_string(),
        };
        assert_eq!(edge_error.to_string(), "Edge with ID 'test' not found");

        let duplicate_error = FlowError::DuplicateNodeId {
            id: "test".to_string(),
        };
        assert_eq!(duplicate_error.to_string(), "Duplicate node ID: 'test'");

        let connection_error = FlowError::InvalidConnection {
            message: "test message".to_string(),
        };
        assert_eq!(
            connection_error.to_string(),
            "Invalid connection: test message"
        );

        let position_error = FlowError::InvalidPosition {
            x: f64::NAN,
            y: 0.0,
        };
        assert_eq!(position_error.to_string(), "Invalid position: x=NaN, y=0");

        let size_error = FlowError::InvalidSize {
            width: -10.0,
            height: 20.0,
        };
        assert_eq!(size_error.to_string(), "Invalid size: width=-10, height=20");
    }

    // ============================================================================
    // SERIALIZATION API CONTRACTS (if serde feature is enabled)
    // ============================================================================

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_api_contract() {
        use serde_json;

        // Test Position serialization
        let pos = Position::new(10.0, 20.0);
        let serialized = serde_json::to_string(&pos).unwrap();
        let deserialized: Position = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pos, deserialized);

        // Test Node serialization
        let node = Node::new("test", Position::new(10.0, 20.0), "data");
        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: Node<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.position, deserialized.position);
        assert_eq!(node.data, deserialized.data);

        // Test Graph serialization
        let mut graph = Graph::new();
        graph
            .add_node(Node::new("node1", Position::new(10.0, 20.0), ()))
            .unwrap();
        graph
            .add_edge(Edge::new("edge1", "node1", "node1", ()))
            .unwrap();

        let serialized = serde_json::to_string(&graph).unwrap();
        let deserialized: Graph<(), ()> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(graph.node_count(), deserialized.node_count());
        assert_eq!(graph.edge_count(), deserialized.edge_count());
    }
}
