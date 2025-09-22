//! Selection state manager

use std::collections::{HashMap, HashSet};

use crate::graph::Graph;
// use crate::groups::GroupManager; // TODO: Implement GroupManager
use crate::types::{NodeId, Position};

use super::modes::{KeyboardShortcut, NavigationDirection, SelectionMode};
use super::visual_feedback::VisualFeedback;

/// Selection state manager
#[derive(Debug, Clone, Default)]
pub struct SelectionManager {
    /// Currently selected nodes
    selected_nodes: HashSet<NodeId>,
    /// Current selection mode
    mode: SelectionMode,
    /// Rectangle selection bounds (start, end)
    rectangle_bounds: Option<(Position, Position)>,
    /// Visual feedback states for nodes
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

    /// Select a node (behavior depends on current mode)
    pub fn select_node(&mut self, node_id: NodeId) {
        match self.mode {
            SelectionMode::Single => {
                // Single mode: replace selection
                // Clear visual feedback for all previously selected nodes
                let previously_selected: Vec<_> = self.selected_nodes.iter().cloned().collect();
                for prev_node_id in previously_selected {
                    self.set_selection_visual_feedback(&prev_node_id, false);
                }
                self.selected_nodes.clear();
                self.selected_nodes.insert(node_id.clone());
                // Update visual feedback
                self.set_selection_visual_feedback(&node_id, true);
            }
            SelectionMode::Multi => {
                // Multi mode: add to selection
                self.selected_nodes.insert(node_id.clone());
                self.set_selection_visual_feedback(&node_id, true);
            }
            SelectionMode::Rectangle => {
                // Rectangle mode: handled by complete_rectangle_selection
            }
        }
    }

    /// Toggle node selection (add if not selected, remove if selected)
    pub fn toggle_node(&mut self, node_id: NodeId) {
        if self.selected_nodes.contains(&node_id) {
            self.deselect_node(&node_id);
        } else {
            self.select_node(node_id);
        }
    }

    /// Deselect a node
    pub fn deselect_node(&mut self, node_id: &NodeId) {
        self.selected_nodes.remove(node_id);
        self.set_selection_visual_feedback(node_id, false);
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        // Clear visual feedback for all selected nodes
        let selected_nodes: Vec<_> = self.selected_nodes.iter().cloned().collect();
        for node_id in selected_nodes {
            self.set_selection_visual_feedback(&node_id, false);
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
                self.set_selection_visual_feedback(node_id, true);
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
    pub fn navigate_selection<N, E>(
        &mut self,
        graph: &Graph<N, E>,
        direction: NavigationDirection,
    ) -> Option<NodeId>
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
                        self.set_selection_visual_feedback(&next_node_id, true);
                        Some(next_node_id)
                    } else {
                        None
                    }
                } else {
                    // No selection, select first node
                    let first_node_id = nodes[0].id.clone();
                    self.selected_nodes.clear();
                    self.selected_nodes.insert(first_node_id.clone());
                    self.set_selection_visual_feedback(&first_node_id, true);
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
                        self.set_selection_visual_feedback(&prev_node_id, true);
                        Some(prev_node_id)
                    } else {
                        None
                    }
                } else {
                    // No selection, select last node
                    let last_node_id = nodes[nodes.len() - 1].id.clone();
                    self.selected_nodes.clear();
                    self.selected_nodes.insert(last_node_id.clone());
                    self.set_selection_visual_feedback(&last_node_id, true);
                    Some(last_node_id)
                }
            }
        }
    }

    // TODO: Implement GroupManager functionality
    // /// Get all selected groups
    // pub fn selected_groups(&self, group_manager: &GroupManager) -> Vec<GroupId> {
    //     let mut selected_groups = Vec::new();
    //     for group_id in group_manager.group_ids() {
    //         if self.is_group_fully_selected(group_manager, &group_id) {
    //             selected_groups.push(group_id);
    //         }
    //     }
    //     selected_groups
    // }

    // /// Check if all nodes in a group are selected
    // pub fn is_group_fully_selected(&self, group_manager: &GroupManager, group_id: &GroupId) -> bool {
    //     if let Some(group) = group_manager.get_group(group_id) {
    //         for node_id in &group.members {
    //             if !self.selected_nodes.contains(node_id) {
    //                 return false;
    //             }
    //         }
    //         !group.members.is_empty()
    //     } else {
    //         false
    //     }
    // }

    // /// Deselect all nodes in a group
    // pub fn deselect_group(&mut self, group_manager: &GroupManager, group_id: &GroupId) {
    //     if let Some(group) = group_manager.get_group(group_id) {
    //         for node_id in &group.members {
    //             self.selected_nodes.remove(node_id);
    //             self.set_selection_visual_feedback(node_id, false);
    //         }
    //     }
    // }

    /// Handle keyboard shortcuts for selection management (read-only operations)
    pub fn handle_keyboard_shortcut<N, E>(
        &mut self,
        graph: &Graph<N, E>,
        shortcut: KeyboardShortcut,
    ) where
        N: Clone,
        E: Clone,
    {
        match shortcut {
            KeyboardShortcut::SelectAll => {
                self.selected_nodes.clear();
                for node in graph.nodes() {
                    self.selected_nodes.insert(node.id.clone());
                    self.set_selection_visual_feedback(&node.id, true);
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
    pub fn handle_destructive_keyboard_shortcut<N, E>(
        &mut self,
        graph: &mut Graph<N, E>,
        shortcut: KeyboardShortcut,
    ) where
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

    /// Set visual feedback for a node
    pub fn set_visual_feedback(&mut self, node_id: &NodeId, feedback: VisualFeedback) {
        if feedback.has_any_state() {
            self.visual_feedback.insert(node_id.clone(), feedback);
        } else {
            self.visual_feedback.remove(node_id);
        }
    }

    /// Get visual feedback for a node
    pub fn get_visual_feedback(&self, node_id: &NodeId) -> Option<&VisualFeedback> {
        self.visual_feedback.get(node_id)
    }

    /// Check if a node has visual feedback
    pub fn has_visual_feedback(&self, node_id: &NodeId) -> bool {
        self.visual_feedback.contains_key(node_id)
    }

    /// Set hover state for a node
    pub fn set_hover_state(&mut self, node_id: &NodeId, hovered: bool) {
        let mut feedback = self.visual_feedback.get(node_id).cloned().unwrap_or_default();
        feedback.set_hovered(hovered);
        self.set_visual_feedback(node_id, feedback);
    }

    /// Set highlight state for a node
    pub fn set_highlight_state(&mut self, node_id: &NodeId, highlighted: bool) {
        let mut feedback = self.visual_feedback.get(node_id).cloned().unwrap_or_default();
        feedback.set_highlighted(highlighted);
        self.set_visual_feedback(node_id, feedback);
    }

    /// Set animation progress for a node
    pub fn set_animation_progress(&mut self, node_id: &NodeId, progress: f64) {
        let mut feedback = self.visual_feedback.get(node_id).cloned().unwrap_or_default();
        feedback.set_animation_progress(progress);
        self.set_visual_feedback(node_id, feedback);
    }

    /// Get all nodes with visual feedback
    pub fn nodes_with_visual_feedback(&self) -> Vec<NodeId> {
        self.visual_feedback.keys().cloned().collect()
    }

    /// Clear all visual feedback
    pub fn clear_all_visual_feedbacks(&mut self) {
        self.visual_feedback.clear();
    }

    /// Clear visual feedback for a specific node
    pub fn clear_visual_feedback(&mut self, node_id: &NodeId) {
        self.visual_feedback.remove(node_id);
    }

    /// Helper method to set selection visual feedback
    fn set_selection_visual_feedback(&mut self, node_id: &NodeId, selected: bool) {
        let mut feedback = self.visual_feedback.get(node_id).cloned().unwrap_or_default();
        feedback.set_selected(selected);
        self.set_visual_feedback(node_id, feedback);
    }

    /// Select all nodes in the graph
    pub fn select_all<N, E>(&mut self, graph: &Graph<N, E>) {
        self.selected_nodes.clear();
        for node in graph.nodes() {
            self.selected_nodes.insert(node.id.clone());
        }
    }

    /// Select a group (replacing current selection)
    pub fn select_group(&mut self, group_manager: &crate::groups::GroupManager, group_id: &crate::types::GroupId) {
        self.selected_nodes.clear();
        if let Some(group) = group_manager.get_group(group_id) {
            for node_id in &group.members {
                self.selected_nodes.insert(node_id.clone());
            }
        }
    }

    /// Select a node, optionally selecting its entire group
    pub fn select_node_with_group(
        &mut self,
        group_manager: &crate::groups::GroupManager,
        node_id: crate::types::NodeId,
        select_whole_group: bool,
    ) {
        if select_whole_group {
            // Find the group containing this node
            if let Some(group_id) = group_manager.get_node_group(&node_id) {
                self.select_group(group_manager, group_id);
                return;
            }
        }
        
        // Select just the node
        self.select_node(node_id);
    }

    /// Get all groups that have at least one selected node
    pub fn get_selected_groups(&self, group_manager: &crate::groups::GroupManager) -> std::collections::HashSet<crate::types::GroupId> {
        let mut selected_groups = std::collections::HashSet::new();
        
        for node_id in &self.selected_nodes {
            if let Some(group_id) = group_manager.get_node_group(node_id) {
                selected_groups.insert(group_id.clone());
            }
        }
        
        selected_groups
    }
}
