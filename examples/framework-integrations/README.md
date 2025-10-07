# Framework Integrations Example

Comprehensive demonstration of integrating Flow-RS with popular Rust web frameworks. This example provides integration patterns, code examples, and best practices for using Flow-RS within different framework ecosystems.

## What it demonstrates

- **Multi-framework support**: Integration examples for Leptos, Yew, Dioxus, Sycamore, and vanilla Rust
- **Framework-specific patterns**: Reactive signals, component composition, hooks-based architecture, context systems
- **Integration guides**: Setup instructions and best practices for each framework
- **Code generation**: Interactive examples and generated integration code
- **Framework comparison**: Detailed comparison of integration approaches and trade-offs
- **Educational content**: Learning resources and integration patterns

## Core Integration Patterns

### **Leptos - Reactive Signals**
```rust
#[component]
pub fn FlowEditor() -> impl IntoView {
    let (graph, set_graph) = create_signal(Graph::new());

    // Reactive graph updates
    let add_node = move |_| {
        set_graph.update(|g| {
            let node = Node::new("node".to_string(), Position::new(100.0, 100.0), "New Node".to_string());
            g.add_node(node).unwrap();
        });
    };

    view! {
        <div class="flow-editor">
            <button on:click=add_node>"Add Node"</button>
            <FlowCanvas graph=graph.get()/>
        </div>
    }
}
```

### **Yew - Component Props**
```rust
#[derive(Clone, Properties, PartialEq)]
pub struct FlowProps {
    pub graph: Graph<String, String>,
    pub on_node_click: Callback<String>,
}

#[function_component(FlowEditor)]
pub fn flow_editor(props: &FlowProps) -> Html {
    html! {
        <div class="flow-editor">
            <FlowNode
                node={node.clone()}
                on_click={props.on_node_click.clone()}
            />
        </div>
    }
}
```

### **Dioxus - Hooks-based**
```rust
#[allow(non_snake_case)]
pub fn FlowEditor(cx: Scope) -> Element {
    let graph = use_state(cx, || Graph::<String, String>::new());

    let add_node = move |_| {
        graph.modify(|g| {
            let node = Node::new("node".to_string(), Position::new(100.0, 100.0), "New Node".to_string());
            g.add_node(node).unwrap();
        });
    };

    cx.render(rsx! {
        div { class: "flow-editor",
            button { onclick: add_node, "Add Node" }
            FlowCanvas { graph: graph.get() }
        }
    })
}
```

### **Sycamore - Context System**
```rust
#[component]
pub fn FlowEditor<G: Html>(cx: Scope) -> View<G> {
    let graph = create_signal(cx, Graph::<String, String>::new());

    // Provide graph context
    provide_context(cx, graph);

    view! { cx,
        div(class="flow-editor") {
            FlowCanvas {}
        }
    }
}
```

### **Generic - Direct DOM**
```rust
pub struct FlowComponent {
    canvas: HtmlCanvasElement,
    graph: Graph<String, String>,
}

impl FlowComponent {
    pub fn add_node(&mut self, label: &str, x: f64, y: f64) {
        let node = Node::new("node".to_string(), Position::new(x, y), label.to_string());
        self.graph.add_node(node).unwrap();
        self.render();
    }
}
```

## Framework Comparison

| Framework | Pattern | Complexity | Best For | Performance |
|-----------|---------|------------|----------|-------------|
| **Leptos** | Reactive Signals | Medium | Complex reactive apps | Excellent |
| **Yew** | Component Props | Low | Enterprise apps | Good |
| **Dioxus** | Hooks-based | Medium | Modern React-like | Excellent |
| **Sycamore** | Context System | High | High-performance | Outstanding |
| **Generic** | Direct DOM | Low | Minimal overhead | Best |

## Integration Features

### **Framework-Specific Guides**
Each framework integration includes:
- **Setup instructions**: Step-by-step configuration
- **Integration patterns**: Best practices and common patterns
- **Pros and cons**: Trade-offs and considerations
- **Use cases**: When to choose each framework
- **Code examples**: Working integration code

### **Interactive Exploration**
- **Framework selection**: Click framework cards to explore options
- **Integration guides**: Detailed setup and usage instructions
- **Code generation**: Generate ready-to-use integration code
- **Pattern comparison**: Side-by-side framework comparison
- **Live examples**: Interactive demonstrations

### **Educational Resources**
- **Pattern explanations**: Understanding different integration approaches
- **Best practices**: Framework-specific recommendations
- **Performance considerations**: Optimization techniques
- **Migration guides**: Moving between frameworks
- **Troubleshooting**: Common integration issues

## JavaScript API

### **Framework Exploration**
```javascript
// Show integration guide for a framework
flow_integrations.show_framework_guide('leptos');

// Show integration patterns
flow_integrations.show_integration_pattern('reactive');

// Compare all frameworks
flow_integrations.compare_frameworks();

// Generate integration code
const code = flow_integrations.generate_integration_code('yew');

// Get framework features
const features = flow_integrations.get_framework_features('dioxus');

// Create sample integration
const sample = flow_integrations.create_sample_integration('leptos');
```

## Technical Architecture

### **Integration Layer**
```rust
// Framework-agnostic Flow-RS integration
pub trait FlowIntegration {
    type Component;
    type State;

    fn create_component(&self, config: IntegrationConfig) -> Self::Component;
    fn update_state(&mut self, state: Self::State);
    fn render(&self) -> Result<(), IntegrationError>;
}
```

### **Framework Adapters**
```rust
// Framework-specific adapters
pub struct LeptosAdapter;
pub struct YewAdapter;
pub struct DioxusAdapter;
pub struct SycamoreAdapter;

// Generic adapter for custom frameworks
pub struct GenericAdapter<T> {
    _phantom: PhantomData<T>,
}
```

### **State Management**
```rust
// Unified state interface
pub trait FlowState {
    fn get_graph(&self) -> &Graph<String, String>;
    fn update_graph<F>(&mut self, updater: F) where F: FnOnce(&mut Graph<String, String>);
    fn subscribe<F>(&self, callback: F) where F: Fn(&Graph<String, String>) + 'static;
}
```

## Performance Considerations

### **Bundle Size Impact**
- **Leptos**: ~50KB additional (signals + components)
- **Yew**: ~45KB additional (component system)
- **Dioxus**: ~40KB additional (hooks + virtual DOM)
- **Sycamore**: ~35KB additional (reactive system)
- **Generic**: ~5KB additional (minimal overhead)

### **Runtime Performance**
- **Sycamore**: Best fine-grained reactivity performance
- **Dioxus**: Excellent virtual DOM optimization
- **Leptos**: Strong reactive performance
- **Yew**: Solid component performance
- **Generic**: Maximum raw performance

### **Development Experience**
- **Dioxus**: Most familiar for React developers
- **Leptos**: Excellent Rust-native experience
- **Yew**: Mature and stable development
- **Sycamore**: Simple and intuitive
- **Generic**: Maximum control and flexibility

## Use Cases by Framework

### **Leptos Integration**
- **Complex reactive applications**: Real-time collaborative editing
- **Data visualization dashboards**: Live data flow monitoring
- **Interactive process modeling**: Complex workflow design
- **Educational tools**: Interactive learning platforms

### **Yew Integration**
- **Enterprise applications**: Large-scale graph editors
- **Content management systems**: Node-based content workflows
- **API design tools**: REST/GraphQL API modeling
- **System architecture tools**: Infrastructure diagramming

### **Dioxus Integration**
- **Cross-platform applications**: Web + desktop + mobile
- **Modern web applications**: React-like development experience
- **Real-time collaboration**: Multi-user editing interfaces
- **Developer tools**: Code visualization and debugging

### **Sycamore Integration**
- **High-performance applications**: Maximum rendering performance
- **Complex state management**: Intricate graph relationships
- **Real-time data processing**: Live data stream visualization
- **Scientific computing**: Mathematical graph modeling

### **Generic Integration**
- **Minimal applications**: Small bundle size requirements
- **Custom frameworks**: Proprietary or specialized frameworks
- **Performance-critical**: Maximum raw performance needed
- **Learning projects**: Understanding core Flow-RS concepts

## Integration Best Practices

### **State Management**
- **Choose appropriate reactivity**: Match framework capabilities to needs
- **Avoid unnecessary updates**: Debounce rapid state changes
- **Handle async operations**: Proper loading and error states
- **Memory management**: Clean up subscriptions and resources

### **Performance Optimization**
- **Virtual scrolling**: Handle large graphs efficiently
- **Incremental rendering**: Only update changed elements
- **Debounced updates**: Prevent excessive re-renders
- **Memory pooling**: Reuse objects to reduce allocations

### **User Experience**
- **Consistent interactions**: Framework-native UI patterns
- **Loading states**: Proper feedback during operations
- **Error handling**: Graceful error recovery
- **Accessibility**: Framework-specific accessibility features

## Migration Between Frameworks

### **Leptos ↔ Dioxus**
- **Similar patterns**: Both use hooks/reactive primitives
- **State conversion**: Signals ↔ use_state hooks
- **Component migration**: Similar component structure

### **Yew ↔ Generic**
- **Props system**: Yew props ↔ direct function parameters
- **State management**: use_state ↔ manual state handling
- **Event handling**: Yew callbacks ↔ direct event listeners

### **Sycamore ↔ Leptos**
- **Reactive systems**: Both use signals/context
- **Component syntax**: view! macro differences
- **Context usage**: provide_context patterns

## Troubleshooting

### **Common Issues**
- **Framework version conflicts**: Ensure compatible versions
- **State update patterns**: Framework-specific update mechanisms
- **Event handling conflicts**: Multiple event system interactions
- **Memory leaks**: Proper cleanup of reactive subscriptions

### **Debugging Techniques**
- **Framework dev tools**: Use framework-specific debugging
- **Console logging**: Flow-RS integration logging
- **Performance profiling**: Framework performance tools
- **State inspection**: Framework state debugging

## Contributing Integration Examples

### **Adding New Frameworks**
1. **Framework analysis**: Study framework patterns and APIs
2. **Integration design**: Design Flow-RS integration layer
3. **Example implementation**: Create working example
4. **Documentation**: Add framework guide and examples
5. **Testing**: Verify integration works correctly

### **Example Structure**
```rust
// New framework integration
pub mod new_framework_integration {
    use flow_rs_core::Graph;

    pub struct NewFrameworkAdapter {
        // Framework-specific state
    }

    impl NewFrameworkAdapter {
        pub fn integrate_flow_rs(&self, graph: &Graph<String, String>) {
            // Integration logic
        }
    }
}
```

---

## Key Takeaway

**Flow-RS Framework Integrations demonstrate how to seamlessly integrate Flow-RS with any Rust web framework, providing developers with the flexibility to choose their preferred framework while leveraging Flow-RS's powerful graph editing capabilities. The integrations showcase different architectural approaches and performance characteristics, enabling developers to select the optimal framework for their specific use case while maintaining consistent Flow-RS functionality.**

The comprehensive integration examples serve as both a reference implementation and a foundation for building sophisticated graph-based applications across the entire Rust web ecosystem.
