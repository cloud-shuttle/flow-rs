//! Tests for group system

// use crate::graph::{Graph, Node}; // Unused imports
// use crate::types::{GroupId, NodeId, Position}; // Unused imports
// use std::collections::HashSet; // Unused imports

// use super::manager::GroupManager; // Unused imports

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GroupId, NodeId, Position};
    use crate::graph::{Graph, Node};
    use crate::groups::manager::GroupManager;
    use std::collections::HashSet;

    fn create_test_nodes() -> Vec<NodeId> {
        vec![
            NodeId::new("node1"),
            NodeId::new("node2"),
            NodeId::new("node3"),
        ]
    }

    #[test]
    fn test_create_group_from_nodes() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.into_iter().take(2).collect();
        let group_id = GroupId::new("test_group");

        let result = manager.create_group(group_id.clone(), members.clone());

        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.members.len(), 2);
        assert!(group.contains_node(&NodeId::new("node1")));
        assert!(group.contains_node(&NodeId::new("node2")));
    }

    #[test]
    fn test_empty_group_creation_fails() {
        let mut manager = GroupManager::new();
        let empty_members = HashSet::new();
        let group_id = GroupId::new("empty_group");

        let result = manager.create_group(group_id, empty_members);

        assert!(result.is_err());
        assert!(matches!(result, Err(crate::error::FlowError::InvalidOperation { .. })));
    }

    #[test]
    fn test_node_already_in_group_fails() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members1: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let members2: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();

        let group_id1 = GroupId::new("group1");
        let group_id2 = GroupId::new("group2");

        manager.create_group(group_id1, members1).unwrap();

        let result = manager.create_group(group_id2, members2);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_group_creation_fails() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("duplicate_group");

        manager.create_group(group_id.clone(), members.clone()).unwrap();

        let result = manager.create_group(group_id, members);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        let group = manager.get_group(&group_id);
        assert!(group.is_some());
        assert_eq!(group.unwrap().id, group_id);
    }

    #[test]
    fn test_get_nonexistent_group() {
        let manager = GroupManager::new();
        let group_id = GroupId::new("nonexistent");

        let group = manager.get_group(&group_id);
        assert!(group.is_none());
    }

    #[test]
    fn test_get_node_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        let node_group = manager.get_node_group(&nodes[0]);
        assert!(node_group.is_some());
        assert_eq!(node_group.unwrap(), &group_id);
    }

    #[test]
    fn test_get_ungrouped_node() {
        let manager = GroupManager::new();
        let node_id = NodeId::new("ungrouped");

        let node_group = manager.get_node_group(&node_id);
        assert!(node_group.is_none());
    }

    #[test]
    fn test_dissolve_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.iter().take(2).cloned().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members.clone()).unwrap();
        assert_eq!(manager.group_count(), 1);

        let dissolved_members = manager.dissolve_group(&group_id).unwrap();
        assert_eq!(dissolved_members, members);
        assert_eq!(manager.group_count(), 0);

        // Verify nodes are no longer mapped
        for node_id in &members {
            assert!(manager.get_node_group(node_id).is_none());
        }
    }

    #[test]
    fn test_dissolve_nonexistent_group() {
        let mut manager = GroupManager::new();
        let group_id = GroupId::new("nonexistent");

        let result = manager.dissolve_group(&group_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_node_to_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let initial_members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), initial_members).unwrap();

        let result = manager.add_node_to_group(&group_id, nodes[1].clone());
        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.member_count(), 2);
        assert!(group.contains_node(&nodes[1]));
        assert!(manager.get_node_group(&nodes[1]).is_some());
    }

    #[test]
    fn test_add_node_to_nonexistent_group() {
        let mut manager = GroupManager::new();
        let node_id = NodeId::new("test_node");
        let group_id = GroupId::new("nonexistent");

        let result = manager.add_node_to_group(&group_id, node_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_node_already_in_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        let result = manager.add_node_to_group(&group_id, nodes[0].clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_node_from_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = nodes.iter().take(2).cloned().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members.clone()).unwrap();

        let result = manager.remove_node_from_group(&group_id, &nodes[0]);
        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.member_count(), 1);
        assert!(!group.contains_node(&nodes[0]));
        assert!(manager.get_node_group(&nodes[0]).is_none());
    }

    #[test]
    fn test_remove_node_from_nonexistent_group() {
        let mut manager = GroupManager::new();
        let node_id = NodeId::new("test_node");
        let group_id = GroupId::new("nonexistent");

        let result = manager.remove_node_from_group(&group_id, &node_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_node_not_in_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        let result = manager.remove_node_from_group(&group_id, &nodes[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_last_node_dissolves_group() {
        let mut manager = GroupManager::new();
        let node_id = NodeId::new("test_node");
        let members: HashSet<NodeId> = [node_id.clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();
        assert_eq!(manager.group_count(), 1);

        let result = manager.remove_node_from_group(&group_id, &node_id);
        assert!(result.is_ok());
        assert_eq!(manager.group_count(), 0);
        assert!(manager.get_group(&group_id).is_none());
    }

    #[test]
    fn test_set_group_name() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        manager.create_group(group_id.clone(), members).unwrap();

        let result = manager.set_group_name(&group_id, Some("New Name".to_string()));
        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.name, Some("New Name".to_string()));
    }

    #[test]
    fn test_set_group_name_nonexistent() {
        let mut manager = GroupManager::new();
        let group_id = GroupId::new("nonexistent");

        let result = manager.set_group_name(&group_id, Some("Name".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_group_bounds() {
        let mut manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();

        // Create nodes at specific positions
        let mut node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let mut node2 = Node::simple("node2", Position::new(200.0, 150.0));
        node1.set_size(crate::types::Size::new(50.0, 30.0));
        node2.set_size(crate::types::Size::new(50.0, 30.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        manager.create_group(group_id.clone(), members).unwrap();

        let result = manager.calculate_group_bounds(&group_id, &graph);
        assert!(result.is_ok());

        let group = manager.get_group(&group_id).unwrap();
        assert_eq!(group.position, Position::new(100.0, 100.0));
        assert_eq!(group.size.width, 150.0); // 250 - 100 (200 + 50 - 100)
        assert_eq!(group.size.height, 80.0); // 180 - 100 (150 + 30 - 100)
    }

    #[test]
    fn test_calculate_group_bounds_nonexistent() {
        let mut manager = GroupManager::new();
        let graph = Graph::<(), ()>::new();
        let group_id = GroupId::new("nonexistent");

        let result = manager.calculate_group_bounds(&group_id, &graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_group_bounds_empty_group() {
        let mut manager = GroupManager::new();
        let graph = Graph::<(), ()>::new();
        let group_id = GroupId::new("empty_group");

        // Create empty group (this should fail, but let's test bounds calculation)
        let empty_members = HashSet::new();
        let result = manager.create_group(group_id.clone(), empty_members);
        assert!(result.is_err()); // Empty groups should not be created
    }

    #[test]
    fn test_group_ids() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();

        let group_id1 = GroupId::new("group1");
        let group_id2 = GroupId::new("group2");
        let members1: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let members2: HashSet<NodeId> = [nodes[1].clone()].into_iter().collect();

        manager.create_group(group_id1.clone(), members1).unwrap();
        manager.create_group(group_id2.clone(), members2).unwrap();

        let group_ids = manager.group_ids();
        assert_eq!(group_ids.len(), 2);
        assert!(group_ids.contains(&group_id1));
        assert!(group_ids.contains(&group_id2));
    }

    #[test]
    fn test_has_group() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        assert!(!manager.has_group(&group_id));

        manager.create_group(group_id.clone(), members).unwrap();

        assert!(manager.has_group(&group_id));
    }

    #[test]
    fn test_group_count() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();

        assert_eq!(manager.group_count(), 0);

        let group_id1 = GroupId::new("group1");
        let group_id2 = GroupId::new("group2");
        let members1: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let members2: HashSet<NodeId> = [nodes[1].clone()].into_iter().collect();

        manager.create_group(group_id1, members1).unwrap();
        assert_eq!(manager.group_count(), 1);

        manager.create_group(group_id2, members2).unwrap();
        assert_eq!(manager.group_count(), 2);
    }

    #[test]
    fn test_is_node_grouped() {
        let mut manager = GroupManager::new();
        let nodes = create_test_nodes();
        let members: HashSet<NodeId> = [nodes[0].clone()].into_iter().collect();
        let group_id = GroupId::new("test_group");

        assert!(!manager.is_node_grouped(&nodes[0]));

        manager.create_group(group_id, members).unwrap();

        assert!(manager.is_node_grouped(&nodes[0]));
        assert!(!manager.is_node_grouped(&nodes[1]));
    }

    // Group Drag Tests

    #[test]
    fn test_start_group_drag() {
        let mut manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();

        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        graph.add_node(node1).unwrap();

        let members: HashSet<NodeId> = [NodeId::new("node1")].into_iter().collect();
        let group_id = GroupId::new("test_group");
        manager.create_group(group_id.clone(), members).unwrap();

        let result = manager.start_group_drag(&group_id, Position::zero(), &graph);
        assert!(result.is_ok());
        assert!(manager.is_group_dragging());
        assert_eq!(manager.get_dragging_group(), Some(&group_id));
    }

    #[test]
    fn test_start_group_drag_nonexistent() {
        let mut manager = GroupManager::new();
        let graph = Graph::<(), ()>::new();
        let group_id = GroupId::new("nonexistent");

        let result = manager.start_group_drag(&group_id, Position::zero(), &graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_start_group_drag_already_dragging() {
        let mut manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();

        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        graph.add_node(node1).unwrap();

        let members: HashSet<NodeId> = [NodeId::new("node1")].into_iter().collect();
        let group_id = GroupId::new("test_group");
        manager.create_group(group_id.clone(), members).unwrap();

        manager.start_group_drag(&group_id, Position::zero(), &graph).unwrap();

        let result = manager.start_group_drag(&group_id, Position::zero(), &graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_group_drag() {
        let mut manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();

        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let node2 = Node::simple("node2", Position::new(200.0, 150.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        manager.create_group(group_id.clone(), members).unwrap();

        manager
            .start_group_drag(&group_id, Position::new(125.0, 110.0), &graph)
            .unwrap();

        let new_pos = Position::new(175.0, 135.0);
        let result = manager.update_group_drag(new_pos, &mut graph);

        assert!(result.is_ok());
        let delta = result.unwrap();
        assert_eq!(delta, Position::new(50.0, 25.0));
        assert_eq!(
            manager.get_drag_delta(),
            Some(Position::new(50.0, 25.0))
        );

        // Check nodes moved
        let node1_after = graph.get_node(&NodeId::new("node1")).unwrap();
        let node2_after = graph.get_node(&NodeId::new("node2")).unwrap();
        assert_eq!(node1_after.position, Position::new(150.0, 125.0));
        assert_eq!(node2_after.position, Position::new(250.0, 175.0));
    }

    #[test]
    fn test_complete_group_drag() {
        let mut manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();

        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        graph.add_node(node1).unwrap();

        let members: HashSet<NodeId> = [NodeId::new("node1")].into_iter().collect();
        let group_id = GroupId::new("test_group");
        manager.create_group(group_id.clone(), members).unwrap();
        manager
            .start_group_drag(&group_id, Position::zero(), &graph)
            .unwrap();

        let result = manager.complete_group_drag();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), group_id);
        assert!(!manager.is_group_dragging());
        assert_eq!(manager.get_dragging_group(), None);
    }

    #[test]
    fn test_cancel_group_drag() {
        let mut manager = GroupManager::new();
        let mut graph = Graph::<(), ()>::new();

        let node1 = Node::simple("node1", Position::new(100.0, 100.0));
        let node2 = Node::simple("node2", Position::new(200.0, 150.0));
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        manager.create_group(group_id.clone(), members).unwrap();

        manager
            .start_group_drag(&group_id, Position::zero(), &graph)
            .unwrap();
        manager
            .update_group_drag(Position::new(50.0, 25.0), &mut graph)
            .unwrap();

        // Verify nodes moved
        assert_eq!(
            graph.get_node(&NodeId::new("node1")).unwrap().position,
            Position::new(150.0, 125.0)
        );

        // Cancel drag
        let result = manager.cancel_group_drag(&mut graph);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), group_id);
        assert!(!manager.is_group_dragging());

        // Verify nodes restored to original positions
        assert_eq!(
            graph.get_node(&NodeId::new("node1")).unwrap().position,
            Position::new(100.0, 100.0)
        );
        assert_eq!(
            graph.get_node(&NodeId::new("node2")).unwrap().position,
            Position::new(200.0, 150.0)
        );
    }

    #[test]
    fn test_drag_operations_without_active_drag() {
        let mut graph = Graph::<(), ()>::new();
        let mut manager = GroupManager::new();

        // Try to update drag without starting
        let result = manager.update_group_drag(Position::zero(), &mut graph);
        assert!(result.is_err());
        assert!(matches!(result, Err(crate::error::FlowError::InvalidOperation { .. })));

        // Try to complete drag without starting
        let result = manager.complete_group_drag();
        assert!(result.is_err());

        // Try to cancel drag without starting
        let result = manager.cancel_group_drag(&mut graph);
        assert!(result.is_err());
    }
}
