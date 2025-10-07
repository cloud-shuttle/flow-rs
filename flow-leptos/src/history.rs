//! Undo/Redo history management for Flow-RS Leptos components
//!
//! Provides comprehensive operation history with:
//! - Undo/Redo stack management
//! - Operation batching for complex actions
//! - Memory-efficient storage
//! - State serialization/deserialization
//! - Integration with keyboard shortcuts

use leptos::prelude::*;
use std::collections::VecDeque;
use flow_rs_core::{Graph, NodeId, Position, Node, Edge};
use serde::{Serialize, Deserialize};

/// History operation types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HistoryOperation {
    /// Add node
    AddNode {
        node_id: NodeId,
        node_data: String,
        position: Position,
    },
    /// Delete node
    DeleteNode {
        node_id: NodeId,
        node_data: String,
        position: Position,
    },
    /// Move node
    MoveNode {
        node_id: NodeId,
        old_position: Position,
        new_position: Position,
    },
    /// Add edge
    AddEdge {
        edge_id: String,
        source: NodeId,
        target: NodeId,
        edge_data: String,
    },
    /// Delete edge
    DeleteEdge {
        edge_id: String,
        source: NodeId,
        target: NodeId,
        edge_data: String,
    },
    /// Batch operation (multiple operations as one undo step)
    Batch(Vec<HistoryOperation>),
}

/// History entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub operation: HistoryOperation,
    pub timestamp: u64,
    pub description: String,
}

/// Undo/Redo manager
pub struct HistoryManager {
    undo_stack: VecDeque<HistoryEntry>,
    redo_stack: VecDeque<HistoryEntry>,
    max_history_size: usize,
    batch_operations: Vec<HistoryOperation>,
    in_batch: bool,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history_size: 50, // Keep last 50 operations
            batch_operations: Vec::new(),
            in_batch: false,
        }
    }

    /// Record an operation
    pub fn record_operation(&mut self, operation: HistoryOperation, description: String) {
        let entry = HistoryEntry {
            operation: operation.clone(),
            timestamp: self.get_timestamp(),
            description,
        };

        if self.in_batch {
            self.batch_operations.push(operation);
        } else {
            self.undo_stack.push_back(entry);
            self.redo_stack.clear(); // Clear redo stack when new operation is recorded

            // Limit history size
            while self.undo_stack.len() > self.max_history_size {
                self.undo_stack.pop_front();
            }
        }
    }

    /// Start batch operation
    pub fn start_batch(&mut self) {
        self.in_batch = true;
        self.batch_operations.clear();
    }

    /// End batch operation
    pub fn end_batch(&mut self, description: String) {
        if self.in_batch && !self.batch_operations.is_empty() {
            let batch_op = HistoryOperation::Batch(self.batch_operations.clone());
            self.record_operation(batch_op, description);
        }
        self.in_batch = false;
        self.batch_operations.clear();
    }

    /// Undo last operation
    pub fn undo(&mut self) -> Option<&HistoryEntry> {
        if let Some(entry) = self.undo_stack.pop_back() {
            self.redo_stack.push_back(entry);
            self.redo_stack.back()
        } else {
            None
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) -> Option<&HistoryEntry> {
        if let Some(entry) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(entry);
            self.undo_stack.back()
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get undo stack size
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get redo stack size
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.batch_operations.clear();
        self.in_batch = false;
    }

    /// Apply operation to graph
    pub fn apply_operation<N: Clone, E: Clone>(
        &self,
        operation: &HistoryOperation,
        graph: &mut Graph<N, E>,
        node_factory: impl Fn(&str, Position) -> N,
        edge_factory: impl Fn(&str, NodeId, NodeId) -> E,
    ) -> Result<(), String> {
        match operation {
            HistoryOperation::AddNode { node_id, node_data, position } => {
                let node = Node::new(node_id.clone(), position.clone(), node_factory(node_data, position.clone()));
                graph.add_node(node).map_err(|e| format!("Failed to add node: {:?}", e))?;
            }
            HistoryOperation::DeleteNode { node_id, .. } => {
                graph.remove_node(node_id).map_err(|e| format!("Failed to remove node: {:?}", e))?;
            }
            HistoryOperation::MoveNode { node_id, new_position, .. } => {
                if let Some(node) = graph.nodes_mut().find(|n| n.id == *node_id) {
                    node.position = new_position.clone();
                } else {
                    return Err(format!("Node {} not found for move operation", node_id));
                }
            }
            HistoryOperation::AddEdge { edge_id, source, target, edge_data } => {
                let edge = flow_rs_core::Edge::new(
                    edge_id.clone(),
                    source.clone(),
                    target.clone(),
                    edge_factory(edge_data, source.clone(), target.clone()),
                );
                graph.add_edge(edge).map_err(|e| format!("Failed to add edge: {:?}", e))?;
            }
            HistoryOperation::DeleteEdge { edge_id, .. } => {
                // Find and remove edge by ID (this would need to be implemented in Graph)
                // For now, this is a placeholder
                return Err("Delete edge operation not yet implemented".to_string());
            }
            HistoryOperation::Batch(operations) => {
                for op in operations {
                    self.apply_operation(op, graph, &node_factory, &edge_factory)?;
                }
            }
        }
        Ok(())
    }

    /// Get inverse operation (for undo)
    pub fn get_inverse(&self, operation: &HistoryOperation) -> HistoryOperation {
        match operation {
            HistoryOperation::AddNode { node_id, node_data, position } => {
                HistoryOperation::DeleteNode {
                    node_id: node_id.clone(),
                    node_data: node_data.clone(),
                    position: position.clone(),
                }
            }
            HistoryOperation::DeleteNode { node_id, node_data, position } => {
                HistoryOperation::AddNode {
                    node_id: node_id.clone(),
                    node_data: node_data.clone(),
                    position: position.clone(),
                }
            }
            HistoryOperation::MoveNode { node_id, old_position, new_position } => {
                HistoryOperation::MoveNode {
                    node_id: node_id.clone(),
                    old_position: new_position.clone(),
                    new_position: old_position.clone(),
                }
            }
            HistoryOperation::AddEdge { edge_id, source, target, edge_data } => {
                HistoryOperation::DeleteEdge {
                    edge_id: edge_id.clone(),
                    source: source.clone(),
                    target: target.clone(),
                    edge_data: edge_data.clone(),
                }
            }
            HistoryOperation::DeleteEdge { edge_id, source, target, edge_data } => {
                HistoryOperation::AddEdge {
                    edge_id: edge_id.clone(),
                    source: source.clone(),
                    target: target.clone(),
                    edge_data: edge_data.clone(),
                }
            }
            HistoryOperation::Batch(operations) => {
                HistoryOperation::Batch(
                    operations.iter().rev().map(|op| self.get_inverse(op)).collect()
                )
            }
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        // In WASM, we can use performance.now() or similar
        // For now, return a placeholder
        0
    }
}

/// Clipboard data for copy/paste operations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipboardData {
    pub nodes: Vec<ClipboardNode>,
    pub edges: Vec<ClipboardEdge>,
    pub offset: Position,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipboardNode {
    pub id: NodeId,
    pub data: String,
    pub position: Position,
    pub relative_position: Position, // Position relative to selection center
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipboardEdge {
    pub source: NodeId,
    pub target: NodeId,
    pub data: String,
}

/// Clipboard manager
pub struct ClipboardManager {
    data: Option<ClipboardData>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self { data: None }
    }

    /// Copy nodes to clipboard
    pub fn copy_nodes<N: Clone, E: Clone>(
        &mut self,
        node_ids: &[NodeId],
        graph: &Graph<N, E>,
    ) where
        N: std::fmt::Display,
        E: std::fmt::Display,
    {
        if node_ids.is_empty() {
            return;
        }

        // Calculate center of selection
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let mut count = 0;

        for node_id in node_ids {
            if let Some(node) = graph.nodes().find(|n| n.id == *node_id) {
                center_x += node.position.x;
                center_y += node.position.y;
                count += 1;
            }
        }

        if count == 0 {
            return;
        }

        center_x /= count as f64;
        center_y /= count as f64;
        let center = Position::new(center_x, center_y);

        let mut clipboard_nodes = Vec::new();
        let mut clipboard_edges = Vec::new();

        // Copy nodes
        for node_id in node_ids {
            if let Some(node) = graph.nodes().find(|n| n.id == *node_id) {
                clipboard_nodes.push(ClipboardNode {
                    id: node.id.clone(),
                    data: format!("{}", node.data),
                    position: node.position.clone(),
                    relative_position: Position::new(
                        node.position.x - center.x,
                        node.position.y - center.y,
                    ),
                });
            }
        }

        // Copy edges between selected nodes
        for edge in graph.edges() {
            if node_ids.contains(&edge.source) && node_ids.contains(&edge.target) {
                clipboard_edges.push(ClipboardEdge {
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    data: format!("{}", edge.data),
                });
            }
        }

        self.data = Some(ClipboardData {
            nodes: clipboard_nodes,
            edges: clipboard_edges,
            offset: center,
        });
    }

    /// Check if clipboard has data
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    /// Paste nodes from clipboard
    pub fn paste_nodes<N: Clone, E: Clone>(
        &self,
        position: Position,
        graph: &mut Graph<N, E>,
        node_factory: impl Fn(&str, Position) -> N,
        edge_factory: impl Fn(&str, NodeId, NodeId) -> E,
        id_generator: impl Fn() -> NodeId,
    ) -> Result<Vec<NodeId>, String> {
        let Some(clipboard_data) = &self.data else {
            return Err("No data in clipboard".to_string());
        };

        let mut pasted_node_ids = Vec::new();
        let mut id_mapping = std::collections::HashMap::new();

        // Paste nodes
        for clipboard_node in &clipboard_data.nodes {
            let new_id = id_generator();
            let new_position = Position::new(
                position.x + clipboard_node.relative_position.x,
                position.y + clipboard_node.relative_position.y,
            );

            let node = Node::new(
                new_id.clone(),
                new_position,
                node_factory(&clipboard_node.data, new_position),
            );

            graph.add_node(node)
                .map_err(|e| format!("Failed to add pasted node: {:?}", e))?;

            id_mapping.insert(clipboard_node.id.clone(), new_id.clone());
            pasted_node_ids.push(new_id);
        }

        // Paste edges
        for clipboard_edge in &clipboard_data.edges {
            if let (Some(new_source), Some(new_target)) = (
                id_mapping.get(&clipboard_edge.source),
                id_mapping.get(&clipboard_edge.target),
            ) {
                let edge = flow_rs_core::Edge::new(
                    format!("edge-{}-{}", new_source, new_target),
                    new_source.clone(),
                    new_target.clone(),
                    edge_factory(&clipboard_edge.data, new_source.clone(), new_target.clone()),
                );

                graph.add_edge(edge)
                    .map_err(|e| format!("Failed to add pasted edge: {:?}", e))?;
            }
        }

        Ok(pasted_node_ids)
    }
}

/// Keyboard shortcuts manager
pub struct KeyboardShortcuts {
    shortcuts: std::collections::HashMap<String, KeyboardShortcut>,
}

#[derive(Clone, Debug)]
pub struct KeyboardShortcut {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub action: String,
    pub description: String,
}

impl KeyboardShortcuts {
    pub fn new() -> Self {
        let mut shortcuts = std::collections::HashMap::new();

        // Standard shortcuts
        shortcuts.insert("undo".to_string(), KeyboardShortcut {
            key: "z".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: "undo".to_string(),
            description: "Undo last action".to_string(),
        });

        shortcuts.insert("redo".to_string(), KeyboardShortcut {
            key: "y".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: "redo".to_string(),
            description: "Redo last undone action".to_string(),
        });

        shortcuts.insert("redo_alt".to_string(), KeyboardShortcut {
            key: "z".to_string(),
            ctrl: true,
            shift: true,
            alt: false,
            action: "redo".to_string(),
            description: "Redo last undone action (alternative)".to_string(),
        });

        shortcuts.insert("copy".to_string(), KeyboardShortcut {
            key: "c".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: "copy".to_string(),
            description: "Copy selected nodes".to_string(),
        });

        shortcuts.insert("cut".to_string(), KeyboardShortcut {
            key: "x".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: "cut".to_string(),
            description: "Cut selected nodes".to_string(),
        });

        shortcuts.insert("paste".to_string(), KeyboardShortcut {
            key: "v".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: "paste".to_string(),
            description: "Paste nodes from clipboard".to_string(),
        });

        shortcuts.insert("select_all".to_string(), KeyboardShortcut {
            key: "a".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: "select_all".to_string(),
            description: "Select all nodes".to_string(),
        });

        shortcuts.insert("delete".to_string(), KeyboardShortcut {
            key: "Delete".to_string(),
            ctrl: false,
            shift: false,
            alt: false,
            action: "delete_selected".to_string(),
            description: "Delete selected nodes".to_string(),
        });

        shortcuts.insert("escape".to_string(), KeyboardShortcut {
            key: "Escape".to_string(),
            ctrl: false,
            shift: false,
            alt: false,
            action: "clear_selection".to_string(),
            description: "Clear selection".to_string(),
        });

        Self { shortcuts }
    }

    /// Get shortcut by action
    pub fn get_shortcut(&self, action: &str) -> Option<&KeyboardShortcut> {
        self.shortcuts.get(action)
    }

    /// Get all shortcuts
    pub fn get_all_shortcuts(&self) -> &std::collections::HashMap<String, KeyboardShortcut> {
        &self.shortcuts
    }

    /// Check if keyboard event matches a shortcut
    pub fn matches_shortcut(&self, key: &str, ctrl: bool, shift: bool, alt: bool) -> Option<&KeyboardShortcut> {
        for shortcut in self.shortcuts.values() {
            if shortcut.key == key &&
               shortcut.ctrl == ctrl &&
               shortcut.shift == shift &&
               shortcut.alt == alt {
                return Some(shortcut);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_manager() {
        let mut manager = HistoryManager::new();
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());

        // Record an operation
        manager.record_operation(
            HistoryOperation::AddNode {
                node_id: NodeId::new("test"),
                node_data: "Test Node".to_string(),
                position: Position::new(100.0, 200.0),
            },
            "Add test node".to_string(),
        );

        assert!(manager.can_undo());
        assert!(!manager.can_redo());
        assert_eq!(manager.undo_count(), 1);

        // Undo operation
        let undone = manager.undo();
        assert!(undone.is_some());
        assert!(!manager.can_undo());
        assert!(manager.can_redo());

        // Redo operation
        let redone = manager.redo();
        assert!(redone.is_some());
        assert!(manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn test_batch_operations() {
        let mut manager = HistoryManager::new();

        manager.start_batch();

        manager.record_operation(
            HistoryOperation::AddNode {
                node_id: NodeId::new("node1"),
                node_data: "Node 1".to_string(),
                position: Position::new(0.0, 0.0),
            },
            "Add node 1".to_string(),
        );

        manager.record_operation(
            HistoryOperation::AddNode {
                node_id: NodeId::new("node2"),
                node_data: "Node 2".to_string(),
                position: Position::new(100.0, 0.0),
            },
            "Add node 2".to_string(),
        );

        manager.end_batch("Add two nodes".to_string());

        assert!(manager.can_undo());
        assert_eq!(manager.undo_count(), 1); // Should be batched as one operation
    }

    #[test]
    fn test_clipboard() {
        let mut clipboard = ClipboardManager::new();
        assert!(!clipboard.has_data());

        let mut graph = Graph::new();
        let node = Node::new(
            NodeId::new("test-node"),
            Position::new(100.0, 200.0),
            "Test Node".to_string(),
        );
        graph.add_node(node).unwrap();

        clipboard.copy_nodes(&[NodeId::new("test-node")], &graph);
        assert!(clipboard.has_data());
    }

    #[test]
    fn test_keyboard_shortcuts() {
        let shortcuts = KeyboardShortcuts::new();

        // Test undo shortcut
        let undo_shortcut = shortcuts.get_shortcut("undo");
        assert!(undo_shortcut.is_some());
        assert_eq!(undo_shortcut.unwrap().key, "z");
        assert!(undo_shortcut.unwrap().ctrl);

        // Test matching shortcut
        let matched = shortcuts.matches_shortcut("z", true, false, false);
        assert!(matched.is_some());
        assert_eq!(matched.unwrap().action, "undo");

        // Test non-matching shortcut
        let non_matched = shortcuts.matches_shortcut("z", false, false, false);
        assert!(non_matched.is_none());
    }

    #[test]
    fn test_operation_inverse() {
        let manager = HistoryManager::new();

        let add_op = HistoryOperation::AddNode {
            node_id: NodeId::new("test"),
            node_data: "Test".to_string(),
            position: Position::new(100.0, 200.0),
        };

        let inverse = manager.get_inverse(&add_op);
        match inverse {
            HistoryOperation::DeleteNode { node_id, .. } => {
                assert_eq!(node_id, NodeId::new("test"));
            }
            _ => panic!("Expected DeleteNode operation"),
        }
    }
}
