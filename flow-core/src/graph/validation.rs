//! Graph validation and constraint checking

use crate::error::{FlowError, Result};
use crate::handle::HandleId;
use crate::types::NodeId;

use super::{Edge, Graph};

impl<N, E> Graph<N, E> {
    /// Add an edge with handle validation
    pub fn add_handle_edge(&mut self, edge: Edge<E>) -> Result<()> {
        // Validate that source and target nodes exist
        let source_node = self
            .get_node(&edge.source)
            .ok_or_else(|| FlowError::node_not_found(edge.source.as_str()))?;
        let target_node = self
            .get_node(&edge.target)
            .ok_or_else(|| FlowError::node_not_found(edge.target.as_str()))?;

        // Validate handle references if specified
        if let Some(source_handle_id) = &edge.source_handle {
            let source_handle_id = HandleId::new(source_handle_id.clone());
            let source_handle = source_node
                .get_handle(&source_handle_id)
                .ok_or_else(|| FlowError::handle_not_found(source_handle_id.as_str()))?;

            // Check connection limit
            if !self.can_handle_accept_connection(&edge.source, &source_handle_id) {
                let current_count =
                    self.get_handle_connection_count(&edge.source, &source_handle_id);
                let limit = source_handle.connection_limit.unwrap_or(usize::MAX);
                return Err(FlowError::connection_limit_exceeded(
                    source_handle_id.as_str(),
                    current_count,
                    limit,
                ));
            }

            if let Some(target_handle_id) = &edge.target_handle {
                let target_handle_id = HandleId::new(target_handle_id.clone());
                let target_handle = target_node
                    .get_handle(&target_handle_id)
                    .ok_or_else(|| FlowError::handle_not_found(target_handle_id.as_str()))?;

                // Check handle compatibility
                if !source_handle.can_connect_to(target_handle) {
                    return Err(FlowError::invalid_connection(
                        "Handle types or connection types are incompatible",
                    ));
                }
            }
        }

        // If validation passes, add the edge normally
        self.add_edge(edge)
    }

    /// Get connection count for a specific handle
    fn get_handle_connection_count(&self, node_id: &NodeId, handle_id: &HandleId) -> usize {
        let handle_id_str = handle_id.as_str();
        self.edges
            .values()
            .filter(|edge| {
                (&edge.source == node_id && edge.source_handle.as_deref() == Some(handle_id_str))
                    || (&edge.target == node_id
                        && edge.target_handle.as_deref() == Some(handle_id_str))
            })
            .count()
    }

    /// Get all edges connected to a specific handle
    ///
    /// This method provides accurate connection counting by examining all edges
    /// in the graph that reference the specified handle.
    pub fn get_handle_connections(&self, node_id: &NodeId, handle_id: &HandleId) -> Vec<&Edge<E>> {
        let handle_id_str = handle_id.as_str();
        self.edges
            .values()
            .filter(|edge| {
                (&edge.source == node_id && edge.source_handle.as_deref() == Some(handle_id_str))
                    || (&edge.target == node_id
                        && edge.target_handle.as_deref() == Some(handle_id_str))
            })
            .collect()
    }

    /// Check if a handle can accept new connections (respects connection limits)
    ///
    /// This method provides accurate connection limit validation by counting
    /// current connections and comparing against the handle's limit.
    pub fn can_handle_accept_connection(&self, node_id: &NodeId, handle_id: &HandleId) -> bool {
        if let Some(node) = self.get_node(node_id) {
            if let Some(handle) = node.get_handle(handle_id) {
                if let Some(limit) = handle.connection_limit {
                    let current_connections = self.get_handle_connections(node_id, handle_id).len();
                    return current_connections < limit;
                }
            }
        }
        true // No limit or handle doesn't exist - allow connection
    }
}
