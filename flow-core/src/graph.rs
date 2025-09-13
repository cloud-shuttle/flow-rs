//! Graph data structures and operations

use std::collections::HashMap;
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::{FlowError, Result};
use crate::types::{Position, Size, Rect, NodeId, EdgeId};
use crate::handle::{Handle, HandleId, HandleManager};

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

    // Handle management
    #[cfg_attr(feature = "serde", serde(skip))]
    handle_manager: HandleManager,
}

impl<T: Clone> Node<T> {
    /// Create a new node
    pub fn new(id: impl Into<NodeId>, position: Position, data: T) -> Self {
        let node_id = id.into();
        Self {
            id: node_id.clone(),
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
            handle_manager: HandleManager::new(node_id),
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
            id: self.id.clone(),
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
            handle_manager: HandleManager::new(self.id),
        }
    }
}

impl<T> Node<T> {
    /// Add a handle to this node
    pub fn add_handle(&mut self, handle: Handle) -> Result<()> {
        self.handle_manager.add_handle(handle)
    }

    /// Remove a handle from this node
    pub fn remove_handle(&mut self, handle_id: &HandleId) -> Result<Handle> {
        self.handle_manager.remove_handle(handle_id)
    }

    /// Get a handle by ID
    pub fn get_handle(&self, handle_id: &HandleId) -> Option<&Handle> {
        self.handle_manager.get_handle(handle_id)
    }

    /// Get all handles on this node
    pub fn handles(&self) -> &[Handle] {
        self.handle_manager.handles()
    }

    /// Find handle at position relative to this node
    pub fn handle_at_position(&self, point: Position, handle_size: f64) -> Option<&Handle> {
        self.handle_manager.handle_at_position(point, self.position, self.size, handle_size)
    }

    /// Get source handles
    pub fn source_handles(&self) -> impl Iterator<Item = &Handle> {
        self.handle_manager.source_handles()
    }

    /// Get target handles
    pub fn target_handles(&self) -> impl Iterator<Item = &Handle> {
        self.handle_manager.target_handles()
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

    /// Set source handle
    pub fn with_source_handle(mut self, handle_id: impl Into<String>) -> Self {
        self.source_handle = Some(handle_id.into());
        self
    }

    /// Set target handle
    pub fn with_target_handle(mut self, handle_id: impl Into<String>) -> Self {
        self.target_handle = Some(handle_id.into());
        self
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

    /// Add an edge with handle validation
    pub fn add_handle_edge(&mut self, edge: Edge<E>) -> Result<()> {
        // Validate that source and target nodes exist
        let source_node = self.get_node(&edge.source)
            .ok_or_else(|| FlowError::node_not_found(edge.source.as_str()))?;
        let target_node = self.get_node(&edge.target)
            .ok_or_else(|| FlowError::node_not_found(edge.target.as_str()))?;

        // Validate handle references if specified
        if let Some(source_handle_id) = &edge.source_handle {
            let source_handle_id = HandleId::new(source_handle_id.clone());
            let source_handle = source_node.get_handle(&source_handle_id)
                .ok_or_else(|| FlowError::handle_not_found(source_handle_id.as_str()))?;

            // Check connection limit
            if !self.can_handle_accept_connection(&edge.source, &source_handle_id) {
                let current_count = self.get_handle_connection_count(&edge.source, &source_handle_id);
                let limit = source_handle.connection_limit.unwrap_or(usize::MAX);
                return Err(FlowError::connection_limit_exceeded(
                    source_handle_id.as_str(),
                    current_count,
                    limit
                ));
            }

            if let Some(target_handle_id) = &edge.target_handle {
                let target_handle_id = HandleId::new(target_handle_id.clone());
                let target_handle = target_node.get_handle(&target_handle_id)
                    .ok_or_else(|| FlowError::handle_not_found(target_handle_id.as_str()))?;

                // Check handle compatibility
                if !source_handle.can_connect_to(target_handle) {
                    return Err(FlowError::invalid_connection(
                        "Handle types or connection types are incompatible"
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
        self.edges.values()
            .filter(|edge| {
                (&edge.source == node_id && edge.source_handle.as_deref() == Some(handle_id_str)) ||
                (&edge.target == node_id && edge.target_handle.as_deref() == Some(handle_id_str))
            })
            .count()
    }

    /// Get all edges connected to a specific handle
    ///
    /// This method provides accurate connection counting by examining all edges
    /// in the graph that reference the specified handle.
    pub fn get_handle_connections(&self, node_id: &NodeId, handle_id: &HandleId) -> Vec<&Edge<E>> {
        let handle_id_str = handle_id.as_str();
        self.edges.values()
            .filter(|edge| {
                (&edge.source == node_id && edge.source_handle.as_deref() == Some(handle_id_str)) ||
                (&edge.target == node_id && edge.target_handle.as_deref() == Some(handle_id_str))
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

    /// Find handle at position in the graph
    pub fn handle_at_position(&self, point: Position, handle_size: f64) -> Option<(&NodeId, &Handle)> {
        for node in self.nodes.values() {
            if let Some(handle) = node.handle_at_position(point, handle_size) {
                return Some((&node.id, handle));
            }
        }
        None
    }

    /// Get all handles of a specific type in the graph
    pub fn get_handles_by_type(&self, handle_type: crate::handle::HandleType) -> Vec<(&NodeId, &Handle)> {
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

    /// Drag & Drop Operations
    /// Apply drag operation to selected nodes
    pub fn apply_node_drag(&mut self, selected_nodes: &std::collections::HashSet<NodeId>, delta: Position) -> Result<()> {
        self.apply_node_drag_with_transform(selected_nodes, delta, |pos, _| pos)
    }

    /// Apply drag operation with bounds constraint
    pub fn apply_node_drag_with_bounds(
        &mut self,
        selected_nodes: &std::collections::HashSet<NodeId>,
        delta: Position,
        bounds: Option<crate::types::Rect>
    ) -> Result<()> {
        self.apply_node_drag_with_transform(selected_nodes, delta, |new_pos, node| {
            if let Some(bounds) = bounds {
                Position::new(
                    new_pos.x.max(bounds.x).min(bounds.x + bounds.width - node.size.width),
                    new_pos.y.max(bounds.y).min(bounds.y + bounds.height - node.size.height),
                )
            } else {
                new_pos
            }
        })
    }

    /// Apply drag operation with grid snapping
    pub fn apply_node_drag_with_snap(
        &mut self,
        selected_nodes: &std::collections::HashSet<NodeId>,
        delta: Position,
        grid_size: f64
    ) -> Result<()> {
        self.apply_node_drag_with_transform(selected_nodes, delta, |new_pos, _| {
            Position::new(
                (new_pos.x / grid_size).round() * grid_size,
                (new_pos.y / grid_size).round() * grid_size,
            )
        })
    }

    /// Apply drag operation with custom constraint function
    pub fn apply_node_drag_with_constraint<F>(
        &mut self,
        selected_nodes: &std::collections::HashSet<NodeId>,
        delta: Position,
        constraint: F
    ) -> Result<()>
    where
        F: Fn(Position) -> Position,
    {
        self.apply_node_drag_with_transform(selected_nodes, delta, |new_pos, _| constraint(new_pos))
    }

    /// Internal method for applying drag operations with position transformation
    fn apply_node_drag_with_transform<F>(
        &mut self,
        selected_nodes: &std::collections::HashSet<NodeId>,
        delta: Position,
        transform: F
    ) -> Result<()>
    where
        F: Fn(Position, &Node<N>) -> Position,
    {
        // Pre-validate all nodes exist to fail fast
        for node_id in selected_nodes {
            if !self.nodes.contains_key(node_id) {
                return Err(FlowError::node_not_found(node_id.as_str()));
            }
        }

        // Apply transformations
        for node_id in selected_nodes {
            if let Some(node) = self.get_node_mut(node_id) {
                let new_pos = Position::new(node.position.x + delta.x, node.position.y + delta.y);
                node.position = transform(new_pos, node);
            }
        }

        Ok(())
    }

    /// Create a drag operation for undo/redo support
    pub fn create_drag_operation(
        &self,
        selected_nodes: &std::collections::HashSet<NodeId>,
        delta: Position
    ) -> Result<crate::drag_operations::DragOperation> {
        // Validate all nodes exist before creating operation
        for node_id in selected_nodes {
            if !self.nodes.contains_key(node_id) {
                return Err(FlowError::node_not_found(node_id.as_str()));
            }
        }

        Ok(crate::drag_operations::DragOperation::new(
            selected_nodes.clone(),
            delta
        ))
    }

    /// Interactive Edge Creation
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
        let mut graph: Graph<(), ()> = Graph::new();

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
        let mut graph: Graph<(), ()> = Graph::new();

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
        let mut graph: Graph<(), ()> = Graph::new();

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

    // Comprehensive Graph Operations Tests

    #[test]
    fn test_comprehensive_node_operations() {
        let mut graph: Graph<i32, ()> = Graph::new();

        // Test adding nodes
        let node1 = Node::new("node1", Position::new(0.0, 0.0), 42);
        let node2 = Node::new("node2", Position::new(100.0, 100.0), 84);

        assert!(graph.add_node(node1.clone()).is_ok());
        assert!(graph.add_node(node2.clone()).is_ok());
        assert_eq!(graph.node_count(), 2);
        assert!(!graph.is_empty());

        // Test getting nodes
        let retrieved = graph.get_node(&NodeId::from("node1"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, 42);

        // Test duplicate node error
        let duplicate = Node::new("node1", Position::new(50.0, 50.0), 100);
        let result = graph.add_node(duplicate);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FlowError::DuplicateNodeId { .. }));

        // Test node removal
        let removed = graph.remove_node(&NodeId::from("node1"));
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().data, 42);
        assert_eq!(graph.node_count(), 1);

        // Test removing non-existent node
        let not_found = graph.remove_node(&NodeId::from("nonexistent"));
        assert!(not_found.is_err());
        assert!(matches!(not_found.unwrap_err(), FlowError::NodeNotFound { .. }));
    }

    #[test]
    fn test_comprehensive_edge_operations() {
        let mut graph: Graph<(), String> = Graph::new();

        // Add nodes first
        graph.add_node(Node::new("A", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("B", Position::new(100.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("C", Position::new(200.0, 0.0), ())).unwrap();

        // Test adding edges
        let edge1 = Edge::new("edge1", "A", "B", "connects_to".to_string());
        let edge2 = Edge::new("edge2", "B", "C", "flows_into".to_string());

        assert!(graph.add_edge(edge1.clone()).is_ok());
        assert!(graph.add_edge(edge2.clone()).is_ok());
        assert_eq!(graph.edge_count(), 2);

        // Test getting edges
        let retrieved = graph.get_edge(&EdgeId::from("edge1"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, "connects_to");

        // Test edge to non-existent node
        let invalid_edge = Edge::new("invalid", "A", "nonexistent", "error".to_string());
        let result = graph.add_edge(invalid_edge);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FlowError::NodeNotFound { .. }));

        // Test duplicate edge
        let duplicate_edge = Edge::new("edge1", "A", "B", "duplicate".to_string());
        let result = graph.add_edge(duplicate_edge);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FlowError::DuplicateEdgeId { .. }));

        // Test edge removal
        let removed = graph.remove_edge(&EdgeId::from("edge1"));
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().data, "connects_to");
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_node_edge_relationships() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create nodes
        graph.add_node(Node::new("A", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("B", Position::new(100.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("C", Position::new(200.0, 0.0), ())).unwrap();

        // Create edges: A -> B, A -> C, B -> C
        graph.add_edge(Edge::new("AB", "A", "B", ())).unwrap();
        graph.add_edge(Edge::new("AC", "A", "C", ())).unwrap();
        graph.add_edge(Edge::new("BC", "B", "C", ())).unwrap();

        // Test connection queries
        assert!(graph.are_connected(&NodeId::from("A"), &NodeId::from("B")));
        assert!(graph.are_connected(&NodeId::from("A"), &NodeId::from("C")));
        assert!(graph.are_connected(&NodeId::from("B"), &NodeId::from("C")));
        assert!(!graph.are_connected(&NodeId::from("B"), &NodeId::from("A"))); // Direction matters

        // Test connected edges
        let a_edges = graph.get_connected_edges(&NodeId::from("A"));
        assert_eq!(a_edges.len(), 2);

        let b_edges = graph.get_connected_edges(&NodeId::from("B"));
        assert_eq!(b_edges.len(), 2);

        let c_edges = graph.get_connected_edges(&NodeId::from("C"));
        assert_eq!(c_edges.len(), 2);

        // Test incoming/outgoing edges
        let a_outgoing = graph.get_outgoing_edges(&NodeId::from("A"));
        assert_eq!(a_outgoing.len(), 2);

        let c_incoming = graph.get_incoming_edges(&NodeId::from("C"));
        assert_eq!(c_incoming.len(), 2);

        let b_incoming = graph.get_incoming_edges(&NodeId::from("B"));
        assert_eq!(b_incoming.len(), 1);

        let b_outgoing = graph.get_outgoing_edges(&NodeId::from("B"));
        assert_eq!(b_outgoing.len(), 1);
    }

    #[test]
    fn test_cascading_operations() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a small graph: A -> B -> C
        graph.add_node(Node::new("A", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("B", Position::new(100.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("C", Position::new(200.0, 0.0), ())).unwrap();

        graph.add_edge(Edge::new("AB", "A", "B", ())).unwrap();
        graph.add_edge(Edge::new("BC", "B", "C", ())).unwrap();

        assert_eq!(graph.edge_count(), 2);

        // Remove node B - should cascade to remove connected edges
        let removed = graph.remove_node(&NodeId::from("B"));
        assert!(removed.is_ok());
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0); // Both edges should be removed

        // Verify edges are gone
        assert!(graph.get_edge(&EdgeId::from("AB")).is_none());
        assert!(graph.get_edge(&EdgeId::from("BC")).is_none());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create DAG: A -> B -> D, A -> C -> D
        graph.add_node(Node::new("A", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("B", Position::new(100.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("C", Position::new(100.0, 100.0), ())).unwrap();
        graph.add_node(Node::new("D", Position::new(200.0, 0.0), ())).unwrap();

        graph.add_edge(Edge::new("AB", "A", "B", ())).unwrap();
        graph.add_edge(Edge::new("AC", "A", "C", ())).unwrap();
        graph.add_edge(Edge::new("BD", "B", "D", ())).unwrap();
        graph.add_edge(Edge::new("CD", "C", "D", ())).unwrap();

        // Test successful topological sort
        let topo_sort = graph.topological_sort();
        assert!(topo_sort.is_ok());
        let sorted = topo_sort.unwrap();
        assert_eq!(sorted.len(), 4);

        // Verify ordering - A should come before B and C, B and C should come before D
        let a_pos = sorted.iter().position(|id| id.as_str() == "A").unwrap();
        let b_pos = sorted.iter().position(|id| id.as_str() == "B").unwrap();
        let c_pos = sorted.iter().position(|id| id.as_str() == "C").unwrap();
        let d_pos = sorted.iter().position(|id| id.as_str() == "D").unwrap();

        assert!(a_pos < b_pos);
        assert!(a_pos < c_pos);
        assert!(b_pos < d_pos);
        assert!(c_pos < d_pos);
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        let mut graph: Graph<(), ()> = Graph::new();

        // Create cycle: A -> B -> C -> A
        graph.add_node(Node::new("A", Position::new(0.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("B", Position::new(100.0, 0.0), ())).unwrap();
        graph.add_node(Node::new("C", Position::new(200.0, 0.0), ())).unwrap();

        graph.add_edge(Edge::new("AB", "A", "B", ())).unwrap();
        graph.add_edge(Edge::new("BC", "B", "C", ())).unwrap();
        graph.add_edge(Edge::new("CA", "C", "A", ())).unwrap(); // Creates cycle

        // Should fail due to cycle
        let result = graph.topological_sort();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FlowError::InvalidOperation { .. }));
    }

    #[test]
    fn test_graph_iterators() {
        let mut graph: Graph<i32, String> = Graph::new();

        // Add test data
        graph.add_node(Node::new("A", Position::new(0.0, 0.0), 1)).unwrap();
        graph.add_node(Node::new("B", Position::new(100.0, 0.0), 2)).unwrap();
        graph.add_node(Node::new("C", Position::new(200.0, 0.0), 3)).unwrap();

        graph.add_edge(Edge::new("AB", "A", "B", "edge1".to_string())).unwrap();
        graph.add_edge(Edge::new("BC", "B", "C", "edge2".to_string())).unwrap();

        // Test node iteration
        let node_ids: Vec<_> = graph.node_ids().cloned().collect();
        assert_eq!(node_ids.len(), 3);
        assert!(node_ids.contains(&NodeId::from("A")));
        assert!(node_ids.contains(&NodeId::from("B")));
        assert!(node_ids.contains(&NodeId::from("C")));

        // Test edge iteration
        let edge_ids: Vec<_> = graph.edge_ids().cloned().collect();
        assert_eq!(edge_ids.len(), 2);
        assert!(edge_ids.contains(&EdgeId::from("AB")));
        assert!(edge_ids.contains(&EdgeId::from("BC")));

        // Test node values iteration
        let node_values: Vec<_> = graph.nodes().map(|n| n.data).collect();
        assert_eq!(node_values.len(), 3);
        assert!(node_values.contains(&1));
        assert!(node_values.contains(&2));
        assert!(node_values.contains(&3));

        // Test edge values iteration
        let edge_values: Vec<_> = graph.edges().map(|e| &e.data).collect();
        assert_eq!(edge_values.len(), 2);
        assert!(edge_values.contains(&&"edge1".to_string()));
        assert!(edge_values.contains(&&"edge2".to_string()));
    }
}

impl<N, E> Graph<N, E>
where
    N: Clone,
    E: Clone,
{
    /// Check if the graph contains any cycles using DFS-based cycle detection
    ///
    /// Uses Depth-First Search with recursion stack tracking to detect back edges.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn has_cycle(&self) -> bool {
        use std::collections::HashSet;

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        // Check each node as a potential starting point
        for node_id in self.node_ids() {
            if !visited.contains(node_id)
                && self.has_cycle_dfs(node_id, &mut visited, &mut rec_stack)
            {
                return true;
            }
        }

        false
    }

    /// DFS helper for cycle detection
    fn has_cycle_dfs(
        &self,
        node_id: &NodeId,
        visited: &mut std::collections::HashSet<NodeId>,
        rec_stack: &mut std::collections::HashSet<NodeId>,
    ) -> bool {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        // Check all neighbors (nodes this node points to)
        // Optimize: only iterate through edges that start from this node
        for edge in self.get_outgoing_edges(node_id) {
            let neighbor = &edge.target;

            // If neighbor not visited, recurse
            if !visited.contains(neighbor) {
                if self.has_cycle_dfs(neighbor, visited, rec_stack) {
                    return true;
                }
            }
            // If neighbor is in recursion stack, we found a back edge (cycle)
            else if rec_stack.contains(neighbor) {
                return true;
            }
        }

        rec_stack.remove(node_id);
        false
    }

    /// Check if adding an edge from source to target would create a cycle
    ///
    /// This is useful for preventing cycles during interactive edge creation.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn creates_cycle(&self, source: &NodeId, target: &NodeId) -> bool {
        // If nodes don't exist, no cycle can be created
        if !self.nodes.contains_key(source) || !self.nodes.contains_key(target) {
            return false;
        }

        // Check if target can reach source (would create cycle if we add source -> target)
        self.can_reach(target, source)
    }

    /// Check if 'from' node can reach 'to' node through existing edges
    fn can_reach(&self, from: &NodeId, to: &NodeId) -> bool {
        use std::collections::{HashSet, VecDeque};

        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(from.clone());
        visited.insert(from.clone());

        while let Some(current) = queue.pop_front() {
            // Check all outgoing edges from current node
            // Optimize: only iterate through edges that start from this node
            for edge in self.get_outgoing_edges(&current) {
                let neighbor = &edge.target;

                if neighbor == to {
                    return true;
                }

                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }

        false
    }

    /// Find a cycle in the graph, returning the cycle path if found
    ///
    /// Returns the first cycle found, or None if the graph is acyclic.
    /// The returned path represents the nodes in the cycle.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn find_cycle(&self) -> Option<Vec<NodeId>> {
        use std::collections::{HashMap, HashSet};

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut parent = HashMap::new();

        // Check each node as potential starting point
        for node_id in self.node_ids() {
            if !visited.contains(node_id) {
                if let Some(cycle) = self.find_cycle_dfs(
                    node_id,
                    &mut visited,
                    &mut rec_stack,
                    &mut parent,
                ) {
                    return Some(cycle);
                }
            }
        }

        None
    }

    /// DFS helper for finding cycle path
    fn find_cycle_dfs(
        &self,
        node_id: &NodeId,
        visited: &mut std::collections::HashSet<NodeId>,
        rec_stack: &mut std::collections::HashSet<NodeId>,
        parent: &mut std::collections::HashMap<NodeId, NodeId>,
    ) -> Option<Vec<NodeId>> {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        // Check all neighbors
        // Optimize: only iterate through edges that start from this node
        for edge in self.get_outgoing_edges(node_id) {
            let neighbor = &edge.target;

            // If neighbor not visited, recurse
            if !visited.contains(neighbor) {
                parent.insert(neighbor.clone(), node_id.clone());
                if let Some(cycle) = self.find_cycle_dfs(neighbor, visited, rec_stack, parent) {
                    return Some(cycle);
                }
            }
            // If neighbor is in recursion stack, we found a cycle
            else if rec_stack.contains(neighbor) {
                // Reconstruct cycle path
                let mut cycle = vec![neighbor.clone()];
                let mut current = node_id.clone();

                // Walk back through parents until we reach the cycle start
                while current != *neighbor {
                    cycle.push(current.clone());
                    current = parent.get(&current).unwrap_or(&current).clone();
                }

                cycle.reverse();
                return Some(cycle);
            }
        }

        rec_stack.remove(node_id);
        None
    }

    /// Perform topological sort on the graph using Kahn's algorithm
    ///
    /// Returns a valid topological ordering of nodes, or Err if the graph contains cycles.
    /// A topological sort is a linear ordering where for every directed edge (u, v),
    /// vertex u comes before v in the ordering.
    /// Time complexity: O(V + E), Space complexity: O(V)
    pub fn topological_sort(&self) -> Result<Vec<NodeId>> {
        use std::collections::{HashMap, VecDeque};

        // Calculate in-degrees
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();

        // Initialize all nodes with in-degree 0
        for node_id in self.node_ids() {
            in_degree.insert(node_id.clone(), 0);
        }

        // Count incoming edges for each node
        for edge in self.edges() {
            *in_degree.entry(edge.target.clone()).or_insert(0) += 1;
        }

        // Find nodes with in-degree 0
        let mut queue = VecDeque::new();
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }

        let mut result = Vec::new();

        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());

            // Reduce in-degree of neighbors
            for edge in self.edges() {
                if edge.source == node_id {
                    let neighbor = &edge.target;
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // If we didn't process all nodes, there must be a cycle
        if result.len() != self.node_count() {
            return Err(FlowError::invalid_operation("Graph contains cycles - topological sort not possible"));
        }

        Ok(result)
    }
}
