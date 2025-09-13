//! Node Selection System
//!
//! Manages multi-node selection, keyboard navigation, and selection state.

use std::collections::{HashSet, HashMap};
use crate::types::{NodeId, Position, GroupId};
use crate::graph::Graph;
use crate::groups::GroupManager;

/// Visual feedback state for nodes
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VisualFeedback {
    /// Node is currently selected
    selected: bool,
    /// Node is currently hovered
    hovered: bool,
    /// Node is highlighted (e.g., during connection)
    highlighted: bool,
    /// Animation state for transitions
    animation_progress: f64,
}

impl VisualFeedback {
    /// Create new visual feedback with default state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create visual feedback with selection state
    pub fn selected() -> Self {
        Self {
            selected: true,
            ..Default::default()
        }
    }

    /// Check if node is selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Check if node is hovered
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Check if node is highlighted
    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn animation_progress(&self) -> f64 {
        self.animation_progress
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Set hover state
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    /// Set highlight state
    pub fn set_highlighted(&mut self, highlighted: bool) {
        self.highlighted = highlighted;
    }

    /// Set animation progress
    pub fn set_animation_progress(&mut self, progress: f64) {
        self.animation_progress = progress.clamp(0.0, 1.0);
    }

    /// Check if any visual state is active
    pub fn has_any_state(&self) -> bool {
        self.selected || self.hovered || self.highlighted
    }
}

/// Selection modes for different interaction patterns
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectionMode {
    /// Single node selection (default)
    #[default]
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
    /// Visual feedback for all nodes
    visual_feedback: HashMap<NodeId, VisualFeedback>,
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
                // Clear visual feedback for previously selected nodes
                for old_node_id in &self.selected_nodes {
                    if let Some(feedback) = self.visual_feedback.get_mut(old_node_id) {
                        feedback.set_selected(false);
                    }
                }
                self.selected_nodes.clear();
                self.selected_nodes.insert(node_id.clone());
            }
            SelectionMode::Multi => {
                self.selected_nodes.insert(node_id.clone());
            }
            SelectionMode::Rectangle => {
                // Rectangle mode doesn't support individual selection
                return;
            }
        }

        // Update visual feedback for selected node
        self.visual_feedback.entry(node_id)
            .or_insert_with(VisualFeedback::new)
            .set_selected(true);
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

        // Update visual feedback for deselected node
        if let Some(feedback) = self.visual_feedback.get_mut(node_id) {
            feedback.set_selected(false);
            // Remove feedback if no states are active
            if !feedback.has_any_state() {
                self.visual_feedback.remove(node_id);
            }
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        // Clear visual feedback for all selected nodes
        for node_id in &self.selected_nodes {
            if let Some(feedback) = self.visual_feedback.get_mut(node_id) {
                feedback.set_selected(false);
                // Remove feedback if no other states are active
                if !feedback.has_any_state() {
                    self.visual_feedback.remove(node_id);
                }
            }
        }

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

    /// Handle keyboard shortcuts for selection management (read-only operations)
    pub fn handle_keyboard_shortcut<N, E>(&mut self, graph: &Graph<N, E>, shortcut: KeyboardShortcut)
    where
        N: Clone,
        E: Clone,
    {
        match shortcut {
            KeyboardShortcut::SelectAll => {
                self.selected_nodes.clear();
                for node in graph.nodes() {
                    self.selected_nodes.insert(node.id.clone());
                }
            }
            KeyboardShortcut::Escape => {
                self.clear_selection();
            }
            KeyboardShortcut::ArrowRight => {
                self.navigate_selection(graph, NavigationDirection::Next);
            }
            KeyboardShortcut::ArrowLeft => {
                self.navigate_selection(graph, NavigationDirection::Previous);
            }
            KeyboardShortcut::ArrowUp => {
                // For now, treat up/down same as left/right
                // In future, we could implement spatial navigation
                self.navigate_selection(graph, NavigationDirection::Previous);
            }
            KeyboardShortcut::ArrowDown => {
                self.navigate_selection(graph, NavigationDirection::Next);
            }
            KeyboardShortcut::Delete => {
                // Delete operation requires mutable graph - this will be handled separately
                // For now, we only clear the selection
                self.clear_selection();
            }
        }
    }

    /// Handle destructive keyboard shortcuts that modify the graph
    pub fn handle_destructive_keyboard_shortcut<N, E>(&mut self, graph: &mut Graph<N, E>, shortcut: KeyboardShortcut)
    where
        N: Clone,
        E: Clone,
    {
        match shortcut {
            KeyboardShortcut::Delete => {
                // Remove all selected nodes from the graph
                let nodes_to_remove: Vec<_> = self.selected_nodes.iter().cloned().collect();
                for node_id in nodes_to_remove {
                    let _ = graph.remove_node(&node_id);
                }
                // Clear selection after deletion
                self.clear_selection();
            }
            _ => {
                // For non-destructive operations, use the regular method
                self.handle_keyboard_shortcut(graph, shortcut);
            }
        }
    }

    // Visual Feedback System Methods

    /// Check if a node has visual feedback
    pub fn has_visual_feedback(&self, node_id: &NodeId) -> bool {
        self.visual_feedback.contains_key(node_id)
    }

    /// Get visual feedback for a node
    pub fn get_visual_feedback(&self, node_id: &NodeId) -> Option<&VisualFeedback> {
        self.visual_feedback.get(node_id)
    }

    /// Get mutable visual feedback for a node
    pub fn get_visual_feedback_mut(&mut self, node_id: &NodeId) -> Option<&mut VisualFeedback> {
        self.visual_feedback.get_mut(node_id)
    }

    /// Set hover state for a node
    pub fn set_hover_state(&mut self, node_id: &NodeId, hovered: bool) {
        let feedback = self.visual_feedback.entry(node_id.clone())
            .or_insert_with(VisualFeedback::new);
        feedback.set_hovered(hovered);

        // Clean up if no states are active
        if !feedback.has_any_state() {
            self.visual_feedback.remove(node_id);
        }
    }

    /// Set highlight state for a node
    pub fn set_highlight_state(&mut self, node_id: &NodeId, highlighted: bool) {
        let feedback = self.visual_feedback.entry(node_id.clone())
            .or_insert_with(VisualFeedback::new);
        feedback.set_highlighted(highlighted);

        // Clean up if no states are active
        if !feedback.has_any_state() {
            self.visual_feedback.remove(node_id);
        }
    }

    /// Clear all visual feedback for a node
    pub fn clear_all_visual_feedback(&mut self, node_id: &NodeId) {
        self.visual_feedback.remove(node_id);
    }

    /// Clear all visual feedback for all nodes
    pub fn clear_all_visual_feedbacks(&mut self) {
        self.visual_feedback.clear();
    }

    /// Update animation progress for a node
    pub fn update_animation_progress(&mut self, node_id: &NodeId, progress: f64) {
        if let Some(feedback) = self.visual_feedback.get_mut(node_id) {
            feedback.set_animation_progress(progress);
        }
    }

    /// Get all nodes with active visual feedback
    pub fn nodes_with_visual_feedback(&self) -> Vec<&NodeId> {
        self.visual_feedback.keys().collect()
    }
}

/// Navigation directions for keyboard selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationDirection {
    Next,
    Previous,
}

/// Keyboard shortcuts for selection management
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyboardShortcut {
    /// Ctrl+A - Select all nodes
    SelectAll,
    /// Delete - Remove selected nodes
    Delete,
    /// Escape - Clear selection
    Escape,
    /// Arrow keys for navigation
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
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
        use crate::groups::GroupManager;
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
        use crate::groups::GroupManager;
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
    fn test_select_all_keyboard_shortcut() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Initially nothing should be selected
        assert_eq!(manager.selection_count(), 0);

        // Ctrl+A should select all nodes
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::SelectAll);

        assert_eq!(manager.selection_count(), 4);
        assert!(manager.is_selected(&"node1".into()));
        assert!(manager.is_selected(&"node2".into()));
        assert!(manager.is_selected(&"node3".into()));
        assert!(manager.is_selected(&"node4".into()));
    }

    #[test]
    fn test_delete_selected_nodes_keyboard_shortcut() {
        let mut manager = SelectionManager::new();
        let mut graph = create_test_graph();

        // Select some nodes first
        manager.select_node("node1".into());
        manager.set_mode(SelectionMode::Multi);
        manager.select_node("node2".into());

        assert_eq!(manager.selection_count(), 2);
        assert_eq!(graph.node_count(), 4);

        // Delete key should remove selected nodes from graph and clear selection
        manager.handle_destructive_keyboard_shortcut(&mut graph, KeyboardShortcut::Delete);

        assert_eq!(manager.selection_count(), 0);
        assert_eq!(graph.node_count(), 2);
        assert!(graph.get_node(&"node3".into()).is_some());
        assert!(graph.get_node(&"node4".into()).is_some());
        assert!(graph.get_node(&"node1".into()).is_none());
        assert!(graph.get_node(&"node2".into()).is_none());
    }

    #[test]
    fn test_escape_clear_selection_keyboard_shortcut() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Select some nodes
        manager.select_node("node1".into());
        manager.set_mode(SelectionMode::Multi);
        manager.select_node("node2".into());

        assert_eq!(manager.selection_count(), 2);

        // Escape should clear selection
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::Escape);

        assert_eq!(manager.selection_count(), 0);
    }

    #[test]
    fn test_arrow_key_navigation_keyboard_shortcuts() {
        let mut manager = SelectionManager::new();
        let graph = create_test_graph();

        // Start with no selection
        assert_eq!(manager.selection_count(), 0);

        // Right arrow should select next node
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowRight);
        assert_eq!(manager.selection_count(), 1);
        let first_selected = manager.selected_nodes().iter().next().cloned();

        // Right arrow again should move to next node
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowRight);
        assert_eq!(manager.selection_count(), 1);
        let second_selected = manager.selected_nodes().iter().next().cloned();
        assert_ne!(first_selected, second_selected);

        // Left arrow should move back
        manager.handle_keyboard_shortcut(&graph, KeyboardShortcut::ArrowLeft);
        assert_eq!(manager.selection_count(), 1);
        let back_selected = manager.selected_nodes().iter().next().cloned();
        assert_eq!(first_selected, back_selected);
    }

    #[test]
    fn test_selection_visual_feedback_implementation() {
        // RED PHASE: Test the visual feedback system for selections
        let mut manager = SelectionManager::new();
        let node_id = NodeId::from("test_node");

        // Test initial state - no visual feedback
        assert!(!manager.has_visual_feedback(&node_id));
        assert!(manager.get_visual_feedback(&node_id).is_none());

        // Test selection creates visual feedback
        manager.select_node(node_id.clone());
        assert!(manager.has_visual_feedback(&node_id));

        // Test visual feedback properties
        let feedback = manager.get_visual_feedback(&node_id).expect("Visual feedback should exist");
        assert!(feedback.is_selected());
        assert!(!feedback.is_hovered());
        assert!(!feedback.is_highlighted());

        // Test hover state
        manager.set_hover_state(&node_id, true);
        let feedback = manager.get_visual_feedback(&node_id).expect("Visual feedback should exist");
        assert!(feedback.is_selected());
        assert!(feedback.is_hovered());

        // Test highlight state
        manager.set_highlight_state(&node_id, true);
        let feedback = manager.get_visual_feedback(&node_id).expect("Visual feedback should exist");
        assert!(feedback.is_selected());
        assert!(feedback.is_hovered());
        assert!(feedback.is_highlighted());

        // Test deselection clears visual feedback
        manager.deselect_node(&node_id);
        // Visual feedback may still exist for hover state
        let feedback = manager.get_visual_feedback(&node_id);
        if let Some(feedback) = feedback {
            assert!(!feedback.is_selected());
            assert!(feedback.is_hovered()); // Still hovering
        }

        // Test clearing all states
        manager.clear_all_visual_feedback(&node_id);
        assert!(!manager.has_visual_feedback(&node_id));
    }

    #[test]
    fn test_visual_feedback_edge_cases() {
        let mut manager = SelectionManager::new();
        let node_id = NodeId::from("test_node");

        // Test setting hover state without selection
        manager.set_hover_state(&node_id, true);
        assert!(manager.has_visual_feedback(&node_id));
        let feedback = manager.get_visual_feedback(&node_id).unwrap();
        assert!(!feedback.is_selected());
        assert!(feedback.is_hovered());

        // Test clearing hover removes feedback if no other states
        manager.set_hover_state(&node_id, false);
        assert!(!manager.has_visual_feedback(&node_id));

        // Test animation progress clamping
        manager.update_animation_progress(&node_id, 1.5);
        // No feedback should be created for animation alone
        assert!(!manager.has_visual_feedback(&node_id));

        // Create feedback first, then test animation
        manager.set_highlight_state(&node_id, true);
        manager.update_animation_progress(&node_id, 1.5); // Should clamp to 1.0
        let feedback = manager.get_visual_feedback(&node_id).unwrap();
        assert_eq!(feedback.animation_progress(), 1.0);

        manager.update_animation_progress(&node_id, -0.5); // Should clamp to 0.0
        let feedback = manager.get_visual_feedback(&node_id).unwrap();
        assert_eq!(feedback.animation_progress(), 0.0);
    }

    #[test]
    fn test_visual_feedback_multi_selection_integration() {
        let mut manager = SelectionManager::new();
        manager.set_mode(SelectionMode::Multi);

        let node1 = NodeId::from("node1");
        let node2 = NodeId::from("node2");
        let node3 = NodeId::from("node3");

        // Select multiple nodes
        manager.select_node(node1.clone());
        manager.select_node(node2.clone());
        manager.select_node(node3.clone());

        // All should have visual feedback for selection
        assert!(manager.has_visual_feedback(&node1));
        assert!(manager.has_visual_feedback(&node2));
        assert!(manager.has_visual_feedback(&node3));

        // Check selection states
        assert!(manager.get_visual_feedback(&node1).unwrap().is_selected());
        assert!(manager.get_visual_feedback(&node2).unwrap().is_selected());
        assert!(manager.get_visual_feedback(&node3).unwrap().is_selected());

        // Add hover to one node
        manager.set_hover_state(&node2, true);
        let feedback2 = manager.get_visual_feedback(&node2).unwrap();
        assert!(feedback2.is_selected());
        assert!(feedback2.is_hovered());

        // Clear selection should update visual feedback
        manager.clear_selection();

        // Only node2 should still have feedback (for hover)
        assert!(!manager.has_visual_feedback(&node1));
        assert!(manager.has_visual_feedback(&node2)); // Still has hover
        assert!(!manager.has_visual_feedback(&node3));

        let feedback2 = manager.get_visual_feedback(&node2).unwrap();
        assert!(!feedback2.is_selected());
        assert!(feedback2.is_hovered());
    }

    #[test]
    fn test_visual_feedback_single_selection_mode() {
        let mut manager = SelectionManager::new(); // Default is Single mode

        let node1 = NodeId::from("node1");
        let node2 = NodeId::from("node2");

        // Select first node
        manager.select_node(node1.clone());
        assert!(manager.get_visual_feedback(&node1).unwrap().is_selected());

        // Select second node - should clear first node's selection state
        manager.select_node(node2.clone());

        // First node should lose selection visual feedback
        if let Some(feedback1) = manager.get_visual_feedback(&node1) {
            assert!(!feedback1.is_selected());
        } else {
            // Or might be completely removed if no other states
            assert!(!manager.has_visual_feedback(&node1));
        }

        // Second node should have selection visual feedback
        assert!(manager.get_visual_feedback(&node2).unwrap().is_selected());
    }

    #[test]
    fn test_visual_feedback_batch_operations() {
        let mut manager = SelectionManager::new();
        let nodes: Vec<NodeId> = (0..10).map(|i| NodeId::from(format!("node{}", i))).collect();

        // Set all nodes to different states
        for (i, node_id) in nodes.iter().enumerate() {
            if i % 3 == 0 {
                manager.select_node(node_id.clone());
            }
            if i % 2 == 0 {
                manager.set_hover_state(node_id, true);
            }
            if i % 5 == 0 {
                manager.set_highlight_state(node_id, true);
            }
        }

        // Verify states
        let feedback_nodes = manager.nodes_with_visual_feedback();
        assert!(!feedback_nodes.is_empty());

        // Clear all visual feedback
        manager.clear_all_visual_feedbacks();

        // Verify all feedback is cleared
        for node_id in &nodes {
            assert!(!manager.has_visual_feedback(node_id));
        }

        // But selection state should remain in the selection manager
        assert!(!manager.selected_nodes().is_empty()); // Some nodes were selected
    }
}
