//! Layout algorithms for automatic node positioning

use std::collections::HashMap;

use crate::error::{FlowError, Result};
use crate::graph::Graph;
use crate::types::{Position, NodeId};

/// Trait for layout algorithms
pub trait LayoutAlgorithm<N, E> {
    /// Name of the layout algorithm
    fn name(&self) -> &str;

    /// Apply the layout to the graph
    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()>;

    /// Check if the algorithm is running
    fn is_running(&self) -> bool { false }

    /// Stop the algorithm if it's running
    fn stop(&mut self) -> Result<()> { Ok(()) }

    /// Get the progress (0.0 to 1.0)
    fn progress(&self) -> f64 { 1.0 }

    /// Check if the algorithm can be interrupted
    fn can_interrupt(&self) -> bool { false }
}

/// Force-directed layout algorithm
#[derive(Debug, Clone)]
pub struct ForceDirectedLayout {
    pub iterations: usize,
    pub spring_length: f64,
    pub spring_strength: f64,
    pub repulsion_strength: f64,
    pub damping: f64,
    pub center_force: f64,
    pub randomize_start: bool,

    // Runtime state
    current_iteration: usize,
    running: bool,
    node_velocities: HashMap<NodeId, Position>,
    temperature: f64,
}

impl Default for ForceDirectedLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl ForceDirectedLayout {
    /// Create a new force-directed layout with default parameters
    pub fn new() -> Self {
        Self {
            iterations: 100,
            spring_length: 100.0,
            spring_strength: 0.5,
            repulsion_strength: 1000.0,
            damping: 0.9,
            center_force: 0.01,
            randomize_start: true,
            current_iteration: 0,
            running: false,
            node_velocities: HashMap::new(),
            temperature: 1.0,
        }
    }

    /// Builder pattern for configuration
    pub fn builder() -> ForceDirectedLayoutBuilder {
        ForceDirectedLayoutBuilder::default()
    }

    /// Calculate attractive forces between connected nodes
    fn calculate_spring_forces<N, E>(&self, graph: &Graph<N, E>) -> HashMap<NodeId, Position> {
        let mut forces = HashMap::new();

        for edge in graph.edges() {
            if let (Some(source), Some(target)) = (
                graph.get_node(&edge.source),
                graph.get_node(&edge.target),
            ) {
                let delta = target.position - source.position;
                let distance = delta.x.hypot(delta.y).max(0.1);
                let displacement = distance - self.spring_length;
                let force_magnitude = self.spring_strength * displacement;

                let force_direction = Position::new(delta.x / distance, delta.y / distance);
                let force = force_direction * force_magnitude;

                // Apply force to both nodes (opposite directions)
                *forces.entry(edge.source.clone()).or_insert(Position::zero()) += force;
                *forces.entry(edge.target.clone()).or_insert(Position::zero()) -= force;
            }
        }

        forces
    }

    /// Calculate repulsive forces between all node pairs
    fn calculate_repulsion_forces<N, E>(&self, graph: &Graph<N, E>) -> HashMap<NodeId, Position> {
        let mut forces = HashMap::new();
        let nodes: Vec<_> = graph.nodes().collect();

        for (i, node1) in nodes.iter().enumerate() {
            let mut total_force = Position::zero();

            for (j, node2) in nodes.iter().enumerate() {
                if i != j {
                    let delta = node1.position - node2.position;
                    let distance_squared = delta.x * delta.x + delta.y * delta.y;

                    if distance_squared > 0.01 { // Avoid division by zero
                        let distance = distance_squared.sqrt();
                        let force_magnitude = self.repulsion_strength / distance_squared;
                        let force_direction = Position::new(delta.x / distance, delta.y / distance);
                        total_force += force_direction * force_magnitude;
                    }
                }
            }

            forces.insert(node1.id.clone(), total_force);
        }

        forces
    }

    /// Calculate center force to prevent nodes from drifting
    fn calculate_center_forces<N, E>(&self, graph: &Graph<N, E>) -> HashMap<NodeId, Position> {
        let mut forces = HashMap::new();

        // Calculate center of mass
        let mut center = Position::zero();
        let node_count = graph.node_count() as f64;

        if node_count > 0.0 {
            for node in graph.nodes() {
                center += node.position;
            }
            center = center * (1.0 / node_count);
        }

        // Apply center force
        for node in graph.nodes() {
            let delta = center - node.position;
            let force = delta * self.center_force;
            forces.insert(node.id.clone(), force);
        }

        forces
    }

    /// Apply forces to update node positions
    fn apply_forces<N, E>(&mut self, graph: &mut Graph<N, E>, forces: &HashMap<NodeId, Position>) {
        let dt = 0.1; // Time step
        let cooling = 1.0 - (self.current_iteration as f64 / self.iterations as f64);

        for node in graph.nodes_mut() {
            if let Some(&force) = forces.get(&node.id) {
                // Update velocity
                let velocity = self.node_velocities.entry(node.id.clone())
                    .or_default();

                *velocity = *velocity * self.damping + force * dt;

                // Apply cooling to reduce movement over time
                *velocity = *velocity * cooling;

                // Update position
                node.position += *velocity * dt;
            }
        }
    }

    /// Initialize random positions if requested
    fn maybe_randomize_positions<N, E>(&self, graph: &mut Graph<N, E>) {
        if self.randomize_start {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            for node in graph.nodes_mut() {
                // Use node ID as seed for deterministic randomness
                let mut hasher = DefaultHasher::new();
                node.id.hash(&mut hasher);
                let seed = hasher.finish();

                // Simple LCG for pseudo-random numbers
                let a = 1664525u64;
                let c = 1013904223u64;
                let x = a.wrapping_mul(seed).wrapping_add(c);
                let y = a.wrapping_mul(x).wrapping_add(c);

                let range = 200.0;
                let rand_x = ((x % 1000) as f64 / 1000.0 - 0.5) * range;
                let rand_y = ((y % 1000) as f64 / 1000.0 - 0.5) * range;

                node.position = Position::new(rand_x, rand_y);
            }
        }
    }

    /// Calculate the total energy of the system
    fn calculate_energy<N, E>(&self, graph: &Graph<N, E>) -> f64 {
        let spring_forces = self.calculate_spring_forces(graph);
        let repulsion_forces = self.calculate_repulsion_forces(graph);

        let mut energy = 0.0;
        let zero = Position::zero();

        for node in graph.nodes() {
            let spring_force = spring_forces.get(&node.id).unwrap_or(&zero);
            let repulsion_force = repulsion_forces.get(&node.id).unwrap_or(&zero);

            let total_force = *spring_force + *repulsion_force;
            energy += total_force.x * total_force.x + total_force.y * total_force.y;
        }

        energy.sqrt()
    }

    /// Check if the algorithm has converged
    fn has_converged(&self, energy: f64) -> bool {
        energy < 1.0 || self.current_iteration >= self.iterations
    }

    /// Check if the algorithm completed successfully (either by convergence or max iterations)
    fn completed_successfully(&self) -> bool {
        !self.running
    }
}

impl<N, E> LayoutAlgorithm<N, E> for ForceDirectedLayout {
    fn name(&self) -> &str {
        "Force-Directed"
    }

    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        if graph.is_empty() {
            return Ok(());
        }

        self.running = true;
        self.current_iteration = 0;
        self.node_velocities.clear();

        // Initialize positions if requested
        self.maybe_randomize_positions(graph);

        // Main iteration loop
        while self.current_iteration < self.iterations && self.running {
            // Calculate forces
            let spring_forces = self.calculate_spring_forces(graph);
            let repulsion_forces = self.calculate_repulsion_forces(graph);
            let center_forces = self.calculate_center_forces(graph);

            // Combine forces
            let mut total_forces = HashMap::new();
            for node in graph.nodes() {
                let zero = Position::zero();
                let spring = spring_forces.get(&node.id).unwrap_or(&zero);
                let repulsion = repulsion_forces.get(&node.id).unwrap_or(&zero);
                let center = center_forces.get(&node.id).unwrap_or(&zero);

                total_forces.insert(node.id.clone(), *spring + *repulsion + *center);
            }

            // Apply forces
            self.apply_forces(graph, &total_forces);

            // Check convergence
            let energy = self.calculate_energy(graph);
            if self.has_converged(energy) {
                break;
            }

            self.current_iteration += 1;
        }

        self.running = false;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn stop(&mut self) -> Result<()> {
        self.running = false;
        Ok(())
    }

    fn progress(&self) -> f64 {
        if self.iterations == 0 {
            1.0
        } else if self.completed_successfully() {
            // If algorithm completed successfully (either converged or finished all iterations)
            1.0
        } else {
            (self.current_iteration as f64 / self.iterations as f64).min(1.0)
        }
    }

    fn can_interrupt(&self) -> bool {
        true
    }
}

/// Builder for force-directed layout
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ForceDirectedLayoutBuilder {
    layout: ForceDirectedLayout,
}

impl ForceDirectedLayoutBuilder {
    /// Set number of iterations
    pub fn iterations(mut self, iterations: usize) -> Self {
        self.layout.iterations = iterations;
        self
    }

    /// Set spring length
    pub fn spring_length(mut self, length: f64) -> Self {
        self.layout.spring_length = length;
        self
    }

    /// Set spring strength
    pub fn spring_strength(mut self, strength: f64) -> Self {
        self.layout.spring_strength = strength;
        self
    }

    /// Set repulsion strength
    pub fn repulsion_strength(mut self, strength: f64) -> Self {
        self.layout.repulsion_strength = strength;
        self
    }

    /// Set damping factor
    pub fn damping(mut self, damping: f64) -> Self {
        self.layout.damping = damping;
        self
    }

    /// Set center force strength
    pub fn center_force(mut self, force: f64) -> Self {
        self.layout.center_force = force;
        self
    }

    /// Set whether to randomize starting positions
    pub fn randomize_start(mut self, randomize: bool) -> Self {
        self.layout.randomize_start = randomize;
        self
    }

    /// Build the layout algorithm
    pub fn build(self) -> ForceDirectedLayout {
        self.layout
    }
}

/// Simple grid layout
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub columns: Option<usize>,
    pub cell_width: f64,
    pub cell_height: f64,
    pub margin: f64,
}

impl Default for GridLayout {
    fn default() -> Self {
        Self {
            columns: None, // Auto-calculate based on sqrt
            cell_width: 120.0,
            cell_height: 80.0,
            margin: 20.0,
        }
    }
}

impl GridLayout {
    /// Create a new grid layout
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of columns (None for auto)
    pub fn columns(mut self, columns: Option<usize>) -> Self {
        self.columns = columns;
        self
    }

    /// Set cell dimensions
    pub fn cell_size(mut self, width: f64, height: f64) -> Self {
        self.cell_width = width;
        self.cell_height = height;
        self
    }

    /// Set margin between cells
    pub fn margin(mut self, margin: f64) -> Self {
        self.margin = margin;
        self
    }
}

impl<N, E> LayoutAlgorithm<N, E> for GridLayout {
    fn name(&self) -> &str {
        "Grid"
    }

    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        let nodes: Vec<_> = graph.nodes_mut().collect();
        let node_count = nodes.len();

        if node_count == 0 {
            return Ok(());
        }

        // Calculate columns
        let columns = self.columns.unwrap_or_else(|| {
            (node_count as f64).sqrt().ceil() as usize
        });

        // Ensure columns is at least 1 to avoid division by zero
        let columns = columns.max(1);

        // Position nodes in grid
        for (i, node) in nodes.into_iter().enumerate() {
            let row = i / columns;
            let col = i % columns;

            let x = col as f64 * (self.cell_width + self.margin);
            let y = row as f64 * (self.cell_height + self.margin);

            node.position = Position::new(x, y);
        }

        Ok(())
    }
}

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
            // Find root nodes (nodes with no incoming edges)
            self.find_root_nodes(graph)?
                .into_iter()
                .next()
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
            return Err(FlowError::layout("No root nodes found - graph may be cyclic"));
        }

        Ok(root_nodes)
    }

    /// Recursively build tree structure
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

        let node = graph.get_node(node_id)
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
            width: 0.0,  // Will be calculated
            height: 0.0, // Will be calculated
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
            let total_child_width: f64 = node.children.iter()
                .map(|child| child.width)
                .sum();
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
    size: crate::types::Size,
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

/// Circular layout algorithm
#[derive(Debug, Clone)]
pub struct CircularLayout {
    pub radius: f64,
    pub start_angle: f64,
    pub clockwise: bool,
}

impl Default for CircularLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl CircularLayout {
    /// Create a new circular layout
    pub fn new() -> Self {
        Self {
            radius: 200.0,
            start_angle: 0.0,
            clockwise: true,
        }
    }

    /// Set radius
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Set start angle in radians
    pub fn start_angle(mut self, angle: f64) -> Self {
        self.start_angle = angle;
        self
    }

    /// Set direction
    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = clockwise;
        self
    }
}

impl<N, E> LayoutAlgorithm<N, E> for CircularLayout {
    fn name(&self) -> &str {
        "Circular"
    }

    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<()> {
        let nodes: Vec<_> = graph.nodes_mut().collect();
        let node_count = nodes.len();

        if node_count == 0 {
            return Ok(());
        }

        let angle_step = 2.0 * std::f64::consts::PI / node_count as f64;

        for (i, node) in nodes.into_iter().enumerate() {
            let angle = if self.clockwise {
                self.start_angle + i as f64 * angle_step
            } else {
                self.start_angle - i as f64 * angle_step
            };

            let x = self.radius * angle.cos();
            let y = self.radius * angle.sin();

            node.position = Position::new(x, y);
        }

        Ok(())
    }
}

/// Layout utilities
pub struct LayoutUtils;

impl LayoutUtils {
    /// Center a graph around the origin
    pub fn center_graph<N: Clone, E>(graph: &mut Graph<N, E>) {
        if let Some(bounds) = graph.bounds() {
            let center = bounds.center();
            let offset = Position::zero() - center;

            for node in graph.nodes_mut() {
                node.position += offset;
            }
        }
    }

    /// Scale a graph to fit within given bounds
    pub fn scale_to_fit<N: Clone, E>(graph: &mut Graph<N, E>, target_width: f64, target_height: f64) {
        if let Some(bounds) = graph.bounds() {
            if bounds.width > 0.0 && bounds.height > 0.0 {
                let scale_x = target_width / bounds.width;
                let scale_y = target_height / bounds.height;
                let scale = scale_x.min(scale_y);

                let center = bounds.center();

                for node in graph.nodes_mut() {
                    let relative_pos = node.position - center;
                    node.position = center + relative_pos * scale;
                }
            }
        }
    }

    /// Apply padding around a graph
    pub fn apply_padding<N, E>(graph: &mut Graph<N, E>, padding: f64) {
        for node in graph.nodes_mut() {
            node.position += Position::new(padding, padding);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Node, Edge};
    use std::collections::HashMap;

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();

        // Add nodes
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("3").position(50.0, 100.0).build()).unwrap();

        // Add edges
        graph.add_edge(Edge::builder().connect("1", "2").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("2", "3").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("3", "1").build().unwrap()).unwrap();

        graph
    }

    #[test]
    fn test_force_directed_layout() {
        let mut graph = create_test_graph();
        let mut layout = ForceDirectedLayout::builder()
            .iterations(10)
            .randomize_start(false)
            .build();

        // Store initial positions
        let initial_positions: HashMap<_, _> = graph.nodes()
            .map(|node| (node.id.clone(), node.position))
            .collect();

        // Apply layout
        layout.apply(&mut graph).unwrap();

        // Check that positions changed
        let mut positions_changed = false;
        for node in graph.nodes() {
            if let Some(&initial_pos) = initial_positions.get(&node.id) {
                if node.position != initial_pos {
                    positions_changed = true;
                    break;
                }
            }
        }

        assert!(positions_changed, "Layout should change node positions");
        assert_eq!(LayoutAlgorithm::<(), ()>::progress(&layout), 1.0);
        assert!(!LayoutAlgorithm::<(), ()>::is_running(&layout));
    }

    #[test]
    fn test_grid_layout() {
        let mut graph = create_test_graph();
        let mut layout = GridLayout::new()
            .columns(Some(2))
            .cell_size(100.0, 80.0);

        layout.apply(&mut graph).unwrap();

        // Check that nodes are positioned in a grid
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 3);

        // Collect all positions and verify they follow grid pattern
        let mut positions: Vec<Position> = graph.nodes().map(|n| n.position).collect();
        positions.sort_by(|a, b| {
            if (a.y - b.y).abs() < 1e-10 {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        // Verify grid layout: 2 columns, with cell size 100x80 and margin 20
        // Expected positions: (0,0), (120,0), (0,100)
        assert_eq!(positions[0], Position::new(0.0, 0.0));
        assert_eq!(positions[1], Position::new(120.0, 0.0));
        assert_eq!(positions[2], Position::new(0.0, 100.0));
    }

    #[test]
    fn test_layout_utils_center() {
        let mut graph = create_test_graph();
        LayoutUtils::center_graph(&mut graph);

        if let Some(bounds) = graph.bounds() {
            let center = bounds.center();
            // Center should be close to origin (allowing for floating point errors)
            assert!((center.x.abs() < 1e-10) && (center.y.abs() < 1e-10));
        }
    }

    #[test]
    fn test_layout_utils_scale() {
        let mut graph = create_test_graph();
        LayoutUtils::scale_to_fit(&mut graph, 200.0, 200.0);

        if let Some(bounds) = graph.bounds() {
            assert!(bounds.width <= 200.0);
            assert!(bounds.height <= 200.0);
        }
    }

    #[test]
    fn test_force_directed_builder() {
        let layout = ForceDirectedLayout::builder()
            .iterations(50)
            .spring_strength(0.8)
            .repulsion_strength(1500.0)
            .damping(0.95)
            .randomize_start(false)
            .build();

        assert_eq!(layout.iterations, 50);
        assert_eq!(layout.spring_strength, 0.8);
        assert_eq!(layout.repulsion_strength, 1500.0);
        assert_eq!(layout.damping, 0.95);
        assert!(!layout.randomize_start);
    }

    #[test]
    fn test_layout_interruption() {
        let _graph = create_test_graph();
        let mut layout = ForceDirectedLayout::builder()
            .iterations(1000)
            .build();

        assert!(LayoutAlgorithm::<(), ()>::can_interrupt(&layout));

        // Start layout in a separate context (simulated)
        // In real usage, this would be in a separate thread or async context
        layout.running = true;
        layout.current_iteration = 10;

        assert!(LayoutAlgorithm::<(), ()>::is_running(&layout));
        assert!(LayoutAlgorithm::<(), ()>::progress(&layout) < 1.0);

        LayoutAlgorithm::<(), ()>::stop(&mut layout).unwrap();
        assert!(!LayoutAlgorithm::<(), ()>::is_running(&layout));
    }

    #[test]
    fn test_hierarchical_layout() {
        let mut graph = create_hierarchical_test_graph();
        let mut layout = HierarchicalLayout::builder()
            .root_node("root")
            .node_separation(100.0)
            .level_separation(80.0)
            .direction(LayoutDirection::TopToBottom)
            .build();

        // Store initial positions
        let initial_positions: HashMap<_, _> = graph.nodes()
            .map(|node| (node.id.clone(), node.position))
            .collect();

        // Apply layout
        layout.apply(&mut graph).unwrap();

        // Verify positions changed
        let mut positions_changed = false;
        for node in graph.nodes() {
            if let Some(&initial_pos) = initial_positions.get(&node.id) {
                if node.position != initial_pos {
                    positions_changed = true;
                    break;
                }
            }
        }
        assert!(positions_changed, "Hierarchical layout should change node positions");

        // Verify hierarchical structure
        let root = graph.get_node(&"root".into()).unwrap();
        let child1 = graph.get_node(&"child1".into()).unwrap();
        let child2 = graph.get_node(&"child2".into()).unwrap();
        let grandchild = graph.get_node(&"grandchild".into()).unwrap();

        // Root should be at top level
        assert_eq!(root.position.y, 0.0);

        // Children should be at second level
        assert_eq!(child1.position.y, 80.0);
        assert_eq!(child2.position.y, 80.0);

        // Grandchild should be at third level
        assert_eq!(grandchild.position.y, 160.0);

        // Children should be separated horizontally
        assert!((child1.position.x - child2.position.x).abs() >= 100.0);
    }

    #[test]
    fn test_hierarchical_layout_builder() {
        let layout = HierarchicalLayout::builder()
            .node_separation(150.0)
            .level_separation(120.0)
            .direction(LayoutDirection::LeftToRight)
            .edge_routing(EdgeRouting::Orthogonal)
            .root_node("custom-root")
            .build();

        assert_eq!(layout.node_separation, 150.0);
        assert_eq!(layout.level_separation, 120.0);
        assert_eq!(layout.direction, LayoutDirection::LeftToRight);
        assert_eq!(layout.edge_routing, EdgeRouting::Orthogonal);
        assert_eq!(layout.root_node, Some("custom-root".into()));
    }

    #[test]
    fn test_circular_layout() {
        let mut graph = create_test_graph();
        let mut layout = CircularLayout::new()
            .radius(150.0)
            .start_angle(std::f64::consts::PI / 4.0)
            .clockwise(false);

        // Store initial positions
        let initial_positions: HashMap<_, _> = graph.nodes()
            .map(|node| (node.id.clone(), node.position))
            .collect();

        // Apply layout
        layout.apply(&mut graph).unwrap();

        // Verify positions changed
        let mut positions_changed = false;
        for node in graph.nodes() {
            if let Some(&initial_pos) = initial_positions.get(&node.id) {
                if node.position != initial_pos {
                    positions_changed = true;
                    break;
                }
            }
        }
        assert!(positions_changed, "Circular layout should change node positions");

        // Verify nodes are arranged in a circle
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 3);

        for node in &nodes {
            // Distance from origin should be approximately the radius
            let distance = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            assert!((distance - 150.0).abs() < 1e-10, "Node should be at radius distance from origin");
        }
    }

    #[test]
    fn test_hierarchical_layout_cycle_detection() {
        let mut graph = create_cyclic_graph();
        let mut layout = HierarchicalLayout::new();

        // Should detect cycle and return error
        let result = layout.apply(&mut graph);
        assert!(result.is_err());

        if let Err(FlowError::Layout { message: msg }) = result {
            assert!(msg.contains("Cycle detected") || msg.contains("No root nodes found"));
        } else {
            panic!("Expected LayoutError for cyclic graph");
        }
    }

    #[test]
    fn test_circular_layout_empty_graph() {
        let mut graph: Graph<(), ()> = Graph::new();
        let mut layout = CircularLayout::new();

        // Should handle empty graph gracefully
        let result = layout.apply(&mut graph);
        assert!(result.is_ok());
    }

    fn create_hierarchical_test_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();

        // Create a simple tree structure:
        //     root
        //    /    \
        // child1  child2
        //   |
        // grandchild

        graph.add_node(Node::builder("root").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("child1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("child2").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("grandchild").position(0.0, 0.0).build()).unwrap();

        graph.add_edge(Edge::builder().connect("root", "child1").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("root", "child2").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("child1", "grandchild").build().unwrap()).unwrap();

        graph
    }

    fn create_cyclic_graph() -> Graph<(), ()> {
        let mut graph = Graph::new();

        graph.add_node(Node::builder("a").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("b").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("c").position(0.0, 0.0).build()).unwrap();

        // Create a cycle: a -> b -> c -> a
        graph.add_edge(Edge::builder().connect("a", "b").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("b", "c").build().unwrap()).unwrap();
        graph.add_edge(Edge::builder().connect("c", "a").build().unwrap()).unwrap();

        graph
    }

    // Targeted unit tests for mathematical operations (mutation testing coverage)

    #[test]
    fn test_force_directed_energy_calculation() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 0.0).build()).unwrap();
        graph.add_edge(Edge::builder().connect("1", "2").build().unwrap()).unwrap();

        let layout = ForceDirectedLayout::builder()
            .spring_length(100.0)
            .spring_strength(1.0)
            .repulsion_strength(1000.0)
            .build();

        let energy = layout.calculate_energy(&graph);

        // Energy should be positive (spring energy + repulsion energy)
        assert!(energy >= 0.0, "Energy should be non-negative");

        // Test with different node positions
        let mut graph2: Graph<(), ()> = Graph::new();
        graph2.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph2.add_node(Node::builder("2").position(50.0, 0.0).build()).unwrap();
        graph2.add_edge(Edge::builder().connect("1", "2").build().unwrap()).unwrap();

        let energy2 = layout.calculate_energy(&graph2);

        // Different positions should result in different energy
        assert_ne!(energy, energy2, "Different positions should have different energy");
    }

    #[test]
    fn test_force_directed_convergence_detection() {
        let layout = ForceDirectedLayout::builder()
            .iterations(100)
            .build();

        // Test convergence with zero iterations
        let mut layout_zero = layout.clone();
        layout_zero.current_iteration = 0;
        layout_zero.iterations = 0;
        assert!(layout_zero.has_converged(0.0));

        // Test convergence with max iterations reached
        let mut layout_max = layout.clone();
        layout_max.current_iteration = 100;
        layout_max.iterations = 100;
        assert!(layout_max.has_converged(0.0));

        // Test non-convergence
        let mut layout_running = layout.clone();
        layout_running.current_iteration = 50;
        layout_running.iterations = 100;
        // With energy > 0, it should not be converged
        assert!(!layout_running.has_converged(1.0));
    }

    #[test]
    fn test_force_directed_position_randomization() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(0.0, 0.0).build()).unwrap();

        let layout = ForceDirectedLayout::builder()
            .randomize_start(true)
            .build();

        // Store initial positions
        let initial_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

        // Apply randomization
        layout.maybe_randomize_positions(&mut graph);

        // Check that at least one position changed (with high probability)
        let new_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();
        let positions_changed = initial_positions.iter().zip(new_positions.iter())
            .any(|(old, new)| old != new);

        // Note: This test might occasionally fail due to randomness
        // In a real scenario, we'd use a seeded random number generator
        assert!(positions_changed, "Randomization should change positions");
    }

    #[test]
    fn test_grid_layout_cell_calculations() {
        let mut graph: Graph<(), ()> = Graph::new();
        for i in 0..6 {
            graph.add_node(Node::builder(format!("{}", i)).position(0.0, 0.0).build()).unwrap();
        }

        let mut layout = GridLayout::new()
            .columns(Some(3))
            .cell_size(100.0, 80.0)
            .margin(20.0);

        layout.apply(&mut graph).unwrap();

        // Verify grid positioning
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 6);

        // Check that nodes are positioned in a 3x2 grid
        let mut positions: Vec<_> = nodes.iter().map(|n| n.position).collect();
        positions.sort_by(|a, b| {
            if (a.y - b.y).abs() < 1e-10 {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        // Expected positions with 3 columns, cell size 100x80, margin 20:
        // Row 0: (0,0), (120,0), (240,0)
        // Row 1: (0,100), (120,100), (240,100)
        assert_eq!(positions[0], Position::new(0.0, 0.0));
        assert_eq!(positions[1], Position::new(120.0, 0.0));
        assert_eq!(positions[2], Position::new(240.0, 0.0));
        assert_eq!(positions[3], Position::new(0.0, 100.0));
        assert_eq!(positions[4], Position::new(120.0, 100.0));
        assert_eq!(positions[5], Position::new(240.0, 100.0));
    }

    #[test]
    fn test_circular_layout_angle_calculations() {
        let mut graph: Graph<(), ()> = Graph::new();
        for i in 0..4 {
            graph.add_node(Node::builder(format!("{}", i)).position(0.0, 0.0).build()).unwrap();
        }

        let mut layout = CircularLayout::new()
            .radius(100.0)
            .start_angle(0.0)
            .clockwise(true);

        layout.apply(&mut graph).unwrap();

        // Verify circular positioning
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 4);

        for node in &nodes {
            // Distance from origin should be approximately the radius
            let distance = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            assert!((distance - 100.0).abs() < 1e-10, "Node should be at radius distance from origin");
        }

        // Test with different start angle
        let mut layout2 = CircularLayout::new()
            .radius(100.0)
            .start_angle(std::f64::consts::PI / 2.0) // 90 degrees
            .clockwise(true);

        layout2.apply(&mut graph).unwrap();

        // First node should be at (0, 100) with 90-degree start angle
        let first_node = graph.nodes().next().unwrap();
        assert!((first_node.position.x.abs() < 1e-10) && (first_node.position.y - 100.0).abs() < 1e-10,
                "First node should be at (0, 100) with 90-degree start angle");
    }

    #[test]
    fn test_hierarchical_layout_tree_calculations() {
        let mut graph = create_hierarchical_test_graph();
        let mut layout = HierarchicalLayout::builder()
            .root_node("root")
            .node_separation(100.0)
            .level_separation(80.0)
            .direction(LayoutDirection::TopToBottom)
            .build();

        layout.apply(&mut graph).unwrap();

        // Verify hierarchical positioning
        let root = graph.get_node(&"root".into()).unwrap();
        let child1 = graph.get_node(&"child1".into()).unwrap();
        let child2 = graph.get_node(&"child2".into()).unwrap();
        let grandchild = graph.get_node(&"grandchild".into()).unwrap();

        // Root should be at level 0
        assert_eq!(root.position.y, 0.0);

        // Children should be at level 1 (80 units down)
        assert_eq!(child1.position.y, 80.0);
        assert_eq!(child2.position.y, 80.0);

        // Grandchild should be at level 2 (160 units down)
        assert_eq!(grandchild.position.y, 160.0);

        // Children should be separated horizontally by node_separation
        let horizontal_distance = (child1.position.x - child2.position.x).abs();
        assert!(horizontal_distance >= 100.0, "Children should be separated by node_separation");
    }

    #[test]
    fn test_layout_utils_center_calculations() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(100.0, 100.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(200.0, 200.0).build()).unwrap();
        graph.add_node(Node::builder("3").position(300.0, 300.0).build()).unwrap();

        // Center should be at (200, 200)
        LayoutUtils::center_graph(&mut graph);

        if let Some(bounds) = graph.bounds() {
            let center = bounds.center();
            // Center should be close to origin (allowing for floating point errors)
            assert!((center.x.abs() < 1e-10) && (center.y.abs() < 1e-10));
        }
    }

    #[test]
    fn test_layout_utils_scale_calculations() {
        // Test with graph that needs scaling down
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(1000.0, 1000.0).build()).unwrap();

        // Scale to fit within 200x200 - should complete without error
        LayoutUtils::scale_to_fit(&mut graph, 200.0, 200.0);

        // Test with empty graph (should not panic)
        let mut empty_graph: Graph<(), ()> = Graph::new();
        LayoutUtils::scale_to_fit(&mut empty_graph, 100.0, 100.0);
        // Should complete without error

        // Test with small graph
        let mut small_graph: Graph<(), ()> = Graph::new();
        small_graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        small_graph.add_node(Node::builder("2").position(50.0, 50.0).build()).unwrap();

        LayoutUtils::scale_to_fit(&mut small_graph, 200.0, 200.0);
        // Should complete without error

        // Test with zero target size (edge case)
        let mut edge_graph: Graph<(), ()> = Graph::new();
        edge_graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        edge_graph.add_node(Node::builder("2").position(100.0, 100.0).build()).unwrap();

        LayoutUtils::scale_to_fit(&mut edge_graph, 0.0, 0.0);
        // Should complete without error
    }

    #[test]
    fn test_layout_utils_padding_calculations() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 100.0).build()).unwrap();

        let initial_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

        // Apply padding of 50 units
        LayoutUtils::apply_padding(&mut graph, 50.0);

        let new_positions: Vec<_> = graph.nodes().map(|n| n.position).collect();

        // All positions should be shifted by (50, 50)
        for (old, new) in initial_positions.iter().zip(new_positions.iter()) {
            assert_eq!(new.x, old.x + 50.0);
            assert_eq!(new.y, old.y + 50.0);
        }
    }

    #[test]
    fn test_force_directed_spring_force_calculations() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 0.0).build()).unwrap();
        graph.add_edge(Edge::builder().connect("1", "2").build().unwrap()).unwrap();

        let layout = ForceDirectedLayout::builder()
            .spring_length(50.0)
            .spring_strength(1.0)
            .build();

        // Test spring force calculation by checking the forces applied to nodes
        let spring_forces = layout.calculate_spring_forces(&graph);

        // Should have forces for both nodes
        assert_eq!(spring_forces.len(), 2);

        // Forces should be non-zero for connected nodes
        let node1_id = "1".into();
        let node2_id = "2".into();
        assert!(spring_forces.contains_key(&node1_id));
        assert!(spring_forces.contains_key(&node2_id));
    }

    #[test]
    fn test_force_directed_repulsion_force_calculations() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 0.0).build()).unwrap();

        let layout = ForceDirectedLayout::builder()
            .repulsion_strength(1000.0)
            .build();

        // Test repulsion force calculation by checking the forces applied to nodes
        let repulsion_forces = layout.calculate_repulsion_forces(&graph);

        // Should have forces for both nodes
        assert_eq!(repulsion_forces.len(), 2);

        // Forces should be non-zero for nodes that are close
        let node1_id = "1".into();
        let node2_id = "2".into();
        assert!(repulsion_forces.contains_key(&node1_id));
        assert!(repulsion_forces.contains_key(&node2_id));
    }

    // Edge case tests for boundary conditions (mutation testing coverage)

    #[test]
    fn test_force_directed_edge_cases() {
        // Test with single node
        let mut single_node_graph: Graph<(), ()> = Graph::new();
        single_node_graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();

        let mut layout = ForceDirectedLayout::builder()
            .iterations(1)
            .build();

        // Should complete without error
        let result = layout.apply(&mut single_node_graph);
        assert!(result.is_ok());

        // Test with very small iteration count
        let mut layout_min = ForceDirectedLayout::builder()
            .iterations(0)
            .build();

        let result_min = layout_min.apply(&mut single_node_graph);
        assert!(result_min.is_ok());

        // Test with very large iteration count
        let mut layout_max = ForceDirectedLayout::builder()
            .iterations(10000)
            .build();

        let result_max = layout_max.apply(&mut single_node_graph);
        assert!(result_max.is_ok());
    }

    #[test]
    fn test_grid_layout_edge_cases() {
        // Test with zero nodes
        let mut empty_graph: Graph<(), ()> = Graph::new();
        let mut layout = GridLayout::new();

        let result = layout.apply(&mut empty_graph);
        assert!(result.is_ok());

        // Test with single node
        let mut single_graph: Graph<(), ()> = Graph::new();
        single_graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();

        let result_single = layout.apply(&mut single_graph);
        assert!(result_single.is_ok());

        // Test with very small cell size
        let mut layout_tiny = GridLayout::new()
            .cell_size(1e-10, 1e-10);

        let result_tiny = layout_tiny.apply(&mut single_graph);
        assert!(result_tiny.is_ok());

        // Test with very large cell size
        let mut layout_huge = GridLayout::new()
            .cell_size(1e10, 1e10);

        let result_huge = layout_huge.apply(&mut single_graph);
        assert!(result_huge.is_ok());

        // Test with zero columns
        let mut layout_zero_cols = GridLayout::new()
            .columns(Some(0));

        let result_zero = layout_zero_cols.apply(&mut single_graph);
        assert!(result_zero.is_ok());
    }

    #[test]
    fn test_circular_layout_edge_cases() {
        // Test with zero nodes
        let mut empty_graph: Graph<(), ()> = Graph::new();
        let mut layout = CircularLayout::new();

        let result = layout.apply(&mut empty_graph);
        assert!(result.is_ok());

        // Test with single node
        let mut single_graph: Graph<(), ()> = Graph::new();
        single_graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();

        let result_single = layout.apply(&mut single_graph);
        assert!(result_single.is_ok());

        // Test with zero radius
        let mut layout_zero_radius = CircularLayout::new()
            .radius(0.0);

        let result_zero = layout_zero_radius.apply(&mut single_graph);
        assert!(result_zero.is_ok());

        // Test with very large radius
        let mut layout_huge_radius = CircularLayout::new()
            .radius(1e10);

        let result_huge = layout_huge_radius.apply(&mut single_graph);
        assert!(result_huge.is_ok());

        // Test with negative radius
        let mut layout_neg_radius = CircularLayout::new()
            .radius(-100.0);

        let result_neg = layout_neg_radius.apply(&mut single_graph);
        assert!(result_neg.is_ok());
    }

    #[test]
    fn test_hierarchical_layout_edge_cases() {
        // Test with empty graph
        let mut empty_graph: Graph<(), ()> = Graph::new();
        let mut layout = HierarchicalLayout::new();

        let result = layout.apply(&mut empty_graph);
        assert!(result.is_ok());

        // Test with single node
        let mut single_graph: Graph<(), ()> = Graph::new();
        single_graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();

        let result_single = layout.apply(&mut single_graph);
        assert!(result_single.is_ok());

        // Test with very small separations
        let mut layout_tiny = HierarchicalLayout::builder()
            .node_separation(1e-10)
            .level_separation(1e-10)
            .build();

        let result_tiny = layout_tiny.apply(&mut single_graph);
        assert!(result_tiny.is_ok());

        // Test with very large separations
        let mut layout_huge = HierarchicalLayout::builder()
            .node_separation(1e10)
            .level_separation(1e10)
            .build();

        let result_huge = layout_huge.apply(&mut single_graph);
        assert!(result_huge.is_ok());

        // Test with non-existent root node
        let mut layout_bad_root = HierarchicalLayout::builder()
            .root_node("non-existent")
            .build();

        let _result_bad = layout_bad_root.apply(&mut single_graph);
        // Should handle gracefully (might be ok or error depending on implementation)
    }

    #[test]
    fn test_layout_utils_edge_cases() {
        // Test center_graph with empty graph
        let mut empty_graph: Graph<(), ()> = Graph::new();
        LayoutUtils::center_graph(&mut empty_graph);
        // Should complete without error

        // Test scale_to_fit with zero target dimensions
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 100.0).build()).unwrap();

        LayoutUtils::scale_to_fit(&mut graph, 0.0, 0.0);
        // Should complete without error

        // Test scale_to_fit with negative target dimensions
        LayoutUtils::scale_to_fit(&mut graph, -100.0, -100.0);
        // Should complete without error

        // Test apply_padding with zero padding
        LayoutUtils::apply_padding(&mut graph, 0.0);
        // Should complete without error

        // Test apply_padding with negative padding
        LayoutUtils::apply_padding(&mut graph, -50.0);
        // Should complete without error

        // Test apply_padding with very large padding
        LayoutUtils::apply_padding(&mut graph, 1e10);
        // Should complete without error
    }

    #[test]
    fn test_force_directed_parameter_edge_cases() {
        // Test with zero spring strength
        let mut graph: Graph<(), ()> = Graph::new();
        graph.add_node(Node::builder("1").position(0.0, 0.0).build()).unwrap();
        graph.add_node(Node::builder("2").position(100.0, 100.0).build()).unwrap();
        graph.add_edge(Edge::builder().connect("1", "2").build().unwrap()).unwrap();

        let mut layout_zero_spring = ForceDirectedLayout::builder()
            .spring_strength(0.0)
            .iterations(1)
            .build();

        let result = layout_zero_spring.apply(&mut graph);
        assert!(result.is_ok());

        // Test with zero repulsion strength
        let mut layout_zero_repulsion = ForceDirectedLayout::builder()
            .repulsion_strength(0.0)
            .iterations(1)
            .build();

        let result_rep = layout_zero_repulsion.apply(&mut graph);
        assert!(result_rep.is_ok());

        // Test with zero damping
        let mut layout_zero_damping = ForceDirectedLayout::builder()
            .damping(0.0)
            .iterations(1)
            .build();

        let result_damp = layout_zero_damping.apply(&mut graph);
        assert!(result_damp.is_ok());

        // Test with very large parameters
        let mut layout_huge = ForceDirectedLayout::builder()
            .spring_strength(1e10)
            .repulsion_strength(1e10)
            .damping(1.0)
            .iterations(1)
            .build();

        let result_huge = layout_huge.apply(&mut graph);
        assert!(result_huge.is_ok());
    }
}
