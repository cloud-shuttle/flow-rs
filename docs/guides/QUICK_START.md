# Leptos Flow Quick Start Guide

## Overview

Get up and running with Leptos Flow in under 10 minutes. This guide walks through installation, basic setup, and creating your first interactive flow diagram.

## Installation

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+ (for development tools)
- A Leptos project (0.6+)

### Add Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.6"
leptos-flow = "0.1"

# Optional: For advanced rendering
[features]
webgpu = ["leptos-flow/webgpu"]
```

### Install Development Tools

```bash
# Install wasm-pack for building
cargo install wasm-pack

# Install trunk for local development
cargo install trunk

# For TypeScript bindings (optional)
npm install -g wasm-pack
```

## Basic Usage

### 1. Create Your First Flow

Create a new component with a simple node editor:

```rust
use leptos::*;
use leptos_flow::*;

#[component]
pub fn SimpleFlow() -> impl IntoView {
    // Define node data structure
    #[derive(Clone, Debug, Default)]
    pub struct NodeData {
        pub label: String,
        pub value: i32,
    }

    // Define edge data structure
    #[derive(Clone, Debug, Default)]
    pub struct EdgeData {
        pub weight: f64,
    }

    // Create reactive signals for nodes and edges
    let (nodes, set_nodes) = create_signal(vec![
        Node::builder("1")
            .position(100.0, 100.0)
            .data(NodeData {
                label: "Input".to_string(),
                value: 42,
            })
            .build(),
        Node::builder("2")
            .position(300.0, 200.0)
            .data(NodeData {
                label: "Output".to_string(),
                value: 0,
            })
            .build(),
    ]);

    let (edges, set_edges) = create_signal(vec![
        Edge::builder()
            .connect("1", "2")
            .data(EdgeData { weight: 1.0 })
            .build(),
    ]);

    view! {
        <div style="width: 100%; height: 600px; border: 1px solid #ccc;">
            <FlowEditor
                nodes=nodes
                edges=edges
                on_nodes_change=set_nodes
                on_edges_change=set_edges
                on_connect=move |connection| {
                    // Handle new connections
                    let new_edge = Edge::builder()
                        .connect(&connection.source, &connection.target)
                        .data(EdgeData { weight: 1.0 })
                        .build();

                    set_edges.update(|edges| edges.push(new_edge));
                }
            />
        </div>
    }
}
```

### 2. Custom Node Components

Create custom node rendering:

```rust
#[component]
pub fn CustomNode(
    #[prop(into)] node: MaybeSignal<Node<NodeData>>,
    #[prop(optional)] on_change: Option<Callback<Node<NodeData>>>,
) -> impl IntoView {
    let node_data = move || node.get();

    view! {
        <div class="custom-node"
             style="
                 padding: 10px;
                 border: 2px solid #1a365d;
                 border-radius: 8px;
                 background: white;
                 box-shadow: 0 2px 4px rgba(0,0,0,0.1);
             ">

            // Node handles for connections
            <Handle
                handle_type=HandleType::Source
                position=HandlePosition::Right
                id="output"
            />
            <Handle
                handle_type=HandleType::Target
                position=HandlePosition::Left
                id="input"
            />

            // Node content
            <div class="node-content">
                <h3>{move || node_data().data.label.clone()}</h3>
                <div class="value">
                    "Value: " {move || node_data().data.value}
                </div>
            </div>
        </div>
    }
}
```

### 3. Using Custom Components

Register and use your custom components:

```rust
view! {
    <div style="width: 100%; height: 600px;">
        <FlowEditor
            nodes=nodes
            edges=edges
            on_nodes_change=set_nodes
            on_edges_change=set_edges
        >
            // Register custom node types
            <NodeType name="custom" component=CustomNode />
            <NodeType name="input" component=InputNode />
            <NodeType name="output" component=OutputNode />

            // Add controls and minimap
            <Controls position=ControlPosition::TopLeft />
            <MiniMap position=MiniMapPosition::BottomRight />
            <Background variant=BackgroundVariant::Dots />
        </FlowEditor>
    </div>
}
```

## Interactive Features

### 1. Node Selection and Multi-Selection

```rust
let (selected_nodes, set_selected_nodes) = create_signal(Vec::<String>::new());

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_selection_change=move |selection: Selection| {
            set_selected_nodes.set(selection.nodes);
        }
        multi_selection=true
        selection_key=Some(SelectionKey::Shift)
    />
}
```

### 2. Custom Event Handling

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_node_click=move |node_id: String| {
            logging::log!("Node clicked: {}", node_id);
        }
        on_node_double_click=move |node_id: String| {
            // Open node editor or details panel
            show_node_details(&node_id);
        }
        on_edge_click=move |edge_id: String| {
            logging::log!("Edge clicked: {}", edge_id);
        }
        on_pane_click=move |event: MouseEvent| {
            // Clear selection or create new node
            if event.detail() == 2 { // Double click
                create_new_node_at(event.client_x(), event.client_y());
            }
        }
    />
}
```

### 3. Dynamic Node Creation

```rust
fn create_new_node_at(x: i32, y: i32) {
    let flow_instance = use_flow_instance();

    // Convert screen coordinates to flow coordinates
    let flow_pos = flow_instance.screen_to_flow_position(Point::new(x as f64, y as f64));

    let new_node = Node::builder(format!("node_{}", generate_id()))
        .position(flow_pos.x, flow_pos.y)
        .data(NodeData {
            label: "New Node".to_string(),
            value: 0,
        })
        .build();

    set_nodes.update(|nodes| nodes.push(new_node));
}
```

## Styling and Theming

### 1. CSS Styling

Add CSS for your custom components:

```css
/* styles.css */
.custom-node {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    transition: all 0.2s ease;
}

.custom-node:hover {
    box-shadow: 0 4px 8px rgba(0,0,0,0.15);
    transform: translateY(-1px);
}

.custom-node.selected {
    border-color: #3182ce;
    box-shadow: 0 0 0 2px rgba(49, 130, 206, 0.3);
}

.node-content h3 {
    margin: 0 0 8px 0;
    font-size: 14px;
    font-weight: 600;
    color: #2d3748;
}

.value {
    font-size: 12px;
    color: #4a5568;
    background: #f7fafc;
    padding: 4px 8px;
    border-radius: 4px;
}
```

### 2. Dynamic Styling with Leptos

```rust
#[component]
pub fn StyledNode(
    #[prop(into)] node: MaybeSignal<Node<NodeData>>,
) -> impl IntoView {
    let node_data = move || node.get();

    // Dynamic styles based on node state
    let node_style = move || {
        let node = node_data();
        let base_color = if node.selected { "#3182ce" } else { "#1a365d" };
        let bg_color = if node.data.value > 50 { "#e6fffa" } else { "#ffffff" };

        format!(
            "border-color: {}; background-color: {}; padding: 10px; border-radius: 8px;",
            base_color, bg_color
        )
    };

    view! {
        <div class="custom-node" style=node_style>
            <Handle handle_type=HandleType::Target position=HandlePosition::Left />
            <Handle handle_type=HandleType::Source position=HandlePosition::Right />

            <div class="content">
                {move || node_data().data.label.clone()}
            </div>
        </div>
    }
}
```

## Layout and Auto-Arrangement

### 1. Force-Directed Layout

```rust
use leptos_flow::layout::*;

let (apply_layout, _) = create_signal(());

// Apply layout when signal changes
create_effect(move |_| {
    apply_layout.get(); // Trigger when signal updates

    let flow = use_flow_instance();
    let layout = ForceDirectedLayout::builder()
        .iterations(100)
        .spring_strength(0.5)
        .repulsion_strength(1000.0)
        .build();

    // Apply layout asynchronously
    spawn_local(async move {
        flow.apply_layout(layout).await.unwrap();
    });
});

view! {
    <div>
        <button on:click=move |_| apply_layout.set(())>
            "Auto Layout"
        </button>
        <FlowEditor nodes=nodes edges=edges />
    </div>
}
```

### 2. Hierarchical Layout

```rust
let hierarchical_layout = HierarchicalLayout::builder()
    .direction(LayoutDirection::TopDown)
    .level_separation(60)
    .node_separation(40)
    .build();

view! {
    <button on:click=move |_| {
        let flow = use_flow_instance();
        spawn_local(async move {
            flow.apply_layout(hierarchical_layout.clone()).await.unwrap();
        });
    }>
        "Arrange Hierarchically"
    </button>
}
```

## State Management

### 1. Undo/Redo

```rust
let history = use_history();

view! {
    <div class="controls">
        <button
            on:click=move |_| history.undo()
            disabled=move || !history.can_undo.get()
        >
            "Undo"
        </button>
        <button
            on:click=move |_| history.redo()
            disabled=move || !history.can_redo.get()
        >
            "Redo"
        </button>
    </div>

    <FlowEditor
        nodes=nodes
        edges=edges
        on_nodes_change=move |new_nodes| {
            history.push_state(FlowState {
                nodes: new_nodes.clone(),
                edges: edges.get(),
            });
            set_nodes.set(new_nodes);
        }
    />
}
```

### 2. Persistence

```rust
// Save flow to localStorage
let save_flow = move || {
    let flow_data = FlowData {
        nodes: nodes.get(),
        edges: edges.get(),
    };

    let json = serde_json::to_string(&flow_data).unwrap();
    let storage = window().local_storage().unwrap().unwrap();
    storage.set_item("flow_data", &json).unwrap();
};

// Load flow from localStorage
let load_flow = move || {
    let storage = window().local_storage().unwrap().unwrap();
    if let Ok(Some(json)) = storage.get_item("flow_data") {
        if let Ok(flow_data) = serde_json::from_str::<FlowData>(&json) {
            set_nodes.set(flow_data.nodes);
            set_edges.set(flow_data.edges);
        }
    }
};

view! {
    <div class="toolbar">
        <button on:click=move |_| save_flow()>"Save"</button>
        <button on:click=move |_| load_flow()>"Load"</button>
    </div>
}
```

## Next Steps

Now that you have a basic flow editor working, explore these advanced topics:

1. **[Custom Renderers](RENDERING.md)** - Create WebGPU or custom Canvas renderers
2. **[Advanced Layouts](../api/LAYOUTS.md)** - Implement custom layout algorithms
3. **[Performance Optimization](../performance/PERFORMANCE.md)** - Handle large graphs efficiently
4. **[Testing](../dev/TESTING.md)** - Test your flow editor components
5. **[Deployment](../dev/DEPLOYMENT.md)** - Build and deploy your application

## Common Patterns

### Real-Time Collaboration

```rust
// WebSocket integration for real-time updates
let ws = use_websocket("ws://localhost:8080/flow");

create_effect(move |_| {
    if let Some(message) = ws.message.get() {
        if let Ok(update) = serde_json::from_str::<FlowUpdate>(&message) {
            match update {
                FlowUpdate::NodeMoved { id, position } => {
                    set_nodes.update(|nodes| {
                        if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                            node.position = position;
                        }
                    });
                }
                // Handle other update types...
            }
        }
    }
});
```

### Data Flow Processing

```rust
// Process data through connected nodes
fn process_data_flow() {
    let flow = use_flow_instance();
    let nodes = flow.get_nodes();
    let edges = flow.get_edges();

    // Build execution graph
    let execution_order = flow.topological_sort();

    for node_id in execution_order {
        let node = flow.get_node(&node_id).unwrap();
        let inputs = flow.get_node_inputs(&node_id);
        let output = process_node_data(&node, inputs);
        flow.set_node_output(&node_id, output);
    }
}
```

This quick start guide should get you productive with Leptos Flow quickly. Check out the other guides for more advanced usage patterns and best practices.
