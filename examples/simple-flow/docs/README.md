# Leptos Flow Simple Example Documentation

Welcome to the comprehensive documentation for the Leptos Flow Simple Example! This documentation provides everything you need to understand, use, and integrate the flow diagram library.

## 📚 Documentation Overview

This documentation is organized into several sections to help you get started quickly and dive deep into advanced usage:

### 🚀 [Getting Started Guide](./GETTING_STARTED.md)
- Quick setup and installation
- Your first flow diagram
- Basic concepts and terminology
- Common use cases

### 🔧 [API Reference](./API_EXAMPLES.md)
- Complete API documentation with examples
- Canvas2D renderer usage
- Graph operations and management
- Viewport and interaction handling
- Error handling and performance optimization

### 💡 [Usage Examples](./USAGE_EXAMPLES.md)
- Practical examples for common scenarios
- Flow diagrams, decision trees, and process flows
- Interactive editors and data visualization
- Workflow management and real-time collaboration
- Custom styling and theming

### 🔗 [Integration Examples](./INTEGRATION_EXAMPLES.md)
- Framework integrations (React, Vue.js, Angular)
- Vanilla JavaScript usage
- WebSocket real-time updates
- REST API integration
- Database and backend integration

### 🧪 [Testing Guide](./TESTING.md)
- Test infrastructure and setup
- Running tests and debugging
- Performance testing and benchmarks
- Continuous integration

## 🎯 Quick Start

### 1. Build the Example

```bash
# Navigate to the example directory
cd examples/simple-flow

# Build the WASM module
./build.sh

# Serve the example
python3 -m http.server 8080
```

### 2. Open in Browser

Visit `http://localhost:8080` to see the example in action.

### 3. Basic Usage

```javascript
import init, { create_simple_flow } from './pkg/simple_flow_example.js';

async function run() {
    await init();
    create_simple_flow();
}

run();
```

## 🏗️ Architecture Overview

The Leptos Flow Simple Example is built with the following components:

### Core Components

- **Canvas2D Renderer**: High-performance 2D canvas rendering
- **Graph Engine**: Node and edge management with spatial indexing
- **Viewport System**: Pan, zoom, and view management
- **Interaction Handler**: Mouse and touch event processing
- **Background System**: Configurable background patterns

### Key Features

- ✅ **High Performance**: Optimized rendering with batching and culling
- ✅ **Interactive**: Drag, select, and manipulate nodes and edges
- ✅ **Customizable**: Extensive styling and theming options
- ✅ **Framework Agnostic**: Works with any JavaScript framework
- ✅ **WASM Powered**: Rust performance in the browser
- ✅ **Type Safe**: Full TypeScript support
- ✅ **Well Tested**: Comprehensive test suite with benchmarks

## 📖 Documentation Sections

### For Beginners
Start with the [Getting Started Guide](./GETTING_STARTED.md) to understand the basics and create your first flow diagram.

### For Developers
Use the [API Reference](./API_EXAMPLES.md) to understand the complete API and see detailed code examples.

### For Integrators
Check out the [Integration Examples](./INTEGRATION_EXAMPLES.md) to see how to integrate with your favorite framework or technology stack.

### For Advanced Users
Explore the [Usage Examples](./USAGE_EXAMPLES.md) for complex scenarios and advanced features.

## 🛠️ Development

### Prerequisites

- Rust (latest stable)
- wasm-pack
- Node.js (for serving)
- Modern web browser

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd leptos-flow

# Build the example
cd examples/simple-flow
./build.sh
```

### Running Tests

```bash
# Run all tests
./run_tests.sh

# Run specific test suites
wasm-pack test --headless --firefox --lib
wasm-pack test --headless --firefox --test integration_tests
wasm-pack test --headless --firefox --test performance_tests
```

## 🎨 Examples Gallery

### Basic Flow Diagram
```rust
let mut graph = Graph::new();
let node1 = Node::simple("start", Position::new(100.0, 100.0));
let node2 = Node::simple("end", Position::new(300.0, 100.0));
let edge = Edge::simple("connection", "start", "end");

graph.add_node(node1)?;
graph.add_node(node2)?;
graph.add_edge(edge)?;
```

### Interactive Editor
```rust
let mut interaction_handler = InteractionHandler::new(
    canvas.clone(),
    renderer,
    graph,
    viewport,
);

// Handle mouse events
interaction_handler.handle_mouse_down(&event);
interaction_handler.render()?;
```

### Custom Styling
```rust
let mut node = Node::simple("custom", Position::new(200.0, 200.0));
node.style = NodeStyle {
    variant: NodeVariant::Rectangle,
    fill_color: "#3b82f6".to_string(),
    stroke_color: "#1e40af".to_string(),
    stroke_width: 2.0,
    width: 120.0,
    height: 80.0,
    corner_radius: 8.0,
    ..Default::default()
};
```

## 🔧 Configuration

### Canvas Configuration
```rust
let mut renderer = Canvas2DRenderer::new(&canvas)?;
renderer.resize(800, 600)?;
```

### Background Configuration
```rust
let bg_config = BackgroundConfig {
    color: "#f8fafc".to_string(),
    pattern_color: "#e2e8f0".to_string(),
    variant: BackgroundVariant::Dots,
    size: 20.0,
    opacity: 0.5,
};
```

### Viewport Configuration
```rust
let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);
```

## 🚀 Performance

The Leptos Flow Simple Example is optimized for performance:

- **Rendering**: 60 FPS with hundreds of nodes
- **Memory**: Efficient memory usage with object pooling
- **Scalability**: Handles large graphs with spatial indexing
- **Responsiveness**: Smooth interactions and animations

### Performance Benchmarks

| Operation | Target | Typical |
|-----------|--------|---------|
| Small graph rendering | ≤ 100ms | ~50ms |
| Large graph rendering | ≤ 500ms | ~200ms |
| Node operations | ≤ 50ms | ~20ms |
| Viewport operations | ≤ 10ms | ~5ms |

## 🐛 Troubleshooting

### Common Issues

1. **Canvas not rendering**: Check if WASM module is loaded
2. **Performance issues**: Reduce node count or enable culling
3. **Memory leaks**: Ensure proper cleanup of event listeners
4. **Build errors**: Verify Rust and wasm-pack versions

### Debug Mode

```rust
// Enable debug logging
console_error_panic_hook::set_once();
```

### Getting Help

- Check the [API Reference](./API_EXAMPLES.md) for detailed examples
- Review the [Usage Examples](./USAGE_EXAMPLES.md) for common patterns
- Look at the [Integration Examples](./INTEGRATION_EXAMPLES.md) for framework-specific help
- Run the test suite to verify your setup

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

We welcome contributions! Please see the contributing guidelines for more information.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

## 📞 Support

- **Documentation**: This comprehensive guide
- **Issues**: GitHub Issues for bug reports
- **Discussions**: GitHub Discussions for questions
- **Examples**: Check the examples directory for more use cases

## 🎉 What's Next?

Now that you have an overview of the documentation, here's what to do next:

1. **New to the project?** Start with the [Getting Started Guide](./GETTING_STARTED.md)
2. **Ready to build?** Check out the [API Reference](./API_EXAMPLES.md)
3. **Need integration help?** See the [Integration Examples](./INTEGRATION_EXAMPLES.md)
4. **Want to see examples?** Browse the [Usage Examples](./USAGE_EXAMPLES.md)
5. **Setting up tests?** Follow the [Testing Guide](./TESTING.md)

Happy coding! 🚀
