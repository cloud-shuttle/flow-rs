//! Edge connection system for Leptos Flow
//!
//! Provides comprehensive edge connection functionality including connection validation,
//! handle detection, edge creation, and connection management.

use flow_core::{Edge, EdgeId, Graph, Node, NodeId, Position};

/// Result of a connection validation operation
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionResult {
    Valid,
    InvalidSelfConnection,
    DuplicateEdge,
    CircularDependency,
    MaxConnectionsExceeded,
    InvalidNode,
}

/// Connection handle types for nodes
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionHandle {
    Input,
    Output,
}

/// Connection constraints for validation
#[derive(Debug, Clone)]
pub struct ConnectionConstraints {
    pub max_connections_per_node: usize,
    pub allow_self_connections: bool,
    pub allow_circular_dependencies: bool,
}

impl Default for ConnectionConstraints {
    fn default() -> Self {
        Self {
            max_connections_per_node: 10,
            allow_self_connections: false,
            allow_circular_dependencies: false,
        }
    }
}

/// Visual feedback for connection operations
#[derive(Debug, Clone)]
pub struct ConnectionFeedback {
    pub is_valid: bool,
    pub can_connect: bool,
    pub message: Option<String>,
}

/// Connection validator for validating edge connections
///
/// Provides comprehensive validation for edge connections including:
/// - Self-connection prevention
/// - Duplicate edge detection
/// - Circular dependency detection
/// - Connection limit enforcement
pub struct ConnectionValidator {
    constraints: ConnectionConstraints,
}

impl ConnectionValidator {
    pub fn new() -> Self {
        Self {
            constraints: ConnectionConstraints::default(),
        }
    }

    pub fn with_constraints(constraints: ConnectionConstraints) -> Self {
        Self { constraints }
    }

    /// Validate a connection between two nodes
    pub fn validate_connection<N, E>(
        &self,
        graph: &Graph<N, E>,
        source: &NodeId,
        target: &NodeId,
    ) -> ConnectionResult
    where
        N: Clone,
        E: Clone,
    {
        // Check if nodes exist
        if graph.get_node(source).is_none() || graph.get_node(target).is_none() {
            return ConnectionResult::InvalidNode;
        }

        // Check for self-connection
        if source == target && !self.constraints.allow_self_connections {
            return ConnectionResult::InvalidSelfConnection;
        }

        // Check for duplicate edge
        for edge in graph.edges() {
            if edge.source == *source && edge.target == *target {
                return ConnectionResult::DuplicateEdge;
            }
        }

        // Check for circular dependency
        if !self.constraints.allow_circular_dependencies {
            if self.would_create_circular_dependency(graph, source, target) {
                return ConnectionResult::CircularDependency;
            }
        }

        // Check max connections per node
        let source_connections = graph.edges().filter(|e| e.source == *source).count();
        if source_connections >= self.constraints.max_connections_per_node {
            return ConnectionResult::MaxConnectionsExceeded;
        }

        ConnectionResult::Valid
    }

    /// Check if adding an edge would create a circular dependency
    fn would_create_circular_dependency<N, E>(
        &self,
        graph: &Graph<N, E>,
        source: &NodeId,
        target: &NodeId,
    ) -> bool
    where
        N: Clone,
        E: Clone,
    {
        // Simple DFS to detect if target can reach source
        self.can_reach(graph, target, source)
    }

    /// Check if node A can reach node B through existing edges
    fn can_reach<N, E>(&self, graph: &Graph<N, E>, from: &NodeId, to: &NodeId) -> bool
    where
        N: Clone,
        E: Clone,
    {
        if from == to {
            return true;
        }

        for edge in graph.edges() {
            if edge.source == *from {
                if self.can_reach(graph, &edge.target, to) {
                    return true;
                }
            }
        }

        false
    }
}

/// Handle detector for finding connection points on nodes
///
/// Detects input and output handles on nodes based on mouse position.
/// Handles are positioned at the left (input) and right (output) edges of nodes.
pub struct HandleDetector;

impl HandleDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect which handle (if any) is at the given position
    pub fn detect_handle(
        &self,
        node: &Node<impl Clone>,
        position: Position,
    ) -> Option<ConnectionHandle> {
        let handle_size = 8.0; // Size of connection handle area

        // Check input handle (left side)
        let input_pos = self.get_input_handle_position(node);
        if self.position_in_handle(position, input_pos, handle_size) {
            return Some(ConnectionHandle::Input);
        }

        // Check output handle (right side)
        let output_pos = self.get_output_handle_position(node);
        if self.position_in_handle(position, output_pos, handle_size) {
            return Some(ConnectionHandle::Output);
        }

        None
    }

    /// Get the position of the input handle for a node
    pub fn get_input_handle_position(&self, node: &Node<impl Clone>) -> Position {
        Position::new(node.position.x, node.position.y + node.size.height / 2.0)
    }

    /// Get the position of the output handle for a node
    pub fn get_output_handle_position(&self, node: &Node<impl Clone>) -> Position {
        Position::new(
            node.position.x + node.size.width,
            node.position.y + node.size.height / 2.0,
        )
    }

    /// Check if a position is within a handle area
    fn position_in_handle(&self, pos: Position, handle_pos: Position, handle_size: f64) -> bool {
        let dx = pos.x - handle_pos.x;
        let dy = pos.y - handle_pos.y;
        dx * dx + dy * dy <= (handle_size / 2.0) * (handle_size / 2.0)
    }
}

/// Connection preview for showing temporary connections during drag
pub struct ConnectionPreview {
    active: bool,
    source_node: Option<NodeId>,
    start_position: Option<Position>,
    current_position: Position,
}

impl ConnectionPreview {
    pub fn new() -> Self {
        Self {
            active: false,
            source_node: None,
            start_position: None,
            current_position: Position::new(0.0, 0.0),
        }
    }

    /// Start a connection preview
    pub fn start_connection<N, E>(
        &mut self,
        _graph: &Graph<N, E>,
        source: &NodeId,
        start_pos: Position,
    ) where
        N: Clone,
        E: Clone,
    {
        self.active = true;
        self.source_node = Some(source.clone());
        self.start_position = Some(start_pos);
        self.current_position = start_pos;
    }

    /// Update the preview position
    pub fn update_preview(&mut self, position: Position) {
        self.current_position = position;
    }

    /// End the preview
    pub fn end_preview(&mut self) {
        self.active = false;
        self.source_node = None;
        self.start_position = None;
    }

    /// Check if preview is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the source node
    pub fn source_node(&self) -> Option<&NodeId> {
        self.source_node.as_ref()
    }

    /// Get the current position
    pub fn current_position(&self) -> Position {
        self.current_position
    }
}

/// Edge creator for creating and managing edges
pub struct EdgeCreator;

impl EdgeCreator {
    pub fn new() -> Self {
        Self
    }

    /// Create an edge between two nodes
    pub fn create_edge<N, E>(
        &self,
        graph: &mut Graph<N, E>,
        source: &NodeId,
        target: &NodeId,
    ) -> ConnectionResult
    where
        N: Clone,
        E: Clone + Default,
    {
        // Validate connection first
        let validator = ConnectionValidator::new();
        let validation_result = validator.validate_connection(graph, source, target);

        if validation_result != ConnectionResult::Valid {
            return validation_result;
        }

        // Create the edge
        let edge_id = EdgeId::new(&format!("edge_{}_{}", source, target));
        let edge = Edge::new(edge_id, source.clone(), target.clone(), Default::default());

        match graph.add_edge(edge) {
            Ok(_) => ConnectionResult::Valid,
            Err(_) => ConnectionResult::DuplicateEdge,
        }
    }
}

/// Connection visualizer for providing visual feedback
pub struct ConnectionVisualizer;

impl ConnectionVisualizer {
    pub fn new() -> Self {
        Self
    }

    /// Get visual feedback for a potential connection
    pub fn get_connection_feedback<N, E>(
        &self,
        graph: &Graph<N, E>,
        source: &NodeId,
        target: &NodeId,
    ) -> ConnectionFeedback
    where
        N: Clone,
        E: Clone,
    {
        let validator = ConnectionValidator::new();
        let result = validator.validate_connection(graph, source, target);

        match result {
            ConnectionResult::Valid => ConnectionFeedback {
                is_valid: true,
                can_connect: true,
                message: None,
            },
            ConnectionResult::InvalidSelfConnection => ConnectionFeedback {
                is_valid: false,
                can_connect: false,
                message: Some("Cannot connect node to itself".to_string()),
            },
            ConnectionResult::DuplicateEdge => ConnectionFeedback {
                is_valid: false,
                can_connect: false,
                message: Some("Edge already exists".to_string()),
            },
            ConnectionResult::CircularDependency => ConnectionFeedback {
                is_valid: false,
                can_connect: false,
                message: Some("Would create circular dependency".to_string()),
            },
            ConnectionResult::MaxConnectionsExceeded => ConnectionFeedback {
                is_valid: false,
                can_connect: false,
                message: Some("Maximum connections exceeded".to_string()),
            },
            ConnectionResult::InvalidNode => ConnectionFeedback {
                is_valid: false,
                can_connect: false,
                message: Some("Invalid node".to_string()),
            },
        }
    }
}

/// Connection history for undo/redo functionality
pub struct ConnectionHistory {
    operations: Vec<ConnectionOperation>,
    current_index: usize,
}

#[derive(Debug, Clone)]
enum ConnectionOperation {
    CreateEdge { source: NodeId, target: NodeId },
    DeleteEdge { edge_id: EdgeId },
}

impl ConnectionHistory {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            current_index: 0,
        }
    }

    /// Record an edge creation operation
    pub fn record_edge_creation(&mut self, source: &NodeId, target: &NodeId) {
        let operation = ConnectionOperation::CreateEdge {
            source: source.clone(),
            target: target.clone(),
        };

        // Remove any operations after current index
        self.operations.truncate(self.current_index);
        self.operations.push(operation);
        self.current_index = self.operations.len();
    }

    /// Undo the last operation
    pub fn undo<N, E>(&mut self, graph: &mut Graph<N, E>)
    where
        N: Clone,
        E: Clone,
    {
        if self.current_index > 0 {
            self.current_index -= 1;
            let operation = &self.operations[self.current_index];

            match operation {
                ConnectionOperation::CreateEdge { source, target } => {
                    // Find and remove the edge
                    let edge_to_remove = graph
                        .edges()
                        .find(|e| e.source == *source && e.target == *target)
                        .map(|e| e.id.clone());

                    if let Some(edge_id) = edge_to_remove {
                        let _ = graph.remove_edge(&edge_id);
                    }
                }
                ConnectionOperation::DeleteEdge { edge_id: _ } => {
                    // Recreate the edge (this would need more context in a real implementation)
                    // For now, we'll just skip this
                }
            }
        }
    }

    /// Redo the next operation
    pub fn redo<N, E>(&mut self, graph: &mut Graph<N, E>)
    where
        N: Clone,
        E: Clone + Default,
    {
        if self.current_index < self.operations.len() {
            let operation = &self.operations[self.current_index];

            match operation {
                ConnectionOperation::CreateEdge { source, target } => {
                    let edge_id = EdgeId::new(&format!("edge_{}_{}", source, target));
                    let edge =
                        Edge::new(edge_id, source.clone(), target.clone(), Default::default());
                    let _ = graph.add_edge(edge);
                }
                ConnectionOperation::DeleteEdge { edge_id } => {
                    let _ = graph.remove_edge(edge_id);
                }
            }

            self.current_index += 1;
        }
    }
}

impl Default for ConnectionValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HandleDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConnectionPreview {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EdgeCreator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConnectionVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConnectionHistory {
    fn default() -> Self {
        Self::new()
    }
}
