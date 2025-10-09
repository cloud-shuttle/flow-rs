//! WebRTC-based Peer-to-Peer Synchronization
//!
//! Handles real-time synchronization of operations and state between
//! participants in a collaborative session using WebRTC data channels.

use std::collections::HashMap;
use std::collections::VecDeque;
use crate::graph::Node;
use crate::types::Position;
use crate::collaboration::collaborative_session::{CollaborativeSession, Cursor, Participant};
use crate::collaboration::operational_transform::{Operation, OperationMetadata, GraphOperation};

#[derive(Clone, Debug)]
pub enum SyncMessage {
    Operation(Operation),
    CursorUpdate(Cursor),
    JoinRequest(Participant),
    LeaveNotification(String),
    Heartbeat { participant_id: String, timestamp: u64 },
    StateSyncRequest { participant_id: String, after_sequence: u64 },
    StateSyncResponse { operations: Vec<Operation>, cursors: HashMap<String, Cursor> },
    Error { code: String, message: String },
}

impl SyncMessage {
    pub fn operation(op: Operation) -> Self {
        SyncMessage::Operation(op)
    }

    pub fn cursor_update(cursor: Cursor) -> Self {
        SyncMessage::CursorUpdate(cursor)
    }

    pub fn join_request(participant: Participant) -> Self {
        SyncMessage::JoinRequest(participant)
    }

    pub fn leave_notification(participant_id: String) -> Self {
        SyncMessage::LeaveNotification(participant_id)
    }

    pub fn heartbeat(participant_id: String, timestamp: u64) -> Self {
        SyncMessage::Heartbeat { participant_id, timestamp }
    }

    pub fn state_sync_request(participant_id: String, after_sequence: u64) -> Self {
        SyncMessage::StateSyncRequest { participant_id, after_sequence }
    }

    pub fn state_sync_response(operations: Vec<Operation>, cursors: HashMap<String, Cursor>) -> Self {
        SyncMessage::StateSyncResponse { operations, cursors }
    }

    pub fn error(code: String, message: String) -> Self {
        SyncMessage::Error { code, message }
    }

    pub fn message_type(&self) -> &'static str {
        match self {
            SyncMessage::Operation(_) => "operation",
            SyncMessage::CursorUpdate(_) => "cursor_update",
            SyncMessage::JoinRequest(_) => "join_request",
            SyncMessage::LeaveNotification(_) => "leave_notification",
            SyncMessage::Heartbeat { .. } => "heartbeat",
            SyncMessage::StateSyncRequest { .. } => "state_sync_request",
            SyncMessage::StateSyncResponse { .. } => "state_sync_response",
            SyncMessage::Error { .. } => "error",
        }
    }
}

/// WebRTC-based peer-to-peer synchronization
pub struct P2PSynchronization {
    session: std::sync::Arc<std::sync::RwLock<CollaborativeSession>>,
    peer_connections: HashMap<String, PeerConnection>,
    message_queue: VecDeque<SyncMessage>,
    heartbeat_interval: u64,
    last_heartbeat: u64,
}

impl P2PSynchronization {
    pub fn new(session: std::sync::Arc<std::sync::RwLock<CollaborativeSession>>) -> Self {
        Self {
            session,
            peer_connections: HashMap::new(),
            message_queue: VecDeque::new(),
            heartbeat_interval: 30000, // 30 seconds
            last_heartbeat: 0,
        }
    }

    /// Send operation to all peers
    pub fn broadcast_operation(&mut self, operation: Operation) -> Result<(), super::collaborative_session::CollaborationError> {
        let message = SyncMessage::operation(operation);

        for connection in self.peer_connections.values_mut() {
            connection.send_message(message.clone())?;
        }

        Ok(())
    }

    /// Send cursor update to all peers
    pub fn broadcast_cursor(&mut self, cursor: Cursor) -> Result<(), super::collaborative_session::CollaborationError> {
        let message = SyncMessage::cursor_update(cursor);

        for connection in self.peer_connections.values_mut() {
            connection.send_message(message.clone())?;
        }

        Ok(())
    }

    /// Send heartbeat to all peers
    pub fn broadcast_heartbeat(&mut self, participant_id: String, timestamp: u64) -> Result<(), super::collaborative_session::CollaborationError> {
        let message = SyncMessage::heartbeat(participant_id, timestamp);
        self.last_heartbeat = timestamp;

        for connection in self.peer_connections.values_mut() {
            connection.send_message(message.clone())?;
        }

        Ok(())
    }

    /// Handle incoming message from peer
    pub fn handle_message(&mut self, from_peer: &str, message: SyncMessage) -> Result<(), super::collaborative_session::CollaborationError> {
        match message {
            SyncMessage::Operation(operation) => {
                let session = self.session.read().unwrap();
                session.apply_operation(operation)?;
            }
            SyncMessage::CursorUpdate(cursor) => {
                let mut session = self.session.write().unwrap();
                let participant_id = cursor.participant_id.clone();
                session.update_cursor(participant_id, cursor);
            }
            SyncMessage::JoinRequest(participant) => {
                let mut session = self.session.write().unwrap();
                session.add_participant(participant);
            }
            SyncMessage::LeaveNotification(participant_id) => {
                let mut session = self.session.write().unwrap();
                session.remove_participant(&participant_id);
            }
            SyncMessage::Heartbeat { participant_id, timestamp } => {
                let mut session = self.session.write().unwrap();
                if let Some(participant) = session.participants.get_mut(&participant_id) {
                    participant.update_activity(timestamp);
                }
            }
            SyncMessage::StateSyncRequest { participant_id, after_sequence } => {
                self.handle_state_sync_request(&participant_id, after_sequence)?;
            }
            SyncMessage::StateSyncResponse { operations, cursors } => {
                self.handle_state_sync_response(operations, cursors)?;
            }
            SyncMessage::Error { code, message } => {
                // Log error and potentially handle recovery
                eprintln!("Received error from peer {}: {} - {}", from_peer, code, message);
            }
        }

        Ok(())
    }

    /// Request state synchronization from a peer
    pub fn request_state_sync(&mut self, from_peer: &str, after_sequence: u64) -> Result<(), super::collaborative_session::CollaborationError> {
        let participant_id = self.session.read().unwrap().session_id.clone();
        let message = SyncMessage::state_sync_request(participant_id, after_sequence);

        if let Some(connection) = self.peer_connections.get_mut(from_peer) {
            connection.send_message(message)?;
        }

        Ok(())
    }

    /// Handle state synchronization request
    fn handle_state_sync_request(&mut self, requesting_peer: &str, after_sequence: u64) -> Result<(), super::collaborative_session::CollaborationError> {
        let session = self.session.read().unwrap();
        let ot = session.operation_history.read().unwrap();

        let operations = ot.get_pending_operations(after_sequence);
        let cursors = session.get_cursors().clone();

        let response = SyncMessage::state_sync_response(operations, cursors);

        if let Some(connection) = self.peer_connections.get_mut(requesting_peer) {
            connection.send_message(response)?;
        }

        Ok(())
    }

    /// Handle state synchronization response
    fn handle_state_sync_response(&mut self, operations: Vec<Operation>, cursors: HashMap<String, Cursor>) -> Result<(), super::collaborative_session::CollaborationError> {
        let mut session = self.session.write().unwrap();

        // Apply operations
        for operation in operations {
            session.apply_operation(operation)?;
        }

        // Update cursors
        for (participant_id, cursor) in cursors {
            session.update_cursor(participant_id, cursor);
        }

        Ok(())
    }

    /// Add a peer connection
    pub fn add_peer(&mut self, peer_id: String, connection: PeerConnection) {
        self.peer_connections.insert(peer_id, connection);
    }

    /// Remove a peer connection
    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peer_connections.remove(peer_id);
    }

    /// Get connected peer count
    pub fn peer_count(&self) -> usize {
        self.peer_connections.len()
    }

    /// Check if connected to a specific peer
    pub fn has_peer(&self, peer_id: &str) -> bool {
        self.peer_connections.contains_key(peer_id)
    }

    /// Process queued messages
    pub fn process_message_queue(&mut self) -> Result<(), super::collaborative_session::CollaborationError> {
        while let Some(message) = self.message_queue.pop_front() {
            // This would need to know which peer sent the message
            // For now, we'll skip this as it requires additional context
            let _ = message;
        }
        Ok(())
    }

    /// Update synchronization state (called periodically)
    pub fn update(&mut self, current_time: u64) -> Result<(), super::collaborative_session::CollaborationError> {
        // Send periodic heartbeats
        if current_time - self.last_heartbeat > self.heartbeat_interval {
            let participant_ids: Vec<String> = {
                let session = self.session.read().unwrap();
                session.get_participants().keys().cloned().collect()
            };

            for participant_id in participant_ids {
                self.broadcast_heartbeat(participant_id, current_time)?;
            }
        }

        // Process any queued messages
        self.process_message_queue()?;

        Ok(())
    }

    /// Get synchronization statistics
    pub fn stats(&self) -> P2PStats {
        P2PStats {
            connected_peers: self.peer_count(),
            queued_messages: self.message_queue.len(),
            last_heartbeat: self.last_heartbeat,
        }
    }
}

/// Synchronization statistics
#[derive(Clone, Debug)]
pub struct P2PStats {
    pub connected_peers: usize,
    pub queued_messages: usize,
    pub last_heartbeat: u64,
}

/// Peer connection abstraction
#[derive(Clone, Debug)]
pub struct PeerConnection {
    peer_id: String,
    // In a real implementation, this would contain WebRTC peer connection details
    // For now, it's a placeholder
    connected: bool,
    last_message_time: u64,
}

impl PeerConnection {
    pub fn new(peer_id: String) -> Self {
        Self {
            peer_id,
            connected: true,
            last_message_time: 0,
        }
    }

    pub fn send_message(&mut self, _message: SyncMessage) -> Result<(), super::collaborative_session::CollaborationError> {
        // In a real implementation, this would send via WebRTC data channel
        self.last_message_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn peer_id(&self) -> &str {
        &self.peer_id
    }

    pub fn last_message_time(&self) -> u64 {
        self.last_message_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};

    #[test]
    fn test_p2p_synchronization_creation() {
        let session = Arc::new(RwLock::new(CollaborativeSession::new("test-session".to_string(), "user-1".to_string())));
        let sync = P2PSynchronization::new(session);

        assert_eq!(sync.peer_count(), 0);
        let stats = sync.stats();
        assert_eq!(stats.connected_peers, 0);
        assert_eq!(stats.queued_messages, 0);
    }

    #[test]
    fn test_peer_connection_management() {
        let session = Arc::new(RwLock::new(CollaborativeSession::new("test-session".to_string(), "user-1".to_string())));
        let mut sync = P2PSynchronization::new(session);

        let connection = PeerConnection::new("peer-1".to_string());
        sync.add_peer("peer-1".to_string(), connection);

        assert_eq!(sync.peer_count(), 1);
        assert!(sync.has_peer("peer-1"));

        sync.remove_peer("peer-1");
        assert_eq!(sync.peer_count(), 0);
        assert!(!sync.has_peer("peer-1"));
    }

    #[test]
    fn test_sync_message_types() {
        let op = Operation {
            operation: GraphOperation::AddNode {
                node: Node::new("test".to_string(), Position::new(0.0, 0.0), "test".to_string()),
                position: Position::new(0.0, 0.0),
            },
            metadata: OperationMetadata {
                id: "test".to_string(),
                client_id: "client".to_string(),
                timestamp: 0,
                sequence_number: 0,
                parent_operations: vec![],
            },
        };

        let operation_msg = SyncMessage::operation(op.clone());
        assert_eq!(operation_msg.message_type(), "operation");

        let heartbeat_msg = SyncMessage::heartbeat("user-1".to_string(), 1234567890);
        assert_eq!(heartbeat_msg.message_type(), "heartbeat");

        let error_msg = SyncMessage::error("TEST_ERROR".to_string(), "Test error message".to_string());
        assert_eq!(error_msg.message_type(), "error");
    }

    #[test]
    fn test_peer_connection() {
        let mut connection = PeerConnection::new("peer-1".to_string());
        assert!(connection.is_connected());
        assert_eq!(connection.peer_id(), "peer-1");

        let message = SyncMessage::heartbeat("user-1".to_string(), 1234567890);
        assert!(connection.send_message(message).is_ok());

        connection.disconnect();
        assert!(!connection.is_connected());
    }
}
