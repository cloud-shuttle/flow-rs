//! Node data structure and builder pattern

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::handle::{Handle, HandleId, HandleManager};
use crate::types::{NodeId, Position, Rect, Size};

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
        self.handle_manager
            .handle_at_position(point, self.position, self.size, handle_size)
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
