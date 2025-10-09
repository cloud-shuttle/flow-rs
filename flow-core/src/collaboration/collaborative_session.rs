//! Collaborative Session Management
//!
//! Manages collaborative editing sessions, participant tracking,
//! cursor synchronization, and graph state coordination.

use crate::graph::Graph;
use crate::types::{NodeId, EdgeId, Position};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::collaboration::operational_transform::{OperationalTransform, Operation, GraphOperation, OTError};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Collaborative session state
#[derive(Clone, Debug)]
pub struct CollaborativeSession {
    pub session_id: String,
    pub participants: HashMap<String, Participant>,
    pub graph_state: Arc<RwLock<Graph<String, String>>>,
    pub operation_history: Arc<RwLock<OperationalTransform>>,
    pub cursors: HashMap<String, Cursor>,
    pub last_sync_timestamp: u64,
}

impl CollaborativeSession {
    pub fn new(session_id: String, owner_id: String) -> Self {
        let graph = Graph::new();
        let ot = OperationalTransform::new(owner_id.clone());

        Self {
            session_id,
            participants: HashMap::new(),
            graph_state: Arc::new(RwLock::new(graph)),
            operation_history: Arc::new(RwLock::new(ot)),
            cursors: HashMap::new(),
            last_sync_timestamp: 0,
        }
    }

    /// Add a participant to the session
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.insert(participant.id.clone(), participant);
    }

    /// Remove a participant from the session
    pub fn remove_participant(&mut self, participant_id: &str) {
        self.participants.remove(participant_id);
        self.cursors.remove(participant_id);
    }

    /// Update cursor position for a participant
    pub fn update_cursor(&mut self, participant_id: String, cursor: Cursor) {
        self.cursors.insert(participant_id, cursor);
    }

    /// Apply an operation to the collaborative graph
    pub fn apply_operation(&self, operation: Operation) -> Result<(), CollaborationError> {
        let mut ot = self.operation_history.write().unwrap();
        let mut graph = self.graph_state.write().unwrap();

        // Apply operation to graph
        match &operation.operation {
            GraphOperation::AddNode { node, position } => {
                let mut positioned_node = node.clone();
                positioned_node.position = *position;
                graph.add_node(positioned_node).map_err(|e| CollaborationError::GraphError(e.to_string()))?;
            }
            GraphOperation::RemoveNode { node_id } => {
                graph.remove_node(node_id);
            }
            GraphOperation::MoveNode { node_id, to, .. } => {
                if let Some(node) = graph.nodes_mut().find(|n| n.id == *node_id) {
                    node.position = *to;
                }
            }
            GraphOperation::UpdateNode { node_id, new_data, .. } => {
                if let Some(node) = graph.nodes_mut().find(|n| n.id == *node_id) {
                    // Note: This assumes String data, would need generalization
                    // For now, we'll leave this as a placeholder
                    let _ = (node_id, new_data);
                }
            }
            GraphOperation::AddEdge { edge, source, target } => {
                let mut edge_with_nodes = edge.clone();
                edge_with_nodes.source = source.clone();
                edge_with_nodes.target = target.clone();
                graph.add_edge(edge_with_nodes).map_err(|e| CollaborationError::GraphError(e.to_string()))?;
            }
            GraphOperation::RemoveEdge { edge_id } => {
                graph.remove_edge(edge_id);
            }
            GraphOperation::UpdateEdge { edge_id, new_data, .. } => {
                if let Some(edge) = graph.edges_mut().find(|e| e.id == *edge_id) {
                    // Note: This assumes String data, would need generalization
                    // For now, we'll leave this as a placeholder
                    let _ = (edge_id, new_data, edge);
                }
            }
        }

        // Apply to operational transform
        ot.apply_operation(operation)?;
        Ok(())
    }

    /// Get current graph snapshot
    pub fn get_graph_snapshot(&self) -> Graph<String, String> {
        self.graph_state.read().unwrap().clone()
    }

    /// Get current participants
    pub fn get_participants(&self) -> &HashMap<String, Participant> {
        &self.participants
    }

    /// Get current cursors
    pub fn get_cursors(&self) -> &HashMap<String, Cursor> {
        &self.cursors
    }

    /// Get session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get last sync timestamp
    pub fn last_sync_timestamp(&self) -> u64 {
        self.last_sync_timestamp
    }

    /// Update last sync timestamp
    pub fn update_sync_timestamp(&mut self, timestamp: u64) {
        self.last_sync_timestamp = timestamp;
    }

    /// Check if participant is owner
    pub fn is_owner(&self, participant_id: &str) -> bool {
        self.participants.get(participant_id)
            .map(|p| p.permissions.is_owner)
            .unwrap_or(false)
    }

    /// Get participant count
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// Check if session is empty (no participants)
    pub fn is_empty(&self) -> bool {
        self.participants.is_empty()
    }
}

/// Participant in a collaborative session
#[derive(Clone, Debug)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub color: String,
    pub avatar_url: Option<String>,
    pub permissions: ParticipantPermissions,
    pub last_activity: u64,
}

impl Participant {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            color: "#007acc".to_string(), // Default blue color
            avatar_url: None,
            permissions: ParticipantPermissions::default(),
            last_activity: 0,
        }
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = color;
        self
    }

    pub fn with_avatar(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn with_permissions(mut self, permissions: ParticipantPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn update_activity(&mut self, timestamp: u64) {
        self.last_activity = timestamp;
    }

    pub fn can_edit(&self) -> bool {
        self.permissions.can_edit
    }

    pub fn can_delete(&self) -> bool {
        self.permissions.can_delete
    }

    pub fn is_owner(&self) -> bool {
        self.permissions.is_owner
    }
}

/// Participant permissions
#[derive(Clone, Debug)]
pub struct ParticipantPermissions {
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_invite: bool,
    pub is_owner: bool,
}

impl Default for ParticipantPermissions {
    fn default() -> Self {
        Self {
            can_edit: true,
            can_delete: true,
            can_invite: false,
            is_owner: false,
        }
    }
}

impl ParticipantPermissions {
    pub fn owner() -> Self {
        Self {
            can_edit: true,
            can_delete: true,
            can_invite: true,
            is_owner: true,
        }
    }

    pub fn viewer() -> Self {
        Self {
            can_edit: false,
            can_delete: false,
            can_invite: false,
            is_owner: false,
        }
    }

    pub fn editor() -> Self {
        Self {
            can_edit: true,
            can_delete: false,
            can_invite: false,
            is_owner: false,
        }
    }
}

/// Cursor position and state
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cursor {
    pub participant_id: String,
    pub position: Position,
    pub selection: Option<Selection>,
    pub timestamp: u64,
}

impl Cursor {
    pub fn new(participant_id: String, position: Position) -> Self {
        Self {
            participant_id,
            position,
            selection: None,
            timestamp: 0,
        }
    }

    pub fn with_selection(mut self, selection: Selection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn update_position(&mut self, position: Position, timestamp: u64) {
        self.position = position;
        self.timestamp = timestamp;
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }
}

/// Selection state for collaborative cursors
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Selection {
    pub nodes: Vec<NodeId>,
    pub edges: Vec<EdgeId>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: Vec<NodeId>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_edges(mut self, edges: Vec<EdgeId>) -> Self {
        self.edges = edges;
        self
    }

    pub fn add_node(&mut self, node_id: NodeId) {
        if !self.nodes.contains(&node_id) {
            self.nodes.push(node_id);
        }
    }

    pub fn add_edge(&mut self, edge_id: EdgeId) {
        if !self.edges.contains(&edge_id) {
            self.edges.push(edge_id);
        }
    }

    pub fn remove_node(&mut self, node_id: &NodeId) {
        self.nodes.retain(|id| id != node_id);
    }

    pub fn remove_edge(&mut self, edge_id: &EdgeId) {
        self.edges.retain(|id| id != edge_id);
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// Collaboration errors
#[derive(Clone, Debug)]
pub enum CollaborationError {
    GraphError(String),
    OTError(OTError),
    NetworkError(String),
    PermissionDenied(String),
    SessionNotFound(String),
    ParticipantNotFound(String),
    InvalidOperation(String),
}

impl std::fmt::Display for CollaborationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollaborationError::GraphError(msg) => write!(f, "Graph error: {}", msg),
            CollaborationError::OTError(err) => write!(f, "OT error: {}", err),
            CollaborationError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            CollaborationError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            CollaborationError::SessionNotFound(msg) => write!(f, "Session not found: {}", msg),
            CollaborationError::ParticipantNotFound(msg) => write!(f, "Participant not found: {}", msg),
            CollaborationError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for CollaborationError {}

impl From<OTError> for CollaborationError {
    fn from(error: OTError) -> Self {
        CollaborationError::OTError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collaborative_session_creation() {
        let session = CollaborativeSession::new("session-1".to_string(), "user-1".to_string());
        assert_eq!(session.session_id(), "session-1");
        assert!(session.get_participants().is_empty());
        assert!(session.is_empty());
    }

    #[test]
    fn test_participant_management() {
        let mut session = CollaborativeSession::new("session-1".to_string(), "user-1".to_string());

        let participant = Participant::new("user-2".to_string(), "User 2".to_string());
        session.add_participant(participant.clone());

        assert_eq!(session.participant_count(), 1);
        assert!(session.get_participants().contains_key("user-2"));

        session.remove_participant("user-2");
        assert!(session.is_empty());
    }

    #[test]
    fn test_cursor_tracking() {
        let mut session = CollaborativeSession::new("session-1".to_string(), "user-1".to_string());

        let cursor = Cursor::new("user-1".to_string(), Position::new(150.0, 200.0));
        session.update_cursor("user-1".to_string(), cursor.clone());

        assert_eq!(session.get_cursors().len(), 1);
        assert!(session.get_cursors().contains_key("user-1"));
    }

    #[test]
    fn test_participant_permissions() {
        let owner = ParticipantPermissions::owner();
        assert!(owner.can_edit && owner.can_delete && owner.can_invite && owner.is_owner);

        let viewer = ParticipantPermissions::viewer();
        assert!(!viewer.can_edit && !viewer.can_delete && !viewer.can_invite && !viewer.is_owner);

        let editor = ParticipantPermissions::editor();
        assert!(editor.can_edit && !editor.can_delete && !editor.can_invite && !editor.is_owner);
    }

    #[test]
    fn test_selection_management() {
        let mut selection = Selection::new();
        assert!(selection.is_empty());

        selection.add_node("node-1".into());
        selection.add_node("node-2".into());
        selection.add_edge("edge-1".into());

        assert_eq!(selection.node_count(), 2);
        assert_eq!(selection.edge_count(), 1);
        assert!(!selection.is_empty());

        selection.remove_node(&"node-1".into());
        assert_eq!(selection.node_count(), 1);

        selection.clear();
        assert!(selection.is_empty());
    }
}
