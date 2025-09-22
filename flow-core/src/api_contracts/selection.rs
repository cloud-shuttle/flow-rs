//! Selection API Contracts

use crate::graph::{Graph, Node};
use crate::selection::{SelectionManager, SelectionMode};
use crate::types::{NodeId, Position};

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
    fn test_selection_manager_creation_api_contract() {
        let manager = SelectionManager::new();
        
        assert_eq!(manager.selection_count(), 0);
        assert!(manager.selected_nodes().is_empty());
        assert_eq!(*manager.mode(), SelectionMode::Single);
    }

    #[test]
    fn test_selection_manager_single_selection_api_contract() {
        let mut manager = SelectionManager::new();
        let mut graph = create_test_graph();

        // Test single selection
        let node_id = NodeId::new("node1");
        manager.select_node(node_id.clone());
        
        assert_eq!(manager.selection_count(), 1);
        assert!(manager.is_selected(&node_id));
        assert!(manager.selected_nodes().contains(&node_id));

        // Test deselecting
        manager.deselect_node(&node_id);
        assert_eq!(manager.selection_count(), 0);
        assert!(!manager.is_selected(&node_id));
    }

    #[test]
    fn test_selection_manager_multi_selection_api_contract() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);
        let mut graph = create_test_graph();

        // Test multiple selection
        let node1 = NodeId::new("node1");
        let node2 = NodeId::new("node2");
        
        manager.select_node(node1.clone());
        manager.select_node(node2.clone());
        
        assert_eq!(manager.selection_count(), 2);
        assert!(manager.is_selected(&node1));
        assert!(manager.is_selected(&node2));

        // Test clear selection
        manager.clear_selection();
        assert_eq!(manager.selection_count(), 0);
        assert!(!manager.is_selected(&node1));
        assert!(!manager.is_selected(&node2));
    }

    #[test]
    fn test_selection_manager_mode_api_contract() {
        let mut manager = SelectionManager::new();
        
        // Test default mode
        assert_eq!(*manager.mode(), SelectionMode::Single);
        
        // Test mode change
        manager.set_mode(SelectionMode::Multi);
        assert_eq!(*manager.mode(), SelectionMode::Multi);
        
        manager.set_mode(SelectionMode::Single);
        assert_eq!(*manager.mode(), SelectionMode::Single);
    }
}
