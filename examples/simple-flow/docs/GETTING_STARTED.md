# Getting Started with Leptos Flow Simple Example

This guide will help you get up and running with the Leptos Flow Simple Example quickly and easily.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Basic Concepts](#basic-concepts)
- [Your First Flow Diagram](#your-first-flow-diagram)
- [Understanding the Code](#understanding-the-code)
- [Next Steps](#next-steps)

## Prerequisites

Before you begin, make sure you have the following installed:

### Required Software

- **Rust** (latest stable version)

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **wasm-pack** (for building WASM modules)

  ```bash
  cargo install wasm-pack
  ```

- **Python 3** (for serving the example)

  ```bash
  # macOS
  brew install python3

  # Ubuntu/Debian
  sudo apt install python3

  # Windows
  # Download from python.org
  ```

### Optional but Recommended

- **Node.js** (for advanced development)

  ```bash
  # Using nvm (recommended)
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
  nvm install node
  ```

- **Modern Web Browser** (Chrome, Firefox, Safari, Edge)

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd leptos-flow
```

### 2. Navigate to the Example

```bash
cd examples/simple-flow
```

### 3. Build the Example

```bash
# Make the build script executable
chmod +x build.sh

# Build the WASM module
./build.sh
```

This will:

- Compile the Rust code to WebAssembly
- Generate JavaScript bindings
- Create the necessary files in the `pkg/` directory

### 4. Serve the Example

```bash
# Start a local server
python3 -m http.server 8080

# Or using Python 2 (if Python 3 is not available)
python -m SimpleHTTPServer 8080
```

### 5. Open in Browser

Visit `http://localhost:8080` in your web browser. You should see a simple flow diagram with nodes and edges.

## Quick Start

### Basic HTML Setup

Create a simple HTML file to get started:

```html
<!DOCTYPE html>
<html>
<head>
    <title>My First Flow Diagram</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 20px;
        }
        #flow-canvas {
            border: 1px solid #ccc;
            border-radius: 8px;
            cursor: crosshair;
        }
    </style>
</head>
<body>
    <h1>My First Flow Diagram</h1>
    <canvas id="flow-canvas" width="800" height="600"></canvas>

    <script type="module">
        import init, { create_simple_flow } from './pkg/simple_flow_example.js';

        async function run() {
            // Initialize the WASM module
            await init();

            // Create a simple flow diagram
            create_simple_flow();
        }

        run().catch(console.error);
    </script>
</body>
</html>
```

### JavaScript Integration

```javascript
// Import the WASM module
import init, {
    create_simple_flow,
    add_node_at_position,
    move_node
} from './pkg/simple_flow_example.js';

// Initialize and use
async function setupFlow() {
    await init();

    // Create the initial flow
    create_simple_flow();

    // Add a new node at position (400, 300)
    add_node_at_position(400, 300);

    // Move a node (if you know the node ID)
    // move_node('node1', 500, 400);
}

setupFlow();
```

## Basic Concepts

### Core Components

The Leptos Flow Simple Example consists of several key components:

#### 1. **Graph**

A data structure that contains nodes and edges:

```rust
let mut graph = Graph::new();
```

#### 2. **Nodes**

Individual elements in the flow diagram:

```rust
let node = Node::simple("node_id", Position::new(100.0, 100.0));
```

#### 3. **Edges**

Connections between nodes:

```rust
let edge = Edge::simple("edge_id", "source_node", "target_node");
```

#### 4. **Viewport**

The visible area and zoom level:

```rust
let viewport = Viewport::default();
```

#### 5. **Renderer**

Handles drawing to the canvas:

```rust
let mut renderer = Canvas2DRenderer::new(&canvas)?;
```

### Coordinate System

- **Origin (0, 0)**: Top-left corner of the canvas
- **X-axis**: Increases to the right
- **Y-axis**: Increases downward
- **Units**: Pixels

### Node Properties

Each node has the following properties:

- **ID**: Unique identifier
- **Position**: X and Y coordinates
- **Size**: Width and height
- **Style**: Colors, borders, shapes

### Edge Properties

Each edge has the following properties:

- **ID**: Unique identifier
- **Source**: Starting node ID
- **Target**: Ending node ID
- **Style**: Color, width, dash pattern

## Your First Flow Diagram

Let's create a simple process flow diagram step by step:

### Step 1: Create the HTML Structure

```html
<!DOCTYPE html>
<html>
<head>
    <title>Process Flow Example</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 20px;
            background-color: #f5f5f5;
        }
        .container {
            max-width: 1000px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        #flow-canvas {
            border: 1px solid #ddd;
            border-radius: 8px;
            background: white;
        }
        .controls {
            margin: 20px 0;
            text-align: center;
        }
        button {
            background: #3b82f6;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            margin: 0 5px;
        }
        button:hover {
            background: #2563eb;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Process Flow Example</h1>
        <div class="controls">
            <button onclick="addNode()">Add Node</button>
            <button onclick="clearCanvas()">Clear</button>
            <button onclick="saveFlow()">Save</button>
        </div>
        <canvas id="flow-canvas" width="800" height="600"></canvas>
    </div>

    <script type="module">
        import init, {
            create_simple_flow,
            add_node_at_position,
            clear_canvas
        } from './pkg/simple_flow_example.js';

        let isInitialized = false;

        async function initialize() {
            await init();
            isInitialized = true;
            create_simple_flow();
        }

        window.addNode = function() {
            if (!isInitialized) return;

            // Add node at random position
            const x = Math.random() * 700 + 50;
            const y = Math.random() * 500 + 50;
            add_node_at_position(x, y);
        };

        window.clearCanvas = function() {
            if (!isInitialized) return;
            clear_canvas();
        };

        window.saveFlow = function() {
            console.log('Saving flow...');
            // In a real application, you would save to a server or local storage
        };

        // Initialize when page loads
        initialize().catch(console.error);
    </script>
</body>
</html>
```

### Step 2: Add Interactive Features

```javascript
// Add mouse interaction
document.getElementById('flow-canvas').addEventListener('click', function(event) {
    if (!isInitialized) return;

    const rect = this.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Add node at click position
    add_node_at_position(x, y);
});

// Add keyboard shortcuts
document.addEventListener('keydown', function(event) {
    if (!isInitialized) return;

    switch(event.key) {
        case 'n':
        case 'N':
            addNode();
            break;
        case 'c':
        case 'C':
            if (event.ctrlKey || event.metaKey) {
                clearCanvas();
            }
            break;
    }
});
```

### Step 3: Customize the Appearance

```javascript
// Custom node styling (this would require extending the WASM functions)
function createCustomNode(id, x, y, color = '#3b82f6') {
    // In a real implementation, you would call a WASM function
    // that creates a node with custom styling
    add_node_at_position(x, y);
}
```

## Understanding the Code

### WASM Module Structure

The compiled WASM module provides these main functions:

```javascript
// Core functions
create_simple_flow()           // Creates a basic flow diagram
add_node_at_position(x, y)     // Adds a node at specified coordinates
move_node(nodeId, x, y)        // Moves an existing node
remove_node(nodeId)            // Removes a node
clear_canvas()                 // Clears the entire canvas

// Advanced functions (if implemented)
create_custom_flow(nodes, edges)  // Creates flow from data
get_node_at_position(x, y)        // Gets node at coordinates
get_all_nodes()                   // Returns all nodes
get_all_edges()                   // Returns all edges
```

### Error Handling

Always wrap WASM calls in try-catch blocks:

```javascript
try {
    await init();
    create_simple_flow();
} catch (error) {
    console.error('Failed to initialize flow:', error);
}
```

### Memory Management

The WASM module handles memory management automatically, but be aware of:

- Large numbers of nodes may impact performance
- Event listeners should be cleaned up when not needed
- Canvas resizing may require re-initialization

## Next Steps

Now that you have a basic understanding, here's what to explore next:

### 1. **Explore the API**

- Read the [API Reference](./API_EXAMPLES.md) for detailed function documentation
- Try different node and edge configurations
- Experiment with viewport and rendering options

### 2. **Build Interactive Features**

- Add drag and drop functionality
- Implement node selection and editing
- Create custom node types and styles

### 3. **Integrate with Frameworks**

- Check out [Integration Examples](./INTEGRATION_EXAMPLES.md) for React, Vue.js, Angular
- Learn how to integrate with your favorite framework
- See examples of real-time collaboration

### 4. **Advanced Usage**

- Explore [Usage Examples](./USAGE_EXAMPLES.md) for complex scenarios
- Learn about performance optimization
- Understand testing and debugging

### 5. **Customization**

- Create custom themes and styles
- Implement custom node shapes
- Add animations and transitions

## Common Issues and Solutions

### Issue: Canvas not rendering

**Solution**: Make sure the WASM module is fully loaded before calling functions:

```javascript
await init(); // Wait for initialization
create_simple_flow(); // Then create the flow
```

### Issue: Functions not found

**Solution**: Check that you're importing the correct function names:

```javascript
import { create_simple_flow } from './pkg/simple_flow_example.js';
```

### Issue: Performance problems

**Solution**: Limit the number of nodes and use efficient rendering:

```javascript
// Don't create too many nodes at once
for (let i = 0; i < 100; i++) { // Reasonable limit
    add_node_at_position(Math.random() * 800, Math.random() * 600);
}
```

### Issue: Canvas not interactive

**Solution**: Make sure event listeners are properly set up:

```javascript
canvas.addEventListener('click', handleClick);
```

## Getting Help

If you run into issues:

1. **Check the console** for error messages
2. **Verify your setup** matches the prerequisites
3. **Look at the examples** in the documentation
4. **Test with a minimal example** to isolate the problem
5. **Check the browser compatibility** (modern browsers required)

## Summary

You've now learned:

- ✅ How to set up the development environment
- ✅ How to build and run the example
- ✅ Basic concepts of nodes, edges, and graphs
- ✅ How to create your first flow diagram
- ✅ How to add interactivity
- ✅ Common issues and solutions

You're ready to start building your own flow diagrams! Check out the other documentation sections for more advanced features and examples.

Happy coding! 🚀
