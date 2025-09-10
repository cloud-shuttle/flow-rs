# Leptos Flow Examples

## Overview

This directory contains comprehensive examples demonstrating various Leptos Flow features and use cases. Each example is self-contained with clear documentation and can serve as a starting point for your own projects.

## Quick Start

To run any example:

```bash
cd examples/[example-name]
trunk serve --open
```

## Basic Examples

### 1. Simple Flow
**Path**: `examples/simple-flow/`  
**Complexity**: Beginner  
**Features**: Basic nodes, edges, drag and drop

A minimal flow editor with two connected nodes. Perfect for understanding the core concepts.

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_nodes_change=set_nodes
        on_edges_change=set_edges
    />
}
```

**Key Learning Points**:
- Basic FlowEditor setup
- Signal-based reactivity
- Node and edge creation
- Simple event handling

---

### 2. Custom Nodes
**Path**: `examples/custom-nodes/`  
**Complexity**: Beginner  
**Features**: Custom node components, styling, handles

Demonstrates how to create custom node types with different appearances and behaviors.

```rust
#[component]
pub fn InputNode() -> impl IntoView {
    view! {
        <div class="input-node">
            <Handle handle_type=HandleType::Source position=HandlePosition::Right />
            <input type="text" placeholder="Enter value..." />
        </div>
    }
}
```

**Key Learning Points**:
- Custom node components
- Handle positioning
- CSS styling integration
- Node type registration

---

### 3. Interactive Controls
**Path**: `examples/interactive-controls/`  
**Complexity**: Beginner  
**Features**: Zoom controls, minimap, selection

Shows how to add interactive controls and enhance user experience.

```rust
view! {
    <FlowEditor nodes=nodes edges=edges>
        <Controls position=ControlPosition::TopLeft />
        <MiniMap position=MiniMapPosition::BottomRight />
        <Background variant=BackgroundVariant::Dots />
    </FlowEditor>
}
```

**Key Learning Points**:
- Built-in UI components
- Viewport management
- Visual enhancements
- Component composition

## Intermediate Examples

### 4. Styled Flow Editor
**Path**: `examples/styled-editor/`  
**Complexity**: Intermediate  
**Features**: Custom themes, CSS-in-Rust, animations

Advanced styling techniques and theming system.

```rust
let dark_theme = Theme {
    background: "#1a1a1a".to_string(),
    node_bg: "#2d2d2d".to_string(),
    edge_color: "#666666".to_string(),
    selection: "#4a9eff".to_string(),
};

view! {
    <FlowEditor nodes=nodes edges=edges theme=dark_theme />
}
```

**Key Learning Points**:
- Theme system usage
- Dynamic styling
- CSS custom properties
- Animation integration

---

### 5. Form-Based Nodes
**Path**: `examples/form-nodes/`  
**Complexity**: Intermediate  
**Features**: Input validation, real-time updates, data binding

Interactive nodes with form controls and validation.

```rust
#[component]
pub fn FormNode(node: Node<FormData>) -> impl IntoView {
    let (value, set_value) = create_signal(node.data.value.clone());
    
    view! {
        <div class="form-node">
            <input 
                type="text"
                value=move || value.get()
                on:input=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_value.set(new_value.clone());
                    update_node_data(node.id.clone(), new_value);
                }
            />
        </div>
    }
}
```

**Key Learning Points**:
- Form integration
- Real-time data binding
- Validation patterns
- State synchronization

---

### 6. Multi-Selection & Keyboard
**Path**: `examples/multi-selection/`  
**Complexity**: Intermediate  
**Features**: Multi-selection, keyboard shortcuts, bulk operations

Advanced selection patterns and keyboard interaction.

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        multi_selection=true
        selection_key=Some(SelectionKey::Shift)
        on_key_down=handle_keyboard_shortcuts
        on_selection_change=handle_selection_change
    />
}
```

**Key Learning Points**:
- Multi-selection implementation
- Keyboard event handling
- Bulk operations
- Accessibility considerations

## Advanced Examples

### 7. Large Graph Performance
**Path**: `examples/large-graph/`  
**Complexity**: Advanced  
**Features**: 10k+ nodes, viewport culling, performance optimization

Demonstrates handling large datasets with optimal performance.

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        only_render_visible_elements=true
        renderer=RendererType::WebGL2
        performance_config=PerformanceConfig::optimized()
    />
}
```

**Key Learning Points**:
- Performance optimization
- Viewport culling
- Renderer selection
- Memory management

---

### 8. Real-Time Collaboration
**Path**: `examples/collaboration/`  
**Complexity**: Advanced  
**Features**: WebSocket integration, conflict resolution, real-time updates

Multi-user collaborative flow editing.

```rust
let ws = use_websocket("ws://localhost:8080/collaboration");

create_effect(move |_| {
    if let Some(message) = ws.message.get() {
        handle_remote_update(message);
    }
});
```

**Key Learning Points**:
- WebSocket integration
- Real-time synchronization
- Conflict resolution
- Multi-user UX patterns

---

### 9. Data Pipeline Builder
**Path**: `examples/data-pipeline/`  
**Complexity**: Advanced  
**Features**: Data processing, type validation, execution engine

Visual programming interface for data transformations.

```rust
#[component]
pub fn ProcessorNode(node: Node<ProcessorData>) -> impl IntoView {
    let process_data = move |input_data: Value| {
        match node.data.processor_type {
            ProcessorType::Map => apply_map_function(input_data),
            ProcessorType::Filter => apply_filter_function(input_data),
            ProcessorType::Reduce => apply_reduce_function(input_data),
        }
    };
    
    // Node implementation...
}
```

**Key Learning Points**:
- Visual programming patterns
- Data flow execution
- Type system integration
- Complex node behaviors

## Specialized Examples

### 10. State Machine Designer
**Path**: `examples/state-machine/`  
**Complexity**: Advanced  
**Features**: State transitions, validation, code generation

Visual state machine editor with validation and export.

**Key Learning Points**:
- Domain-specific modeling
- Validation systems
- Code generation
- Export functionality

---

### 11. Network Topology Viewer
**Path**: `examples/network-topology/`  
**Complexity**: Advanced  
**Features**: Live data integration, monitoring, alerts

Network infrastructure visualization with real-time monitoring.

**Key Learning Points**:
- Live data integration
- Monitoring patterns
- Alert systems
- Domain-specific visualization

---

### 12. Workflow Automation
**Path**: `examples/workflow-automation/`  
**Complexity**: Advanced  
**Features**: Business processes, approval flows, integration

Business process automation and workflow design.

**Key Learning Points**:
- Business process modeling
- Approval workflows
- Integration patterns
- Enterprise features

## Testing & Quality Examples

### 13. Visual Regression Testing
**Path**: `examples/visual-testing/`  
**Complexity**: Advanced  
**Features**: Automated testing, screenshot comparison

Comprehensive visual regression testing setup.

```rust
#[cfg(test)]
mod visual_tests {
    use super::*;
    use leptos_flow_testing::*;

    #[tokio::test]
    async fn test_node_rendering() {
        let component = create_test_flow(test_data());
        let screenshot = render_component(component).await;
        assert_visual_match("node_rendering", screenshot);
    }
}
```

**Key Learning Points**:
- Testing strategies
- Visual regression testing
- Test automation
- Quality assurance

## Performance Examples

### 14. Benchmark Suite
**Path**: `examples/benchmarks/`  
**Complexity**: Advanced  
**Features**: Performance measurement, profiling, optimization

Comprehensive performance benchmarking and profiling tools.

**Key Learning Points**:
- Performance measurement
- Profiling techniques
- Optimization strategies
- Benchmark automation

## Integration Examples

### 15. React Interop
**Path**: `examples/react-integration/`  
**Complexity**: Advanced  
**Features**: React components, WASM bindings, TypeScript

Integration with existing React applications.

**Key Learning Points**:
- Framework interoperability
- WASM bindings
- TypeScript integration
- Migration strategies

---

### 16. Tauri Desktop App
**Path**: `examples/tauri-desktop/`  
**Complexity**: Advanced  
**Features**: Desktop application, file system, native features

Desktop application using Tauri with native features.

**Key Learning Points**:
- Desktop application development
- File system integration
- Native API access
- Cross-platform deployment

## Getting Started

### Prerequisites

```bash
# Install Rust and wasm32 target
rustup target add wasm32-unknown-unknown

# Install trunk for web development
cargo install trunk

# Install wasm-pack (optional, for manual builds)
cargo install wasm-pack
```

### Running Examples

1. **Navigate to example directory**:
   ```bash
   cd examples/simple-flow
   ```

2. **Start development server**:
   ```bash
   trunk serve --open
   ```

3. **Build for production**:
   ```bash
   trunk build --release
   ```

### Example Structure

Each example follows a consistent structure:

```
example-name/
├── Cargo.toml          # Dependencies and configuration
├── Trunk.toml          # Trunk build configuration
├── index.html          # HTML template
├── src/
│   ├── main.rs         # Entry point
│   ├── app.rs          # Main application component
│   ├── components/     # Custom components
│   └── styles/         # CSS styles
├── README.md           # Example-specific documentation
└── screenshots/        # Example screenshots
```

### Common Dependencies

Most examples use these common dependencies:

```toml
[dependencies]
leptos = "0.6"
leptos-flow = "0.1"
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
web-sys = "0.3"
serde = { version = "1.0", features = ["derive"] }
```

## Learning Path

### Beginner Path
1. Simple Flow → Custom Nodes → Interactive Controls
2. Focus on basic concepts and API familiarity
3. Experiment with styling and theming

### Intermediate Path
1. Form-Based Nodes → Multi-Selection → Styled Editor
2. Learn advanced interaction patterns
3. Understand performance considerations

### Advanced Path
1. Large Graph → Real-Time Collaboration → Data Pipeline
2. Master performance optimization
3. Build complex, domain-specific applications

## Contributing Examples

We welcome new examples! When contributing:

1. **Follow the standard structure** outlined above
2. **Include comprehensive README** with learning objectives
3. **Add meaningful comments** explaining key concepts
4. **Provide screenshots** showing the expected result
5. **Test thoroughly** across different browsers
6. **Keep examples focused** on specific features or use cases

### Example Contribution Checklist

- [ ] Clear learning objectives defined
- [ ] Code is well-commented and documented
- [ ] README includes setup instructions
- [ ] Screenshots demonstrate functionality
- [ ] Example builds and runs without errors
- [ ] Performance considerations documented
- [ ] Accessibility features included where relevant
- [ ] Mobile responsiveness tested

## Support and Community

- **Discord**: Join our community for real-time help
- **GitHub Issues**: Report bugs or request new examples
- **Discussions**: Share your own examples and use cases
- **Documentation**: Refer to the comprehensive API docs

## License

All examples are provided under the same license as Leptos Flow (MIT/Apache-2.0), allowing you to use them as starting points for your own projects.