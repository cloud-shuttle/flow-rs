# Leptos Flow API Design Specification

## Overview

This document defines the API surface for Leptos Flow, focusing on developer experience, type safety, and seamless integration with Leptos' reactive patterns.

## Design Philosophy

### 1. Leptos-First Integration
- Native signal-based reactivity
- Component-centric API design
- Idiomatic Leptos patterns and conventions
- Zero-cost abstractions with compile-time optimization

### 2. Builder Pattern for Configuration
- Fluent, discoverable API
- Compile-time validation
- Sensible defaults with customization options
- Progressive disclosure of complexity

### 3. Type-Safe Extensibility
- Generic node/edge data types
- Trait-based customization points
- Compile-time validation of graph constraints
- Zero-overhead abstractions

## Core API Surface

### 1. Declarative Component API

#### Basic Flow Editor
```rust
use leptos::*;
use leptos_flow::*;

#[component]
pub fn App() -> impl IntoView {
    let (nodes, set_nodes) = create_signal(vec![
        Node::new("1")
            .position(100.0, 100.0)
            .data(MyNodeData::default()),
        Node::new("2")
            .position(300.0, 200.0)
            .data(MyNodeData::default()),
    ]);
    
    let (edges, set_edges) = create_signal(vec![
        Edge::new("e1")
            .connect("1", "2")
            .data(MyEdgeData::default()),
    ]);

    view! {
        <FlowEditor
            nodes=nodes
            edges=edges
            on_nodes_change=set_nodes
            on_edges_change=set_edges
            on_connect=|source, target| {
                // Handle new connection
            }
            width="100%"
            height="600px"
        >
            // Custom node types as child components
            <NodeType name="custom" component=CustomNode />
            <EdgeType name="custom" component=CustomEdge />
        </FlowEditor>
    }
}
```

#### Advanced Configuration
```rust
view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        on_nodes_change=set_nodes
        on_edges_change=set_edges
        
        // Interaction Configuration
        node_drag_threshold=5.0
        selection_key=Some(SelectionKey::Shift)
        multi_selection=true
        delete_key=Some("Delete")
        
        // Viewport Configuration
        min_zoom=0.1
        max_zoom=4.0
        fit_view_on_init=true
        snap_to_grid=Some(SnapToGrid::new(10))
        
        // Visual Configuration
        background=Background::Dots { spacing: 20, color: "#ddd" }
        connection_mode=ConnectionMode::Loose
        connection_line_type=ConnectionLineType::SmoothStep
        
        // Performance Configuration
        only_render_visible_elements=true
        node_extent=Some(Extent::new(0, 0, 1000, 1000))
        translate_extent=Some(Extent::infinite())
        
        // Event Handlers
        on_init=|flow_instance| { /* Setup */ }
        on_nodes_change=set_nodes
        on_edges_change=set_edges
        on_connect=handle_connect
        on_connect_start=|_, _| { /* Connection start */ }
        on_connect_end=|_| { /* Connection end */ }
        on_node_click=|node| { /* Node clicked */ }
        on_node_double_click=|node| { /* Node double clicked */ }
        on_edge_click=|edge| { /* Edge clicked */ }
        on_selection_change=|selection| { /* Selection changed */ }
        on_move_end=|event| { /* Node/selection move ended */ }
        on_pane_click=|event| { /* Pane clicked */ }
        on_pane_scroll=|event| { /* Pane scrolled */ }
        on_pane_context_menu=|event| { /* Pane right clicked */ }
    >
        <NodeType name="input" component=InputNode />
        <NodeType name="output" component=OutputNode />
        <EdgeType name="default" component=DefaultEdge />
        <EdgeType name="animated" component=AnimatedEdge />
        
        // Optional child components
        <Controls position=ControlPosition::TopLeft />
        <MiniMap 
            position=MiniMapPosition::BottomRight
            mask_color="#f0f0f0"
            node_color="#333"
        />
        <Background variant=BackgroundVariant::Dots />
    </FlowEditor>
}
```

### 2. Imperative Flow Instance API

#### Flow Instance Methods
```rust
use leptos_flow::*;

#[derive(Clone)]
pub struct FlowInstance {
    // Core graph operations
    pub fn add_node(&self, node: Node) -> Result<(), FlowError>;
    pub fn remove_node(&self, node_id: &str) -> Result<Node, FlowError>;
    pub fn update_node(&self, node_id: &str, updates: NodeUpdate) -> Result<(), FlowError>;
    pub fn get_node(&self, node_id: &str) -> Option<Node>;
    pub fn get_nodes(&self) -> Vec<Node>;
    
    pub fn add_edge(&self, edge: Edge) -> Result<(), FlowError>;
    pub fn remove_edge(&self, edge_id: &str) -> Result<Edge, FlowError>;
    pub fn update_edge(&self, edge_id: &str, updates: EdgeUpdate) -> Result<(), FlowError>;
    pub fn get_edge(&self, edge_id: &str) -> Option<Edge>;
    pub fn get_edges(&self) -> Vec<Edge>;
    
    // Viewport operations
    pub fn fit_view(&self, options: FitViewOptions) -> Result<(), FlowError>;
    pub fn zoom_to(&self, zoom: f64) -> Result<(), FlowError>;
    pub fn zoom_in(&self, options: ZoomOptions) -> Result<(), FlowError>;
    pub fn zoom_out(&self, options: ZoomOptions) -> Result<(), FlowError>;
    pub fn set_center(&self, x: f64, y: f64) -> Result<(), FlowError>;
    pub fn get_zoom(&self) -> f64;
    pub fn get_viewport(&self) -> Viewport;
    
    // Selection operations
    pub fn select_node(&self, node_id: &str) -> Result<(), FlowError>;
    pub fn select_nodes(&self, node_ids: Vec<String>) -> Result<(), FlowError>;
    pub fn select_edge(&self, edge_id: &str) -> Result<(), FlowError>;
    pub fn select_edges(&self, edge_ids: Vec<String>) -> Result<(), FlowError>;
    pub fn clear_selection(&self) -> Result<(), FlowError>;
    pub fn get_selected_nodes(&self) -> Vec<Node>;
    pub fn get_selected_edges(&self) -> Vec<Edge>;
    
    // Spatial queries
    pub fn get_nodes_in_rect(&self, rect: Rect) -> Vec<Node>;
    pub fn get_intersecting_nodes(&self, point: Point, radius: Option<f64>) -> Vec<Node>;
    pub fn screen_to_flow_position(&self, screen_pos: Point) -> Point;
    pub fn flow_to_screen_position(&self, flow_pos: Point) -> Point;
    
    // Layout operations
    pub fn apply_layout(&self, algorithm: LayoutAlgorithm) -> Result<(), FlowError>;
    pub fn stop_layout(&self) -> Result<(), FlowError>;
    
    // Export operations
    pub fn to_object(&self) -> FlowObject;
    pub fn to_json(&self) -> Result<String, FlowError>;
    pub fn export_as_svg(&self) -> Result<String, FlowError>;
    pub fn export_as_png(&self) -> Result<Vec<u8>, FlowError>;
}
```

### 3. Hook-Based API for Custom Behaviors

#### Core Hooks
```rust
// Node management hooks
pub fn use_nodes<T>() -> (ReadSignal<Vec<Node<T>>>, WriteSignal<Vec<Node<T>>>) {
    // Returns reactive nodes signal from current flow context
}

pub fn use_edges<T>() -> (ReadSignal<Vec<Edge<T>>>, WriteSignal<Vec<Edge<T>>>) {
    // Returns reactive edges signal from current flow context
}

pub fn use_flow_instance() -> FlowInstance {
    // Returns imperative API for current flow
}

// Viewport hooks
pub fn use_viewport() -> (ReadSignal<Viewport>, Callback<Viewport>) {
    // Returns current viewport and setter
}

pub fn use_zoom() -> (ReadSignal<f64>, Callback<f64>) {
    // Returns current zoom level and setter
}

// Selection hooks
pub fn use_selection() -> (ReadSignal<Selection>, Callback<Selection>) {
    // Returns current selection state
}

pub fn use_selected_nodes<T>() -> ReadSignal<Vec<Node<T>>> {
    // Returns currently selected nodes
}

pub fn use_selected_edges<T>() -> ReadSignal<Vec<Edge<T>>> {
    // Returns currently selected edges
}

// Interaction hooks
pub fn use_drag() -> DragHandlers {
    // Returns drag event handlers for custom drag behavior
}

pub fn use_connection() -> ConnectionHandlers {
    // Returns connection event handlers for custom connection logic
}

// Layout hooks
pub fn use_layout() -> LayoutHandlers {
    // Returns layout controls and status
}
```

#### Custom Hook Examples
```rust
// Custom node selection hook
pub fn use_multi_select() -> (ReadSignal<Vec<String>>, Callback<String>) {
    let (selected, set_selected) = create_signal(Vec::new());
    
    let toggle_selection = Callback::new(move |node_id: String| {
        set_selected.update(|selection| {
            if selection.contains(&node_id) {
                selection.retain(|id| id != &node_id);
            } else {
                selection.push(node_id);
            }
        });
    });
    
    (selected, toggle_selection)
}

// Custom undo/redo hook
pub fn use_history<T>() -> HistoryHandlers<T> {
    let (history, set_history) = create_signal(History::new());
    
    HistoryHandlers {
        undo: Callback::new(move || { /* Undo logic */ }),
        redo: Callback::new(move || { /* Redo logic */ }),
        can_undo: create_memo(move |_| history.get().can_undo()),
        can_redo: create_memo(move |_| history.get().can_redo()),
        push_state: Callback::new(move |state: T| { /* Push state */ }),
    }
}
```

## Type System Design

### 1. Generic Node/Edge Types
```rust
// Node with custom data type
#[derive(Clone, Debug, PartialEq)]
pub struct Node<T = ()> {
    pub id: String,
    pub position: Position,
    pub data: T,
    pub node_type: Option<String>,
    pub style: Option<NodeStyle>,
    pub class_name: Option<String>,
    pub selected: bool,
    pub dragging: bool,
    pub selectable: bool,
    pub connectable: bool,
    pub deletable: bool,
    pub dragHandle: Option<String>,
    pub extent: Option<Extent>,
    pub parent_node: Option<String>,
    pub z_index: Option<i32>,
    pub hidden: bool,
    pub measured: Option<Dimensions>,
}

// Edge with custom data type
#[derive(Clone, Debug, PartialEq)]
pub struct Edge<T = ()> {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
    pub data: T,
    pub edge_type: Option<String>,
    pub style: Option<EdgeStyle>,
    pub class_name: Option<String>,
    pub animated: bool,
    pub hidden: bool,
    pub selected: bool,
    pub selectable: bool,
    pub deletable: bool,
    pub marker_start: Option<Marker>,
    pub marker_end: Option<Marker>,
    pub z_index: Option<i32>,
    pub label: Option<String>,
    pub label_style: Option<LabelStyle>,
    pub label_show_bg: bool,
    pub label_bg_style: Option<LabelBgStyle>,
    pub label_bg_padding: Option<[f64; 2]>,
    pub label_bg_border_radius: Option<f64>,
}
```

### 2. Trait-Based Extensibility
```rust
// Custom node behavior
pub trait NodeBehavior {
    fn on_drag_start(&mut self, event: DragEvent) -> EventResult;
    fn on_drag(&mut self, event: DragEvent) -> EventResult;
    fn on_drag_end(&mut self, event: DragEvent) -> EventResult;
    fn on_click(&mut self, event: ClickEvent) -> EventResult;
    fn on_double_click(&mut self, event: DoubleClickEvent) -> EventResult;
    fn on_context_menu(&mut self, event: ContextMenuEvent) -> EventResult;
    fn on_connect(&mut self, connection: Connection) -> EventResult;
    fn validate_connection(&self, connection: &Connection) -> bool;
}

// Custom edge behavior
pub trait EdgeBehavior {
    fn on_click(&mut self, event: ClickEvent) -> EventResult;
    fn on_double_click(&mut self, event: DoubleClickEvent) -> EventResult;
    fn on_context_menu(&mut self, event: ContextMenuEvent) -> EventResult;
    fn get_path(&self, source_pos: Position, target_pos: Position) -> String;
    fn get_center(&self, source_pos: Position, target_pos: Position) -> Position;
}

// Custom layout algorithms
pub trait LayoutAlgorithm {
    fn name(&self) -> &str;
    fn layout(&mut self, nodes: &mut [Node], edges: &[Edge]) -> Result<(), LayoutError>;
    fn is_animated(&self) -> bool { false }
    fn options(&self) -> LayoutOptions;
}
```

### 3. Compile-Time Validation
```rust
// Type-safe node/edge creation
pub struct NodeBuilder<T> {
    node: Node<T>,
}

impl<T> NodeBuilder<T> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            node: Node {
                id: id.into(),
                data: T::default(),
                ..Default::default()
            }
        }
    }
    
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.node.position = Position::new(x, y);
        self
    }
    
    pub fn data(mut self, data: T) -> Self {
        self.node.data = data;
        self
    }
    
    pub fn build(self) -> Node<T> {
        self.node
    }
}

// Type-safe edge creation with connection validation
pub struct EdgeBuilder<T> {
    edge: Edge<T>,
}

impl<T> EdgeBuilder<T> {
    pub fn connect<S: Into<String>, T: Into<String>>(
        source: S, 
        target: T
    ) -> ConnectionBuilder<T> {
        ConnectionBuilder::new(source.into(), target.into())
    }
}

// Ensures valid connections at compile time
pub struct ConnectionBuilder<T> {
    source: String,
    target: String,
    _phantom: PhantomData<T>,
}

impl<T> ConnectionBuilder<T> {
    pub fn with_handles(
        self, 
        source_handle: impl Into<String>, 
        target_handle: impl Into<String>
    ) -> EdgeBuilder<T> {
        EdgeBuilder {
            edge: Edge {
                id: format!("{}_{}", self.source, self.target),
                source: self.source,
                target: self.target,
                source_handle: Some(source_handle.into()),
                target_handle: Some(target_handle.into()),
                ..Default::default()
            }
        }
    }
}
```

## Event System Design

### 1. Event Types
```rust
#[derive(Clone, Debug)]
pub enum FlowEvent {
    // Node events
    NodeClick { node_id: String, event: MouseEvent },
    NodeDoubleClick { node_id: String, event: MouseEvent },
    NodeContextMenu { node_id: String, event: MouseEvent },
    NodeDragStart { node_id: String, event: DragEvent },
    NodeDrag { node_id: String, event: DragEvent },
    NodeDragEnd { node_id: String, event: DragEvent },
    
    // Edge events  
    EdgeClick { edge_id: String, event: MouseEvent },
    EdgeDoubleClick { edge_id: String, event: MouseEvent },
    EdgeContextMenu { edge_id: String, event: MouseEvent },
    
    // Connection events
    ConnectStart { node_id: String, handle_id: Option<String> },
    ConnectEnd { connection: Option<Connection> },
    Connect { connection: Connection },
    
    // Selection events
    SelectionChange { selection: Selection },
    
    // Viewport events
    ViewportChange { viewport: Viewport },
    ZoomChange { zoom: f64 },
    
    // Pane events
    PaneClick { event: MouseEvent },
    PaneContextMenu { event: MouseEvent },
    PaneScroll { event: WheelEvent },
}
```

### 2. Event Handling
```rust
// Reactive event handling with Leptos signals
pub fn use_flow_events() -> FlowEventHandlers {
    let (events, set_events) = create_signal(Vec::new());
    
    FlowEventHandlers {
        events: events.into(),
        emit: Callback::new(move |event: FlowEvent| {
            set_events.update(|events| events.push(event));
        }),
        clear: Callback::new(move || {
            set_events.set(Vec::new());
        }),
    }
}
```

## Performance Considerations

### 1. Lazy Evaluation
- Nodes and edges only rendered when visible
- Layout calculations deferred until needed
- Event handlers registered on-demand

### 2. Memory Management
- Automatic cleanup of unused resources
- Object pooling for frequently created/destroyed objects
- Efficient spatial indexing with R-tree

### 3. Bundle Size Optimization
- Tree-shaking friendly API design
- Feature flags for optional functionality
- Minimal WASM binary size

This API design provides a comprehensive, type-safe, and performant interface for building flow-based applications with Leptos, while maintaining flexibility for advanced use cases and custom extensions.