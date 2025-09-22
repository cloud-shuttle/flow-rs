//! Hierarchical layout algorithm implementation

use crate::error::{FlowError, Result};
use crate::graph::Graph;
use crate::layout::LayoutAlgorithm;
use crate::types::{NodeId, Position, Size};

/// Hierarchical layout algorithm for tree-like structures
#[derive(Debug, Clone)]
pub struct HierarchicalLayout {
    pub node_separation: f64,
    pub level_separation: f64,
    pub edge_routing: EdgeRouting,
    pub direction: LayoutDirection,
    pub root_node: Option<NodeId>,
}

/// Edge routing style for hierarchical layouts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeRouting {
    Straight,
    Orthogonal,
    Curved,
}

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutDirection {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
}

impl Default for HierarchicalLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl HierarchicalLayout {
    /// Create a new hierarchical layout with default settings
    pub fn new() -> Self {
        Self {
            node_separation: 80.0,
            level_separation: 100.0,
            edge_routing: EdgeRouting::Straight,
            direction: LayoutDirection::TopToBottom,
            root_node: None,
        }
    }

    /// Builder pattern for configuration
    pub fn builder() -> HierarchicalLayoutBuilder {
        HierarchicalLayoutBuilder::default()
    }

    /// Set the root node for the hierarchy
    pub fn with_root(mut self, root_id: NodeId) -> Self {
        self.root_node = Some(root_id);
        self
    }

    /// Build the tree structure from the graph
    fn build_tree<N, E>(&self, graph: &Graph<N, E>) -> Result<TreeNode> {
        let root_id = if let Some(ref root) = self.root_node {
            root.clone()
        } else {
            let root_nodes = self.find_root_nodes(graph)?;
            root_nodes.into_iter().next()
                .ok_or_else(|| FlowError::layout("No root node found for hierarchical layout"))?
        };

        self.build_tree_recursive(&root_id, graph, &mut std::collections::HashSet::new())
    }

    /// Find nodes with no incoming edges
    fn find_root_nodes<N, E>(&self, graph: &Graph<N, E>) -> Result<Vec<NodeId>> {
        let mut root_nodes = Vec::new();

        for node in graph.nodes() {
            let incoming_edges = graph.get_incoming_edges(&node.id);
            if incoming_edges.is_empty() {
                root_nodes.push(node.id.clone());
            }
        }

        if root_nodes.is_empty() {
            return Err(FlowError::layout(
                "No root nodes found - graph may be cyclic",
            ));
        }

        Ok(root_nodes)
    }

    /// Recursively build tree structure
    #[allow(clippy::only_used_in_recursion)]
    fn build_tree_recursive<N, E>(
        &self,
        node_id: &NodeId,
        graph: &Graph<N, E>,
        visited: &mut std::collections::HashSet<NodeId>,
    ) -> Result<TreeNode> {
        if visited.contains(node_id) {
            return Err(FlowError::layout("Cycle detected in graph"));
        }

        visited.insert(node_id.clone());

        let node = graph
            .get_node(node_id)
            .ok_or_else(|| FlowError::node_not_found(node_id.as_str()))?;

        let mut children = Vec::new();
        let outgoing_edges = graph.get_outgoing_edges(node_id);

        for edge in outgoing_edges {
            let child_tree = self.build_tree_recursive(&edge.target, graph, visited)?;
            children.push(child_tree);
        }

        visited.remove(node_id);

        Ok(TreeNode {
            id: node_id.clone(),
            size: node.size,
            children,
            position: Position::zero(), // Will be calculated during layout
            width: 0.0,                 // Will be calculated
            height: 0.0,                // Will be calculated
        })
    }

    /// Calculate layout positions using Walker's algorithm
    fn calculate_positions(&self, tree: &mut TreeNode) {
        // Phase 1: Calculate initial positions using post-order traversal
        self.first_walk(tree, 0);

        // Phase 2: Calculate final positions using pre-order traversal
        self.second_walk(tree, 0.0, 0);
    }

    /// First walk - calculate preliminary x coordinates
    #[allow(clippy::only_used_in_recursion)]
    fn first_walk(&self, node: &mut TreeNode, level: usize) {
        if node.children.is_empty() {
            // Leaf node
            node.width = node.size.width;
            node.height = node.size.height;
        } else {
            // Internal node - process children first
            for child in &mut node.children {
                self.first_walk(child, level + 1);
            }

            // Calculate width as sum of children plus separations
            let total_child_width: f64 = node.children.iter().map(|child| child.width).sum();
            let separations = (node.children.len().saturating_sub(1)) as f64 * self.node_separation;

            node.width = total_child_width + separations;
            node.height = node.size.height;

            // Position children
            let mut current_x = 0.0;
            for child in &mut node.children {
                child.position.x = current_x + child.width / 2.0;
                current_x += child.width + self.node_separation;
            }
        }
    }

    /// Second walk - calculate final coordinates
    fn second_walk(&self, node: &mut TreeNode, x_offset: f64, level: usize) {
        let y_position = match self.direction {
            LayoutDirection::TopToBottom => level as f64 * self.level_separation,
            LayoutDirection::BottomToTop => -(level as f64 * self.level_separation),
            LayoutDirection::LeftToRight => level as f64 * self.level_separation,
            LayoutDirection::RightToLeft => -(level as f64 * self.level_separation),
        };

        // Set final position
        match self.direction {
            LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => {
                node.position.x += x_offset;
                node.position.y = y_position;
            }
            LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => {
                node.position.x = y_position;
                node.position.y += x_offset;
            }
        }

        // Process children
        let children_start_x = node.position.x - node.width / 2.0;
        for child in &mut node.children {
            self.second_walk(child, children_start_x, level + 1);
        }
    }

    /// Apply tree positions to graph nodes
    #[allow(clippy::only_used_in_recursion)]
    fn apply_tree_to_graph<N, E>(&self, tree: &TreeNode, graph: &mut Graph<N, E>) {
        if let Some(node) = graph.get_node_mut(&tree.id) {
            node.position = tree.position;
        }

        for child in &tree.children {
            self.apply_tree_to_graph(child, graph);
        }
    }
}

impl<N, E> LayoutAlgorithm<N, E> for HierarchicalLayout {
    fn name(&self) -> &str {
        "Hierarchical"
    }

    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        if graph.is_empty() {
            return Ok(());
        }

        // Build tree structure
        let mut tree = self.build_tree(graph)?;

        // Calculate positions
        self.calculate_positions(&mut tree);

        // Apply to graph
        self.apply_tree_to_graph(&tree, graph);

        Ok(())
    }
}

/// Tree node for hierarchical layout calculation
#[derive(Debug, Clone)]
struct TreeNode {
    id: NodeId,
    size: Size,
    children: Vec<TreeNode>,
    position: Position,
    width: f64,
    height: f64,
}

/// Builder for hierarchical layout
#[derive(Debug, Clone, Default)]
pub struct HierarchicalLayoutBuilder {
    layout: HierarchicalLayout,
}

impl HierarchicalLayoutBuilder {
    /// Set node separation distance
    pub fn node_separation(mut self, separation: f64) -> Self {
        self.layout.node_separation = separation;
        self
    }

    /// Set level separation distance
    pub fn level_separation(mut self, separation: f64) -> Self {
        self.layout.level_separation = separation;
        self
    }

    /// Set edge routing style
    pub fn edge_routing(mut self, routing: EdgeRouting) -> Self {
        self.layout.edge_routing = routing;
        self
    }

    /// Set layout direction
    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.layout.direction = direction;
        self
    }

    /// Set root node
    pub fn root_node(mut self, root_id: impl Into<NodeId>) -> Self {
        self.layout.root_node = Some(root_id.into());
        self
    }

    /// Build the layout algorithm
    pub fn build(self) -> HierarchicalLayout {
        self.layout
    }
}
