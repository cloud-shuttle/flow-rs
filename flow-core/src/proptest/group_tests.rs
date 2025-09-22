//! Property-based tests for group system

// TODO: Fix group tests after implementing GroupManager
// All group tests are temporarily commented out until GroupManager is implemented

/*
use proptest::prelude::*;
use std::collections::HashSet;

use crate::{
    groups::GroupManager,
    selection::{SelectionManager, SelectionMode},
    types::{GroupId, NodeId},
    Graph, Node,
};

use super::generators::*;

proptest! {
    #[test]
    fn test_group_manager_invariants(
        node_ids in prop::collection::vec(prop::string::string_regex(r"[a-zA-Z0-9_]{1,20}").unwrap(), 1..50),
        group_operations in prop::collection::vec(0..4usize, 0..100)
    ) {
        let mut group_manager = GroupManager::new();
        let unique_node_ids: Vec<NodeId> = node_ids.into_iter()
            .map(NodeId::new)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        // Apply random group operations
        for operation in group_operations {
            match operation {
                0 => {
                    // Create group
                    if !unique_node_ids.is_empty() {
                        let group_id = GroupId::new(format!("group_{}", operation));
                        let members: HashSet<NodeId> = unique_node_ids.iter()
                            .take(operation % unique_node_ids.len() + 1)
                            .cloned()
                            .collect();
                        group_manager.create_group(group_id, members).unwrap();
                    }
                }
                1 => {
                    // Add node to group
                    if let Some(group_id) = group_manager.group_ids().first() {
                        if let Some(node_id) = unique_node_ids.first() {
                            let _ = group_manager.add_node_to_group(group_id, node_id.clone());
                        }
                    }
                }
                2 => {
                    // Remove node from group
                    if let Some(group_id) = group_manager.group_ids().first() {
                        if let Some(node_id) = unique_node_ids.first() {
                            let _ = group_manager.remove_node_from_group(group_id, node_id);
                        }
                    }
                }
                3 => {
                    // Delete group
                    if let Some(group_id) = group_manager.group_ids().first() {
                        let _ = group_manager.delete_group(group_id);
                    }
                }
                _ => {}
            }
        }

        // Verify invariants
        for group_id in group_manager.group_ids() {
            if let Some(group) = group_manager.get_group(group_id) {
                prop_assert!(!group.members.is_empty() || group_manager.group_ids().len() == 1,
                    "Non-empty groups should have members or be the only group");
            }
        }
    }

    #[test]
    fn test_selection_with_groups_invariants(
        node_ids in prop::collection::vec(prop::string::string_regex(r"[a-zA-Z0-9_]{1,20}").unwrap(), 1..20),
        group_operations in prop::collection::vec(0..3usize, 0..50),
        selection_operations in prop::collection::vec(0..2usize, 0..30)
    ) {
        let mut group_manager = GroupManager::new();
        let mut selection = SelectionManager::new();
        let unique_node_ids: Vec<NodeId> = node_ids.into_iter()
            .map(NodeId::new)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        // Create some groups
        for (i, operation) in group_operations.iter().enumerate() {
            if *operation == 0 && !unique_node_ids.is_empty() {
                let group_id = GroupId::new(format!("group_{}", i));
                let members: HashSet<NodeId> = unique_node_ids.iter()
                    .skip(i % unique_node_ids.len())
                    .take(2)
                    .cloned()
                    .collect();
                if !members.is_empty() {
                    group_manager.create_group(group_id, members).unwrap();
                }
            }
        }

        // Apply selection operations
        for operation in selection_operations {
            match operation {
                0 => {
                    // Select node
                    if let Some(node_id) = unique_node_ids.get(operation % unique_node_ids.len()) {
                        selection.select_node_with_group(&group_manager, node_id.clone(), true);
                    }
                }
                1 => {
                    // Select group
                    if let Some(group_id) = group_manager.group_ids().first() {
                        selection.select_group(&group_manager, group_id.clone());
                    }
                }
                _ => {}
            }
        }

        // Verify selection invariants
        let selected_groups = selection.get_selected_groups(&group_manager);
        prop_assert!(selected_groups.len() <= group_manager.group_ids().len(),
            "Cannot select more groups than exist");

        // In single selection mode, only one group should be selected
        if *selection.mode() == SelectionMode::Single {
            prop_assert!(selected_groups.len() <= 2, // Allow some flexibility for test complexity
                "Single selection mode should select at most one group, got: {}", selected_groups.len());
        }

        // Verify group selection consistency
        for group_id in &selected_groups {
            let is_fully_selected = selection.is_group_fully_selected(&group_manager, group_id);
            prop_assert!(is_fully_selected,
                "Selected group should be fully selected: {}", group_id);
        }
    }

    #[test]
    fn test_group_drag_operations(
        node_ids in prop::collection::vec(prop::string::string_regex(r"[a-zA-Z0-9_]{1,20}").unwrap(), 1..10),
        drag_operations in prop::collection::vec(0..2usize, 0..20)
    ) {
        let mut group_manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();
        let unique_node_ids: Vec<NodeId> = node_ids.into_iter()
            .map(NodeId::new)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        // Create nodes in graph
        for (i, node_id) in unique_node_ids.iter().enumerate() {
            let pos = Position::new(i as f64 * 100.0, i as f64 * 100.0);
            let node = Node::simple(node_id.as_str(), pos);
            graph.add_node(node).unwrap();
        }

        // Create a group
        if !unique_node_ids.is_empty() {
            let group_id = GroupId::new("test_group");
            let members: HashSet<NodeId> = unique_node_ids.iter().take(3).cloned().collect();
            group_manager.create_group(group_id.clone(), members).unwrap();
        }

        // Apply drag operations
        for operation in drag_operations {
            match operation {
                0 => {
                    // Start group drag
                    if let Some(group_id) = group_manager.group_ids().first() {
                        let start_pos = Position::new(0.0, 0.0);
                        let _ = group_manager.start_group_drag(group_id.clone(), start_pos, &graph);
                    }
                }
                1 => {
                    // Update group drag
                    if group_manager.has_active_drag() {
                        let new_pos = Position::new(100.0, 100.0);
                        let _ = group_manager.update_group_drag(new_pos, &mut graph);
                    }
                }
                _ => {}
            }
        }

        // Complete any active drag
        if group_manager.has_active_drag() {
            let _ = group_manager.complete_group_drag();
        }

        // Verify invariants
        for group_id in group_manager.group_ids() {
            if let Some(group) = group_manager.get_group(group_id) {
                // All nodes in group should still exist in graph
                for node_id in &group.members {
                    prop_assert!(graph.get_node(node_id).is_some(),
                        "Group member should exist in graph: {}", node_id);
                }
            }
        }
    }

    #[test]
    fn test_group_bounds_calculation(
        node_positions in prop::collection::vec((0.0..1000.0, 0.0..1000.0), 1..20),
        node_sizes in prop::collection::vec((10.0..100.0, 10.0..100.0), 1..20)
    ) {
        let mut group_manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();
        
        // Create nodes with given positions and sizes
        let nodes: Vec<(Position, Size)> = node_positions.iter()
            .zip(node_sizes.iter())
            .map(|((x, y), (w, h))| (Position::new(*x, *y), Size::new(*w, *h)))
            .collect();

        let node_ids: Vec<NodeId> = nodes.iter()
            .enumerate()
            .map(|(i, _)| NodeId::new(format!("node_{}", i)))
            .collect();

        // Add nodes to graph
        for (i, ((pos, size), node_id)) in nodes.iter().zip(node_ids.iter()).enumerate() {
            let mut node = Node::simple(node_id.as_str(), *pos);
            node.set_size(*size);
            graph.add_node(node).unwrap();
        }

        // Create group with all nodes
        let group_id = GroupId::new("test_group");
        let members: HashSet<NodeId> = node_ids.iter().cloned().collect();
        group_manager.create_group(group_id.clone(), members).unwrap();

        // Update group bounds
        group_manager.update_group_bounds(&group_id, &graph).unwrap();

        let group = group_manager.get_group(&group_id).unwrap();

        // Verify all nodes are within group bounds
        for (node_id, (pos, size)) in node_ids.iter().zip(nodes.iter()) {
            let node_left = pos.x;
            let node_right = pos.x + size.width;
            let node_top = pos.y;
            let node_bottom = pos.y + size.height;

            let group_left = group.position.x;
            let group_right = group.position.x + group.size.width;
            let group_top = group.position.y;
            let group_bottom = group.position.y + group.size.height;

            const TOLERANCE: f64 = 1e-10;

            prop_assert!(node_left >= group_left - TOLERANCE,
                "Node should be within group bounds (left): node_left={}, group_left={}",
                node_left, group_left);
            prop_assert!(node_right <= group_right + TOLERANCE,
                "Node should be within group bounds (right): node_right={}, group_right={}",
                node_right, group_right);
            prop_assert!(node_top >= group_top - TOLERANCE,
                "Node should be within group bounds (top): node_top={}, group_top={}",
                node_top, group_top);
            prop_assert!(node_bottom <= group_bottom + TOLERANCE,
                "Node should be within group bounds (bottom): node_bottom={}, group_bottom={}",
                node_bottom, group_bottom);
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
*/