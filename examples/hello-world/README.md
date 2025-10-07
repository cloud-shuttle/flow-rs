# Hello World Example

The simplest possible Flow-RS example demonstrating the absolute minimum code needed to display a flow diagram.

## What it demonstrates

- **Basic graph creation**: Creating nodes and edges with simple APIs
- **Canvas2D rendering**: Using the built-in renderer to display graphs
- **WebAssembly integration**: Running Rust code in the browser
- **Minimal setup**: ~50 lines of code for a working flow diagram

## Features shown

- ✅ Node creation with `Node::simple()`
- ✅ Edge creation with `Edge::simple()`
- ✅ Graph construction and rendering
- ✅ Canvas2D background with dot pattern
- ✅ Static diagram display (no interactions)

## Running the example

### Quick Start (Recommended)

```bash
# Navigate to the example directory
cd examples/hello-world

# Build and serve with the build script
./build.sh

# Or build manually with wasm-pack
wasm-pack build --target web --out-dir pkg --dev

# Serve the example (requires a local server)
python3 -m http.server 8000

# Open http://localhost:8000 in your browser
```

### Alternative: Using trunk (if installed)

```bash
# If you have trunk installed
trunk serve --open
```

### What you'll see

- A clean, minimal interface with a canvas
- Three nodes (Start, Process, End) connected by two edges
- A dotted background pattern
- No interactive elements (this is a static display example)

## Code overview

```rust
// Create a simple graph
let mut graph = Graph::new();

// Add nodes
graph.add_node(Node::simple("start", Position::new(150.0, 150.0)))?;
graph.add_node(Node::simple("process", Position::new(400.0, 150.0)))?;
graph.add_node(Node::simple("end", Position::new(650.0, 150.0)))?;

// Add edges
graph.add_edge(Edge::simple("flow1", "start", "process"))?;
graph.add_edge(Edge::simple("flow2", "process", "end"))?;

// Render
renderer.render_graph_dyn(&graph, &viewport)?;
```

## What this example doesn't include

- User interactions (drag, select, etc.)
- Custom node types or styling
- Complex layouts or algorithms
- State management or reactivity

For these features, see the other examples in this gallery.

## Performance

This example renders a static diagram and demonstrates:
- Fast initial load (< 100ms)
- Efficient rendering pipeline
- Minimal memory usage
- WebAssembly performance benefits

## Browser compatibility

Works in all modern browsers with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+
