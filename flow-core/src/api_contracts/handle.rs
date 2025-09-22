//! Handle API Contracts

use crate::graph::{Graph, Node};
use crate::handle::{Handle, HandleId, HandleManager, HandlePosition, HandleType};
use crate::types::{NodeId, Position};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();
        graph.add_node(Node::new("node1", Position::new(10.0, 20.0), ())).unwrap();
        graph.add_node(Node::new("node2", Position::new(30.0, 40.0), ())).unwrap();
        graph
    }

    #[test]
    fn test_handle_creation_api_contract() {
        let handle = Handle::new("test_handle", HandleType::Source, HandlePosition::Top);
        
        assert_eq!(handle.id.as_str(), "test_handle");
        assert_eq!(handle.handle_type, HandleType::Source);
        assert_eq!(handle.position, HandlePosition::Top);
        assert!(handle.connection_limit.is_none());
    }

    #[test]
    fn test_handle_manager_creation_api_contract() {
        let node_id = NodeId::new("test_node");
        let manager = HandleManager::new(node_id);
        
        assert_eq!(manager.handles().len(), 0);
        assert!(manager.handles().is_empty());
    }

    #[test]
    fn test_handle_manager_operations_api_contract() {
        let node_id = NodeId::new("node1");
        let mut manager = HandleManager::new(node_id.clone());
        let mut graph = create_test_graph();

        let handle = Handle::new("handle1", HandleType::Source, HandlePosition::Top);

        // Test handle addition
        assert!(manager.add_handle(handle).is_ok());
        assert_eq!(manager.handles().len(), 1);

        // Test handle retrieval
        let handle_id = HandleId::new("handle1");
        assert!(manager.get_handle(&handle_id).is_some());
        let nonexistent_id = HandleId::new("nonexistent");
        assert!(manager.get_handle(&nonexistent_id).is_none());

        // Test handle removal
        assert!(manager.remove_handle(&handle_id).is_ok());
        assert_eq!(manager.handles().len(), 0);
        assert!(manager.get_handle(&handle_id).is_none());
    }

    #[test]
    fn test_handle_manager_node_handles_api_contract() {
        let node_id = NodeId::new("node1");
        let mut manager = HandleManager::new(node_id.clone());
        let mut graph = create_test_graph();

        let handle1 = Handle::new("handle1", HandleType::Source, HandlePosition::Top);
        let handle2 = Handle::new("handle2", HandleType::Target, HandlePosition::Bottom);

        manager.add_handle(handle1).unwrap();
        manager.add_handle(handle2).unwrap();

        // Test getting all handles for a node
        let handles = manager.handles();
        assert_eq!(handles.len(), 2);

        // Test getting handles by type
        let source_handles: Vec<_> = manager.source_handles().collect();
        assert_eq!(source_handles.len(), 1);
        assert_eq!(source_handles[0].id.as_str(), "handle1");

        let target_handles: Vec<_> = manager.target_handles().collect();
        assert_eq!(target_handles.len(), 1);
        assert_eq!(target_handles[0].id.as_str(), "handle2");
    }

    #[test]
    fn test_handle_connection_api_contract() {
        let node_id = NodeId::new("node1");
        let mut manager = HandleManager::new(node_id.clone());
        let mut graph = create_test_graph();

        let handle1 = Handle::new("handle1", HandleType::Source, HandlePosition::Right);
        let handle2 = Handle::new("handle2", HandleType::Target, HandlePosition::Left);

        manager.add_handle(handle1).unwrap();
        manager.add_handle(handle2).unwrap();

        // Test handle retrieval
        let handle_id1 = HandleId::new("handle1");
        let handle_id2 = HandleId::new("handle2");
        
        let handle1 = manager.get_handle(&handle_id1).unwrap();
        let handle2 = manager.get_handle(&handle_id2).unwrap();
        
        // Test handle types
        assert!(handle1.handle_type.is_source());
        assert!(handle2.handle_type.is_target());
        
        // Test connection compatibility
        assert!(handle1.can_connect_to(handle2));
        assert!(handle2.can_connect_to(handle1));
    }
}
