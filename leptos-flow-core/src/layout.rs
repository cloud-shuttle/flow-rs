//! Layout algorithms for automatic node positioning

use std::collections::HashMap;

use crate::error::{FlowError, Result};
use crate::graph::{Graph, Node, Edge};
use crate::types::{Position, NodeId, EdgeId};

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
                    .or_insert(Position::zero());

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
pub struct ForceDirectedLayoutBuilder {
    layout: ForceDirectedLayout,
}

impl Default for ForceDirectedLayoutBuilder {
    fn default() -> Self {
        Self {
            layout: ForceDirectedLayout::new(),
        }
    }
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
        assert_eq!(layout.progress(), 1.0);
        assert!(!layout.is_running());
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

        // First node should be at origin
        let node1 = graph.get_node(&"1".into()).unwrap();
        assert_eq!(node1.position, Position::new(0.0, 0.0));

        // Second node should be in next column
        let node2 = graph.get_node(&"2".into()).unwrap();
        assert_eq!(node2.position, Position::new(120.0, 0.0)); // 100 + 20 margin

        // Third node should be in second row
        let node3 = graph.get_node(&"3".into()).unwrap();
        assert_eq!(node3.position, Position::new(0.0, 100.0)); // 80 + 20 margin
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
        let mut graph = create_test_graph();
        let mut layout = ForceDirectedLayout::builder()
            .iterations(1000)
            .build();

        assert!(layout.can_interrupt());

        // Start layout in a separate context (simulated)
        // In real usage, this would be in a separate thread or async context
        layout.running = true;
        layout.current_iteration = 10;

        assert!(layout.is_running());
        assert!(layout.progress() < 1.0);

        layout.stop().unwrap();
        assert!(!layout.is_running());
    }
}
