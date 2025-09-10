//! Graph data structures and operations

use std::collections::HashMap;
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::{FlowError, Result};
use crate::types::{Position, Size, Rect, NodeId, EdgeId};

/// Node in a flow graph
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node<T = ()> {
    pub id: NodeId,
    pub position: Position,
    pub size: Size,
    pub data: T,
    
    // Optional properties
    pub node_type: Option<String>,
    pub selected: bool,
    pub dragging: bool,
    pub selectable: bool,
    pub connectable: bool,
    pub deletable: bool,
    pub drag_handle: Option<String>,
    pub parent_node: Option<NodeId>,
    pub z_index: Option<i32>,
    pub hidden: bool,
    
    // Computed properties (not serialized)
    #[cfg_attr(feature = "serde", serde(skip))]
    pub measured: Option<Size>,
}

impl<T: Clone> Node<T> {
    /// Create a new node
    pub fn new(id: impl Into<NodeId>, position: Position, data: T) -> Self {
        Self {
            id: id.into(),
            position,
            size: Size::default(),
            data,
            node_type: None,
            selected: false,
            dragging: false,
            selectable: true,
            connectable: true,
            deletable: true,
            drag_handle: None,
            parent_node: None,
            z_index: None,
            hidden: false,
            measured: None,
        }
    }

    /// Create a builder for fluent construction
    pub fn builder(id: impl Into<NodeId>) -> NodeBuilder<T> 
    where
        T: Default,
    {
        NodeBuilder::new(id)
    }

    /// Get the bounding rectangle
    pub fn bounds(&self) -> Rect {
        Rect::from_pos_size(self.position, self.size)
    }

    /// Get the center position
    pub fn center(&self) -> Position {
        self.bounds().center()
    }

    /// Check if point is inside the node
    pub fn contains_point(&self, point: Position) -> bool {
        self.bounds().contains_point(point)
    }

    /// Update position
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Update size
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Set dragging state
    pub fn set_dragging(&mut self, dragging: bool) {
        self.dragging = dragging;
    }

    /// Convert to a different data type
    pub fn map_data<U>(self, f: impl FnOnce(T) -> U) -> Node<U> {
        Node {
            id: self.id,
            position: self.position,
            size: self.size,
            data: f(self.data),
            node_type: self.node_type,
            selected: self.selected,
            dragging: self.dragging,
            selectable: self.selectable,
            connectable: self.connectable,
            deletable: self.deletable,
            drag_handle: self.drag_handle,
            parent_node: self.parent_node,
            z_index: self.z_index,
            hidden: self.hidden,
            measured: self.measured,
        }
    }
}

impl Node<()> {
    /// Create a simple node with default data
    pub fn simple(id: impl Into<NodeId>, position: Position) -> Self {
        Self::new(id, position, ())
    }
}

impl<T: Default> Node<T> {
    /// Create a node with default data
    pub fn with_default_data(id: impl Into<NodeId>, position: Position) -> Self 
    where
        T: Default + Clone,
    {
        Self::new(id, position, T::default())
    }
}

/// Builder for fluent node construction
#[derive(Debug)]
pub struct NodeBuilder<T> {
    node: Node<T>,
}

impl<T: Default + Clone> NodeBuilder<T> {
    /// Create a new node builder
    pub fn new(id: impl Into<NodeId>) -> Self {
        Self {
            node: Node::new(id, Position::zero(), T::default()),
        }
    }
}

impl<T: Clone> NodeBuilder<T> {
    /// Create a builder with specific data
    pub fn with_data(id: impl Into<NodeId>, data: T) -> Self {
        Self {
            node: Node::new(id, Position::zero(), data),
        }
    }

    /// Set position
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.node.position = Position::new(x, y);
        self
    }

    /// Set position from Position
    pub fn at(mut self, position: Position) -> Self {
        self.node.position = position;
        self
    }

    /// Set size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.node.size = Size::new(width, height);
        self
    }

    /// Set size from Size
    pub fn with_size(mut self, size: Size) -> Self {
        self.node.size = size;
        self
    }

    /// Set node type
    pub fn node_type(mut self, node_type: impl Into<String>) -> Self {
        self.node.node_type = Some(node_type.into());
        self
    }

    /// Set selectable flag
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.node.selectable = selectable;
        self
    }

    /// Set connectable flag
    pub fn connectable(mut self, connectable: bool) -> Self {
        self.node.connectable = connectable;
        self
    }

    /// Set deletable flag
    pub fn deletable(mut self, deletable: bool) -> Self {
        self.node.deletable = deletable;
        self
    }

    /// Set drag handle
    pub fn drag_handle(mut self, handle: impl Into<String>) -> Self {
        self.node.drag_handle = Some(handle.into());
        self
    }

    /// Set parent node
    pub fn parent(mut self, parent_id: impl Into<NodeId>) -> Self {
        self.node.parent_node = Some(parent_id.into());
        self
    }

    /// Set z-index
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.node.z_index = Some(z_index);
        self
    }

    /// Set hidden flag
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.node.hidden = hidden;
        self
    }

    /// Build the node
    pub fn build(self) -> Node<T> {
        self.node
    }
}

/// Edge connecting two nodes
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edge<T = ()> {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub data: T,
    
    // Optional properties
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
    pub edge_type: Option<String>,
    pub selected: bool,
    pub animated: bool,
    pub hidden: bool,
    pub selectable: bool,
    pub deletable: bool,
    pub z_index: Option<i32>,
    pub label: Option<String>,
}

impl<T> Edge<T> {
    /// Create a new edge
    pub fn new(
        id: impl Into<EdgeId>,
        source: impl Into<NodeId>,
        target: impl Into<NodeId>,
        data: T,
    ) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            target: target.into(),
            data,
            source_handle: None,
            target_handle: None,
            edge_type: None,
            selected: false,
            animated: false,
            hidden: false,
            selectable: true,
            deletable: true,
            z_index: None,
            label: None,
        }
    }

    /// Create a builder for fluent construction
    pub fn builder() -> EdgeBuilder<T> {
        EdgeBuilder::new()
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Check if edge connects the given nodes
    pub fn connects(&self, source_id: &NodeId, target_id: &NodeId) -> bool {
        &self.source == source_id && &self.target == target_id
    }

    /// Check if edge is connected to the given node
    pub fn is_connected_to(&self, node_id: &NodeId) -> bool {
        &self.source == node_id || &self.target == node_id
    }

    /// Convert to a different data type
    pub fn map_data<U>(self, f: impl FnOnce(T) -> U) -> Edge<U> {
        Edge {
            id: self.id,
            source: self.source,
            target: self.target,
            data: f(self.data),
            source_handle: self.source_handle,
            target_handle: self.target_handle,
            edge_type: self.edge_type,
            selected: self.selected,
            animated: self.animated,
            hidden: self.hidden,
            selectable: self.selectable,
            deletable: self.deletable,
            z_index: self.z_index,
            label: self.label,
        }
    }
}

impl Edge<()> {
    /// Create a simple edge with default data
    pub fn simple(
        id: impl Into<EdgeId>,
        source: impl Into<NodeId>,
        target: impl Into<NodeId>,
    ) -> Self {
        Self::new(id, source, target, ())
    }
}

impl<T: Default> Edge<T> {
    /// Create an edge with default data
    pub fn with_default_data(
        id: impl Into<EdgeId>,
        source: impl Into<NodeId>,
        target: impl Into<NodeId>,
    ) -> Self {
        Self::new(id, source, target, T::default())
    }
}

/// Builder for fluent edge construction
#[derive(Debug)]
pub struct EdgeBuilder<T> {
    id: Option<EdgeId>,
    source: Option<NodeId>,
    target: Option<NodeId>,
    data: Option<T>,
    source_handle: Option<String>,
    target_handle: Option<String>,
    edge_type: Option<String>,
    selected: bool,
    animated: bool,
    hidden: bool,
    selectable: bool,
    deletable: bool,
    z_index: Option<i32>,
    label: Option<String>,
}

impl<T> EdgeBuilder<T> {
    /// Create a new edge builder
    pub fn new() -> Self {
        Self {
            id: None,
            source: None,
            target: None,
            data: None,
            source_handle: None,
            target_handle: None,
            edge_type: None,
            selected: false,
            animated: false,
            hidden: false,
            selectable: true,
            deletable: true,
            z_index: None,
            label: None,
        }
    }

    /// Set edge ID
    pub fn id(mut self, id: impl Into<EdgeId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Connect two nodes
    pub fn connect(mut self, source: impl Into<NodeId>, target: impl Into<NodeId>) -> Self {
        self.source = Some(source.into());
        self.target = Some(target.into());
        self
    }

    /// Connect specific handles
    pub fn connect_handles(
        mut self,
        source: impl Into<NodeId>,
        source_handle: impl Into<String>,
        target: impl Into<NodeId>,
        target_handle: impl Into<String>,
    ) -> Self {
        self.source = Some(source.into());
        self.target = Some(target.into());
        self.source_handle = Some(source_handle.into());
        self.target_handle = Some(target_handle.into());
        self
    }

    /// Set data
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Set edge type
    pub fn edge_type(mut self, edge_type: impl Into<String>) -> Self {
        self.edge_type = Some(edge_type.into());
        self
    }

    /// Set animated flag
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Build the edge
    pub fn build(self) -> Result<Edge<T>>
    where
        T: Default,
    {
        let id = self.id.unwrap_or_else(EdgeId::generate);
        let source = self.source.ok_or_else(|| {
            FlowError::invalid_connection("Source node not specified")
        })?;
        let target = self.target.ok_or_else(|| {
            FlowError::invalid_connection("Target node not specified")
        })?;
        let data = self.data.unwrap_or_default();

        if source == target {
            return Err(FlowError::SelfConnection);
        }

        Ok(Edge {
            id,
            source,
            target,
            data,
            source_handle: self.source_handle,
            target_handle: self.target_handle,
            edge_type: self.edge_type,
            selected: self.selected,
            animated: self.animated,
            hidden: self.hidden,
            selectable: self.selectable,
            deletable: self.deletable,
            z_index: self.z_index,
            label: self.label,
        })
    }

    /// Build the edge with specific data
    pub fn build_with_data(self, data: T) -> Result<Edge<T>> {
        let id = self.id.unwrap_or_else(EdgeId::generate);
        let source = self.source.ok_or_else(|| {
            FlowError::invalid_connection("Source node not specified")
        })?;
        let target = self.target.ok_or_else(|| {
            FlowError::invalid_connection("Target node not specified")
        })?;

        if source == target {
            return Err(FlowError::SelfConnection);
        }

        Ok(Edge {
            id,
            source,
            target,
            data,
            source_handle: self.source_handle,
            target_handle: self.target_handle,
            edge_type: self.edge_type,
            selected: self.selected,
            animated: self.animated,
            hidden: self.hidden,
            selectable: self.selectable,
            deletable: self.deletable,
            z_index: self.z_index,
            label: self.label,
        })
    }
}

impl<T> Default for EdgeBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Graph container for nodes and edges
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<N = (), E = ()> {
    nodes: HashMap<NodeId, Node<N>>,
    edges: HashMap<EdgeId, Edge<E>>,
    
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
        let node = self.nodes.remove(node_id)
            .ok_or_else(|| FlowError::node_not_found(node_id.as_str()))?;

        // Remove all connected edges
        self.edges.retain(|_, edge| {
            !edge.is_connected_to(node_id)
        });

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
        self.edges.remove(edge_id)
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
        self.edges.values()
            .filter(|edge| edge.is_connected_to(node_id))
            .collect()
    }

    /// Get incoming edges for a node
    pub fn get_incoming_edges(&self, node_id: &NodeId) -> Vec<&Edge<E>> {
        self.edges.values()
            .filter(|edge| &edge.target == node_id)
            .collect()
    }

    /// Get outgoing edges for a node
    pub fn get_outgoing_edges(&self, node_id: &NodeId) -> Vec<&Edge<E>> {
        self.edges.values()
            .filter(|edge| &edge.source == node_id)
            .collect()
    }

    /// Check if two nodes are connected
    pub fn are_connected(&self, source: &NodeId, target: &NodeId) -> bool {
        self.edges.values().any(|edge| edge.connects(source, target))
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
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::simple("test", Position::new(10.0, 20.0));
        assert_eq!(node.id.as_str(), "test");
        assert_eq!(node.position, Position::new(10.0, 20.0));
        assert!(node.selectable);
    }

    #[test]
    fn test_node_builder() {
        let node = Node::<()>::builder("test")
            .position(100.0, 200.0)
            .size(80.0, 40.0)
            .node_type("custom")
            .selectable(false)
            .build();

        assert_eq!(node.position, Position::new(100.0, 200.0));
        assert_eq!(node.size, Size::new(80.0, 40.0));
        assert_eq!(node.node_type, Some("custom".to_string()));
        assert!(!node.selectable);
    }

    #[test]
    fn test_edge_creation() {
        let edge = Edge::simple("e1", "node1", "node2");
        assert_eq!(edge.source.as_str(), "node1");
        assert_eq!(edge.target.as_str(), "node2");
        assert!(edge.connects(&"node1".into(), &"node2".into()));
    }

    #[test]
    fn test_edge_builder() {
        let edge = Edge::<()>::builder()
            .connect("source", "target")
            .animated(true)
            .label("test connection")
            .build()
            .unwrap();

        assert!(edge.animated);
        assert_eq!(edge.label, Some("test connection".to_string()));
    }

    #[test]
    fn test_edge_builder_self_connection() {
        let result = Edge::<()>::builder()
            .connect("node1", "node1")
            .build();

        assert!(matches!(result, Err(FlowError::SelfConnection)));
    }

    #[test]
    fn test_graph_operations() {
        let mut graph = Graph::new();
        
        let node1 = Node::simple("node1", Position::new(0.0, 0.0));
        let node2 = Node::simple("node2", Position::new(100.0, 100.0));
        
        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();
        
        assert_eq!(graph.node_count(), 2);
        assert!(graph.get_node(&"node1".into()).is_some());
        
        let edge = Edge::simple("edge1", "node1", "node2");
        graph.add_edge(edge).unwrap();
        
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.are_connected(&"node1".into(), &"node2".into()));
    }

    #[test]
    fn test_graph_cascade_delete() {
        let mut graph = Graph::new();
        
        graph.add_node(Node::simple("node1", Position::zero())).unwrap();
        graph.add_node(Node::simple("node2", Position::zero())).unwrap();
        graph.add_edge(Edge::simple("edge1", "node1", "node2")).unwrap();
        
        assert_eq!(graph.edge_count(), 1);
        
        graph.remove_node(&"node1".into()).unwrap();
        
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0); // Edge should be removed
    }

    #[test]
    fn test_graph_bounds() {
        let mut graph = Graph::new();
        
        graph.add_node(
            Node::builder("node1")
                .position(0.0, 0.0)
                .size(100.0, 50.0)
                .build()
        ).unwrap();
        
        graph.add_node(
            Node::builder("node2")
                .position(200.0, 300.0)
                .size(100.0, 50.0)
                .build()
        ).unwrap();
        
        let bounds = graph.bounds().unwrap();
        assert_eq!(bounds, Rect::new(0.0, 0.0, 300.0, 350.0));
    }
}