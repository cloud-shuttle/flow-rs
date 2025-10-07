//! Subflows - Nested Graph Hierarchy for Flow-RS
//!
//! Provides hierarchical graph structures where nodes can contain entire sub-graphs.
//! This enables complex nested workflows, modular design, and scalable architectures.
//!
//! Key Features:
//! - Parent-child graph relationships
//! - Zoom navigation between hierarchy levels
//! - Breadcrumb navigation
//! - Cross-level edge connections
//! - Subflow-specific layouts and behaviors

use crate::graph::Graph;
use crate::types::{NodeId, Position};
use std::collections::HashMap;

/// Represents a hierarchical graph with nested subflows
#[derive(Clone, Debug)]
pub struct HierarchicalGraph<N, E> {
    /// Root level graph
    pub root_graph: Graph<N, E>,
    /// Map of node IDs to their subflow graphs
    pub subflows: HashMap<NodeId, Graph<N, E>>,
    /// Current navigation path (stack of node IDs)
    pub navigation_path: Vec<NodeId>,
    /// Cross-level connections (edges that span hierarchy levels)
    pub cross_level_edges: Vec<CrossLevelEdge>,
}

#[derive(Clone, Debug)]
pub struct CrossLevelEdge {
    /// Source node ID (can be in any level)
    pub source: HierarchicalNodeRef,
    /// Target node ID (can be in any level)
    pub target: HierarchicalNodeRef,
    /// Edge data
    pub data: String,
    /// Visual properties for cross-level rendering
    pub visual_props: CrossLevelEdgeProps,
}

#[derive(Clone, Debug)]
pub struct HierarchicalNodeRef {
    /// Path to the node (empty for root level)
    pub path: Vec<NodeId>,
    /// Node ID within its graph
    pub node_id: NodeId,
}

#[derive(Clone, Debug)]
pub struct CrossLevelEdgeProps {
    /// Edge color
    pub color: String,
    /// Edge thickness
    pub thickness: f64,
    /// Dash pattern (if any)
    pub dash_pattern: Option<Vec<f64>>,
    /// Animation properties
    pub animated: bool,
}

#[derive(Clone, Debug)]
pub struct NavigationState {
    /// Current navigation path
    pub path: Vec<NodeId>,
    /// Viewport state for each level (placeholder for future implementation)
    pub viewport_states: HashMap<String, ViewportState>,
    /// Zoom level for each level
    pub zoom_levels: HashMap<String, f64>,
}

#[derive(Clone, Debug)]
pub struct ViewportState {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl<N, E> HierarchicalGraph<N, E> {
    pub fn new() -> Self {
        Self {
            root_graph: Graph::new(),
            subflows: HashMap::new(),
            navigation_path: Vec::new(),
            cross_level_edges: Vec::new(),
        }
    }

    /// Get the current graph (root or subflow based on navigation path)
    pub fn current_graph(&self) -> &Graph<N, E> {
        if self.navigation_path.is_empty() {
            &self.root_graph
        } else {
            // Navigate to the current subflow
            let mut current_graph = &self.root_graph;
            for node_id in &self.navigation_path {
                if let Some(subflow) = self.subflows.get(node_id) {
                    current_graph = subflow;
                } else {
                    // Invalid path, return root
                    return &self.root_graph;
                }
            }
            current_graph
        }
    }

    /// Get the current graph mutably
    /// Note: This is a simplified version that only works for the root graph
    /// For full hierarchical editing, use the specific methods below
    pub fn current_graph_mut(&mut self) -> &mut Graph<N, E> {
        if self.navigation_path.is_empty() {
            &mut self.root_graph
        } else {
            // For now, return root if we're in a subflow
            // Full implementation would require more complex borrowing
            &mut self.root_graph
        }
    }

    /// Navigate into a subflow
    pub fn enter_subflow(&mut self, node_id: NodeId) -> Result<(), String> {
        // Check if the node exists in current graph and has a subflow
        if self.current_graph().nodes().any(|n| n.id == node_id) {
            if self.subflows.contains_key(&node_id) {
                self.navigation_path.push(node_id);
                Ok(())
            } else {
                Err(format!("Node {} has no subflow", node_id))
            }
        } else {
            Err(format!("Node {} not found in current graph", node_id))
        }
    }

    /// Navigate up one level
    pub fn exit_subflow(&mut self) -> bool {
        self.navigation_path.pop().is_some()
    }

    /// Navigate to root level
    pub fn go_to_root(&mut self) {
        self.navigation_path.clear();
    }

    /// Create a new subflow for a node
    pub fn create_subflow(&mut self, node_id: NodeId) -> Result<(), String> {
        if self.current_graph().nodes().any(|n| n.id == node_id) {
            if !self.subflows.contains_key(&node_id) {
                self.subflows.insert(node_id, Graph::new());
                Ok(())
            } else {
                Err(format!("Node {} already has a subflow", node_id))
            }
        } else {
            Err(format!("Node {} not found in current graph", node_id))
        }
    }

    /// Delete a subflow
    pub fn delete_subflow(&mut self, node_id: &NodeId) -> bool {
        self.subflows.remove(node_id).is_some()
    }

    /// Get breadcrumb path for navigation
    pub fn get_breadcrumb_path(&self) -> Vec<(NodeId, String)> where N: std::fmt::Debug {
        let mut breadcrumbs = vec![(NodeId::new("root".to_string()), "Root".to_string())];

        let mut current_graph = &self.root_graph;
        for node_id in &self.navigation_path {
            if let Some(node) = current_graph.nodes().find(|n| n.id == *node_id) {
                breadcrumbs.push((node.id.clone(), format!("{:?}", node.data)));
            }
            if let Some(subflow) = self.subflows.get(node_id) {
                current_graph = subflow;
            }
        }

        breadcrumbs
    }

    /// Add a cross-level edge
    pub fn add_cross_level_edge(
        &mut self,
        source_path: Vec<NodeId>,
        source_node: NodeId,
        target_path: Vec<NodeId>,
        target_node: NodeId,
        data: String,
    ) -> Result<(), String> {
        // Validate that source and target nodes exist
        let source_ref = HierarchicalNodeRef {
            path: source_path,
            node_id: source_node,
        };
        let target_ref = HierarchicalNodeRef {
            path: target_path,
            node_id: target_node,
        };

        if self.node_exists(&source_ref) && self.node_exists(&target_ref) {
            let cross_edge = CrossLevelEdge {
                source: source_ref,
                target: target_ref,
                data,
                visual_props: CrossLevelEdgeProps {
                    color: "#666".to_string(),
                    thickness: 2.0,
                    dash_pattern: Some(vec![5.0, 5.0]),
                    animated: true,
                },
            };
            self.cross_level_edges.push(cross_edge);
            Ok(())
        } else {
            Err("Source or target node does not exist".to_string())
        }
    }

    /// Check if a hierarchical node reference exists
    pub fn node_exists(&self, node_ref: &HierarchicalNodeRef) -> bool {
        if node_ref.path.is_empty() {
            // Root level
            self.root_graph.nodes().any(|n| n.id == node_ref.node_id)
        } else {
            // Navigate to the subflow
            let mut current_graph = &self.root_graph;
            for path_node in &node_ref.path {
                if let Some(subflow) = self.subflows.get(path_node) {
                    current_graph = subflow;
                } else {
                    return false;
                }
            }
            current_graph.nodes().any(|n| n.id == node_ref.node_id)
        }
    }

    /// Get all visible cross-level edges for current navigation level
    pub fn get_visible_cross_level_edges(&self) -> Vec<&CrossLevelEdge> {
        self.cross_level_edges.iter()
            .filter(|edge| {
                // Edge is visible if either source or target is in current level
                // or if the edge spans from parent to child levels
                self.is_edge_visible(edge)
            })
            .collect()
    }

    fn is_edge_visible(&self, edge: &CrossLevelEdge) -> bool {
        let current_path = &self.navigation_path;

        // Check if source or target is in current level
        let source_in_current = edge.source.path == *current_path;
        let target_in_current = edge.target.path == *current_path;

        // Check if edge spans from current level to child level
        let source_is_parent = edge.source.path.len() < current_path.len() &&
                              current_path.starts_with(&edge.source.path);
        let target_is_child = edge.target.path.starts_with(current_path) &&
                             edge.target.path.len() == current_path.len() + 1;

        source_in_current || target_in_current || (source_is_parent && target_is_child)
    }

    /// Apply layout to current graph level
    pub fn apply_layout_to_current(&mut self, layout_type: &str) -> Result<(), String> {
        // This would integrate with the layout system
        // For now, just a placeholder
        Ok(())
    }

    /// Get statistics for the entire hierarchy
    pub fn get_hierarchy_stats(&self) -> HierarchyStats {
        let mut total_nodes = 0;
        let mut total_edges = 0;
        let mut max_depth = 0;
        let mut subflow_count = self.subflows.len();

        // Count nodes and edges in root
        total_nodes += self.root_graph.nodes().count();
        total_edges += self.root_graph.edges().count();

        // Recursively count in subflows
        let mut to_visit = vec![(&self.root_graph, 1)];
        while let Some((graph, depth)) = to_visit.pop() {
            max_depth = max_depth.max(depth);

            for node in graph.nodes() {
                if let Some(subflow) = self.subflows.get(&node.id) {
                    total_nodes += subflow.nodes().count();
                    total_edges += subflow.edges().count();
                    to_visit.push((subflow, depth + 1));
                }
            }
        }

        HierarchyStats {
            total_nodes,
            total_edges,
            max_depth,
            subflow_count,
            cross_level_edges: self.cross_level_edges.len(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HierarchyStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub max_depth: usize,
    pub subflow_count: usize,
    pub cross_level_edges: usize,
}

/// Subflow renderer for visual representation
/// Note: Rendering functionality is implemented in the flow-rs-renderer crate
pub struct SubflowRenderer;

impl<N, E> HierarchicalGraph<N, E> {
    /// Get the position of a hierarchical node reference
    fn get_node_position(&self, node_ref: &HierarchicalNodeRef) -> Option<Position> {
        if node_ref.path.is_empty() {
            // Root level
            self.root_graph.nodes()
                .find(|n| n.id == node_ref.node_id)
                .map(|n| n.position)
        } else {
            // Navigate to the subflow
            let mut current_graph = &self.root_graph;
            for path_node in &node_ref.path {
                if let Some(subflow) = self.subflows.get(path_node) {
                    current_graph = subflow;
                } else {
                    return None;
                }
            }
            current_graph.nodes()
                .find(|n| n.id == node_ref.node_id)
                .map(|n| n.position)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_graph_creation() {
        let hierarchy: HierarchicalGraph<String, String> = HierarchicalGraph::new();
        assert!(hierarchy.navigation_path.is_empty());
        assert!(hierarchy.subflows.is_empty());
        assert!(hierarchy.cross_level_edges.is_empty());
    }

    #[test]
    fn test_subflow_creation() {
        let mut hierarchy: HierarchicalGraph<String, String> = HierarchicalGraph::new();

        // Add a node to root
        let node = Node::new("node1".to_string(), Position::new(100.0, 100.0), "Test Node".to_string());
        hierarchy.root_graph.add_node(node).unwrap();

        // Create subflow
        assert!(hierarchy.create_subflow("node1".into()).is_ok());
        assert!(hierarchy.subflows.contains_key(&"node1".into()));
    }

    #[test]
    fn test_navigation() {
        let mut hierarchy: HierarchicalGraph<String, String> = HierarchicalGraph::new();

        // Add nodes
        let node1 = Node::new("node1".to_string(), Position::new(100.0, 100.0), "Node 1".to_string());
        hierarchy.root_graph.add_node(node1).unwrap();
        hierarchy.create_subflow("node1".into()).unwrap();

        // Navigate into subflow
        assert!(hierarchy.enter_subflow("node1".into()).is_ok());
        assert_eq!(hierarchy.navigation_path, vec!["node1".into()]);

        // Exit subflow
        assert!(hierarchy.exit_subflow());
        assert!(hierarchy.navigation_path.is_empty());
    }

    #[test]
    fn test_cross_level_edges() {
        let mut hierarchy: HierarchicalGraph<String, String> = HierarchicalGraph::new();

        // Add nodes to root and subflow
        let root_node = Node::new("root1".to_string(), Position::new(100.0, 100.0), "Root Node".to_string());
        hierarchy.root_graph.add_node(root_node).unwrap();

        hierarchy.create_subflow("root1".into()).unwrap();
        let sub_node = Node::new("sub1".to_string(), Position::new(50.0, 50.0), "Sub Node".to_string());
        hierarchy.subflows.get_mut(&"root1".into()).unwrap().add_node(sub_node).unwrap();

        // Add cross-level edge
        assert!(hierarchy.add_cross_level_edge(
            vec![], // root level
            "root1".into(),
            vec!["root1".into()], // subflow level
            "sub1".into(),
            "cross-level connection".to_string(),
        ).is_ok());

        assert_eq!(hierarchy.cross_level_edges.len(), 1);
    }

    #[test]
    fn test_hierarchy_stats() {
        let mut hierarchy: HierarchicalGraph<String, String> = HierarchicalGraph::new();

        // Add nodes to multiple levels
        let root_node = Node::new("root1".to_string(), Position::new(100.0, 100.0), "Root".to_string());
        hierarchy.root_graph.add_node(root_node).unwrap();

        hierarchy.create_subflow("root1".into()).unwrap();
        let sub_node1 = Node::new("sub1".to_string(), Position::new(50.0, 50.0), "Sub 1".to_string());
        let sub_node2 = Node::new("sub2".to_string(), Position::new(150.0, 150.0), "Sub 2".to_string());
        hierarchy.subflows.get_mut(&"root1".into()).unwrap().add_node(sub_node1).unwrap();
        hierarchy.subflows.get_mut(&"root1".into()).unwrap().add_node(sub_node2).unwrap();

        let stats = hierarchy.get_hierarchy_stats();
        assert_eq!(stats.total_nodes, 3); // 1 root + 2 sub
        assert_eq!(stats.max_depth, 2);
        assert_eq!(stats.subflow_count, 1);
    }
}
