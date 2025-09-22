//! Edge data structure and builder pattern

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::{FlowError, Result};
use crate::types::{EdgeId, NodeId};

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
        let source = self
            .source
            .ok_or_else(|| FlowError::invalid_connection("Source node not specified"))?;
        let target = self
            .target
            .ok_or_else(|| FlowError::invalid_connection("Target node not specified"))?;
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
        let source = self
            .source
            .ok_or_else(|| FlowError::invalid_connection("Source node not specified"))?;
        let target = self
            .target
            .ok_or_else(|| FlowError::invalid_connection("Target node not specified"))?;

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
