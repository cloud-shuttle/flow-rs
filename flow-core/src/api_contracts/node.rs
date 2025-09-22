//! Node API Contracts

use crate::graph::Node;
use crate::types::{Position, Rect, Size};

#[cfg(test)]
mod tests {
    use super::*;

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
}
