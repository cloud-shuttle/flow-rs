# Basic Layouts Example

Demonstrates Flow-RS's automatic graph layout algorithms with interactive switching between different positioning strategies.

## What it demonstrates

- **Force-directed layout**: Physics-based organic positioning
- **Grid layout**: Structured regular spacing
- **Hierarchical layout**: Tree-based level organization
- **Interactive switching**: Live layout algorithm changes
- **Performance comparison**: Different approaches for different use cases

## Layout algorithms shown

### **Force-Directed Layout** 🌊
- Physics-based algorithm using repulsive and attractive forces
- Connected nodes are positioned closer together
- Creates organic, aesthetically pleasing arrangements
- Best for: General-purpose diagrams, complex relationships

### **Grid Layout** 📏
- Regular grid-based positioning with consistent spacing
- Predictable, structured appearance
- Easy to scan and understand
- Best for: Structured data, regular patterns, technical diagrams

### **Hierarchical Layout** 🏗️
- Tree-based layout organizing nodes by hierarchy levels
- Clear parent-child relationships
- Left-to-right or top-to-bottom flow
- Best for: Flowcharts, organizational charts, process diagrams

## Interactive features

### **Algorithm Selection**
- Dropdown to switch between layout types
- Instant visual feedback
- Preserves graph structure

### **Re-layout Button**
- Manually trigger layout recalculation
- Useful for seeing algorithm variations
- Maintains same graph connectivity

### **Live Updates**
- Real-time layout switching
- Smooth transitions (when implemented)
- Performance metrics display

## Sample graph structure

The example uses a 9-node graph with the following structure:

```
Start
├── Process 1
│   └── Process 2
│       └── Decision
│           ├── Output 1
│           │   └── Subprocess 1
│           └── Output 2
│               └── Subprocess 2
└── End (connected from both subprocesses)
```

## Implementation details

```rust
// Layout algorithm selection
enum LayoutAlgorithm {
    ForceDirected,
    Grid,
    Hierarchical,
}

// Apply selected layout
match algorithm {
    LayoutAlgorithm::ForceDirected => {
        // Physics-based positioning
        apply_force_directed_layout(&mut graph);
    }
    LayoutAlgorithm::Grid => {
        // Regular grid arrangement
        apply_grid_layout(&mut graph);
    }
    LayoutAlgorithm::Hierarchical => {
        // Level-based organization
        apply_hierarchical_layout(&mut graph);
    }
}
```

## Algorithm characteristics

### **Performance**
- **Force-directed**: O(n²) complexity, best for <100 nodes
- **Grid**: O(n) complexity, scales well
- **Hierarchical**: O(n) complexity, deterministic

### **Visual Properties**
- **Force-directed**: Organic, variable spacing
- **Grid**: Uniform, predictable spacing
- **Hierarchical**: Level-based, directional flow

### **Use Cases**
- **Force-directed**: General diagrams, network visualization
- **Grid**: Technical documentation, structured data
- **Hierarchical**: Process flows, organizational charts

## Running the example

### Quick Start

```bash
# Navigate to the example directory
cd examples/basic-layouts

# Build and serve
./build.sh

# Or build manually
wasm-pack build --target web --out-dir pkg --dev

# Serve with Python
python3 -m http.server 8000

# Open http://localhost:8000
```

## Code structure

```
src/lib.rs
├── LayoutAlgorithm enum (algorithm types)
├── setup_layout_controls() (interactive UI)
├── create_and_layout_graph() (main layout function)
├── apply_force_directed_layout() (physics-based)
├── apply_grid_layout() (structured grid)
├── apply_hierarchical_layout() (tree-based)
└── render_graph() (canvas rendering)
```

## Advanced features

### **Future Enhancements**
- Animated layout transitions
- Custom layout algorithms
- Constraint-based positioning
- Incremental layout updates
- Layout performance profiling

## Browser compatibility

Compatible with all modern browsers supporting:
- WebAssembly
- Canvas 2D API
- ES6 modules

## Related examples

- **Hello World**: Basic static flow display
- **Custom Styles**: Node and edge customization
- **Custom Edges**: Edge styling and routing
- **Background Variants**: Canvas styling options

## Educational value

This example teaches:
- Different approaches to graph layout
- Algorithm selection based on use case
- Performance characteristics of layout methods
- Visual design principles for node-link diagrams
