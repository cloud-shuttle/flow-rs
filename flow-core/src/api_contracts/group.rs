//! Group API Contracts

use crate::graph::{Graph, Node};
use crate::groups::GroupManager;
use crate::types::{GroupId, NodeId, Position};
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        graph.add_node(Node::new("node1", Position::new(10.0, 20.0), ())).unwrap();
        graph.add_node(Node::new("node2", Position::new(30.0, 40.0), ())).unwrap();
        graph.add_node(Node::new("node3", Position::new(50.0, 60.0), ())).unwrap();
        graph
    }

    #[test]
    fn test_group_manager_creation_api_contract() {
        let manager = GroupManager::new();
        
        assert_eq!(manager.group_count(), 0);
        assert!(manager.group_ids().is_empty());
    }

    #[test]
    fn test_group_creation_api_contract() {
        let mut manager = GroupManager::new();
        let mut graph = create_test_graph();

        // Test group creation
        let group_id = GroupId::new("test_group");
        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        
        assert!(manager.create_group(group_id.clone(), members).is_ok());
        assert_eq!(manager.group_count(), 1);
        assert!(manager.get_group(&group_id).is_some());

        // Test duplicate group creation
        let duplicate_members: HashSet<NodeId> = ["node3"].iter().map(|&s| NodeId::new(s)).collect();
        assert!(manager.create_group(group_id.clone(), duplicate_members).is_err());
    }

    #[test]
    fn test_group_member_management_api_contract() {
        let mut manager = GroupManager::new();
        let mut graph = create_test_graph();

        let group_id = GroupId::new("test_group");
        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        manager.create_group(group_id.clone(), members).unwrap();

        // Test adding member
        let node3 = NodeId::new("node3");
        assert!(manager.add_node_to_group(&group_id, node3.clone()).is_ok());
        
        let group = manager.get_group(&group_id).unwrap();
        assert!(group.contains_node(&node3));

        // Test removing member
        assert!(manager.remove_node_from_group(&group_id, &node3).is_ok());
        
        let group = manager.get_group(&group_id).unwrap();
        assert!(!group.contains_node(&node3));
    }

    #[test]
    fn test_group_deletion_api_contract() {
        let mut manager = GroupManager::new();
        let mut graph = create_test_graph();

        let group_id = GroupId::new("test_group");
        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        manager.create_group(group_id.clone(), members).unwrap();

        assert_eq!(manager.group_count(), 1);

        // Test group dissolution
        assert!(manager.dissolve_group(&group_id).is_ok());
        assert_eq!(manager.group_count(), 0);
        assert!(manager.get_group(&group_id).is_none());

        // Test dissolving non-existent group
        assert!(manager.dissolve_group(&group_id).is_err());
    }

    #[test]
    fn test_group_bounds_calculation_api_contract() {
        let mut manager = GroupManager::new();
        let mut graph = create_test_graph();

        let group_id = GroupId::new("test_group");
        let members: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        manager.create_group(group_id.clone(), members).unwrap();

        // Test bounds calculation
        assert!(manager.calculate_group_bounds(&group_id, &graph).is_ok());
        
        let group = manager.get_group(&group_id).unwrap();
        assert!(group.position.x >= 0.0);
        assert!(group.position.y >= 0.0);
        assert!(group.size.width > 0.0);
        assert!(group.size.height > 0.0);
    }
}
