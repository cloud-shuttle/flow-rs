//! Graph data structures and operations

use std::collections::HashMap;
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::{FlowError, Result};
use crate::handle::Handle;
use crate::types::{EdgeId, NodeId, Position, Rect};

pub mod drag_operations;
pub mod edge;
pub mod node;
pub mod traversal;
pub mod validation;

pub use edge::{Edge, EdgeBuilder};
pub use node::{Node, NodeBuilder};

/// Graph container for nodes and edges
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<N = (), E = ()> {
    pub(crate) nodes: HashMap<NodeId, Node<N>>,
    pub(crate) edges: HashMap<EdgeId, Edge<E>>,

    #[cfg_attr(feature = "serde", serde(skip))]
    _phantom: PhantomData<(N, E)>,
}

impl<N, E> Graph<N, E> {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            _phantom: PhantomData,
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Node<N>) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(FlowError::duplicate_node_id(node.id.as_str()));
        }

        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Remove a node and all connected edges
    pub fn remove_node(&mut self, node_id: &NodeId) -> Result<Node<N>> {
        let node = self
            .nodes
            .remove(node_id)
            .ok_or_else(|| FlowError::node_not_found(node_id.as_str()))?;

        // Remove all connected edges
        self.edges.retain(|_, edge| !edge.is_connected_to(node_id));

        Ok(node)
    }

    /// Get a reference to a node
    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node<N>> {
        self.nodes.get(node_id)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut Node<N>> {
        self.nodes.get_mut(node_id)
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge<E>) -> Result<()> {
        // Validate that source and target nodes exist
        if !self.nodes.contains_key(&edge.source) {
            return Err(FlowError::node_not_found(edge.source.as_str()));
        }
        if !self.nodes.contains_key(&edge.target) {
            return Err(FlowError::node_not_found(edge.target.as_str()));
        }

        if self.edges.contains_key(&edge.id) {
            return Err(FlowError::duplicate_edge_id(edge.id.as_str()));
        }

        self.edges.insert(edge.id.clone(), edge);
        Ok(())
    }

    /// Remove an edge
    pub fn remove_edge(&mut self, edge_id: &EdgeId) -> Result<Edge<E>> {
        self.edges
            .remove(edge_id)
            .ok_or_else(|| FlowError::edge_not_found(edge_id.as_str()))
    }

    /// Get a reference to an edge
    pub fn get_edge(&self, edge_id: &EdgeId) -> Option<&Edge<E>> {
        self.edges.get(edge_id)
    }

    /// Get a mutable reference to an edge
    pub fn get_edge_mut(&mut self, edge_id: &EdgeId) -> Option<&mut Edge<E>> {
        self.edges.get_mut(edge_id)
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &Node<N>> {
        self.nodes.values()
    }

    /// Get all nodes mutably
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node<N>> {
        self.nodes.values_mut()
    }

    /// Get all edges
    pub fn edges(&self) -> impl Iterator<Item = &Edge<E>> {
        self.edges.values()
    }

    /// Get all edges mutably
    pub fn edges_mut(&mut self) -> impl Iterator<Item = &mut Edge<E>> {
        self.edges.values_mut()
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Check if graph is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Clear all nodes and edges
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// Get edges connected to a node
    pub fn get_connected_edges(&self, node_id: &NodeId) -> Vec<&Edge<E>> {
        self.edges
            .values()
            .filter(|edge| edge.is_connected_to(node_id))
            .collect()
    }

    /// Get incoming edges for a node
    pub fn get_incoming_edges(&self, node_id: &NodeId) -> Vec<&Edge<E>> {
        self.edges
            .values()
            .filter(|edge| &edge.target == node_id)
            .collect()
    }

    /// Get outgoing edges for a node
    pub fn get_outgoing_edges(&self, node_id: &NodeId) -> Vec<&Edge<E>> {
        self.edges
            .values()
            .filter(|edge| &edge.source == node_id)
            .collect()
    }

    /// Check if two nodes are connected
    pub fn are_connected(&self, source: &NodeId, target: &NodeId) -> bool {
        self.edges
            .values()
            .any(|edge| edge.connects(source, target))
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }

    /// Get all edge IDs
    pub fn edge_ids(&self) -> impl Iterator<Item = &EdgeId> {
        self.edges.keys()
    }

    /// Calculate bounding rectangle of all nodes
    pub fn bounds(&self) -> Option<Rect>
    where
        N: Clone,
    {
        if self.nodes.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for node in self.nodes.values() {
            let bounds = node.bounds();
            min_x = min_x.min(bounds.x);
            min_y = min_y.min(bounds.y);
            max_x = max_x.max(bounds.x + bounds.width);
            max_y = max_y.max(bounds.y + bounds.height);
        }

        Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
    }

    /// Find handle at position in the graph
    pub fn handle_at_position(
        &self,
        point: Position,
        handle_size: f64,
    ) -> Option<(&NodeId, &Handle)> {
        for node in self.nodes.values() {
            if let Some(handle) = node.handle_at_position(point, handle_size) {
                return Some((&node.id, handle));
            }
        }
        None
    }

    /// Get all handles of a specific type in the graph
    pub fn get_handles_by_type(
        &self,
        handle_type: crate::handle::HandleType,
    ) -> Vec<(&NodeId, &Handle)> {
        let mut handles = Vec::new();
        for node in self.nodes.values() {
            for handle in node.handles() {
                if handle.handle_type == handle_type {
                    handles.push((&node.id, handle));
                }
            }
        }
        handles
    }

    /// Create a new edge creator for this graph
    pub fn create_edge_creator(&self) -> crate::edge_creator::EdgeCreator {
        crate::edge_creator::EdgeCreator::new()
    }
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

// Include traversal algorithms

#[cfg(test)]
mod tests;
