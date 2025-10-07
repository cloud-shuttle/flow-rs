# Advanced Layouts Example

Comprehensive demonstration of sophisticated layout algorithms and advanced graph layout techniques in Flow-RS. This example showcases constrained layouts, performance optimization, custom algorithms, and interactive layout controls that go beyond basic force-directed and grid layouts.

## What it demonstrates

- **Constrained force-directed layouts**: Pinning, grouping, and spatial constraints
- **Custom layout algorithms**: Circular, radial, layered, spiral, and constrained grid
- **Layout performance optimization**: Benchmarking and performance analysis
- **Interactive constraint management**: Real-time constraint addition and modification
- **Layout algorithm comparison**: Side-by-side performance and quality analysis
- **Advanced layout constraints**: Alignment rules, spacing rules, boundary constraints

## Core Layout Algorithms

### **Constrained Force-Directed Layout**
```rust
// Advanced force-directed with constraints
struct LayoutConstraints {
    pub pinned_nodes: Vec<String>,      // Nodes that cannot move
    pub grouped_nodes: Vec<Vec<String>>, // Nodes that must stay together
    pub alignment_rules: Vec<AlignmentRule>,
    pub spacing_rules: Vec<SpacingRule>,
    pub boundary_constraints: Vec<BoundaryConstraint>,
}
```

**Features:**
- **Pinning**: Lock specific nodes in position
- **Grouping**: Keep related nodes together
- **Alignment**: Horizontal/vertical alignment rules
- **Spacing**: Minimum/maximum distance constraints
- **Boundaries**: Confine nodes to specific regions

### **Circular Layout Algorithm**
```rust
// Perfect circle arrangement
fn apply_circular_layout(graph: &mut Graph<String, String>) {
    let angle_step = 2.0 * PI / node_count as f64;
    for (i, node) in graph.nodes_mut().enumerate() {
        let angle = i as f64 * angle_step;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        node.position = Position::new(x, y);
    }
}
```

**Best for:** Symmetric structures, ring topologies, equal-weight nodes

### **Radial Layout Algorithm**
```rust
// Hierarchical radial arrangement
fn apply_radial_layout(graph: &mut Graph<String, String>) {
    let center_node = find_center_node(graph);
    // Position center node at origin
    // Arrange other nodes radially based on hierarchy
}
```

**Best for:** Tree structures, hierarchical data, network topologies

### **Layered Layout Algorithm**
```rust
// Hierarchical layered arrangement
fn apply_layered_layout(graph: &mut Graph<String, String>) {
    let layers = assign_layers(graph); // Topological sort
    for (layer_index, layer) in layers.iter().enumerate() {
        let y = layer_index as f64 * layer_height;
        // Position nodes horizontally within layer
    }
}
```

**Best for:** Flowcharts, process diagrams, dependency graphs

### **Custom Spiral Layout**
```rust
// Artistic spiral arrangement
fn apply_spiral_layout(graph: &mut Graph<String, String>) {
    let mut angle = 0.0;
    let mut radius = initial_radius;
    for node in graph.nodes_mut() {
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        node.position = Position::new(x, y);
        angle += angle_step;
        radius += radius_step;
    }
}
```

**Best for:** Visual appeal, artistic layouts, space-filling arrangements

### **Constrained Grid Layout**
```rust
// Grid with constraint awareness
fn apply_constrained_grid_layout(graph: &mut Graph<String, String>) {
    let grid_positions = generate_grid_positions(node_count);
    // Assign positions respecting pinned nodes and groups
}
```

**Best for:** Structured layouts, tabular data, constraint-heavy graphs

## Constraint System

### **Node Pinning**
```javascript
// Pin a node to prevent movement
flow_layouts.add_constraint('pin', 'node-1');

// Unpin to allow movement
flow_layouts.add_constraint('unpin', 'node-1');
```

### **Node Grouping**
```javascript
// Create a group of related nodes
flow_layouts.create_group('node-a,node-b,node-c');
```

### **Advanced Constraints**
```rust
// Alignment constraints
let alignment = AlignmentRule {
    nodes: vec!["node-1".to_string(), "node-2".to_string()],
    axis: Axis::Horizontal,
    alignment: AlignmentType::Center,
};

// Spacing constraints
let spacing = SpacingRule {
    node_pairs: vec![("node-1".to_string(), "node-2".to_string())],
    min_distance: 100.0,
    max_distance: Some(200.0),
};

// Boundary constraints
let boundary = BoundaryConstraint {
    nodes: vec!["node-1".to_string()],
    bounds: Rectangle {
        x: 100.0, y: 100.0,
        width: 300.0, height: 200.0
    },
};
```

## Performance Analysis

### **Benchmarking System**
```javascript
// Run performance benchmark
const result = flow_layouts.benchmark_layout('constrained_force_directed', 1000);
console.log(result); // "Benchmark complete: 1000 iterations, average 2.34ms per layout"
```

### **Performance Metrics**
```rust
#[derive(Debug)]
struct LayoutPerformance {
    pub algorithm: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub computation_time_ms: f64,
    pub iterations: u32,
    pub convergence_achieved: bool,
    pub final_energy: f64,
}
```

### **Comparative Analysis**
```javascript
// Compare all layout algorithms
const comparison = flow_layouts.compare_layouts();
console.log(comparison);
// 📊 Layout Performance Comparison:
// 🎯 constrained_force_directed: 15.23ms (14 nodes, 24 edges)
// 🎯 circular: 2.45ms (14 nodes, 24 edges)
// 🎯 radial: 3.12ms (14 nodes, 24 edges)
```

## Interactive Features

### **Real-time Layout Application**
```javascript
// Apply layout with immediate visual feedback
document.getElementById('run-constrained-force').addEventListener('click', () => {
    const result = flow_layouts.run_layout('constrained_force_directed');
    console.log(result); // "✅ Applied constrained_force_directed layout in 15.23ms"
});
```

### **Dynamic Constraint Management**
```javascript
// Add constraints interactively
const pinButton = document.getElementById('pin-node');
pinButton.addEventListener('click', () => {
    const nodeId = document.getElementById('constraint-node').value;
    flow_layouts.add_constraint('pin', nodeId);
});
```

### **Performance Monitoring**
```javascript
// Monitor layout performance in real-time
const compareButton = document.getElementById('compare-layouts');
compareButton.addEventListener('click', () => {
    const comparison = flow_layouts.compare_layouts();
    updatePerformanceDisplay(comparison);
});
```

## Algorithm Complexity Analysis

### **Time Complexity**

| Algorithm | Time Complexity | Space Complexity | Best Case | Worst Case |
|-----------|-----------------|------------------|-----------|------------|
| Force-Directed | O(n²) | O(n) | O(n) | O(n²) |
| Circular | O(n) | O(1) | O(n) | O(n) |
| Radial | O(n log n) | O(n) | O(n) | O(n log n) |
| Layered | O(n + e) | O(n) | O(n) | O(n + e) |
| Spiral | O(n) | O(1) | O(n) | O(n) |
| Grid | O(n) | O(n) | O(n) | O(n) |

### **Performance Characteristics**

- **Force-directed**: High quality, slow convergence, good for organic layouts
- **Circular**: Fast, predictable, good for symmetric data
- **Radial**: Medium speed, hierarchical, good for trees
- **Layered**: Fast, structured, good for flowcharts
- **Spiral**: Fast, aesthetic, good for dense graphs
- **Grid**: Fastest, structured, good for regular data

## Advanced Use Cases

### **Network Visualization**
```rust
// Visualize computer networks with constraints
let constraints = LayoutConstraints {
    pinned_nodes: vec!["router-1".to_string(), "server-1".to_string()],
    grouped_nodes: vec![
        vec!["web-1".to_string(), "web-2".to_string(), "web-3".to_string()],
        vec!["db-1".to_string(), "db-2".to_string()],
    ],
    ..Default::default()
};
```

### **Process Flow Diagrams**
```rust
// Layout business process with proper hierarchy
let layered_layout = apply_layered_layout(&mut process_graph, &constraints);
// Automatic layer assignment based on process flow
```

### **Organizational Charts**
```rust
// Hierarchical organization layout
let radial_layout = apply_radial_layout(&mut org_graph, &constraints);
// CEO at center, departments arranged radially
```

### **Dependency Graphs**
```rust
// Software dependency visualization
let constrained_force = apply_constrained_force_directed_layout(&mut dep_graph, &constraints);
// Respect version conflicts and compatibility constraints
```

## Constraint Types

### **Static Constraints**
- **Position pinning**: Lock nodes at specific coordinates
- **Group constraints**: Maintain relative positioning within groups
- **Boundary limits**: Confine nodes to rectangular regions

### **Dynamic Constraints**
- **Distance constraints**: Maintain minimum/maximum spacing
- **Alignment constraints**: Keep nodes aligned horizontally/vertically
- **Connectivity constraints**: Preserve edge lengths or angles

### **Soft vs Hard Constraints**
- **Hard constraints**: Must be satisfied (pinning, boundaries)
- **Soft constraints**: Should be satisfied when possible (alignment, spacing)

## Layout Quality Metrics

### **Aesthetic Criteria**
- **Node distribution**: Even spacing, minimal overlap
- **Edge lengths**: Consistent, appropriate lengths
- **Angular resolution**: Clear edge directions
- **Symmetry**: Preservation of inherent symmetries

### **Readability Metrics**
- **Crossings**: Minimize edge crossings
- **Bends**: Minimize edge bends
- **Orthogonality**: Prefer horizontal/vertical edges
- **Compactness**: Minimize total area used

### **Performance Metrics**
- **Computation time**: Time to compute layout
- **Stability**: Consistency across runs
- **Scalability**: Performance with increasing graph size
- **Convergence**: Reliable convergence to good solutions

## JavaScript API

### **Layout Operations**
```javascript
// Run specific layout algorithms
flow_layouts.run_layout('constrained_force_directed');
flow_layouts.run_layout('circular');
flow_layouts.run_layout('radial');

// Apply constraints
flow_layouts.add_constraint('pin', 'node-1');
flow_layouts.create_group('node-a,node-b,node-c');

// Analysis and benchmarking
flow_layouts.compare_layouts();
flow_layouts.benchmark_layout('force_directed', 1000);

// Data export
const data = flow_layouts.export_layout_data();
const info = flow_layouts.get_layout_info();
```

## Implementation Details

### **Force Calculation**
```rust
// Repulsive forces between nodes
let repulsive_force = k * k / distance;

// Attractive forces along edges
let attractive_force = distance * distance / k;

// Apply damping for stability
let damped_force = force * damping_factor;
```

### **Constraint Satisfaction**
```rust
// Check constraint violations
for constraint in &constraints.pinned_nodes {
    if node.position != pinned_position {
        // Apply correction force
        let correction = pinned_position - node.position;
        node.velocity += correction * constraint_strength;
    }
}
```

### **Performance Optimization**
```rust
// Spatial indexing for force calculations
let spatial_index = build_spatial_index(nodes);

// Barnes-Hut approximation for distant nodes
let far_field_force = calculate_far_field_force(node, spatial_index);

// Quad-tree optimization
let quad_tree = build_quad_tree(nodes);
let optimized_forces = calculate_optimized_forces(node, quad_tree);
```

## Integration with Flow-RS

### **Graph Structure**
```rust
// Flow-RS graph integration
use flow_rs_core::{Graph, Node, Edge, Position};

let mut graph: Graph<String, String> = Graph::new();
let node = Node::new("node-1".to_string(), Position::new(100.0, 100.0), "Node Data".to_string());
graph.add_node(node);
```

### **Layout Application**
```rust
// Apply layout to Flow-RS graph
fn apply_layout_to_flow_graph(graph: &mut Graph<String, String>, algorithm: LayoutAlgorithm) {
    match algorithm {
        LayoutAlgorithm::ForceDirected => apply_force_directed_layout(graph),
        LayoutAlgorithm::Circular => apply_circular_layout(graph),
        // ... other algorithms
    }
}
```

### **Real-time Updates**
```rust
// Update layout in real-time
fn update_layout_realtime(graph: &mut Graph<String, String>, delta_time: f64) {
    // Incremental layout updates
    apply_incremental_layout_step(graph, delta_time);

    // Check convergence
    if has_converged(graph) {
        finalize_layout(graph);
    }
}
```

## Educational Value

This example teaches:
- Advanced graph layout algorithms and their applications
- Constraint-based layout systems and satisfaction
- Performance optimization techniques for layout computation
- Interactive constraint management and user interfaces
- Comparative algorithm analysis and benchmarking
- Real-time layout updates and incremental computation
- Integration patterns between layout engines and graph libraries

---

## Key Takeaway

**Flow-RS Advanced Layouts demonstrates the full spectrum of graph layout capabilities, from basic algorithms to sophisticated constrained layouts with real-time performance monitoring. The comprehensive constraint system, multiple layout algorithms, and performance benchmarking showcase how Flow-RS can handle complex layout requirements while maintaining excellent performance and usability.**

The advanced layouts example serves as both a demonstration of cutting-edge layout techniques and a foundation for building professional graph visualization applications that require precise control over node positioning and relationship visualization.
