# Migration from React Flow/xyflow

## Overview

This guide helps developers migrate from React Flow or xyflow to Leptos Flow. While the core concepts remain similar, there are important differences in API design, reactivity patterns, and performance characteristics.

## Key Differences

### Reactivity Model

- **React Flow**: useState/useCallback hooks with manual re-renders
- **Leptos Flow**: Signal-based fine-grained reactivity with automatic updates

### Type System

- **React Flow**: TypeScript with runtime type checking
- **Leptos Flow**: Rust with compile-time type safety and zero-cost abstractions

### Performance

- **React Flow**: Virtual DOM diffing and reconciliation
- **Leptos Flow**: Direct DOM updates with WASM-optimized rendering

## API Mapping

### Components

| React Flow | Leptos Flow | Notes |
|------------|-------------|-------|
| `ReactFlow` | `FlowEditor` | Main component |
| `ReactFlowProvider` | `FlowProvider` | Context provider |
| `Handle` | `Handle` | Connection points |
| `Controls` | `Controls` | Zoom/pan controls |
| `MiniMap` | `MiniMap` | Overview map |
| `Background` | `Background` | Grid/dot background |
| `Panel` | `Panel` | Overlay panels |

### Hooks and Context

| React Flow | Leptos Flow | Notes |
|------------|-------------|-------|
| `useReactFlow()` | `use_flow_instance()` | Imperative API access |
| `useNodes()` | `use_nodes()` | Node state management |
| `useEdges()` | `use_edges()` | Edge state management |
| `useViewport()` | `use_viewport()` | Viewport state |
| `useNodesState()` | `create_signal(nodes)` | Reactive nodes |
| `useEdgesState()` | `create_signal(edges)` | Reactive edges |

### Event Handlers

| React Flow | Leptos Flow | Notes |
|------------|-------------|-------|
| `onNodesChange` | `on_nodes_change` | Node updates |
| `onEdgesChange` | `on_edges_change` | Edge updates |
| `onConnect` | `on_connect` | New connections |
| `onNodeClick` | `on_node_click` | Node interactions |
| `onEdgeClick` | `on_edge_click` | Edge interactions |
| `onPaneClick` | `on_pane_click` | Background clicks |

### Node/Edge Properties

| React Flow | Leptos Flow | Notes |
|------------|-------------|-------|
| `data` | `data` | Custom data (strongly typed) |
| `position` | `position` | X,Y coordinates |
| `type` | `node_type` | Node type string |
| `selected` | `selected` | Selection state |
| `dragging` | `dragging` | Drag state |
| `style` | `style` | CSS styling |

## Migration Steps

### 1. Project Setup

#### React Flow Project

```typescript
// package.json
{
  "dependencies": {
    "react": "^18.0.0",
    "reactflow": "^11.0.0"
  }
}
```

#### Leptos Flow Project

```toml
# Cargo.toml
[dependencies]
leptos = "0.6"
leptos-flow = "0.1"
```

### 2. Basic Component Migration

#### React Flow Component

```typescript
import React, { useState, useCallback } from 'react';
import ReactFlow, {
  Node,
  Edge,
  addEdge,
  Connection,
  useNodesState,
  useEdgesState,
} from 'reactflow';

const initialNodes: Node[] = [
  { id: '1', position: { x: 100, y: 100 }, data: { label: 'Node 1' } },
  { id: '2', position: { x: 300, y: 200 }, data: { label: 'Node 2' } },
];

const initialEdges: Edge[] = [
  { id: 'e1-2', source: '1', target: '2' }
];

export default function FlowComponent() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      onConnect={onConnect}
    />
  );
}
```

#### Leptos Flow Component

```rust
use leptos::*;
use leptos_flow::*;

#[derive(Clone, Debug, Default)]
pub struct NodeData {
    pub label: String,
}

#[component]
pub fn FlowComponent() -> impl IntoView {
    let initial_nodes = vec![
        Node::builder("1")
            .position(100.0, 100.0)
            .data(NodeData { label: "Node 1".to_string() })
            .build(),
        Node::builder("2")
            .position(300.0, 200.0)
            .data(NodeData { label: "Node 2".to_string() })
            .build(),
    ];

    let initial_edges = vec![
        Edge::builder()
            .id("e1-2")
            .connect("1", "2")
            .build(),
    ];

    let (nodes, set_nodes) = create_signal(initial_nodes);
    let (edges, set_edges) = create_signal(initial_edges);

    let on_connect = move |connection: Connection| {
        let new_edge = Edge::builder()
            .connect(&connection.source, &connection.target)
            .build();
        set_edges.update(|edges| edges.push(new_edge));
    };

    view! {
        <FlowEditor
            nodes=nodes
            edges=edges
            on_nodes_change=set_nodes
            on_edges_change=set_edges
            on_connect=on_connect
        />
    }
}
```

### 3. Custom Node Migration

#### React Flow Custom Node

```typescript
import React from 'react';
import { Handle, Position, NodeProps } from 'reactflow';

interface CustomNodeData {
  label: string;
  value: number;
}

function CustomNode({ data, selected }: NodeProps<CustomNodeData>) {
  return (
    <div
      className={`custom-node ${selected ? 'selected' : ''}`}
      style={{
        padding: '10px',
        border: '2px solid #1a365d',
        borderRadius: '8px',
        background: 'white',
      }}
    >
      <Handle type="target" position={Position.Left} />
      <div>
        <h3>{data.label}</h3>
        <div>Value: {data.value}</div>
      </div>
      <Handle type="source" position={Position.Right} />
    </div>
  );
}
```

#### Leptos Flow Custom Node

```rust
#[derive(Clone, Debug)]
pub struct CustomNodeData {
    pub label: String,
    pub value: i32,
}

#[component]
pub fn CustomNode(
    #[prop(into)] node: MaybeSignal<Node<CustomNodeData>>,
) -> impl IntoView {
    let node_data = move || node.get();

    view! {
        <div
            class="custom-node"
            class:selected=move || node_data().selected
            style="
                padding: 10px;
                border: 2px solid #1a365d;
                border-radius: 8px;
                background: white;
            "
        >
            <Handle
                handle_type=HandleType::Target
                position=HandlePosition::Left
            />
            <div>
                <h3>{move || node_data().data.label.clone()}</h3>
                <div>"Value: " {move || node_data().data.value}</div>
            </div>
            <Handle
                handle_type=HandleType::Source
                position=HandlePosition::Right
            />
        </div>
    }
}
```

### 4. Event Handling Migration

#### React Flow Events

```typescript
const onNodeClick = useCallback((event: React.MouseEvent, node: Node) => {
  console.log('Node clicked:', node.id);
}, []);

const onEdgeClick = useCallback((event: React.MouseEvent, edge: Edge) => {
  console.log('Edge clicked:', edge.id);
}, []);

const onPaneClick = useCallback((event: React.MouseEvent) => {
  console.log('Pane clicked');
}, []);

return (
  <ReactFlow
    nodes={nodes}
    edges={edges}
    onNodeClick={onNodeClick}
    onEdgeClick={onEdgeClick}
    onPaneClick={onPaneClick}
  />
);
```

#### Leptos Flow Events

```rust
let on_node_click = move |node_id: String| {
    logging::log!("Node clicked: {}", node_id);
};

let on_edge_click = move |edge_id: String| {
    logging::log!("Edge clicked: {}", edge_id);
};

let on_pane_click = move |_event: MouseEvent| {
    logging::log!("Pane clicked");
};

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_node_click=on_node_click
        on_edge_click=on_edge_click
        on_pane_click=on_pane_click
    />
}
```

### 5. State Management Migration

#### React Flow with Redux/Zustand

```typescript
// Zustand store
interface FlowState {
  nodes: Node[];
  edges: Edge[];
  addNode: (node: Node) => void;
  updateNode: (id: string, updates: Partial<Node>) => void;
}

const useFlowStore = create<FlowState>((set) => ({
  nodes: [],
  edges: [],
  addNode: (node) => set((state) => ({
    nodes: [...state.nodes, node]
  })),
  updateNode: (id, updates) => set((state) => ({
    nodes: state.nodes.map(n => n.id === id ? { ...n, ...updates } : n)
  })),
}));
```

#### Leptos Flow with Global State

```rust
use leptos::*;

// Global state context
#[derive(Clone)]
pub struct FlowState<N, E> {
    pub nodes: RwSignal<Vec<Node<N>>>,
    pub edges: RwSignal<Vec<Edge<E>>>,
}

impl<N, E> FlowState<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: create_rw_signal(Vec::new()),
            edges: create_rw_signal(Vec::new()),
        }
    }

    pub fn add_node(&self, node: Node<N>) {
        self.nodes.update(|nodes| nodes.push(node));
    }

    pub fn update_node<F>(&self, id: &str, updater: F)
    where
        F: Fn(&mut Node<N>)
    {
        self.nodes.update(|nodes| {
            if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                updater(node);
            }
        });
    }
}

// Provide context
#[component]
pub fn FlowApp() -> impl IntoView {
    let flow_state = FlowState::<NodeData, EdgeData>::new();
    provide_context(flow_state);

    view! {
        <FlowComponent />
    }
}

// Use context in components
#[component]
pub fn FlowComponent() -> impl IntoView {
    let flow_state = use_context::<FlowState<NodeData, EdgeData>>().unwrap();

    view! {
        <FlowEditor
            nodes=flow_state.nodes
            edges=flow_state.edges
        />
    }
}
```

## Performance Considerations

### React Flow Optimization

```typescript
// React Flow performance patterns
const nodeTypes = useMemo(() => ({
  custom: CustomNode
}), []);

const edges = useMemo(() =>
  edgeData.map(edge => ({ ...edge, type: 'smoothstep' })),
  [edgeData]
);

return (
  <ReactFlow
    nodes={nodes}
    edges={edges}
    nodeTypes={nodeTypes}
    onlyRenderVisibleElements={true}
  />
);
```

### Leptos Flow Optimization

```rust
// Leptos Flow automatically optimizes through:
// 1. Compile-time optimizations
// 2. Fine-grained reactivity
// 3. WASM performance
// 4. Built-in viewport culling

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        only_render_visible_elements=true  // Built-in optimization
    >
        <NodeType name="custom" component=CustomNode />
    </FlowEditor>
}
```

## Advanced Features Migration

### 1. Layout Algorithms

#### React Flow with dagre

```typescript
import dagre from 'dagre';

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const getLayoutedElements = (nodes: Node[], edges: Edge[]) => {
  dagreGraph.setGraph({ rankdir: 'TB' });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: 150, height: 50 });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  return nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    return {
      ...node,
      position: {
        x: nodeWithPosition.x - 75,
        y: nodeWithPosition.y - 25,
      },
    };
  });
};
```

#### Leptos Flow Built-in Layouts

```rust
use leptos_flow::layout::*;

let apply_layout = move || {
    let flow = use_flow_instance();
    let layout = HierarchicalLayout::builder()
        .direction(LayoutDirection::TopDown)
        .level_separation(60)
        .node_separation(40)
        .build();

    spawn_local(async move {
        flow.apply_layout(layout).await.unwrap();
    });
};

view! {
    <button on:click=move |_| apply_layout()>
        "Apply Layout"
    </button>
}
```

### 2. Minimap and Controls

#### React Flow

```typescript
import { MiniMap, Controls, Background } from 'reactflow';

return (
  <ReactFlow nodes={nodes} edges={edges}>
    <Controls />
    <MiniMap />
    <Background variant="dots" gap={12} size={1} />
  </ReactFlow>
);
```

#### Leptos Flow

```rust
view! {
    <FlowEditor nodes=nodes edges=edges>
        <Controls position=ControlPosition::TopLeft />
        <MiniMap position=MiniMapPosition::BottomRight />
        <Background variant=BackgroundVariant::Dots gap=12 size=1 />
    </FlowEditor>
}
```

## Feature Parity Checklist

### Core Features

- [x] Node creation and positioning
- [x] Edge connections
- [x] Drag and drop interactions
- [x] Selection (single and multi)
- [x] Zoom and pan
- [x] Custom node types
- [x] Custom edge types
- [x] Event handling

### Advanced Features

- [x] Auto-layout algorithms
- [x] Minimap
- [x] Controls
- [x] Background patterns
- [x] Export functionality
- [x] Undo/redo
- [x] Viewport management
- [x] Spatial indexing

### Performance Features

- [x] Viewport culling
- [x] Level-of-detail rendering
- [x] Memory optimization
- [x] Large graph handling (10k+ nodes)

## Common Migration Patterns

### 1. Component Lifecycle

```typescript
// React Flow
useEffect(() => {
  // Component mount logic
  return () => {
    // Cleanup logic
  };
}, []);
```

```rust
// Leptos Flow
create_effect(move |_| {
    // Reactive effect logic
});

on_cleanup(move || {
    // Cleanup logic
});
```

### 2. Conditional Rendering

```typescript
// React Flow
{showMiniMap && <MiniMap />}
```

```rust
// Leptos Flow
<Show when=move || show_minimap.get()>
    <MiniMap />
</Show>
```

### 3. Dynamic Node Types

```typescript
// React Flow
const nodeTypes = useMemo(() => ({
  input: InputNode,
  default: DefaultNode,
  output: OutputNode,
}), []);
```

```rust
// Leptos Flow - Handled at compile time
view! {
    <FlowEditor>
        <NodeType name="input" component=InputNode />
        <NodeType name="default" component=DefaultNode />
        <NodeType name="output" component=OutputNode />
    </FlowEditor>
}
```

## Troubleshooting

### Common Issues

1. **Reactivity not updating**: Ensure you're using signals correctly and not mutating signal values directly
2. **Performance issues**: Enable viewport culling and use proper signal dependencies
3. **Type errors**: Rust's type system is stricter - define your data types explicitly
4. **WASM build errors**: Ensure proper Rust toolchain and WASM target installation

### Migration Tools

Consider building these utilities to help with migration:

```rust
// JSON import helper for React Flow data
pub fn import_react_flow_data(json: &str) -> Result<(Vec<Node<serde_json::Value>>, Vec<Edge<serde_json::Value>>), serde_json::Error> {
    #[derive(Deserialize)]
    struct ReactFlowData {
        nodes: Vec<Node<serde_json::Value>>,
        edges: Vec<Edge<serde_json::Value>>,
    }

    let data: ReactFlowData = serde_json::from_str(json)?;
    Ok((data.nodes, data.edges))
}
```

This migration guide should help you transition from React Flow to Leptos Flow while taking advantage of Rust's performance and type safety benefits.
