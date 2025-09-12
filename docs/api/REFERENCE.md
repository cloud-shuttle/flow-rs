# Leptos Flow API Reference

## Overview

This document provides a comprehensive API reference for Leptos Flow. For detailed examples and usage patterns, see the [User Manual](../manual/USER_MANUAL.md) and [Quick Start Guide](../guides/QUICK_START.md).

## Core Types

### SpatialIndex

High-performance spatial indexing for efficient node queries with comprehensive safety features.

```rust
pub struct SpatialIndex {
    // Internal grid-based spatial indexing
    // MAX_GRID_CELLS limit (10,000) for performance safety
    // Bounds checking to prevent infinite loops
}
```

#### Methods

##### `SpatialIndex::new() -> SpatialIndex`

Creates a new spatial index with default cell size.

##### `SpatialIndex::with_cell_size(cell_size: f64) -> SpatialIndex`

Creates a new spatial index with custom cell size.

##### `spatial_index.insert(node: &Node) -> Result<(), FlowError>`

Inserts a node into the spatial index with bounds checking.

##### `spatial_index.query_rect(bounds: &Rect) -> Vec<NodeId>`

Queries nodes within rectangular bounds with safety limits.

##### `spatial_index.query_radius(center: Position, radius: f64) -> Vec<NodeId>`

Queries nodes within circular radius with special handling for zero radius.

##### `spatial_index.nearest(point: Position) -> Option<NodeId>`

Finds the nearest node to a point with brute force fallback for extreme cases.

#### Safety Features

- **Infinite Loop Prevention**: MAX_GRID_CELLS limit prevents resource exhaustion
- **Bounds Checking**: Validates input bounds to prevent NaN/infinite values
- **Graceful Degradation**: Returns empty results for extreme cases instead of hanging
- **Timeout Protection**: Comprehensive test timeout handling

### Node<T>

Represents a node in the flow graph.

```rust
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
    pub drag_handle: Option<String>,
    pub extent: Option<Extent>,
    pub parent_node: Option<String>,
    pub z_index: Option<i32>,
    pub hidden: bool,
    pub measured: Option<Dimensions>,
}
```

#### Methods

##### `Node::new(id: impl Into<String>, position: Position) -> Node<()>`

Creates a new node with the given ID and position.

##### `Node::builder(id: impl Into<String>) -> NodeBuilder<T>`

Returns a builder for fluent node construction.

##### `node.with_data<U>(data: U) -> Node<U>`

Returns a new node with the given data type.

##### `node.with_position(x: f64, y: f64) -> Node<T>`

Returns a new node with the given position.

##### `node.with_size(width: f64, height: f64) -> Node<T>`

Returns a new node with the given size.

### Edge<T>

Represents an edge (connection) between two nodes.

```rust
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
}
```

#### Methods

##### `Edge::new(id: impl Into<String>, source: impl Into<String>, target: impl Into<String>) -> Edge<()>`

Creates a new edge connecting the specified nodes.

##### `Edge::builder() -> EdgeBuilder<T>`

Returns a builder for fluent edge construction.

##### `edge.with_handles(source_handle: impl Into<String>, target_handle: impl Into<String>) -> Edge<T>`

Returns a new edge with specific handle connections.

### Graph<N, E>

Container for nodes and edges with spatial indexing.

```rust
pub struct Graph<N = (), E = ()> {
    // Private fields
}
```

#### Methods

##### `Graph::new() -> Graph<(), ()>`

Creates a new empty graph.

##### `Graph::with_spatial_index() -> Graph<(), ()>`

Creates a new graph with spatial indexing enabled for performance.

##### `graph.add_node(&mut self, node: Node<N>) -> Result<(), FlowError>`

Adds a node to the graph.

##### `graph.remove_node(&mut self, id: &str) -> Result<Node<N>, FlowError>`

Removes a node and all connected edges.

##### `graph.get_node(&self, id: &str) -> Option<&Node<N>>`

Gets a reference to a node by ID.

##### `graph.get_node_mut(&mut self, id: &str) -> Option<&mut Node<N>>`

Gets a mutable reference to a node by ID.

##### `graph.add_edge(&mut self, edge: Edge<E>) -> Result<(), FlowError>`

Adds an edge to the graph.

##### `graph.remove_edge(&mut self, id: &str) -> Result<Edge<E>, FlowError>`

Removes an edge from the graph.

##### `graph.get_nodes_in_viewport(&self, viewport: &Viewport) -> Vec<&Node<N>>`

Returns nodes visible in the given viewport (requires spatial indexing).

## Components

### FlowEditor

Main component for rendering flow diagrams.

```rust
#[component]
pub fn FlowEditor<N, E>(
    // Required props
    nodes: ReadSignal<Vec<Node<N>>>,
    edges: ReadSignal<Vec<Edge<E>>>,

    // Event handlers
    #[prop(optional)] on_nodes_change: Option<WriteSignal<Vec<Node<N>>>>,
    #[prop(optional)] on_edges_change: Option<WriteSignal<Vec<Edge<E>>>>,
    #[prop(optional)] on_connect: Option<Callback<Connection>>,
    #[prop(optional)] on_connect_start: Option<Callback<(String, Option<String>)>>,
    #[prop(optional)] on_connect_end: Option<Callback<ConnectionEvent>>,

    // Node events
    #[prop(optional)] on_node_click: Option<Callback<String>>,
    #[prop(optional)] on_node_double_click: Option<Callback<String>>,
    #[prop(optional)] on_node_context_menu: Option<Callback<(String, MouseEvent)>>,
    #[prop(optional)] on_node_drag_start: Option<Callback<String>>,
    #[prop(optional)] on_node_drag: Option<Callback<(String, Position)>>,
    #[prop(optional)] on_node_drag_stop: Option<Callback<(String, Position)>>,

    // Edge events
    #[prop(optional)] on_edge_click: Option<Callback<String>>,
    #[prop(optional)] on_edge_double_click: Option<Callback<String>>,
    #[prop(optional)] on_edge_context_menu: Option<Callback<(String, MouseEvent)>>,

    // Selection events
    #[prop(optional)] on_selection_change: Option<Callback<Selection>>,

    // Viewport events
    #[prop(optional)] on_move: Option<Callback<Viewport>>,
    #[prop(optional)] on_zoom: Option<Callback<f64>>,

    // Pane events
    #[prop(optional)] on_pane_click: Option<Callback<MouseEvent>>,
    #[prop(optional)] on_pane_context_menu: Option<Callback<MouseEvent>>,
    #[prop(optional)] on_pane_scroll: Option<Callback<WheelEvent>>,

    // Interaction configuration
    #[prop(optional, default = 5.0)] node_drag_threshold: f64,
    #[prop(optional)] selection_key: Option<SelectionKey>,
    #[prop(optional, default = false)] multi_selection: bool,
    #[prop(optional)] delete_key: Option<String>,

    // Viewport configuration
    #[prop(optional, default = 0.1)] min_zoom: f64,
    #[prop(optional, default = 4.0)] max_zoom: f64,
    #[prop(optional, default = 1.0)] default_zoom: f64,
    #[prop(optional, default = false)] fit_view_on_init: bool,
    #[prop(optional)] snap_to_grid: Option<SnapToGrid>,
    #[prop(optional)] translate_extent: Option<Extent>,
    #[prop(optional)] node_extent: Option<Extent>,

    // Visual configuration
    #[prop(optional)] background: Option<Background>,
    #[prop(optional, default = ConnectionMode::Strict)] connection_mode: ConnectionMode,
    #[prop(optional, default = ConnectionLineType::Bezier)] connection_line_type: ConnectionLineType,

    // Performance configuration
    #[prop(optional, default = false)] only_render_visible_elements: bool,
    #[prop(optional)] renderer: Option<RendererType>,

    // Children (custom node/edge types)
    children: Children,
) -> impl IntoView
```

### Handle

Connection point for nodes.

```rust
#[component]
pub fn Handle(
    handle_type: HandleType,
    position: HandlePosition,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] class_name: Option<String>,
    #[prop(optional, default = true)] connectable: bool,
    #[prop(optional)] connection_limit: Option<usize>,
) -> impl IntoView
```

### Controls

Zoom and pan controls.

```rust
#[component]
pub fn Controls(
    #[prop(optional, default = ControlPosition::TopLeft)] position: ControlPosition,
    #[prop(optional, default = true)] show_zoom: bool,
    #[prop(optional, default = true)] show_fit_view: bool,
    #[prop(optional, default = false)] show_interactive: bool,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] class_name: Option<String>,
) -> impl IntoView
```

### MiniMap

Overview map of the entire flow.

```rust
#[component]
pub fn MiniMap(
    #[prop(optional, default = MiniMapPosition::BottomRight)] position: MiniMapPosition,
    #[prop(optional)] width: Option<f64>,
    #[prop(optional)] height: Option<f64>,
    #[prop(optional, default = "#f8f9fa".to_string())] background_color: String,
    #[prop(optional, default = "#e9ecef".to_string())] mask_color: String,
    #[prop(optional, default = "#495057".to_string())] node_color: String,
    #[prop(optional, default = "#6c757d".to_string())] edge_color: String,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] class_name: Option<String>,
) -> impl IntoView
```

### Background

Background pattern for the flow editor.

```rust
#[component]
pub fn Background(
    #[prop(optional, default = BackgroundVariant::Dots)] variant: BackgroundVariant,
    #[prop(optional, default = 12)] gap: i32,
    #[prop(optional, default = 1)] size: i32,
    #[prop(optional, default = "#e9ecef".to_string())] color: String,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] class_name: Option<String>,
) -> impl IntoView
```

### NodeType

Register custom node components.

```rust
#[component]
pub fn NodeType<N>(
    name: String,
    component: fn(MaybeSignal<Node<N>>) -> impl IntoView,
) -> impl IntoView
```

### EdgeType

Register custom edge components.

```rust
#[component]
pub fn EdgeType<E>(
    name: String,
    component: fn(MaybeSignal<Edge<E>>) -> impl IntoView,
) -> impl IntoView
```

## Hooks

### use_flow_instance()

Returns the flow instance for imperative control.

```rust
pub fn use_flow_instance() -> FlowInstance
```

### use_nodes<T>()

Returns reactive signals for nodes.

```rust
pub fn use_nodes<T>() -> (ReadSignal<Vec<Node<T>>>, WriteSignal<Vec<Node<T>>>)
```

### use_edges<T>()

Returns reactive signals for edges.

```rust
pub fn use_edges<T>() -> (ReadSignal<Vec<Edge<T>>>, WriteSignal<Vec<Edge<T>>>)
```

### use_viewport()

Returns the current viewport state.

```rust
pub fn use_viewport() -> (ReadSignal<Viewport>, Callback<Viewport>)
```

### use_selection()

Returns the current selection state.

```rust
pub fn use_selection() -> (ReadSignal<Selection>, Callback<Selection>)
```

### use_history<T>()

Provides undo/redo functionality.

```rust
pub fn use_history<T>() -> HistoryHandlers<T>
where T: Clone + PartialEq + 'static
```

## FlowInstance

Imperative API for flow control.

```rust
impl FlowInstance {
    // Node operations
    pub fn add_node(&self, node: Node) -> Result<(), FlowError>;
    pub fn remove_node(&self, node_id: &str) -> Result<Node, FlowError>;
    pub fn update_node(&self, node_id: &str, updates: NodeUpdate) -> Result<(), FlowError>;
    pub fn get_node(&self, node_id: &str) -> Option<Node>;
    pub fn get_nodes(&self) -> Vec<Node>;

    // Edge operations
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
    pub fn select_nodes(&self, node_ids: Vec<String>) -> Result<(), FlowError>;
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
    pub fn apply_layout(&self, algorithm: Box<dyn LayoutAlgorithm>) -> Result<(), FlowError>;
    pub fn stop_layout(&self) -> Result<(), FlowError>;

    // Export operations
    pub fn to_json(&self) -> Result<String, FlowError>;
    pub fn from_json(&self, json: &str) -> Result<(), FlowError>;
    pub fn export_as_svg(&self) -> Result<String, FlowError>;
    pub fn export_as_png(&self) -> Result<Vec<u8>, FlowError>;
}
```

## Supporting Types

### Position

2D position coordinates.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self;
    pub fn zero() -> Self;
    pub fn distance_to(&self, other: &Position) -> f64;
}
```

### Size

2D dimensions.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self;
    pub fn zero() -> Self;
    pub fn area(&self) -> f64;
}
```

### Viewport

Camera/view information.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub width: f64,
    pub height: f64,
}

impl Viewport {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self;
    pub fn contains_point(&self, point: &Position) -> bool;
    pub fn contains_rect(&self, rect: &Rect) -> bool;
    pub fn intersects_rect(&self, rect: &Rect) -> bool;
}
```

### Selection

Current selection state.

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Selection {
    pub nodes: Vec<String>,
    pub edges: Vec<String>,
}

impl Selection {
    pub fn new() -> Self;
    pub fn is_empty(&self) -> bool;
    pub fn contains_node(&self, node_id: &str) -> bool;
    pub fn contains_edge(&self, edge_id: &str) -> bool;
}
```

### Connection

Connection attempt information.

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    pub source: String,
    pub target: String,
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
}
```

## Enums

### HandleType

Type of connection handle.

```rust
pub enum HandleType {
    Source,  // Output handle
    Target,  // Input handle
}
```

### HandlePosition

Position of handle on node.

```rust
pub enum HandlePosition {
    Top,
    Right,
    Bottom,
    Left,
}
```

### RendererType

Available rendering backends.

```rust
pub enum RendererType {
    Canvas2D,
    WebGL2,
    WebGPU,
}
```

### BackgroundVariant

Background pattern types.

```rust
pub enum BackgroundVariant {
    Dots,
    Lines,
    Cross,
}
```

### ConnectionMode

Connection validation mode.

```rust
pub enum ConnectionMode {
    Strict,  // Only allow valid connections
    Loose,   // Allow any connections
}
```

### ConnectionLineType

Connection line rendering style.

```rust
pub enum ConnectionLineType {
    Bezier,
    Straight,
    Step,
    SmoothStep,
}
```

## Error Types

### FlowError

Main error type for flow operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Node with ID '{id}' not found")]
    NodeNotFound { id: String },

    #[error("Edge with ID '{id}' not found")]
    EdgeNotFound { id: String },

    #[error("Duplicate node ID: '{id}'")]
    DuplicateNodeId { id: String },

    #[error("Duplicate edge ID: '{id}'")]
    DuplicateEdgeId { id: String },

    #[error("Invalid connection: {reason}")]
    InvalidConnection { reason: String },

    #[error("Renderer error: {0}")]
    Renderer(#[from] RendererError),

    #[error("Layout error: {0}")]
    Layout(#[from] LayoutError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

## Traits

### NodeData

Trait for custom node data types.

```rust
pub trait NodeData: Clone + PartialEq + 'static {
    fn node_type(&self) -> Option<&str> { None }
    fn validate(&self) -> Result<(), String> { Ok(()) }
}

// Automatic implementation for most types
impl<T> NodeData for T where T: Clone + PartialEq + 'static {}
```

### EdgeData

Trait for custom edge data types.

```rust
pub trait EdgeData: Clone + PartialEq + 'static {
    fn edge_type(&self) -> Option<&str> { None }
    fn validate(&self) -> Result<(), String> { Ok(()) }
}

// Automatic implementation for most types
impl<T> EdgeData for T where T: Clone + PartialEq + 'static {}
```

### LayoutAlgorithm

Trait for custom layout algorithms.

```rust
pub trait LayoutAlgorithm {
    fn name(&self) -> &str;
    fn apply(&mut self, graph: &mut Graph) -> Result<(), LayoutError>;
    fn stop(&mut self) -> Result<(), LayoutError>;
    fn is_running(&self) -> bool;
    fn progress(&self) -> f64;
}
```

## Builder Patterns

### NodeBuilder<T>

Fluent builder for nodes.

```rust
impl<T> NodeBuilder<T> {
    pub fn position(self, x: f64, y: f64) -> Self;
    pub fn size(self, width: f64, height: f64) -> Self;
    pub fn data(self, data: T) -> Self;
    pub fn node_type(self, node_type: impl Into<String>) -> Self;
    pub fn style(self, style: NodeStyle) -> Self;
    pub fn class_name(self, class_name: impl Into<String>) -> Self;
    pub fn selectable(self, selectable: bool) -> Self;
    pub fn draggable(self, draggable: bool) -> Self;
    pub fn build(self) -> Node<T>;
}
```

### EdgeBuilder<T>

Fluent builder for edges.

```rust
impl<T> EdgeBuilder<T> {
    pub fn id(self, id: impl Into<String>) -> Self;
    pub fn connect(self, source: impl Into<String>, target: impl Into<String>) -> Self;
    pub fn connect_handles(
        self,
        source: impl Into<String>,
        source_handle: impl Into<String>,
        target: impl Into<String>,
        target_handle: impl Into<String>
    ) -> Self;
    pub fn data(self, data: T) -> Self;
    pub fn edge_type(self, edge_type: impl Into<String>) -> Self;
    pub fn style(self, style: EdgeStyle) -> Self;
    pub fn animated(self, animated: bool) -> Self;
    pub fn label(self, label: impl Into<String>) -> Self;
    pub fn build(self) -> Edge<T>;
}
```

This API reference provides comprehensive coverage of all public types and methods in Leptos Flow. For usage examples and patterns, refer to the User Manual and implementation guides.
