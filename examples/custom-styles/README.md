# Custom Node Styles Example

Demonstrates Flow-RS's powerful node customization capabilities with different colors, shapes, sizes, and styling options.

## What it demonstrates

- **Node shape variety**: Rectangles, rounded rectangles, and circles
- **Size options**: Small, medium, and large node sizes
- **Color customization**: Full hex color control
- **Custom data structures**: Extending nodes with styling properties
- **Type-safe styling**: Rust compile-time validation

## Features shown

- ✅ Custom node data structures with styling properties
- ✅ Multiple shape types (Rectangle, Rounded, Circle)
- ✅ Size variants (Small: 80x40, Medium: 120x60, Large: 160x80)
- ✅ Color customization with hex codes
- ✅ Label positioning and typography
- ✅ Flow diagram with styled connections

## Running the example

### Quick Start

```bash
# Navigate to the example directory
cd examples/custom-styles

# Build and serve
./build.sh

# Or build manually
wasm-pack build --target web --out-dir pkg --dev

# Serve with Python
python3 -m http.server 8000

# Open http://localhost:8000
```

## Visual showcase

The example displays five different node types:

1. **Input Node** (Blue, Rounded, Medium)
   - Represents data input points
   - Rounded corners for friendly appearance

2. **Process Node** (Green, Rectangle, Large)
   - Shows processing operations
   - Larger size for emphasis

3. **Decision Node** (Orange, Circle, Medium)
   - Represents decision points
   - Circular shape for branching logic

4. **Output Node** (Purple, Rounded, Medium)
   - Shows data outputs
   - Consistent with input styling

5. **Error Node** (Red, Rectangle, Small)
   - Represents error states
   - Compact size, attention-grabbing color

## Code structure

```rust
// Define custom styling data
#[derive(Clone, Debug)]
struct StyledNodeData {
    pub label: String,
    pub color: String,
    pub shape: NodeShape,
    pub size: NodeSize,
}

// Create styled nodes
let node = Node {
    id: "node_id".to_string(),
    position: Position::new(x, y),
    data: StyledNodeData::new("Label", "#color", shape, size),
    ..Default::default()
};
```

## Customization options

### Shapes
- `NodeShape::Rectangle` - Standard rectangular nodes
- `NodeShape::Rounded` - Rounded corner rectangles
- `NodeShape::Circle` - Circular nodes

### Sizes
- `NodeSize::Small` (80×40 pixels)
- `NodeSize::Medium` (120×60 pixels)
- `NodeSize::Large` (160×80 pixels)

### Colors
- Full hex color support (`#4299e1`, `#48bb78`, etc.)
- RGBA support for transparency
- CSS color names support

## Performance

- **Static rendering**: No runtime style calculations
- **Minimal memory overhead**: Style data stored efficiently
- **Fast compilation**: Type-safe styling validated at compile time
- **WebAssembly optimized**: Direct canvas rendering

## Browser compatibility

Compatible with all modern browsers supporting:
- WebAssembly
- Canvas 2D API
- ES6 modules

## Next steps

This example shows basic styling. For advanced features, see:
- **Custom Edges**: Connection styling and routing
- **Interactive Styling**: Dynamic style changes
- **Theme Systems**: Coordinated color schemes
- **Animation**: Style transitions and effects
