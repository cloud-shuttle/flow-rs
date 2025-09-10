# API Examples for Leptos Flow Simple Example

This document provides comprehensive examples of how to use the Leptos Flow Simple Example API.

## Table of Contents

- [Basic Setup](#basic-setup)
- [Canvas2D Renderer](#canvas2d-renderer)
- [Graph Operations](#graph-operations)
- [Viewport Management](#viewport-management)
- [Interaction Handling](#interaction-handling)
- [Background Configuration](#background-configuration)
- [Error Handling](#error-handling)
- [Performance Optimization](#performance-optimization)

## Basic Setup

### Creating a Canvas and Renderer

```rust
use leptos_flow_renderer::{Canvas2DRenderer, Renderer};
use web_sys::HtmlCanvasElement;

// Get canvas element from DOM
let canvas = document
    .get_element_by_id("flow-canvas")
    .unwrap()
    .dyn_into::<HtmlCanvasElement>()
    .unwrap();

// Create renderer
let mut renderer = Canvas2DRenderer::new(&canvas)?;

// Set canvas size
renderer.resize(800, 600)?;
```

### Creating a Basic Graph

```rust
use leptos_flow_core::{Graph, Node, Edge, Position};
use leptos_flow_core::types::{NodeId, EdgeId};

// Create empty graph
let mut graph = Graph::new();

// Add nodes
let node1 = Node::simple("node1", Position::new(100.0, 100.0));
let node2 = Node::simple("node2", Position::new(300.0, 200.0));
let node3 = Node::simple("node3", Position::new(500.0, 150.0));

graph.add_node(node1)?;
graph.add_node(node2)?;
graph.add_node(node3)?;

// Add edges
let edge1 = Edge::simple("edge1", "node1", "node2");
let edge2 = Edge::simple("edge2", "node2", "node3");

graph.add_edge(edge1)?;
graph.add_edge(edge2)?;
```

## Canvas2D Renderer

### Basic Rendering

```rust
use leptos_flow_core::Viewport;

// Create viewport
let viewport = Viewport::default();

// Clear canvas
renderer.clear(Some("#ffffff"))?;

// Render background
let bg_config = BackgroundConfig {
    color: "#f8fafc".to_string(),
    pattern_color: "#e2e8f0".to_string(),
    variant: BackgroundVariant::Dots,
    size: 20.0,
    opacity: 0.5,
};
renderer.render_background(&bg_config, &viewport)?;

// Render graph
renderer.render_graph(&graph, &viewport)?;

// Present frame
renderer.present()?;
```

### Custom Node Styling

```rust
use leptos_flow_renderer::traits::{NodeStyle, NodeVariant};

// Create node with custom style
let mut node = Node::simple("custom_node", Position::new(200.0, 200.0));
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

graph.add_node(node)?;
```

### Custom Edge Styling

```rust
use leptos_flow_renderer::traits::{EdgeStyle, EdgeVariant};

// Create edge with custom style
let mut edge = Edge::simple("custom_edge", "node1", "node2");
edge.style = EdgeStyle {
    variant: EdgeVariant::Straight,
    color: "#ef4444".to_string(),
    width: 3.0,
    dash_pattern: Some(vec![5.0, 5.0]),
    ..Default::default()
};

graph.add_edge(edge)?;
```

## Graph Operations

### Adding and Removing Nodes

```rust
// Add multiple nodes efficiently
let positions = vec![
    Position::new(100.0, 100.0),
    Position::new(200.0, 100.0),
    Position::new(300.0, 100.0),
];

for (i, pos) in positions.iter().enumerate() {
    let node_id = format!("node_{}", i);
    let node = Node::simple(node_id, *pos);
    graph.add_node(node)?;
}

// Remove a node
graph.remove_node(&"node_1".into())?;

// Check if node exists
if let Some(node) = graph.get_node(&"node_0".into()) {
    println!("Node found: {:?}", node);
}
```

### Adding and Removing Edges

```rust
// Add edges between existing nodes
let edge_ids = vec![
    ("edge_0_1", "node_0", "node_1"),
    ("edge_1_2", "node_1", "node_2"),
];

for (edge_id, source, target) in edge_ids {
    let edge = Edge::simple(edge_id, source, target);
    graph.add_edge(edge)?;
}

// Remove an edge
graph.remove_edge(&"edge_0_1".into())?;

// Get all edges
for edge in graph.edges() {
    println!("Edge: {} -> {}", edge.source, edge.target);
}
```

### Graph Traversal

```rust
// Iterate over all nodes
for node in graph.nodes() {
    println!("Node: {} at {:?}", node.id, node.position);
}

// Find nodes in a specific area
let search_rect = Rect::new(50.0, 50.0, 200.0, 200.0);
let nodes_in_area: Vec<_> = graph.nodes()
    .filter(|node| search_rect.contains(node.position))
    .collect();

// Get connected nodes
if let Some(node) = graph.get_node(&"node_1".into()) {
    let connected_edges: Vec<_> = graph.edges()
        .filter(|edge| edge.source == node.id || edge.target == node.id)
        .collect();
}
```

## Viewport Management

### Basic Viewport Operations

```rust
use leptos_flow_core::Viewport;

// Create viewport with custom settings
let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

// Pan viewport
let pan_offset = Position::new(50.0, 30.0);
let panned_viewport = viewport.pan(pan_offset);

// Zoom to a specific point
let zoom_point = Position::new(100.0, 100.0);
let zoomed_viewport = panned_viewport.zoom_to_point(zoom_point, 2.0);

// Get viewport bounds
let bounds = zoomed_viewport.bounds();
println!("Viewport bounds: {:?}", bounds);
```

### Viewport with Offset

```rust
// Create viewport with offset
let offset = Position::new(100.0, 50.0);
let viewport = Viewport::with_offset(0.0, 0.0, 800.0, 600.0, 1.0, offset);

// Pan with offset
let new_offset = Position::new(150.0, 80.0);
let panned_viewport = viewport.pan(new_offset);
```

## Interaction Handling

### Basic Interaction Setup

```rust
use crate::interactions::{InteractionHandler, InteractionState};

// Create interaction handler
let mut interaction_handler = InteractionHandler::new(
    canvas.clone(),
    renderer,
    graph,
    viewport,
);

// Handle mouse events
let mouse_down_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
    interaction_handler.handle_mouse_down(&event);
}) as Box<dyn FnMut(_)>);

let mouse_move_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
    interaction_handler.handle_mouse_move(&event);
}) as Box<dyn FnMut(_)>);

let mouse_up_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
    interaction_handler.handle_mouse_up(&event);
}) as Box<dyn FnMut(_)>);

// Add event listeners
canvas.add_event_listener_with_callback(
    "mousedown",
    mouse_down_handler.as_ref().unchecked_ref(),
)?;

canvas.add_event_listener_with_callback(
    "mousemove", 
    mouse_move_handler.as_ref().unchecked_ref(),
)?;

canvas.add_event_listener_with_callback(
    "mouseup",
    mouse_up_handler.as_ref().unchecked_ref(),
)?;
```

### Node Selection

```rust
// Get selected nodes
let selected_nodes = interaction_handler.get_selected_nodes();
println!("Selected nodes: {:?}", selected_nodes);

// Check if specific node is selected
let node_id = "node1".into();
if interaction_handler.is_node_selected(&node_id) {
    println!("Node {} is selected", node_id);
}

// Clear selection
interaction_handler.clear_selection();
```

### Custom Interaction Logic

```rust
// Custom mouse down handler
fn custom_mouse_down(
    interaction_handler: &mut InteractionHandler,
    event: &MouseEvent,
) -> Result<(), JsValue> {
    let mouse_pos = interaction_handler.get_mouse_position(event);
    
    // Check for node click
    if let Some(node_id) = interaction_handler.get_node_at_position(mouse_pos) {
        if event.ctrl_key() {
            // Multi-select
            interaction_handler.state.selected_nodes.insert(node_id.clone());
        } else {
            // Single select
            interaction_handler.clear_selection();
            interaction_handler.state.selected_nodes.insert(node_id);
        }
    } else {
        // Start panning
        interaction_handler.state.is_panning = true;
        interaction_handler.state.last_mouse_pos = Some(mouse_pos);
    }
    
    Ok(())
}
```

## Background Configuration

### Different Background Patterns

```rust
use leptos_flow_renderer::traits::{BackgroundConfig, BackgroundVariant};

// Dots pattern
let dots_bg = BackgroundConfig {
    color: "#ffffff".to_string(),
    pattern_color: "#e2e8f0".to_string(),
    variant: BackgroundVariant::Dots,
    size: 20.0,
    opacity: 0.5,
};

// Grid pattern
let grid_bg = BackgroundConfig {
    color: "#f8fafc".to_string(),
    pattern_color: "#cbd5e1".to_string(),
    variant: BackgroundVariant::Grid,
    size: 25.0,
    opacity: 0.3,
};

// Solid color
let solid_bg = BackgroundConfig {
    color: "#f1f5f9".to_string(),
    pattern_color: "#f1f5f9".to_string(),
    variant: BackgroundVariant::Solid,
    size: 0.0,
    opacity: 1.0,
};

// Render with different backgrounds
renderer.render_background(&dots_bg, &viewport)?;
```

### Dynamic Background Changes

```rust
// Change background based on zoom level
let bg_config = if viewport.zoom > 2.0 {
    // High zoom - fine grid
    BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Grid,
        size: 10.0,
        opacity: 0.2,
    }
} else if viewport.zoom > 1.0 {
    // Medium zoom - dots
    BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.5,
    }
} else {
    // Low zoom - solid
    BackgroundConfig {
        color: "#f8fafc".to_string(),
        pattern_color: "#f8fafc".to_string(),
        variant: BackgroundVariant::Solid,
        size: 0.0,
        opacity: 1.0,
    }
};

renderer.render_background(&bg_config, &viewport)?;
```

## Error Handling

### Comprehensive Error Handling

```rust
use leptos_flow_renderer::error::RendererError;

// Handle renderer errors
match renderer.render_graph(&graph, &viewport) {
    Ok(_) => {
        // Rendering successful
        renderer.present()?;
    }
    Err(RendererError::CanvasError(msg)) => {
        console::error_1(&format!("Canvas error: {}", msg).into());
        // Handle canvas-specific error
    }
    Err(RendererError::InvalidState(msg)) => {
        console::error_1(&format!("Invalid state: {}", msg).into());
        // Handle state error
    }
    Err(e) => {
        console::error_1(&format!("Renderer error: {:?}", e).into());
        // Handle other errors
    }
}

// Handle graph errors
match graph.add_node(node) {
    Ok(_) => {
        // Node added successfully
    }
    Err(e) => {
        console::error_1(&format!("Graph error: {:?}", e).into());
        // Handle graph error
    }
}
```

### Error Recovery

```rust
// Retry mechanism for renderer operations
fn render_with_retry(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<(), ()>,
    viewport: &Viewport,
    max_retries: usize,
) -> Result<(), RendererError> {
    for attempt in 0..max_retries {
        match renderer.render_graph(graph, viewport) {
            Ok(_) => return Ok(()),
            Err(e) if attempt == max_retries - 1 => return Err(e),
            Err(e) => {
                console::warn_1(&format!("Render attempt {} failed: {:?}", attempt + 1, e).into());
                // Wait before retry (in real app, you might use setTimeout)
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
    unreachable!()
}
```

## Performance Optimization

### Efficient Rendering

```rust
// Only render when necessary
struct RenderState {
    needs_redraw: bool,
    last_render_time: f64,
}

impl RenderState {
    fn should_render(&self, current_time: f64) -> bool {
        self.needs_redraw || (current_time - self.last_render_time) > 16.0 // 60 FPS
    }
    
    fn mark_rendered(&mut self, current_time: f64) {
        self.needs_redraw = false;
        self.last_render_time = current_time;
    }
}

// Use in render loop
let mut render_state = RenderState {
    needs_redraw: true,
    last_render_time: 0.0,
};

let render_loop = Closure::wrap(Box::new(move || {
    let current_time = web_sys::js_sys::Date::now();
    
    if render_state.should_render(current_time) {
        renderer.clear(Some("#ffffff"))?;
        renderer.render_background(&bg_config, &viewport)?;
        renderer.render_graph(&graph, &viewport)?;
        renderer.present()?;
        
        render_state.mark_rendered(current_time);
    }
    
    web_sys::window().unwrap().request_animation_frame(render_loop.as_ref().unchecked_ref()).unwrap();
}) as Box<dyn FnMut()>);
```

### Batch Operations

```rust
// Batch node additions
fn add_nodes_batch(graph: &mut Graph<(), ()>, nodes: Vec<Node<()>>) -> Result<(), Box<dyn std::error::Error>> {
    for node in nodes {
        graph.add_node(node)?;
    }
    Ok(())
}

// Batch edge additions
fn add_edges_batch(graph: &mut Graph<(), ()>, edges: Vec<Edge<()>>) -> Result<(), Box<dyn std::error::Error>> {
    for edge in edges {
        graph.add_edge(edge)?;
    }
    Ok(())
}
```

### Memory Management

```rust
// Clean up resources
impl Drop for InteractionHandler {
    fn drop(&mut self) {
        // Remove event listeners
        // Clean up any allocated resources
    }
}

// Use RAII for temporary resources
struct TemporaryRenderer<'a> {
    renderer: &'a mut Canvas2DRenderer,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> TemporaryRenderer<'a> {
    fn new(renderer: &'a mut Canvas2DRenderer) -> Self {
        Self {
            renderer,
            _marker: std::marker::PhantomData,
        }
    }
    
    fn render(&mut self, graph: &Graph<(), ()>, viewport: &Viewport) -> Result<(), RendererError> {
        self.renderer.render_graph(graph, viewport)
    }
}
```

## Complete Example

Here's a complete example that demonstrates all the concepts:

```rust
use leptos_flow_core::{Graph, Node, Edge, Position, Viewport};
use leptos_flow_renderer::{Canvas2DRenderer, Renderer};
use leptos_flow_renderer::traits::{BackgroundConfig, BackgroundVariant, NodeStyle, NodeVariant};
use web_sys::{HtmlCanvasElement, MouseEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn create_flow_diagram(canvas_id: &str) -> Result<(), JsValue> {
    // Get canvas element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Create renderer
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(800, 600)?;

    // Create graph
    let mut graph = Graph::new();
    
    // Add nodes with custom styling
    let nodes = vec![
        ("start", Position::new(100.0, 100.0), "#10b981"),
        ("process", Position::new(300.0, 100.0), "#3b82f6"),
        ("decision", Position::new(500.0, 100.0), "#f59e0b"),
        ("end", Position::new(700.0, 100.0), "#ef4444"),
    ];

    for (id, pos, color) in nodes {
        let mut node = Node::simple(id, pos);
        node.style = NodeStyle {
            variant: NodeVariant::Rectangle,
            fill_color: color.to_string(),
            stroke_color: "#1f2937".to_string(),
            stroke_width: 2.0,
            width: 100.0,
            height: 60.0,
            corner_radius: 8.0,
            ..Default::default()
        };
        graph.add_node(node)?;
    }

    // Add edges
    let edges = vec![
        ("e1", "start", "process"),
        ("e2", "process", "decision"),
        ("e3", "decision", "end"),
    ];

    for (id, source, target) in edges {
        let edge = Edge::simple(id, source, target);
        graph.add_edge(edge)?;
    }

    // Create viewport
    let viewport = Viewport::default();

    // Render
    renderer.clear(Some("#ffffff"))?;
    
    let bg_config = BackgroundConfig {
        color: "#f8fafc".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.5,
    };
    renderer.render_background(&bg_config, &viewport)?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;

    Ok(())
}
```

This comprehensive API documentation provides examples for all major functionality of the Leptos Flow Simple Example, from basic setup to advanced performance optimization techniques.
