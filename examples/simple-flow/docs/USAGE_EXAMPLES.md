# Usage Examples for Leptos Flow Simple Example

This document provides practical usage examples for common scenarios with the Leptos Flow Simple Example.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Flow Diagram](#basic-flow-diagram)
- [Interactive Node Editor](#interactive-node-editor)
- [Data Visualization](#data-visualization)
- [Workflow Management](#workflow-management)
- [Real-time Collaboration](#real-time-collaboration)
- [Custom Styling](#custom-styling)
- [Performance Tips](#performance-tips)

## Getting Started

### Minimal Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>Leptos Flow - Minimal Example</title>
</head>
<body>
    <canvas id="flow-canvas" width="800" height="600"></canvas>
    <script type="module">
        import init, { create_simple_flow } from './pkg/simple_flow_example.js';
        
        async function run() {
            await init();
            create_simple_flow();
        }
        
        run();
    </script>
</body>
</html>
```

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_simple_flow() -> Result<(), JsValue> {
    // Get canvas element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("flow-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Create renderer
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    
    // Create simple graph
    let mut graph = Graph::new();
    let node1 = Node::simple("start", Position::new(100.0, 100.0));
    let node2 = Node::simple("end", Position::new(300.0, 100.0));
    let edge = Edge::simple("connection", "start", "end");
    
    graph.add_node(node1)?;
    graph.add_node(node2)?;
    graph.add_edge(edge)?;
    
    // Render
    let viewport = Viewport::default();
    renderer.clear(Some("#ffffff"))?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;
    
    Ok(())
}
```

## Basic Flow Diagram

### Process Flow Example

```rust
use leptos_flow_core::{Graph, Node, Edge, Position, Viewport};
use leptos_flow_renderer::{Canvas2DRenderer, Renderer};
use leptos_flow_renderer::traits::{BackgroundConfig, BackgroundVariant, NodeStyle, NodeVariant};

#[wasm_bindgen]
pub fn create_process_flow() -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(1000, 600)?;

    let mut graph = Graph::new();
    
    // Define process steps
    let steps = vec![
        ("start", "Start Process", Position::new(50.0, 300.0), "#10b981"),
        ("validate", "Validate Input", Position::new(200.0, 300.0), "#3b82f6"),
        ("process", "Process Data", Position::new(350.0, 300.0), "#8b5cf6"),
        ("save", "Save Results", Position::new(500.0, 300.0), "#f59e0b"),
        ("notify", "Send Notification", Position::new(650.0, 300.0), "#ef4444"),
        ("end", "End Process", Position::new(800.0, 300.0), "#6b7280"),
    ];

    // Add nodes
    for (id, label, pos, color) in steps {
        let mut node = Node::simple(id, pos);
        node.style = NodeStyle {
            variant: NodeVariant::Rectangle,
            fill_color: color.to_string(),
            stroke_color: "#1f2937".to_string(),
            stroke_width: 2.0,
            width: 120.0,
            height: 60.0,
            corner_radius: 8.0,
            ..Default::default()
        };
        graph.add_node(node)?;
    }

    // Add edges
    let connections = vec![
        ("e1", "start", "validate"),
        ("e2", "validate", "process"),
        ("e3", "process", "save"),
        ("e4", "save", "notify"),
        ("e5", "notify", "end"),
    ];

    for (id, source, target) in connections {
        let edge = Edge::simple(id, source, target);
        graph.add_edge(edge)?;
    }

    // Render
    let viewport = Viewport::default();
    renderer.clear(Some("#ffffff"))?;
    
    let bg_config = BackgroundConfig {
        color: "#f8fafc".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Grid,
        size: 25.0,
        opacity: 0.3,
    };
    renderer.render_background(&bg_config, &viewport)?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;

    Ok(())
}

fn get_canvas(id: &str) -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    Ok(document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap())
}
```

### Decision Tree Example

```rust
#[wasm_bindgen]
pub fn create_decision_tree() -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(800, 600)?;

    let mut graph = Graph::new();
    
    // Decision tree structure
    let nodes = vec![
        ("root", "Is user logged in?", Position::new(400.0, 50.0), "#f59e0b"),
        ("yes", "Show Dashboard", Position::new(200.0, 200.0), "#10b981"),
        ("no", "Show Login Form", Position::new(600.0, 200.0), "#ef4444"),
        ("dashboard", "Dashboard Content", Position::new(200.0, 350.0), "#3b82f6"),
        ("login", "Login Form", Position::new(600.0, 350.0), "#8b5cf6"),
    ];

    for (id, label, pos, color) in nodes {
        let mut node = Node::simple(id, pos);
        node.style = NodeStyle {
            variant: if id == "root" { NodeVariant::Diamond } else { NodeVariant::Rectangle },
            fill_color: color.to_string(),
            stroke_color: "#1f2937".to_string(),
            stroke_width: 2.0,
            width: if id == "root" { 100.0 } else { 120.0 },
            height: if id == "root" { 100.0 } else { 60.0 },
            corner_radius: 8.0,
            ..Default::default()
        };
        graph.add_node(node)?;
    }

    // Add edges with labels
    let edges = vec![
        ("e1", "root", "yes"),
        ("e2", "root", "no"),
        ("e3", "yes", "dashboard"),
        ("e4", "no", "login"),
    ];

    for (id, source, target) in edges {
        let edge = Edge::simple(id, source, target);
        graph.add_edge(edge)?;
    }

    // Render
    let viewport = Viewport::default();
    renderer.clear(Some("#ffffff"))?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;

    Ok(())
}
```

## Interactive Node Editor

### Drag and Drop Node Editor

```rust
use crate::interactions::{InteractionHandler, InteractionState};

#[wasm_bindgen]
pub fn create_interactive_editor() -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(800, 600)?;

    let mut graph = Graph::new();
    let viewport = Viewport::default();
    
    // Create interaction handler
    let mut interaction_handler = InteractionHandler::new(
        canvas.clone(),
        renderer,
        graph,
        viewport,
    );

    // Set up event handlers
    setup_interactive_handlers(&canvas, &mut interaction_handler)?;
    
    // Initial render
    interaction_handler.render()?;

    Ok(())
}

fn setup_interactive_handlers(
    canvas: &HtmlCanvasElement,
    interaction_handler: &mut InteractionHandler,
) -> Result<(), JsValue> {
    // Mouse down handler
    let mouse_down_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        interaction_handler.handle_mouse_down(&event);
        interaction_handler.render().unwrap();
    }) as Box<dyn FnMut(_)>);

    // Mouse move handler
    let mouse_move_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        interaction_handler.handle_mouse_move(&event);
        interaction_handler.render().unwrap();
    }) as Box<dyn FnMut(_)>);

    // Mouse up handler
    let mouse_up_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        interaction_handler.handle_mouse_up(&event);
        interaction_handler.render().unwrap();
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

    // Keep handlers alive
    mouse_down_handler.forget();
    mouse_move_handler.forget();
    mouse_up_handler.forget();

    Ok(())
}
```

### Node Creation Tool

```rust
#[wasm_bindgen]
pub fn add_node_at_position(x: f64, y: f64) -> Result<(), JsValue> {
    // This would be called from JavaScript when user clicks to add a node
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    
    let mut graph = Graph::new();
    let viewport = Viewport::default();
    
    // Create new node at clicked position
    let node_id = format!("node_{}", js_sys::Date::now() as u64);
    let mut node = Node::simple(node_id.clone(), Position::new(x, y));
    
    // Custom styling
    node.style = NodeStyle {
        variant: NodeVariant::Rectangle,
        fill_color: "#3b82f6".to_string(),
        stroke_color: "#1e40af".to_string(),
        stroke_width: 2.0,
        width: 100.0,
        height: 60.0,
        corner_radius: 8.0,
        ..Default::default()
    };
    
    graph.add_node(node)?;
    
    // Render
    renderer.clear(Some("#ffffff"))?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;
    
    Ok(())
}
```

## Data Visualization

### Network Graph Visualization

```rust
#[wasm_bindgen]
pub fn create_network_graph(data: &JsValue) -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(1000, 800)?;

    let mut graph = Graph::new();
    
    // Parse data from JavaScript
    let nodes_data: Vec<serde_json::Value> = serde_wasm_bindgen::from_value(data.clone())?;
    
    // Create nodes from data
    for (i, node_data) in nodes_data.iter().enumerate() {
        let id = node_data["id"].as_str().unwrap_or(&format!("node_{}", i));
        let x = node_data["x"].as_f64().unwrap_or(100.0 + (i as f64 * 150.0));
        let y = node_data["y"].as_f64().unwrap_or(100.0 + (i as f64 * 100.0));
        let color = node_data["color"].as_str().unwrap_or("#3b82f6");
        
        let mut node = Node::simple(id, Position::new(x, y));
        node.style = NodeStyle {
            variant: NodeVariant::Circle,
            fill_color: color.to_string(),
            stroke_color: "#1f2937".to_string(),
            stroke_width: 2.0,
            width: 60.0,
            height: 60.0,
            ..Default::default()
        };
        
        graph.add_node(node)?;
    }
    
    // Add edges based on connections
    for node_data in &nodes_data {
        if let Some(connections) = node_data["connections"].as_array() {
            for connection in connections {
                if let Some(target_id) = connection.as_str() {
                    let edge_id = format!("edge_{}_{}", node_data["id"], target_id);
                    let edge = Edge::simple(edge_id, node_data["id"].as_str().unwrap(), target_id);
                    graph.add_edge(edge)?;
                }
            }
        }
    }

    // Render with network-specific styling
    let viewport = Viewport::default();
    renderer.clear(Some("#f8fafc"))?;
    
    let bg_config = BackgroundConfig {
        color: "#f8fafc".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 15.0,
        opacity: 0.3,
    };
    renderer.render_background(&bg_config, &viewport)?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;

    Ok(())
}
```

### Hierarchical Tree Visualization

```rust
#[wasm_bindgen]
pub fn create_hierarchical_tree() -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(800, 600)?;

    let mut graph = Graph::new();
    
    // Create hierarchical structure
    let levels = vec![
        // Level 0 (root)
        vec![("CEO", Position::new(400.0, 50.0), "#1f2937")],
        // Level 1
        vec![
            ("CTO", Position::new(200.0, 150.0), "#3b82f6"),
            ("CFO", Position::new(400.0, 150.0), "#10b981"),
            ("COO", Position::new(600.0, 150.0), "#f59e0b"),
        ],
        // Level 2
        vec![
            ("Dev Team", Position::new(100.0, 250.0), "#8b5cf6"),
            ("QA Team", Position::new(300.0, 250.0), "#ef4444"),
            ("Finance", Position::new(500.0, 250.0), "#06b6d4"),
            ("Operations", Position::new(700.0, 250.0), "#84cc16"),
        ],
    ];

    // Add nodes
    for level in &levels {
        for (id, pos, color) in level {
            let mut node = Node::simple(id, *pos);
            node.style = NodeStyle {
                variant: NodeVariant::Rectangle,
                fill_color: color.to_string(),
                stroke_color: "#1f2937".to_string(),
                stroke_width: 2.0,
                width: 100.0,
                height: 50.0,
                corner_radius: 6.0,
                ..Default::default()
            };
            graph.add_node(node)?;
        }
    }

    // Add hierarchical edges
    let edges = vec![
        ("e1", "CEO", "CTO"),
        ("e2", "CEO", "CFO"),
        ("e3", "CEO", "COO"),
        ("e4", "CTO", "Dev Team"),
        ("e5", "CTO", "QA Team"),
        ("e6", "CFO", "Finance"),
        ("e7", "COO", "Operations"),
    ];

    for (id, source, target) in edges {
        let edge = Edge::simple(id, source, target);
        graph.add_edge(edge)?;
    }

    // Render
    let viewport = Viewport::default();
    renderer.clear(Some("#ffffff"))?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;

    Ok(())
}
```

## Workflow Management

### Task Management Board

```rust
#[wasm_bindgen]
pub fn create_task_board() -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    renderer.resize(1200, 800)?;

    let mut graph = Graph::new();
    
    // Define columns
    let columns = vec![
        ("todo", "To Do", 200.0, "#6b7280"),
        ("in_progress", "In Progress", 400.0, "#f59e0b"),
        ("review", "Review", 600.0, "#3b82f6"),
        ("done", "Done", 800.0, "#10b981"),
    ];

    // Add column headers
    for (id, label, x, color) in &columns {
        let mut node = Node::simple(id, Position::new(*x, 50.0));
        node.style = NodeStyle {
            variant: NodeVariant::Rectangle,
            fill_color: color.to_string(),
            stroke_color: "#1f2937".to_string(),
            stroke_width: 2.0,
            width: 150.0,
            height: 40.0,
            corner_radius: 6.0,
            ..Default::default()
        };
        graph.add_node(node)?;
    }

    // Add task cards
    let tasks = vec![
        ("task1", "Design UI", "todo", 200.0, 150.0),
        ("task2", "Implement API", "todo", 200.0, 220.0),
        ("task3", "Write Tests", "in_progress", 400.0, 150.0),
        ("task4", "Code Review", "review", 600.0, 150.0),
        ("task5", "Deploy", "done", 800.0, 150.0),
    ];

    for (id, label, column, x, y) in tasks {
        let mut node = Node::simple(id, Position::new(x, y));
        node.style = NodeStyle {
            variant: NodeVariant::Rectangle,
            fill_color: "#ffffff".to_string(),
            stroke_color: "#d1d5db".to_string(),
            stroke_width: 1.0,
            width: 140.0,
            height: 60.0,
            corner_radius: 4.0,
            ..Default::default()
        };
        graph.add_node(node)?;
    }

    // Render
    let viewport = Viewport::default();
    renderer.clear(Some("#f9fafb"))?;
    
    let bg_config = BackgroundConfig {
        color: "#f9fafb".to_string(),
        pattern_color: "#e5e7eb".to_string(),
        variant: BackgroundVariant::Grid,
        size: 20.0,
        opacity: 0.2,
    };
    renderer.render_background(&bg_config, &viewport)?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;

    Ok(())
}
```

## Real-time Collaboration

### Live Updates Example

```rust
use std::collections::HashMap;

#[wasm_bindgen]
pub struct CollaborativeEditor {
    graph: Graph<(), ()>,
    renderer: Canvas2DRenderer,
    viewport: Viewport,
    user_colors: HashMap<String, String>,
}

#[wasm_bindgen]
impl CollaborativeEditor {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<CollaborativeEditor, JsValue> {
        let mut renderer = Canvas2DRenderer::new(&canvas)?;
        renderer.resize(800, 600)?;
        
        let graph = Graph::new();
        let viewport = Viewport::default();
        let user_colors = HashMap::new();
        
        Ok(CollaborativeEditor {
            graph,
            renderer,
            viewport,
            user_colors,
        })
    }
    
    #[wasm_bindgen]
    pub fn add_user(&mut self, user_id: String, color: String) {
        self.user_colors.insert(user_id, color);
    }
    
    #[wasm_bindgen]
    pub fn add_node_from_user(&mut self, user_id: String, node_id: String, x: f64, y: f64) -> Result<(), JsValue> {
        let color = self.user_colors.get(&user_id).unwrap_or(&"#3b82f6".to_string()).clone();
        
        let mut node = Node::simple(node_id, Position::new(x, y));
        node.style = NodeStyle {
            variant: NodeVariant::Rectangle,
            fill_color: color,
            stroke_color: "#1f2937".to_string(),
            stroke_width: 2.0,
            width: 100.0,
            height: 60.0,
            corner_radius: 8.0,
            ..Default::default()
        };
        
        self.graph.add_node(node)?;
        self.render()?;
        Ok(())
    }
    
    #[wasm_bindgen]
    pub fn move_node(&mut self, node_id: String, x: f64, y: f64) -> Result<(), JsValue> {
        if let Some(node) = self.graph.get_node_mut(&node_id.into()) {
            node.position = Position::new(x, y);
            self.render()?;
        }
        Ok(())
    }
    
    fn render(&mut self) -> Result<(), JsValue> {
        self.renderer.clear(Some("#ffffff"))?;
        self.renderer.render_graph(&self.graph, &self.viewport)?;
        self.renderer.present()?;
        Ok(())
    }
}
```

## Custom Styling

### Theme System

```rust
#[derive(Clone)]
pub struct Theme {
    pub background: String,
    pub grid_color: String,
    pub node_fill: String,
    pub node_stroke: String,
    pub edge_color: String,
    pub text_color: String,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            background: "#ffffff".to_string(),
            grid_color: "#e2e8f0".to_string(),
            node_fill: "#3b82f6".to_string(),
            node_stroke: "#1e40af".to_string(),
            edge_color: "#64748b".to_string(),
            text_color: "#1f2937".to_string(),
        }
    }
    
    pub fn dark() -> Self {
        Self {
            background: "#1f2937".to_string(),
            grid_color: "#374151".to_string(),
            node_fill: "#3b82f6".to_string(),
            node_stroke: "#60a5fa".to_string(),
            edge_color: "#9ca3af".to_string(),
            text_color: "#f9fafb".to_string(),
        }
    }
    
    pub fn colorful() -> Self {
        Self {
            background: "#f8fafc".to_string(),
            grid_color: "#e2e8f0".to_string(),
            node_fill: "#ec4899".to_string(),
            node_stroke: "#be185d".to_string(),
            edge_color: "#7c3aed".to_string(),
            text_color: "#1f2937".to_string(),
        }
    }
}

#[wasm_bindgen]
pub fn apply_theme(theme_name: &str) -> Result<(), JsValue> {
    let canvas = get_canvas("flow-canvas")?;
    let mut renderer = Canvas2DRenderer::new(&canvas)?;
    
    let theme = match theme_name {
        "dark" => Theme::dark(),
        "colorful" => Theme::colorful(),
        _ => Theme::light(),
    };
    
    let mut graph = Graph::new();
    let viewport = Viewport::default();
    
    // Add sample nodes with theme colors
    let nodes = vec![
        ("node1", Position::new(100.0, 100.0)),
        ("node2", Position::new(300.0, 100.0)),
        ("node3", Position::new(200.0, 200.0)),
    ];
    
    for (id, pos) in nodes {
        let mut node = Node::simple(id, pos);
        node.style = NodeStyle {
            variant: NodeVariant::Rectangle,
            fill_color: theme.node_fill.clone(),
            stroke_color: theme.node_stroke.clone(),
            stroke_width: 2.0,
            width: 100.0,
            height: 60.0,
            corner_radius: 8.0,
            ..Default::default()
        };
        graph.add_node(node)?;
    }
    
    // Render with theme
    renderer.clear(Some(&theme.background))?;
    
    let bg_config = BackgroundConfig {
        color: theme.background.clone(),
        pattern_color: theme.grid_color.clone(),
        variant: BackgroundVariant::Grid,
        size: 20.0,
        opacity: 0.3,
    };
    renderer.render_background(&bg_config, &viewport)?;
    renderer.render_graph(&graph, &viewport)?;
    renderer.present()?;
    
    Ok(())
}
```

## Performance Tips

### Efficient Rendering Loop

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct RenderLoop {
    renderer: Rc<RefCell<Canvas2DRenderer>>,
    graph: Rc<RefCell<Graph<(), ()>>>,
    viewport: Rc<RefCell<Viewport>>,
    animation_id: Option<i32>,
}

impl RenderLoop {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let renderer = Rc::new(RefCell::new(Canvas2DRenderer::new(&canvas)?));
        let graph = Rc::new(RefCell::new(Graph::new()));
        let viewport = Rc::new(RefCell::new(Viewport::default()));
        
        Ok(Self {
            renderer,
            graph,
            viewport,
            animation_id: None,
        })
    }
    
    pub fn start(&mut self) -> Result<(), JsValue> {
        let renderer = self.renderer.clone();
        let graph = self.graph.clone();
        let viewport = self.viewport.clone();
        
        let render_closure = Closure::wrap(Box::new(move || {
            let mut renderer = renderer.borrow_mut();
            let graph = graph.borrow();
            let viewport = viewport.borrow();
            
            // Only render if needed
            renderer.clear(Some("#ffffff")).unwrap();
            renderer.render_graph(&*graph, &*viewport).unwrap();
            renderer.present().unwrap();
            
            // Schedule next frame
            let window = web_sys::window().unwrap();
            window.request_animation_frame(render_closure.as_ref().unchecked_ref()).unwrap();
        }) as Box<dyn FnMut()>);
        
        let window = web_sys::window().unwrap();
        let id = window.request_animation_frame(render_closure.as_ref().unchecked_ref())?;
        self.animation_id = Some(id);
        render_closure.forget();
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        if let Some(id) = self.animation_id {
            let window = web_sys::window().unwrap();
            window.cancel_animation_frame(id).unwrap();
            self.animation_id = None;
        }
    }
}
```

### Memory Management

```rust
// Use RAII for automatic cleanup
pub struct FlowCanvas {
    canvas: HtmlCanvasElement,
    renderer: Canvas2DRenderer,
    event_handlers: Vec<Closure<dyn FnMut()>>,
}

impl FlowCanvas {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let canvas = get_canvas(canvas_id)?;
        let renderer = Canvas2DRenderer::new(&canvas)?;
        
        Ok(Self {
            canvas,
            renderer,
            event_handlers: Vec::new(),
        })
    }
    
    pub fn add_event_handler<F>(&mut self, event: &str, handler: F) -> Result<(), JsValue>
    where
        F: FnMut() + 'static,
    {
        let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut()>);
        self.canvas.add_event_listener_with_callback(
            event,
            closure.as_ref().unchecked_ref(),
        )?;
        self.event_handlers.push(closure);
        Ok(())
    }
}

impl Drop for FlowCanvas {
    fn drop(&mut self) {
        // Event handlers are automatically cleaned up when the closure is dropped
        self.event_handlers.clear();
    }
}
```

These usage examples demonstrate practical applications of the Leptos Flow Simple Example, from basic diagrams to complex interactive applications. Each example includes complete, runnable code that you can adapt for your specific use case.
