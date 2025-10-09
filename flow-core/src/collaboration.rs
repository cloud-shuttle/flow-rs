//! Real-time Collaboration for Flow-RS
//!
//! Provides operational transformation, conflict resolution, and real-time synchronization
//! for collaborative graph editing. Supports multiple users editing the same graph simultaneously.
//!
//! Key Features:
//! - Operational Transformation (OT) for conflict-free editing
//! - CRDT-inspired conflict resolution
//! - WebRTC/P2P synchronization
//! - Collaborative cursors and presence
//! - Live editing indicators
//! - Change tracking and synchronization

pub mod operational_transform;
pub mod collaborative_session;
pub mod p2p_synchronization;
pub mod change_tracking;
pub mod conflict_resolution;

// Re-exports for backward compatibility
pub use operational_transform::*;
pub use collaborative_session::*;
pub use p2p_synchronization::*;
pub use change_tracking::*;
pub use conflict_resolution::*;
