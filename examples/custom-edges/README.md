# Custom Edges Example

Demonstrates Flow-RS's comprehensive edge customization capabilities with different routing algorithms, visual styles, and interactive features.

## What it demonstrates

- **Edge routing algorithms**: Straight, Bezier, Step, and Smooth Step
- **Visual customization**: Colors, thickness, dash patterns, labels
- **Animation support**: Animated edges for active flows
- **Multiple connections**: Handling multiple edges to/from nodes
- **Type-safe styling**: Rust compile-time validation

## Features shown

- ✅ **4 edge types**: Straight, Bezier, Step, SmoothStep routing
- ✅ **7 color schemes**: Full spectrum color customization
- ✅ **3 thickness levels**: Fine control over line width
- ✅ **Dash patterns**: Dotted and dashed line styles
- ✅ **Edge labels**: Descriptive text on connections
- ✅ **Animation**: Dynamic edge highlighting
- ✅ **Multiple edges**: Complex node connectivity

## Edge types demonstrated

### **Straight Edges** 📏
- Direct line connections between nodes
- Most efficient for simple diagrams
- Clean, minimal appearance

### **Bezier Curves** 🌊
- Smooth curved connections using cubic Bezier algorithms
- Organic, flowing appearance
- Best for aesthetic diagrams

### **Step Edges** ⚡
- Angular connections with horizontal/vertical segments
- Technical, structured appearance
- Common in flowcharts and technical diagrams

### **Smooth Step Edges** 🎯
- Rounded step connections combining angular routing with smooth corners
- Professional appearance with technical precision
- Balance between aesthetics and structure

## Visual features

### **Styling Options**
- **Colors**: Hex color codes with full RGB support
- **Thickness**: 1px to 5px line widths
- **Dash patterns**: Custom dash/gap arrays
- **Labels**: Text labels positioned along edges
- **Animation**: Pulsing effects for active connections

### **Connection Patterns**
- **One-to-one**: Basic node connections
- **One-to-many**: Single source to multiple targets
- **Many-to-one**: Multiple sources to single target
- **Complex networks**: Interconnected node graphs

## Running the example

### Quick Start

```bash
# Navigate to the example directory
cd examples/custom-edges

# Build and serve
./build.sh

# Or build manually
wasm-pack build --target web --out-dir pkg --dev

# Serve with Python
python3 -m http.server 8000

# Open http://localhost:8000
```

## Architecture

```
src/lib.rs
├── StyledEdgeData struct (customization data)
├── EdgeType enum (routing algorithms)
├── create_styled_edges_graph() (demo data)
├── render_styled_edges_graph() (rendering logic)
└── WASM entry point
```

## Code structure

```rust
// Define custom edge styling
#[derive(Clone, Debug)]
struct StyledEdgeData {
    pub edge_type: EdgeType,           // Routing algorithm
    pub color: String,                 // Hex color
    pub thickness: f64,                // Line width
    pub dash_pattern: Option<Vec<f64>>, // Dash pattern
    pub label: Option<String>,         // Edge label
    pub animated: bool,                // Animation flag
}

// Create styled edge
let edge = Edge {
    id: "styled-edge".to_string(),
    source: "source_node".to_string(),
    target: "target_node".to_string(),
    data: StyledEdgeData::new(
        EdgeType::Bezier,
        "#4299e1",
        2.0,
        Some("Data Flow"),
        false,
    ),
};
```

## Performance considerations

- **Efficient rendering**: All edge types optimized for performance
- **Memory usage**: Minimal overhead for styling data
- **Animation performance**: Smooth 60fps animations
- **Scalability**: Handles hundreds of styled edges

## Browser compatibility

Compatible with all modern browsers supporting:
- WebAssembly
- Canvas 2D API
- ES6 modules

## Related examples

- **Hello World**: Basic static flow display
- **Custom Styles**: Node customization
- **Background Variants**: Canvas styling
- **Controlled Uncontrolled**: State management

## Advanced features

### **Future Enhancements**
- Arrow markers and connection points
- Interactive edge editing
- Edge bundling for complex graphs
- Custom routing algorithms
- Collision detection and avoidance
