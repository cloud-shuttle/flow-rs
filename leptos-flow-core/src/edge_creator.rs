//! Interactive edge creation system
//!
//! Provides functionality for creating edges interactively through drag operations
//! from source handles to target handles or nodes.

use crate::{FlowError, Result, NodeId, Position, Graph, Edge};

/// Helper function to create edge with default data
fn create_edge_with_defaults<E: Default>(
    id: impl Into<crate::EdgeId>,
    source: impl Into<NodeId>,
    target: impl Into<NodeId>,
) -> Edge<E> {
    Edge::new(id, source, target, E::default())
}

/// Preview edge shown during interactive edge creation
#[derive(Debug, Clone)]
pub struct PreviewEdge {
    pub source_node: NodeId,
    pub source_handle: Option<String>,
    pub start_position: Position,
    pub end_position: Position,
}

/// Feedback for connection validation during edge creation
#[derive(Debug, Clone)]
pub struct ConnectionFeedback {
    pub is_valid: bool,
    pub can_connect: bool,
    pub message: Option<String>,
}

/// Interactive edge creator for handling edge creation workflow
#[derive(Debug)]
pub struct EdgeCreator {
    /// Current preview edge being created
    preview_edge: Option<PreviewEdge>,
    /// Whether we're currently in edge creation mode
    is_creating: bool,
}

impl EdgeCreator {
    /// Create a new edge creator
    pub fn new() -> Self {
        Self {
            preview_edge: None,
            is_creating: false,
        }
    }

    /// Check if currently creating an edge
    pub fn is_creating_edge(&self) -> bool {
        self.is_creating
    }

    /// Validate basic connection constraints
    fn validate_basic_connection(&self, source_node: &NodeId, target_node: &NodeId) -> Result<()> {
        // Check for self-connection
        if source_node == target_node {
            return Err(FlowError::SelfConnection);
        }

        Ok(())
    }

    /// Validate handle compatibility and connection limits
    fn validate_handle_compatibility<N, E>(
        &self,
        graph: &Graph<N, E>,
        source_node_ref: &crate::Node<N>,
        target_node_ref: &crate::Node<N>,
        source_handle: &str,
        target_handle: &str,
        source_node_id: &NodeId,
    ) -> Result<()> {
        // Get handles from nodes
        let source_handle_ref = source_node_ref
            .get_handle(&source_handle.into())
            .ok_or_else(|| FlowError::handle_not_found(source_handle))?;

        let target_handle_ref = target_node_ref
            .get_handle(&target_handle.into())
            .ok_or_else(|| FlowError::handle_not_found(target_handle))?;

        // Check handle compatibility
        if !source_handle_ref.can_connect_to(target_handle_ref) {
            return Err(FlowError::invalid_connection(
                format!("Incompatible handles: {} cannot connect to {}", source_handle, target_handle)
            ));
        }

        // Check connection limits for source handle
        if let Some(limit) = source_handle_ref.connection_limit {
            let current_connections = graph.edges()
                .filter(|edge| {
                    &edge.source == source_node_id &&
                    edge.source_handle.as_ref().map(|h| h.as_str()) == Some(source_handle)
                })
                .count();

            if current_connections >= limit {
                return Err(FlowError::connection_limit_exceeded(
                    source_handle,
                    current_connections,
                    limit
                ));
            }
        }

        // Check connection limits for target handle
        if let Some(limit) = target_handle_ref.connection_limit {
            let target_node_id: NodeId = target_node_ref.id.clone();
            let current_connections = graph.edges()
                .filter(|edge| {
                    &edge.target == &target_node_id &&
                    edge.target_handle.as_ref().map(|h| h.as_str()) == Some(target_handle)
                })
                .count();

            if current_connections >= limit {
                return Err(FlowError::connection_limit_exceeded(
                    target_handle,
                    current_connections,
                    limit
                ));
            }
        }

        Ok(())
    }

    /// Get the current preview edge
    pub fn get_preview_edge(&self) -> Option<&PreviewEdge> {
        self.preview_edge.as_ref()
    }

    /// Start edge creation from a source handle
    pub fn start_edge_creation<N, E>(
        &mut self,
        graph: &Graph<N, E>,
        source_node: &str,
        source_handle: &str,
        start_position: Position,
    ) -> Result<()> {
        let node_id: NodeId = source_node.into();

        // Validate source node exists
        if graph.get_node(&node_id).is_none() {
            return Err(FlowError::node_not_found(source_node));
        }

        // Create preview edge
        self.preview_edge = Some(PreviewEdge {
            source_node: node_id,
            source_handle: Some(source_handle.to_string()),
            start_position,
            end_position: start_position,
        });
        self.is_creating = true;

        Ok(())
    }

    /// Update the preview edge end position
    pub fn update_edge_preview(&mut self, end_position: Position) {
        if let Some(preview) = &mut self.preview_edge {
            preview.end_position = end_position;
        }
    }

    /// Complete edge creation to a target node/handle
    pub fn complete_edge_creation<N, E>(
        &mut self,
        graph: &mut Graph<N, E>,
        target_node: &str,
        target_handle: Option<&str>,
        _end_position: Position,
    ) -> Result<()>
    where
        N: Clone,
        E: Clone + Default,
    {
        let preview = self.preview_edge.take().ok_or_else(|| {
            FlowError::InvalidOperation {
                message: "No edge creation in progress".to_string(),
            }
        })?;

        let target_node_id: NodeId = target_node.into();

        // Validate target node exists
        if graph.get_node(&target_node_id).is_none() {
            return Err(FlowError::node_not_found(target_node));
        }

        // Validate basic constraints
        self.validate_basic_connection(&preview.source_node, &target_node_id)?;

        // Get source and target nodes for validation
        let source_node_ref = graph.get_node(&preview.source_node).unwrap();
        let target_node_ref = graph.get_node(&target_node_id).unwrap();

        // Validate handle compatibility if both handles specified
        if let (Some(source_handle), Some(target_handle)) = (&preview.source_handle, target_handle) {
            self.validate_handle_compatibility(
                graph,
                source_node_ref,
                target_node_ref,
                source_handle,
                target_handle,
                &preview.source_node
            )?;
        }

        // Create the actual edge
        let edge_id = format!("edge_{}_to_{}", preview.source_node.as_str(), target_node);
        let mut edge = create_edge_with_defaults::<E>(edge_id, preview.source_node.as_str(), target_node_id.as_str());

        // Set handle references if provided
        if let Some(source_handle) = preview.source_handle {
            edge = edge.with_source_handle(source_handle);
        }
        if let Some(target_handle) = target_handle {
            edge = edge.with_target_handle(target_handle.to_string());
        }

        graph.add_edge(edge)?;
        self.is_creating = false;

        Ok(())
    }

    /// Cancel edge creation
    pub fn cancel_edge_creation(&mut self) {
        self.preview_edge = None;
        self.is_creating = false;
    }

    /// Get potential drop target at given position
    pub fn get_drop_target<N, E>(
        &self,
        graph: &Graph<N, E>,
        position: Position,
        tolerance: f64,
    ) -> Option<(NodeId, Option<String>)> {
        // First check for handle hits
        for node in graph.nodes() {
            for handle in node.handles() {
                let handle_pos = node.position + handle.position.to_position();
                let distance = ((position.x - handle_pos.x).powi(2) + (position.y - handle_pos.y).powi(2)).sqrt();

                if distance <= tolerance {
                    return Some((node.id.clone(), Some(handle.id.as_str().to_string())));
                }
            }
        }

        // Then check for node hits
        for node in graph.nodes() {
            let node_bounds = crate::types::Rect::new(
                node.position.x,
                node.position.y,
                node.size.width,
                node.size.height,
            );

            if node_bounds.contains_point(position) {
                return Some((node.id.clone(), None));
            }
        }

        None
    }

    /// Get connection feedback for validation
    pub fn get_connection_feedback<N, E>(
        &self,
        _graph: &Graph<N, E>,
        target_node: &str,
        _target_handle: Option<&str>,
    ) -> ConnectionFeedback {
        let preview = match &self.preview_edge {
            Some(preview) => preview,
            None => {
                return ConnectionFeedback {
                    is_valid: false,
                    can_connect: false,
                    message: Some("No edge creation in progress".to_string()),
                };
            }
        };

        let target_node_id: NodeId = target_node.into();

        // Check for self-connection
        if preview.source_node == target_node_id {
            return ConnectionFeedback {
                is_valid: false,
                can_connect: false,
                message: Some("Cannot connect to self".to_string()),
            };
        }

        // For now, assume all other connections are valid
        ConnectionFeedback {
            is_valid: true,
            can_connect: true,
            message: None,
        }
    }
}

impl Default for EdgeCreator {
    fn default() -> Self {
        Self::new()
    }
}
