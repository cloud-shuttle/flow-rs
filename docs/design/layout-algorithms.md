# Layout Algorithms Design

## Purpose
Provides automated node positioning algorithms for Flow-RS graphs. Transforms logical graph structures into visually meaningful 2D layouts with configurable spacing, clustering, and aesthetic properties.

## Public API

### Core Trait
```rust
pub trait LayoutAlgorithm {
    fn apply(&mut self, graph: &mut Graph) -> Result<(), FlowError>;
    fn name(&self) -> &'static str;
    fn supports_incremental(&self) -> bool { false }
}
```

### Algorithm Implementations
```rust
pub struct CircularLayout {
    pub center: Position,
    pub radius: f64,
    pub start_angle: f64,
}

pub struct ForceDirectedLayout {
    pub iterations: usize,
    pub spring_strength: f64,
    pub repulsion_strength: f64,
    pub damping: f64,
}

pub struct GridLayout {
    pub cell_size: Size,
    pub spacing: f64,
    pub alignment: GridAlignment,
}

pub struct HierarchicalLayout {
    pub level_separation: f64,
    pub node_separation: f64,
    pub direction: LayoutDirection,
}
```

## Internal Architecture

### Algorithm Classification

#### Static Algorithms
- **CircularLayout**: Arranges nodes in concentric circles
- **GridLayout**: Places nodes in a regular grid pattern
- **HierarchicalLayout**: Organizes nodes by dependency levels

#### Dynamic Algorithms
- **ForceDirectedLayout**: Physics simulation with springs and repulsion
- **SpringEmbedder**: Variant of force-directed with different physics

### Implementation Strategy

#### Phase 1: Basic Layouts (Week 1)
```rust
impl LayoutAlgorithm for CircularLayout {
    fn apply(&mut self, graph: &mut Graph) -> Result<(), FlowError> {
        let nodes: Vec<_> = graph.nodes().collect();
        let angle_step = 2.0 * std::f64::consts::PI / nodes.len() as f64;

        for (i, node) in nodes.iter().enumerate() {
            let angle = self.start_angle + (i as f64 * angle_step);
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();

            graph.update_node_position(node.id(), Position::new(x, y))?;
        }
        Ok(())
    }
}
```

#### Phase 2: Force-Directed Layout (Week 2-3)
```rust
impl LayoutAlgorithm for ForceDirectedLayout {
    fn apply(&mut self, graph: &mut Graph) -> Result<(), FlowError> {
        let mut forces = HashMap::new();

        for _ in 0..self.iterations {
            forces.clear();

            // Calculate repulsion forces between all node pairs
            for node_a in graph.nodes() {
                for node_b in graph.nodes() {
                    if node_a.id() != node_b.id() {
                        let force = self.calculate_repulsion(node_a, node_b);
                        forces.entry(node_a.id()).or_insert(Position::ZERO).add(&force);
                    }
                }
            }

            // Calculate attraction forces along edges
            for edge in graph.edges() {
                let source = graph.node(edge.source()).unwrap();
                let target = graph.node(edge.target()).unwrap();
                let force = self.calculate_attraction(source, target);

                forces.entry(edge.source()).or_insert(Position::ZERO).add(&force);
                forces.entry(edge.target()).or_insert(Position::ZERO).subtract(&force);
            }

            // Apply forces with damping
            for (node_id, force) in forces.iter() {
                let current_pos = graph.node(*node_id).unwrap().position();
                let new_pos = current_pos + (*force * self.damping);
                graph.update_node_position(*node_id, new_pos)?;
            }
        }
        Ok(())
    }
}
```

### Performance Optimizations

#### Spatial Acceleration
- Use spatial indexing to limit force calculations to nearby nodes
- Implement Barnes-Hut algorithm for O(n log n) force computation
- Cache distance calculations between iterations

#### Convergence Detection
```rust
fn has_converged(&self, old_positions: &HashMap<NodeId, Position>,
                 new_positions: &HashMap<NodeId, Position>) -> bool {
    let threshold = 1.0; // pixels
    old_positions.iter().all(|(id, old_pos)| {
        new_positions.get(id)
            .map(|new_pos| old_pos.distance_to(new_pos) < threshold)
            .unwrap_or(false)
    })
}
```

## Dependencies

### Core Dependencies
- `flow-core::Graph` - Graph data structure
- `flow-core::types` - Position, Size, NodeId types
- `nalgebra` - Vector mathematics for force calculations

### Optional Dependencies
- `rayon` - Parallel force calculation (feature flag)
- `spade` - Spatial data structures for acceleration

## Performance Characteristics

### Time Complexity
| Algorithm | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| Circular | O(n) | O(1) | Trivial calculation |
| Grid | O(n) | O(n) | Collision detection required |
| Hierarchical | O(n + e) | O(n) | Topological sort + positioning |
| Force-Directed | O(i × n²) | O(n) | i = iterations, n = nodes |

### Performance Targets
- **Small graphs (n < 100)**: <10ms for any algorithm
- **Medium graphs (n < 1000)**: <100ms for force-directed
- **Large graphs (n < 5000)**: <500ms with spatial optimization

### Memory Usage
- Minimize temporary allocations during layout
- Reuse force vectors between iterations
- Stream processing for very large graphs

## Testing Strategy

### Unit Tests
```rust
#[test]
fn circular_layout_positions_nodes_on_circumference() {
    let mut graph = create_test_graph(8);
    let mut layout = CircularLayout {
        center: Position::new(200.0, 200.0),
        radius: 100.0,
        start_angle: 0.0
    };

    layout.apply(&mut graph).unwrap();

    // Verify each node is exactly radius distance from center
    for node in graph.nodes() {
        let distance = node.position().distance_to(&layout.center);
        assert!((distance - layout.radius).abs() < 1.0);
    }
}
```

### Property-Based Tests
```rust
proptest! {
    #[test]
    fn force_directed_converges_for_any_graph(
        nodes in 3..50usize,
        edges in 0..100usize
    ) {
        let graph = generate_random_graph(nodes, edges);
        let mut layout = ForceDirectedLayout::default();

        let initial_energy = calculate_system_energy(&graph);
        layout.apply(&mut graph).unwrap();
        let final_energy = calculate_system_energy(&graph);

        // Energy should decrease or remain stable
        prop_assert!(final_energy <= initial_energy);
    }
}
```

### Integration Tests
```rust
#[test]
fn layouts_work_with_real_world_graphs() {
    let graph = load_graph_from_file("test_data/medium_graph.json");

    for algorithm in [CircularLayout::default(), GridLayout::default()] {
        let mut test_graph = graph.clone();
        algorithm.apply(&mut test_graph).unwrap();

        // Verify no nodes overlap
        assert_no_overlapping_nodes(&test_graph);

        // Verify reasonable bounds
        let bounds = calculate_bounding_box(&test_graph);
        assert!(bounds.width() > 0.0 && bounds.height() > 0.0);
    }
}
```

## Current Implementation Status

### ✅ Complete
- CircularLayout trait definition
- GridLayout struct and configuration
- Basic performance benchmarks

### 🚧 In Progress
- ForceDirectedLayout physics implementation
- Spatial acceleration structures
- Convergence detection

### ❓ Missing
- HierarchicalLayout topological sorting
- Parallel force computation
- Interactive layout (user constraints)

## Future Considerations

### Advanced Algorithms
- **Stress Majorization**: Better initial positioning
- **Multi-level Graph Layout**: Hierarchical coarsening for large graphs
- **Constrained Layout**: User-specified position constraints
- **Animated Transitions**: Smooth interpolation between layouts

### Interactive Features
```rust
pub trait InteractiveLayout: LayoutAlgorithm {
    fn add_constraint(&mut self, node: NodeId, position: Position);
    fn remove_constraint(&mut self, node: NodeId);
    fn apply_incremental(&mut self, graph: &mut Graph, changed_nodes: &[NodeId]) -> Result<(), FlowError>;
}
```

### Performance Enhancements
- GPU-accelerated force computation using WebGPU compute shaders
- Distributed layout computation for massive graphs
- Incremental layout updates for real-time editing

## Migration Path

### From Current Stub Implementation
1. **Week 1**: Implement CircularLayout and GridLayout with full positioning
2. **Week 2-3**: Complete ForceDirectedLayout with basic physics
3. **Week 4**: Add HierarchicalLayout and performance optimizations
4. **Week 5**: Integration testing and documentation

### Breaking Changes
- Layout trait signature may change to support constraints
- Configuration structs will gain additional fields
- Error types may be refined for specific layout failures

This design enables both simple positioning for basic use cases and sophisticated physics simulation for advanced applications, while maintaining performance targets for production deployment.
