//! Force-directed layout algorithm implementation

use std::collections::HashMap;

use crate::error::Result;
use crate::graph::Graph;
use crate::layout::LayoutAlgorithm;
use crate::types::{NodeId, Position};

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
    #[cfg(test)]
    pub current_iteration: usize,
    #[cfg(not(test))]
    current_iteration: usize,
    #[cfg(test)]
    pub running: bool,
    #[cfg(not(test))]
    running: bool,
    node_velocities: HashMap<NodeId, Position>,
    #[allow(dead_code)]
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
            if let (Some(source), Some(target)) =
                (graph.get_node(&edge.source), graph.get_node(&edge.target))
            {
                let delta = target.position - source.position;
                let distance = delta.x.hypot(delta.y).max(0.1);
                let displacement = distance - self.spring_length;
                let force_magnitude = self.spring_strength * displacement;

                let force_direction = Position::new(delta.x / distance, delta.y / distance);
                let force = force_direction * force_magnitude;

                // Apply force to both nodes (opposite directions)
                *forces
                    .entry(edge.source.clone())
                    .or_insert(Position::zero()) += force;
                *forces
                    .entry(edge.target.clone())
                    .or_insert(Position::zero()) -= force;
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

                    if distance_squared > 0.01 {
                        // Avoid division by zero
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
                let velocity = self.node_velocities.entry(node.id.clone()).or_default();

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
    pub fn calculate_energy<N, E>(&self, graph: &Graph<N, E>) -> f64 {
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
#[derive(Debug, Clone, Default)]
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
