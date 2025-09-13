# Leptos Flow User Manual

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Concepts](#basic-concepts)
3. [Creating Nodes](#creating-nodes)
4. [Connecting Edges](#connecting-edges)
5. [Interactions](#interactions)
6. [Styling](#styling)
7. [Custom Node Types](#custom-node-types)
8. [Performance Tuning](#performance-tuning)
9. [Troubleshooting](#troubleshooting)
10. [Advanced Topics](#advanced-topics)

## Getting Started

### What is Leptos Flow?

Leptos Flow is a powerful, reactive flow-based node editor built specifically for the Leptos framework. It enables you to create interactive diagrams, visual programming interfaces, workflow designers, and data flow applications with high performance and type safety.

### Key Features

- 🚀 **High Performance**: Handles 10,000+ nodes at 60 FPS
- 🦀 **Rust + WASM**: Type safety and zero-cost abstractions
- ⚡ **Reactive**: Built on Leptos' fine-grained reactivity
- 🎨 **Customizable**: Flexible styling and theming system
- 📱 **Cross-Platform**: Works on desktop and mobile browsers
- 🔧 **Developer Friendly**: Comprehensive API and tooling

### Installation

Add Leptos Flow to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.6"
flow-rs = "0.1"
```

For advanced rendering features:

```toml
[features]
default = ["flow-rs/canvas2d"]
webgpu = ["flow-rs/webgpu"]
full = ["flow-rs/full"]
```

## Basic Concepts

### Core Components

#### FlowEditor

The main component that renders your flow diagram:

```rust
view! {
    <FlowEditor
        nodes=nodes_signal
        edges=edges_signal
        on_nodes_change=set_nodes
        on_edges_change=set_edges
    />
}
```

#### Nodes

Interactive elements that can be connected:

```rust
let node = Node::builder("unique-id")
    .position(100.0, 200.0)
    .data(MyCustomData::default())
    .build();
```

#### Edges

Connections between nodes:

```rust
let edge = Edge::builder()
    .connect("source-node-id", "target-node-id")
    .build();
```

### Data Flow

Leptos Flow follows a reactive data flow pattern:

```
User Interaction → Event → Signal Update → Re-render
```

All state changes flow through Leptos signals, ensuring efficient updates and consistent state management.

## Creating Nodes

### Basic Node Creation

```rust
use leptos::*;
use leptos_flow::*;

#[component]
pub fn MyFlowEditor() -> impl IntoView {
    // Define your data structure
    #[derive(Clone, Debug)]
    pub struct NodeData {
        pub title: String,
        pub value: i32,
    }

    // Create nodes with your data
    let initial_nodes = vec![
        Node::builder("node-1")
            .position(100.0, 100.0)
            .data(NodeData {
                title: "Input".to_string(),
                value: 42,
            })
            .build(),
        Node::builder("node-2")
            .position(400.0, 200.0)
            .data(NodeData {
                title: "Process".to_string(),
                value: 0,
            })
            .build(),
    ];

    let (nodes, set_nodes) = create_signal(initial_nodes);
    let (edges, set_edges) = create_signal(Vec::new());

    view! {
        <FlowEditor
            nodes=nodes
            edges=edges
            on_nodes_change=set_nodes
            on_edges_change=set_edges
        />
    }
}
```

### Node Properties

#### Position and Size

```rust
let node = Node::builder("my-node")
    .position(100.0, 150.0)           // X, Y coordinates
    .size(200.0, 100.0)               // Width, height
    .build();
```

#### Node Types

```rust
let input_node = Node::builder("input")
    .node_type("input")               // Custom type identifier
    .position(50.0, 100.0)
    .build();

let output_node = Node::builder("output")
    .node_type("output")
    .position(350.0, 100.0)
    .build();
```

#### Visual Properties

```rust
let styled_node = Node::builder("styled")
    .position(200.0, 200.0)
    .style(NodeStyle {
        background_color: Some("#ff6b6b".to_string()),
        border_color: Some("#c92a2a".to_string()),
        border_width: Some(2.0),
        border_radius: Some(8.0),
        ..Default::default()
    })
    .class_name("my-custom-node")     // CSS class
    .build();
```

### Dynamic Node Creation

Add nodes based on user interaction:

```rust
let create_node_at = move |x: f64, y: f64| {
    let new_node = Node::builder(format!("node-{}", generate_id()))
        .position(x, y)
        .data(NodeData::default())
        .build();

    set_nodes.update(|nodes| nodes.push(new_node));
};

view! {
    <div on:dblclick=move |e| {
        let rect = e.target().unwrap().get_bounding_client_rect();
        create_node_at(
            e.client_x() as f64 - rect.left(),
            e.client_y() as f64 - rect.top()
        );
    }>
        <FlowEditor nodes=nodes edges=edges />
    </div>
}
```

## Connecting Edges

### Basic Edge Creation

```rust
// Connect two nodes
let edge = Edge::builder()
    .id("connection-1")
    .connect("source-node", "target-node")
    .build();

// Add to edges signal
set_edges.update(|edges| edges.push(edge));
```

### Handle-Based Connections

Define specific connection points on nodes:

```rust
#[component]
pub fn CustomNode() -> impl IntoView {
    view! {
        <div class="node">
            // Input handle (left side)
            <Handle
                handle_type=HandleType::Target
                position=HandlePosition::Left
                id="input"
            />

            // Node content
            <div class="content">"My Node"</div>

            // Output handle (right side)
            <Handle
                handle_type=HandleType::Source
                position=HandlePosition::Right
                id="output"
            />
        </div>
    }
}

// Connect using specific handles
let edge = Edge::builder()
    .connect_handles("node-1", "output", "node-2", "input")
    .build();
```

### Connection Events

Handle user-initiated connections:

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_connect=move |connection: Connection| {
            // User dragged from one handle to another
            let new_edge = Edge::builder()
                .connect(&connection.source, &connection.target)
                .source_handle(connection.source_handle)
                .target_handle(connection.target_handle)
                .build();

            set_edges.update(|edges| edges.push(new_edge));
        }
        on_connect_start=move |_node_id, _handle_id| {
            // Connection drag started
            logging::log!("Connection started");
        }
        on_connect_end=move |event| {
            // Connection drag ended (may not result in connection)
            if event.connection.is_none() {
                logging::log!("Connection cancelled");
            }
        }
    />
}
```

### Edge Styling

```rust
let styled_edge = Edge::builder()
    .connect("node-1", "node-2")
    .style(EdgeStyle {
        stroke: Some("#ff6b6b".to_string()),
        stroke_width: Some(3.0),
        stroke_dasharray: Some("5,5".to_string()),
        ..Default::default()
    })
    .animated(true)                   // Animated flow
    .label("Data Flow".to_string())   // Edge label
    .build();
```

## Interactions

### Selection

#### Single Selection

```rust
let (selected_nodes, set_selected_nodes) = create_signal(Vec::<String>::new());

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_selection_change=move |selection: Selection| {
            set_selected_nodes.set(selection.nodes);
        }
    />
}
```

#### Multi-Selection

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        multi_selection=true
        selection_key=Some(SelectionKey::Shift)  // Hold Shift for multi-select
        on_selection_change=move |selection| {
            // Handle multiple selected nodes
            for node_id in &selection.nodes {
                logging::log!("Selected: {}", node_id);
            }
        }
    />
}
```

### Dragging

Control drag behavior:

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        node_drag_threshold=5.0        // Pixels before drag starts
        snap_to_grid=Some(SnapToGrid::new(10))  // Snap to 10px grid
        on_node_drag_start=move |node_id| {
            logging::log!("Started dragging: {}", node_id);
        }
        on_node_drag=move |node_id, position| {
            // Real-time drag updates
        }
        on_node_drag_stop=move |node_id, position| {
            // Final position
            set_nodes.update(|nodes| {
                if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
                    node.position = position;
                }
            });
        }
    />
}
```

### Panning and Zooming

#### Viewport Controls

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        min_zoom=0.1                   // 10% minimum zoom
        max_zoom=4.0                   // 400% maximum zoom
        default_zoom=1.0               // Start at 100%
        fit_view_on_init=true          // Automatically fit content
        pan_on_scroll=true             // Pan with scroll wheel
        zoom_on_scroll=true            // Zoom with Ctrl+scroll
        on_move=move |viewport| {
            // Viewport changed (pan/zoom)
            logging::log!("Viewport: {:?}", viewport);
        }
    />
}
```

#### Programmatic Control

```rust
let flow_instance = use_flow_instance();

// Zoom controls
let zoom_in = move |_| {
    flow_instance.zoom_in(ZoomOptions::default());
};

let zoom_out = move |_| {
    flow_instance.zoom_out(ZoomOptions::default());
};

let fit_view = move |_| {
    flow_instance.fit_view(FitViewOptions::default());
};

view! {
    <div class="controls">
        <button on:click=zoom_in>"Zoom In"</button>
        <button on:click=zoom_out>"Zoom Out"</button>
        <button on:click=fit_view>"Fit View"</button>
    </div>
    <FlowEditor nodes=nodes edges=edges />
}
```

### Keyboard Shortcuts

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        delete_key_code=Some("Delete")  // Delete selected items
        multi_selection_key_code=Some("Shift")
        on_key_down=move |event| {
            match event.code().as_str() {
                "KeyA" if event.ctrl_key() => {
                    // Ctrl+A: Select all
                    select_all_nodes();
                }
                "KeyC" if event.ctrl_key() => {
                    // Ctrl+C: Copy selection
                    copy_selection();
                }
                "KeyV" if event.ctrl_key() => {
                    // Ctrl+V: Paste
                    paste_selection();
                }
                _ => {}
            }
        }
    />
}
```

## Styling

### CSS Classes

Leptos Flow applies predictable CSS classes you can style:

```css
/* Main container */
.flow-editor {
    width: 100%;
    height: 100%;
    position: relative;
    overflow: hidden;
}

/* Individual nodes */
.flow-node {
    position: absolute;
    cursor: pointer;
    user-select: none;
}

.flow-node.selected {
    box-shadow: 0 0 0 2px #1a365d;
}

.flow-node.dragging {
    opacity: 0.8;
    z-index: 1000;
}

/* Edges */
.flow-edge {
    pointer-events: visibleStroke;
    fill: none;
    stroke-width: 2;
}

.flow-edge.selected {
    stroke: #1a365d;
}

.flow-edge.animated {
    animation: dash 20s linear infinite;
}

@keyframes dash {
    to {
        stroke-dashoffset: -1000;
    }
}

/* Handles */
.flow-handle {
    width: 12px;
    height: 12px;
    background: #1a365d;
    border: 2px solid white;
    border-radius: 50%;
    position: absolute;
}

.flow-handle.source {
    right: -6px;
    top: 50%;
    transform: translateY(-50%);
}

.flow-handle.target {
    left: -6px;
    top: 50%;
    transform: translateY(-50%);
}
```

### Themes

Create reusable themes:

```rust
#[derive(Clone, Debug)]
pub struct FlowTheme {
    pub background_color: String,
    pub node_background: String,
    pub node_border: String,
    pub edge_color: String,
    pub selection_color: String,
}

impl Default for FlowTheme {
    fn default() -> Self {
        Self {
            background_color: "#f8f9fa".to_string(),
            node_background: "#ffffff".to_string(),
            node_border: "#e9ecef".to_string(),
            edge_color: "#6c757d".to_string(),
            selection_color: "#1a365d".to_string(),
        }
    }
}

let dark_theme = FlowTheme {
    background_color: "#1a1a1a".to_string(),
    node_background: "#2d2d2d".to_string(),
    node_border: "#404040".to_string(),
    edge_color: "#808080".to_string(),
    selection_color: "#4a9eff".to_string(),
};

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        theme=dark_theme
    />
}
```

### Dynamic Styling

Update styles based on node state:

```rust
#[component]
pub fn StatefulNode(node: Node<NodeData>) -> impl IntoView {
    let node_style = move || {
        let base_style = "padding: 12px; border-radius: 6px; border: 2px solid;";

        match node.data.status {
            NodeStatus::Active => format!("{} background: #e6fffa; border-color: #38b2ac;", base_style),
            NodeStatus::Error => format!("{} background: #fed7d7; border-color: #e53e3e;", base_style),
            NodeStatus::Disabled => format!("{} background: #f7fafc; border-color: #cbd5e0; opacity: 0.6;", base_style),
        }
    };

    view! {
        <div style=node_style>
            {node.data.title}
        </div>
    }
}
```

## Custom Node Types

### Creating Custom Components

```rust
#[derive(Clone, Debug)]
pub struct CalculatorNodeData {
    pub operation: Operation,
    pub result: f64,
}

#[component]
pub fn CalculatorNode(
    #[prop(into)] node: MaybeSignal<Node<CalculatorNodeData>>,
    #[prop(optional)] on_change: Option<Callback<Node<CalculatorNodeData>>>,
) -> impl IntoView {
    let node_data = move || node.get();

    view! {
        <div class="calculator-node">
            // Input handles
            <Handle
                handle_type=HandleType::Target
                position=HandlePosition::Left
                id="input-a"
                style="top: 25%;"
            />
            <Handle
                handle_type=HandleType::Target
                position=HandlePosition::Left
                id="input-b"
                style="top: 75%;"
            />

            // Node content
            <div class="calculator-content">
                <div class="operation">
                    {move || format!("{:?}", node_data().data.operation)}
                </div>
                <div class="result">
                    {move || node_data().data.result}
                </div>
            </div>

            // Output handle
            <Handle
                handle_type=HandleType::Source
                position=HandlePosition::Right
                id="result"
            />
        </div>
    }
}

// Register the custom node type
view! {
    <FlowEditor nodes=nodes edges=edges>
        <NodeType name="calculator" component=CalculatorNode />
    </FlowEditor>
}
```

### Form-Based Nodes

Create interactive nodes with form controls:

```rust
#[component]
pub fn InputNode(
    #[prop(into)] node: MaybeSignal<Node<InputNodeData>>,
    #[prop(optional)] on_change: Option<Callback<Node<InputNodeData>>>,
) -> impl IntoView {
    let node_data = move || node.get();
    let (local_value, set_local_value) = create_signal(String::new());

    let update_node = move |new_value: String| {
        if let Some(callback) = on_change {
            let mut updated_node = node_data();
            updated_node.data.value = new_value;
            callback.call(updated_node);
        }
    };

    view! {
        <div class="input-node">
            <Handle
                handle_type=HandleType::Source
                position=HandlePosition::Right
                id="output"
            />

            <div class="input-content">
                <label>"Value:"</label>
                <input
                    type="text"
                    value=move || node_data().data.value.clone()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_local_value.set(value.clone());
                        update_node(value);
                    }
                />
            </div>
        </div>
    }
}
```

### Resizable Nodes

```rust
#[component]
pub fn ResizableNode(
    #[prop(into)] node: MaybeSignal<Node<ResizableNodeData>>,
) -> impl IntoView {
    let node_data = move || node.get();
    let (is_resizing, set_is_resizing) = create_signal(false);

    view! {
        <div
            class="resizable-node"
            class:resizing=is_resizing
            style=move || format!(
                "width: {}px; height: {}px;",
                node_data().size.width,
                node_data().size.height
            )
        >
            // Node content
            <div class="node-content">
                {move || node_data().data.content.clone()}
            </div>

            // Resize handle
            <div
                class="resize-handle"
                on:mousedown=move |_| set_is_resizing.set(true)
                on:mouseup=move |_| set_is_resizing.set(false)
            />
        </div>
    }
}
```

## Performance Tuning

### Viewport Culling

Only render visible elements for better performance:

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        only_render_visible_elements=true  // Enable viewport culling
        node_extent=Some(Extent::new(        // Limit node movement area
            -1000, -1000, 2000, 2000
        ))
    />
}
```

### Memory Management

```rust
// Use object pooling for frequently created/destroyed nodes
let node_pool = use_node_pool();

let create_temp_node = move || {
    let node = node_pool.acquire();
    // Configure node
    node.set_data(temp_data);
    node
};

let cleanup_temp_node = move |node: Node| {
    node_pool.release(node);
};
```

### Lazy Loading

Load large datasets incrementally:

```rust
let (visible_nodes, set_visible_nodes) = create_signal(Vec::new());
let all_nodes = use_context::<Vec<Node>>().unwrap();

// Load nodes based on viewport
create_effect(move |_| {
    let viewport = use_viewport().get();
    let visible = all_nodes.iter()
        .filter(|node| viewport.contains_point(&node.position))
        .take(1000)  // Limit to 1000 visible nodes
        .cloned()
        .collect();

    set_visible_nodes.set(visible);
});

view! {
    <FlowEditor
        nodes=visible_nodes  // Only render visible nodes
        edges=edges
    />
}
```

### Renderer Selection

Choose optimal renderer for your use case:

```rust
let renderer = move || {
    let node_count = nodes.get().len();

    if node_count > 5000 && webgpu_available() {
        RendererType::WebGPU     // Best for large graphs
    } else if node_count > 100 {
        RendererType::WebGL2     // Good balance
    } else {
        RendererType::Canvas2D   // Best compatibility
    }
};

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        renderer=renderer
    />
}
```

## Troubleshooting

### Common Issues

#### Nodes Not Rendering

```rust
// Check that nodes have valid positions
let validate_nodes = move || {
    for node in nodes.get() {
        if node.position.x.is_nan() || node.position.y.is_nan() {
            logging::error!("Invalid position for node: {}", node.id);
        }
    }
};
```

#### Performance Issues

```rust
// Enable performance monitoring
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        debug_mode=true  // Shows performance metrics
        on_performance_warning=move |warning| {
            logging::warn!("Performance warning: {:?}", warning);
        }
    />
}
```

#### Memory Leaks

```rust
// Use cleanup effects
create_effect(move |_| {
    // Setup expensive resources
    let resource = create_expensive_resource();

    on_cleanup(move || {
        // Cleanup when component unmounts
        resource.cleanup();
    });
});
```

### Debug Mode

Enable comprehensive debugging:

```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        debug_mode=true
        debug_config=DebugConfig {
            show_bounding_boxes: true,
            show_handles: true,
            show_viewport: true,
            log_events: true,
            performance_monitor: true,
        }
    />
}
```

## Advanced Topics

### Real-Time Collaboration

```rust
// WebSocket integration for real-time updates
let ws = use_websocket("ws://localhost:8080/flow");

create_effect(move |_| {
    if let Some(message) = ws.message.get() {
        if let Ok(update) = serde_json::from_str::<FlowUpdate>(&message) {
            apply_remote_update(update);
        }
    }
});

// Send local changes to other clients
let on_nodes_change = move |new_nodes| {
    set_nodes.set(new_nodes.clone());

    let update = FlowUpdate::NodesChanged { nodes: new_nodes };
    let message = serde_json::to_string(&update).unwrap();
    ws.send(&message);
};
```

### Undo/Redo System

```rust
let history = use_history();

view! {
    <div class="editor-controls">
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

### Export and Import

```rust
// Export flow as JSON
let export_flow = move || {
    let flow_data = FlowData {
        nodes: nodes.get(),
        edges: edges.get(),
        viewport: use_viewport().get(),
    };

    let json = serde_json::to_string_pretty(&flow_data).unwrap();

    // Download as file
    let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&json.into())).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let link = document().create_element("a").unwrap();
    link.set_attribute("href", &url).unwrap();
    link.set_attribute("download", "flow.json").unwrap();
    link.dyn_into::<web_sys::HtmlElement>().unwrap().click();
};

// Import flow from JSON
let import_flow = move |json: String| {
    if let Ok(flow_data) = serde_json::from_str::<FlowData>(&json) {
        set_nodes.set(flow_data.nodes);
        set_edges.set(flow_data.edges);

        // Restore viewport
        let flow = use_flow_instance();
        flow.set_viewport(flow_data.viewport);
    }
};
```

This comprehensive user manual should help users understand and effectively use Leptos Flow for their interactive diagram and workflow needs.
