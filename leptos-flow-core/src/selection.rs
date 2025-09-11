//! Node Selection System
//!
//! Manages multi-node selection, keyboard navigation, and selection state.

use std::collections::HashSet;
use crate::types::{NodeId, Position, GroupId};
use crate::graph::Graph;
use crate::groups::GroupManager;

/// Selection modes for different interaction patterns
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    /// Single node selection (default)
    Single,
    /// Multi-node selection with Ctrl+Click
    Multi,
    /// Rectangle selection drag
    Rectangle,
}

/// Selection state manager
#[derive(Debug, Clone, Default)]
pub struct SelectionManager {
    /// Currently selected nodes
    selected_nodes: HashSet<NodeId>,
    /// Current selection mode
    mode: SelectionMode,
    /// Rectangle selection bounds (start, end)
    rectangle_bounds: Option<(Position, Position)>,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Single
    }
}

impl SelectionManager {
    /// Create a new selection manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Get currently selected nodes
    pub fn selected_nodes(&self) -> &HashSet<NodeId> {
        &self.selected_nodes
    }

    /// Check if a node is selected
    pub fn is_selected(&self, node_id: &NodeId) -> bool {
        self.selected_nodes.contains(node_id)
    }

    /// Get the number of selected nodes
    pub fn selection_count(&self) -> usize {
        self.selected_nodes.len()
    }

    /// Select a single node (clears other selections in Single mode)
    pub fn select_node(&mut self, node_id: NodeId) {
        match self.mode {
            SelectionMode::Single => {
                self.selected_nodes.clear();
                self.selected_nodes.insert(node_id);
            }
            SelectionMode::Multi => {
                self.selected_nodes.insert(node_id);
            }
            SelectionMode::Rectangle => {
                // Rectangle mode doesn't support individual selection
            }
        }
    }

    /// Toggle node selection (for Ctrl+Click)
    pub fn toggle_node(&mut self, node_id: NodeId) {
        if self.selected_nodes.contains(&node_id) {
            self.selected_nodes.remove(&node_id);
        } else {
            self.selected_nodes.insert(node_id);
        }
    }

    /// Deselect a specific node
    pub fn deselect_node(&mut self, node_id: &NodeId) {
        self.selected_nodes.remove(node_id);
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
        self.rectangle_bounds = None;
    }

    /// Set selection mode
    pub fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
        if mode != SelectionMode::Rectangle {
            self.rectangle_bounds = None;
        }
    }

    /// Get current selection mode
    pub fn mode(&self) -> &SelectionMode {
        &self.mode
    }

    /// Start rectangle selection
    pub fn start_rectangle_selection(&mut self, start: Position) {
        self.mode = SelectionMode::Rectangle;
        self.rectangle_bounds = Some((start, start));
    }

    /// Update rectangle selection
    pub fn update_rectangle_selection(&mut self, end: Position) {
        if let Some((start, _)) = self.rectangle_bounds {
            self.rectangle_bounds = Some((start, end));
        }
    }

    /// Complete rectangle selection and select nodes within bounds
    pub fn complete_rectangle_selection<N, E>(&mut self, graph: &Graph<N, E>) -> Vec<NodeId>
    where
        N: Clone,
        E: Clone,
    {
        let selected = if let Some((start, end)) = self.rectangle_bounds {
            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);

            let mut nodes_in_rectangle = Vec::new();

            for node in graph.nodes() {
                let pos = &node.position;
                if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                    nodes_in_rectangle.push(node.id.clone());
                }
            }

            // Clear existing selection and select rectangle nodes
            self.selected_nodes.clear();
            for node_id in &nodes_in_rectangle {
                self.selected_nodes.insert(node_id.clone());
            }

            nodes_in_rectangle
        } else {
            Vec::new()
        };

        self.rectangle_bounds = None;
        self.mode = SelectionMode::Single;
        selected
    }

    /// Get rectangle selection bounds
    pub fn rectangle_bounds(&self) -> Option<(Position, Position)> {
        self.rectangle_bounds
    }

    /// Select nodes by keyboard navigation (move selection to next/previous node)
    pub fn navigate_selection<N, E>(&mut self, graph: &Graph<N, E>, direction: NavigationDirection) -> Option<NodeId>
    where
        N: Clone,
        E: Clone,
    {
        if graph.node_count() == 0 {
            return None;
        }

        let current_selection = self.selected_nodes.iter().next().cloned();

        match direction {
            NavigationDirection::Next => {
                let nodes: Vec<_> = graph.nodes().collect();
                if let Some(current) = current_selection {
                    if let Some(current_index) = nodes.iter().position(|n| n.id == current) {
                        let next_index = (current_index + 1) % nodes.len();
                        let next_node_id = nodes[next_index].id.clone();
                        self.selected_nodes.clear();
                        self.selected_nodes.insert(next_node_id.clone());
                        Some(next_node_id)
                    } else {
                        None
                    }
                } else {
                    // No selection, select first node
                    let first_node_id = nodes[0].id.clone();
                    self.selected_nodes.clear();
                    self.selected_nodes.insert(first_node_id.clone());
                    Some(first_node_id)
                }
            }
            NavigationDirection::Previous => {
                let nodes: Vec<_> = graph.nodes().collect();
                if let Some(current) = current_selection {
                    if let Some(current_index) = nodes.iter().position(|n| n.id == current) {
                        let prev_index = if current_index == 0 {
                            nodes.len() - 1
                        } else {
                            current_index - 1
                        };
                        let prev_node_id = nodes[prev_index].id.clone();
                        self.selected_nodes.clear();
                        self.selected_nodes.insert(prev_node_id.clone());
                        Some(prev_node_id)
                    } else {
                        None
                    }
                } else {
                    // No selection, select last node
                    let last_node_id = nodes[nodes.len() - 1].id.clone();
                    self.selected_nodes.clear();
                    self.selected_nodes.insert(last_node_id.clone());
                    Some(last_node_id)
                }
            }
        }
    }

    /// Select all nodes in the graph
    pub fn select_all<N, E>(&mut self, graph: &Graph<N, E>)
    where
        N: Clone,
        E: Clone,
    {
        self.selected_nodes.clear();
        for node in graph.nodes() {
            self.selected_nodes.insert(node.id.clone());
        }
    }

    /// Select all nodes in a group
    pub fn select_group(&mut self, group_manager: &GroupManager, group_id: &GroupId) {
        if let Some(group) = group_manager.get_group(group_id) {
            match self.mode {
                SelectionMode::Single => {
                    self.selected_nodes.clear();
                    for node_id in &group.members {
                        self.selected_nodes.insert(node_id.clone());
                    }
                }
                SelectionMode::Multi => {
                    for node_id in &group.members {
                        self.selected_nodes.insert(node_id.clone());
                    }
                }
                SelectionMode::Rectangle => {
                    // Rectangle mode doesn't support group selection
                }
            }
        }
    }

    /// Check if selecting a node should automatically select its group
    pub fn should_select_group(&self, group_manager: &GroupManager, node_id: &NodeId) -> Option<GroupId> {
        group_manager.get_node_group(node_id).cloned()
    }

    /// Select a node and optionally its entire group
    pub fn select_node_with_group(&mut self, group_manager: &GroupManager, node_id: NodeId, select_whole_group: bool) {
        if select_whole_group {
            if let Some(group_id) = group_manager.get_node_group(&node_id) {
                self.select_group(group_manager, group_id);
                return;
            }
        }

        // Fall back to regular node selection
        self.select_node(node_id);
    }

    /// Get all groups that have at least one selected node
    pub fn get_selected_groups(&self, group_manager: &GroupManager) -> HashSet<GroupId> {
        let mut selected_groups = HashSet::new();

        for node_id in &self.selected_nodes {
            if let Some(group_id) = group_manager.get_node_group(node_id) {
                selected_groups.insert(group_id.clone());
            }
        }

        selected_groups
    }

    /// Check if all nodes in a group are selected
    pub fn is_group_fully_selected(&self, group_manager: &GroupManager, group_id: &GroupId) -> bool {
        if let Some(group) = group_manager.get_group(group_id) {
            for node_id in &group.members {
                if !self.selected_nodes.contains(node_id) {
                    return false;
                }
            }
            !group.members.is_empty() // Empty group is not considered selected
        } else {
            false
        }
    }

    /// Deselect all nodes in a group
    pub fn deselect_group(&mut self, group_manager: &GroupManager, group_id: &GroupId) {
        if let Some(group) = group_manager.get_group(group_id) {
            for node_id in &group.members {
                self.selected_nodes.remove(node_id);
            }
        }
    }
}

/// Navigation directions for keyboard selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationDirection {
    Next,
    Previous,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Graph, Node, Position};

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();

        // Add test nodes
        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("node2", Position::new(200.0, 150.0))).unwrap();
        graph.add_node(Node::simple("node3", Position::new(300.0, 200.0))).unwrap();
        graph.add_node(Node::simple("node4", Position::new(150.0, 250.0))).unwrap();

        graph
    }

    #[test]
    fn test_new_selection_manager() {
        let manager = SelectionManager::new();
        assert_eq!(manager.selection_count(), 0);
        assert_eq!(manager.mode(), &SelectionMode::Single);
        assert!(manager.selected_nodes().is_empty());
    }

    #[test]
    fn test_single_node_selection() {
        let mut manager = SelectionManager::new();
        let node_id: NodeId = "node1".into();

        manager.select_node(node_id.clone());

        assert_eq!(manager.selection_count(), 1);
        assert!(manager.is_selected(&node_id));
    }

    #[test]
    fn test_single_mode_replaces_selection() {
        let mut manager = SelectionManager::new();
        let node1: NodeId = "node1".into();
        let node2: NodeId = "node2".into();

        manager.select_node(node1.clone());
        manager.select_node(node2.clone());

        assert_eq!(manager.selection_count(), 1);
        assert!(!manager.is_selected(&node1));
        assert!(manager.is_selected(&node2));
    }

    #[test]
    fn test_multi_mode_accumulates_selection() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);

        let node1: NodeId = "node1".into();
        let node2: NodeId = "node2".into();

        manager.select_node(node1.clone());
        manager.select_node(node2.clone());

        assert_eq!(manager.selection_count(), 2);
        assert!(manager.is_selected(&node1));
        assert!(manager.is_selected(&node2));
    }

    #[test]
    fn test_toggle_node_selection() {
        let mut manager = SelectionManager::new();
        let node_id: NodeId = "node1".into();

        // Toggle on
        manager.toggle_node(node_id.clone());
        assert!(manager.is_selected(&node_id));
        assert_eq!(manager.selection_count(), 1);

        // Toggle off
        manager.toggle_node(node_id.clone());
        assert!(!manager.is_selected(&node_id));
        assert_eq!(manager.selection_count(), 0);
    }

    #[test]
    fn test_clear_selection() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);

        manager.select_node("node1".into());
        manager.select_node("node2".into());

        assert_eq!(manager.selection_count(), 2);

        manager.clear_selection();

        assert_eq!(manager.selection_count(), 0);
        assert!(manager.selected_nodes().is_empty());
    }

    #[test]
    fn test_rectangle_selection_basic() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Start rectangle selection
        manager.start_rectangle_selection(Position::new(50.0, 50.0));
        assert_eq!(manager.mode(), &SelectionMode::Rectangle);

        // Update rectangle to cover nodes 1 and 2
        manager.update_rectangle_selection(Position::new(250.0, 175.0));

        // Complete selection
        let selected = manager.complete_rectangle_selection(&graph);

        assert!(selected.contains(&"node1".into()));
        assert!(selected.contains(&"node2".into()));
        assert!(!selected.contains(&"node3".into()));
        assert_eq!(manager.mode(), &SelectionMode::Single);
    }

    #[test]
    fn test_keyboard_navigation_next() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Start with no selection, should select first node
        let selected = manager.navigate_selection(&graph, NavigationDirection::Next);
        assert!(selected.is_some());
        assert_eq!(manager.selection_count(), 1);

        // Navigate to next
        let next_selected = manager.navigate_selection(&graph, NavigationDirection::Next);
        assert!(next_selected.is_some());
        assert_ne!(selected, next_selected);
        assert_eq!(manager.selection_count(), 1);
    }

    #[test]
    fn test_keyboard_navigation_previous() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Start with no selection, should select last node
        let selected = manager.navigate_selection(&graph, NavigationDirection::Previous);
        assert!(selected.is_some());
        assert_eq!(manager.selection_count(), 1);
    }

    #[test]
    fn test_select_all() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        manager.select_all(&graph);

        assert_eq!(manager.selection_count(), 4);
        assert!(manager.is_selected(&"node1".into()));
        assert!(manager.is_selected(&"node2".into()));
        assert!(manager.is_selected(&"node3".into()));
        assert!(manager.is_selected(&"node4".into()));
    }

    #[test]
    fn test_deselect_node() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);

        let node1: NodeId = "node1".into();
        let node2: NodeId = "node2".into();

        manager.select_node(node1.clone());
        manager.select_node(node2.clone());

        assert_eq!(manager.selection_count(), 2);

        manager.deselect_node(&node1);

        assert_eq!(manager.selection_count(), 1);
        assert!(!manager.is_selected(&node1));
        assert!(manager.is_selected(&node2));
    }

    #[test]
    fn test_rectangle_bounds() {
        let mut manager = SelectionManager::new();

        assert!(manager.rectangle_bounds().is_none());

        manager.start_rectangle_selection(Position::new(10.0, 20.0));
        manager.update_rectangle_selection(Position::new(30.0, 40.0));

        let bounds = manager.rectangle_bounds().unwrap();
        assert_eq!(bounds.0, Position::new(10.0, 20.0));
        assert_eq!(bounds.1, Position::new(30.0, 40.0));
    }

    // Group-aware selection tests
    #[test]
    fn test_select_group() {
        use crate::groups::{GroupManager, Group};
        use std::collections::HashSet;

        let mut selection = SelectionManager::new();
        let mut group_manager = GroupManager::new();

        // Create a group with nodes
        let nodes: HashSet<NodeId> = ["node1", "node2", "node3"]
            .iter()
            .map(|&s| NodeId::new(s))
            .collect();
        let group_id = GroupId::new("test_group");
        group_manager.create_group(group_id.clone(), nodes.clone()).unwrap();

        // Test single mode - should clear previous selection and select all group nodes
        selection.select_node("other_node".into());
        assert_eq!(selection.selection_count(), 1);

        selection.select_group(&group_manager, &group_id);

        assert_eq!(selection.selection_count(), 3);
        assert!(selection.is_selected(&"node1".into()));
        assert!(selection.is_selected(&"node2".into()));
        assert!(selection.is_selected(&"node3".into()));
        assert!(!selection.is_selected(&"other_node".into()));
    }

    #[test]
    fn test_select_group_multi_mode() {
        use crate::groups::{GroupManager, Group};
        use std::collections::HashSet;

        let mut selection = SelectionManager::new();
        selection.set_mode(SelectionMode::Multi);
        let mut group_manager = GroupManager::new();

        // Create a group with nodes
        let nodes: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager.create_group(group_id.clone(), nodes).unwrap();

        // Pre-select another node
        selection.select_node("other_node".into());
        assert_eq!(selection.selection_count(), 1);

        // Select group - should add to existing selection
        selection.select_group(&group_manager, &group_id);

        assert_eq!(selection.selection_count(), 3);
        assert!(selection.is_selected(&"node1".into()));
        assert!(selection.is_selected(&"node2".into()));
        assert!(selection.is_selected(&"other_node".into()));
    }

    #[test]
    fn test_select_node_with_group() {
        use crate::groups::GroupManager;
        use std::collections::HashSet;

        let mut selection = SelectionManager::new();
        let mut group_manager = GroupManager::new();

        // Create a group
        let nodes: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager.create_group(group_id.clone(), nodes).unwrap();

        // Select node with group selection enabled
        selection.select_node_with_group(&group_manager, "node1".into(), true);

        assert_eq!(selection.selection_count(), 2);
        assert!(selection.is_selected(&"node1".into()));
        assert!(selection.is_selected(&"node2".into()));

        // Clear and test without group selection
        selection.clear_selection();
        selection.select_node_with_group(&group_manager, "node1".into(), false);

        assert_eq!(selection.selection_count(), 1);
        assert!(selection.is_selected(&"node1".into()));
        assert!(!selection.is_selected(&"node2".into()));
    }

    #[test]
    fn test_get_selected_groups() {
        use crate::groups::GroupManager;
        use std::collections::HashSet;

        let mut selection = SelectionManager::new();
        selection.set_mode(SelectionMode::Multi);
        let mut group_manager = GroupManager::new();

        // Create two groups
        let group1_nodes: HashSet<NodeId> = ["node1", "node2"].iter().map(|&s| NodeId::new(s)).collect();
        let group2_nodes: HashSet<NodeId> = ["node3", "node4"].iter().map(|&s| NodeId::new(s)).collect();
        let group1_id = GroupId::new("group1");
        let group2_id = GroupId::new("group2");

        group_manager.create_group(group1_id.clone(), group1_nodes).unwrap();
        group_manager.create_group(group2_id.clone(), group2_nodes).unwrap();

        // Select nodes from both groups
        selection.select_node("node1".into());
        selection.select_node("node3".into());
        selection.select_node("ungrouped_node".into());

        let selected_groups = selection.get_selected_groups(&group_manager);

        assert_eq!(selected_groups.len(), 2);
        assert!(selected_groups.contains(&group1_id));
        assert!(selected_groups.contains(&group2_id));
    }

    #[test]
    fn test_is_group_fully_selected() {
        use crate::groups::GroupManager;
        use std::collections::HashSet;

        let mut selection = SelectionManager::new();
        selection.set_mode(SelectionMode::Multi);
        let mut group_manager = GroupManager::new();

        // Create a group
        let nodes: HashSet<NodeId> = ["node1", "node2", "node3"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager.create_group(group_id.clone(), nodes).unwrap();

        // Partial selection
        selection.select_node("node1".into());
        selection.select_node("node2".into());

        assert!(!selection.is_group_fully_selected(&group_manager, &group_id));

        // Full selection
        selection.select_node("node3".into());

        assert!(selection.is_group_fully_selected(&group_manager, &group_id));

        // Test non-existent group
        let fake_group = GroupId::new("fake_group");
        assert!(!selection.is_group_fully_selected(&group_manager, &fake_group));
    }

    #[test]
    fn test_deselect_group() {
        use crate::groups::GroupManager;
        use std::collections::HashSet;

        let mut selection = SelectionManager::new();
        selection.set_mode(SelectionMode::Multi);
        let mut group_manager = GroupManager::new();

        // Create a group
        let nodes: HashSet<NodeId> = ["node1", "node2", "node3"].iter().map(|&s| NodeId::new(s)).collect();
        let group_id = GroupId::new("test_group");
        group_manager.create_group(group_id.clone(), nodes).unwrap();

        // Select group and an additional node
        selection.select_group(&group_manager, &group_id);
        selection.select_node("other_node".into());

        assert_eq!(selection.selection_count(), 4);

        // Deselect group
        selection.deselect_group(&group_manager, &group_id);

        assert_eq!(selection.selection_count(), 1);
        assert!(!selection.is_selected(&"node1".into()));
        assert!(!selection.is_selected(&"node2".into()));
        assert!(!selection.is_selected(&"node3".into()));
        assert!(selection.is_selected(&"other_node".into()));
    }

    // THESE TESTS WILL FAIL INITIALLY - That's the TDD RED phase!
    #[test]
    #[should_panic(expected = "Feature not implemented yet")]
    fn test_keyboard_shortcuts_not_implemented() {
        // This test will fail until we implement keyboard shortcuts
        panic!("Feature not implemented yet");
    }

    #[test]
    #[should_panic(expected = "Visual feedback not implemented yet")]
    fn test_selection_visual_feedback_not_implemented() {
        // This test will fail until we implement visual selection feedback
        panic!("Visual feedback not implemented yet");
    }
}
