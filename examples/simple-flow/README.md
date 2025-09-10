# Simple Flow Example

A basic demonstration of the Leptos Flow Canvas2D renderer showing nodes and edges in a simple flow diagram.

## Features

- **Canvas2D Rendering**: Demonstrates the Canvas2D renderer working with basic shapes
- **Node Display**: Shows three sample nodes with different positions
- **Edge Connections**: Displays connections between nodes using bezier curves
- **Background Pattern**: Renders a dotted background pattern
- **Interactive Controls**: Basic UI controls (placeholder for future functionality)

## What You'll Learn

- How to set up a basic Leptos Flow application
- Canvas2D renderer initialization and usage
- Creating and rendering nodes and edges
- Basic viewport and background configuration
- Integration with Leptos reactive framework

## Running the Example

### Prerequisites

Make sure you have the following installed:

```bash
# Install Rust and wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-pack for WASM builds
cargo install wasm-pack
```

### Running

1. Navigate to the example directory:
   ```bash
   cd examples/simple-flow
   ```

2. Build the WASM module:
   ```bash
   ./build.sh
   ```

3. Serve the example (in a new terminal):
   ```bash
   python3 -m http.server 8080
   ```

4. Open your browser to `http://localhost:8080`

### Alternative Build Method

You can also build manually:

```bash
# Build the WASM module
wasm-pack build --target web --out-dir pkg --dev

# Serve with any HTTP server
python3 -m http.server 8080
```

## Code Structure

### Main Components

- **`main()`**: The WASM entry point that initializes the renderer and sets up the canvas
- **`create_sample_graph()`**: Creates a sample graph with 3 nodes and 2 edges
- **`render_graph()`**: Handles the complete rendering pipeline
- **`setup_event_handlers()`**: Sets up button click handlers (placeholder for future functionality)

### Key Concepts

#### WASM Entry Point
```rust
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize renderer and render the graph
}
```

#### Graph Creation
```rust
let mut graph = Graph::new();
let node1 = Node::simple("node1", Position::new(100.0, 100.0));
graph.add_node(node1).unwrap();
```

#### Renderer Setup
```rust
let mut renderer = Canvas2DRenderer::new(&canvas)?;
renderer.render_graph(&graph, &viewport)?;
```

#### Background Configuration
```rust
let bg_config = BackgroundConfig {
    color: "#ffffff".to_string(),
    pattern_color: "#e2e8f0".to_string(),
    variant: BackgroundVariant::Dots,
    size: 20.0,
    opacity: 0.5,
};
```

## Expected Output

You should see:
- A white canvas with a dotted background pattern
- Three rectangular nodes positioned at different locations
- Two curved edges connecting the nodes
- Control buttons in the top-right corner

## Next Steps

This example demonstrates the basic rendering capabilities. To extend it:

1. **Add Interaction**: Implement drag and drop for nodes
2. **Custom Styling**: Apply different colors and styles to nodes
3. **Dynamic Content**: Add/remove nodes and edges dynamically
4. **Event Handling**: Respond to mouse clicks and keyboard input
5. **Layout Algorithms**: Implement automatic node positioning

## Troubleshooting

### Common Issues

1. **Canvas not rendering**: Check browser console for JavaScript errors
2. **Build failures**: Ensure you have the correct Rust target installed
3. **Performance issues**: The example is optimized for demonstration, not performance

### Debug Mode

To enable debug logging, add this to your browser's console:
```javascript
localStorage.setItem('debug', 'leptos-flow:*');
```

## Related Examples

- **Custom Nodes**: Learn to create custom node components
- **Interactive Controls**: Add zoom, pan, and selection features
- **Styled Editor**: Apply themes and custom styling
- **Form-Based Nodes**: Create interactive nodes with form controls

## Contributing

Found an issue or want to improve this example? Please:

1. Check existing issues on GitHub
2. Create a new issue with detailed description
3. Submit a pull request with your improvements

## License

This example is provided under the same license as Leptos Flow (MIT/Apache-2.0).
