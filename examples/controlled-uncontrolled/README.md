# Controlled vs Uncontrolled Example

Demonstrates Flow-RS's two state management patterns: controlled (reactive) and uncontrolled modes, showing how to choose the right approach for your use case.

## What it demonstrates

- **Controlled Mode**: Reactive state management with Leptos signals
- **Uncontrolled Mode**: Internal state management by Flow components
- **State Synchronization**: How changes flow through the application
- **API Differences**: Contrasting the two programming models

## Patterns shown

### Controlled Mode ✅
```rust
// State managed by parent component
let (nodes, set_nodes) = create_signal(create_initial_nodes());
let (edges, set_edges) = create_signal(create_initial_edges());

// Reactive updates
let add_node = move |_| {
    set_nodes.update(|nodes| {
        nodes.push(Node::simple("new_node", Position::new(x, y)));
    });
};

// Pass reactive state to Flow
<Flow nodes=nodes edges=edges />
```

### Uncontrolled Mode ✅
```rust
// Just provide initial data
let initial_nodes = create_initial_nodes();
let initial_edges = create_initial_edges();

// Flow manages its own state internally
<Flow nodes=initial_nodes edges=initial_edges />
```

## When to use each pattern

### Controlled Mode (Recommended for):
- Complex applications with multiple data sources
- When you need to coordinate flow state with other UI state
- Real-time collaboration features
- Undo/redo functionality
- Integration with external state management
- Advanced validation and business logic

### Uncontrolled Mode (Recommended for):
- Simple flow diagrams
- Prototyping and demos
- Static diagrams that don't change
- When you don't need external state coordination
- Minimal setup requirements

## Features demonstrated

- ✅ **Reactive State Management**: Leptos signals integration
- ✅ **Dynamic Updates**: Adding nodes programmatically
- ✅ **State Reset**: Restoring initial state
- ✅ **Side-by-side Comparison**: Both patterns running simultaneously
- ✅ **Leptos Integration**: Full reactive component ecosystem

## Running the example

### Quick Start

```bash
# Navigate to the example directory
cd examples/controlled-uncontrolled

# Build and serve
./build.sh

# Or build manually
wasm-pack build --target web --out-dir pkg --dev

# Serve with Python
python3 -m http.server 8000

# Open http://localhost:8000
```

## Interactive features

### Controlled Section
- **Add Node Button**: Dynamically adds new nodes to the flow
- **Reset Button**: Restores the flow to its initial state
- **Reactive Updates**: All changes are reflected immediately

### Uncontrolled Section
- **Static Display**: Shows the same initial flow
- **Internal State**: Flow manages its own state internally
- **No External Control**: Cannot be modified from outside

## Architecture benefits

### Controlled Mode Benefits
- **Full Control**: Complete authority over state changes
- **Predictable**: Changes flow through defined channels
- **Testable**: Easy to test state transformations
- **Composable**: Works well with other reactive systems

### Uncontrolled Mode Benefits
- **Simple API**: Minimal setup and boilerplate
- **Encapsulated**: Component manages its own concerns
- **Performant**: Less reactive overhead for static content
- **Stable**: Fewer moving parts, more predictable

## Performance considerations

- **Controlled**: Small performance cost for reactivity
- **Uncontrolled**: Better performance for static content
- **Hybrid**: Combine both patterns in complex applications

## Code structure

```
src/lib.rs
├── App component (main container)
├── ControlledFlowExample (reactive state)
├── UncontrolledFlowExample (internal state)
├── Helper functions (create_initial_*)
└── WASM entry point
```

## Browser compatibility

Compatible with all browsers supporting:
- WebAssembly
- ES6 modules
- Leptos reactive runtime

## Related examples

- **Hello World**: Basic static flow display
- **Custom Styles**: Node customization
- **Interactive**: User interactions and events
- **Real-time**: Live data updates
