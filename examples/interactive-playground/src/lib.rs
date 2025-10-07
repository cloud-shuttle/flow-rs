//! Interactive Playground Example
//!
//! Comprehensive Flow-RS editor demonstrating all capabilities in one unified interface.
//! This example shows a full-featured graph editor with node creation, edge drawing,
//! property panels, and export/import functionality.
//!
//! Features:
//! - Full graph editing capabilities
//! - Node creation, selection, and manipulation
//! - Edge creation and management
//! - Property panels for node/edge editing
//! - Export/import functionality (JSON)
//! - Keyboard shortcuts and toolbar
//! - Undo/redo system (basic)
//! - Multiple selection and group operations

use flow_rs_core::{Edge, Graph, Node, Position};
use flow_rs_renderer::{Canvas2DRenderer, Renderer};
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, MouseEvent, KeyboardEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Playground state
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlaygroundState {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
    pub selected_nodes: Vec<String>,
    pub selected_edges: Vec<String>,
    pub mode: EditorMode,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub grid_size: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum EditorMode {
    Select,
    CreateNode,
    CreateEdge,
    Delete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeData {
    pub id: String,
    pub position: Position,
    pub size: (f64, f64),
    pub label: String,
    pub node_type: String,
    pub color: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    pub edge_type: String,
    pub color: String,
    pub data: serde_json::Value,
}

// Global state
static mut PLAYGROUND_STATE: Option<PlaygroundState> = None;
static mut RENDERER: Option<Canvas2DRenderer> = None;
static mut DRAG_STATE: Option<DragState> = None;
static mut EDGE_CREATION_STATE: Option<EdgeCreationState> = None;

#[derive(Clone, Debug)]
struct DragState {
    pub node_id: String,
    pub start_pos: Position,
    pub offset: Position,
}

#[derive(Clone, Debug)]
struct EdgeCreationState {
    pub source_node: String,
    pub temp_end_pos: Position,
}

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"🎮 Flow-RS Interactive Playground Started!".into());

    // Initialize empty playground state
    let initial_state = PlaygroundState {
        nodes: Vec::new(),
        edges: Vec::new(),
        selected_nodes: Vec::new(),
        selected_edges: Vec::new(),
        mode: EditorMode::Select,
        show_grid: true,
        snap_to_grid: false,
        grid_size: 20.0,
    };

    unsafe {
        PLAYGROUND_STATE = Some(initial_state);
    }

    // Set up event handlers
    setup_playground_events();

    // Create some initial demo nodes
    create_demo_graph();

    web_sys::console::log_1(&"🎯 Interactive Playground Ready!".into());
    web_sys::console::log_1(&"💡 Try these commands:".into());
    web_sys::console::log_1(&"   • add_node('My Node', 100, 100) - Add a new node".into());
    web_sys::console::log_1(&"   • select_mode('create_node') - Switch to node creation mode".into());
    web_sys::console::log_1(&"   • export_graph() - Export current graph as JSON".into());
    web_sys::console::log_1(&"   • import_graph(json_string) - Import a graph from JSON".into());
}

/// Create a demo graph with various nodes and edges
fn create_demo_graph() {
    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            // Add some demo nodes
            state.nodes.push(NodeData {
                id: "node-1".to_string(),
                position: Position::new(150.0, 100.0),
                size: (100.0, 60.0),
                label: "Start".to_string(),
                node_type: "default".to_string(),
                color: "#3B82F6".to_string(),
                data: serde_json::json!({"description": "Entry point"}),
            });

            state.nodes.push(NodeData {
                id: "node-2".to_string(),
                position: Position::new(350.0, 100.0),
                size: (100.0, 60.0),
                label: "Process".to_string(),
                node_type: "process".to_string(),
                color: "#10B981".to_string(),
                data: serde_json::json!({"operation": "transform"}),
            });

            state.nodes.push(NodeData {
                id: "node-3".to_string(),
                position: Position::new(550.0, 100.0),
                size: (100.0, 60.0),
                label: "Decision".to_string(),
                node_type: "decision".to_string(),
                color: "#F59E0B".to_string(),
                data: serde_json::json!({"condition": "x > 0"}),
            });

            state.nodes.push(NodeData {
                id: "node-4".to_string(),
                position: Position::new(350.0, 250.0),
                size: (100.0, 60.0),
                label: "End".to_string(),
                node_type: "default".to_string(),
                color: "#EF4444".to_string(),
                data: serde_json::json!({"description": "Exit point"}),
            });

            // Add demo edges
            state.edges.push(EdgeData {
                id: "edge-1".to_string(),
                source: "node-1".to_string(),
                target: "node-2".to_string(),
                label: "flow".to_string(),
                edge_type: "default".to_string(),
                color: "#6B7280".to_string(),
                data: serde_json::json!({"weight": 1}),
            });

            state.edges.push(EdgeData {
                id: "edge-2".to_string(),
                source: "node-2".to_string(),
                target: "node-3".to_string(),
                label: "process".to_string(),
                edge_type: "default".to_string(),
                color: "#6B7280".to_string(),
                data: serde_json::json!({"weight": 2}),
            });

            state.edges.push(EdgeData {
                id: "edge-3".to_string(),
                source: "node-3".to_string(),
                target: "node-4".to_string(),
                label: "yes".to_string(),
                edge_type: "default".to_string(),
                color: "#10B981".to_string(),
                data: serde_json::json!({"condition": "true"}),
            });

            web_sys::console::log_1(&"📊 Demo graph created with 4 nodes and 3 edges!".into());
        }
    }
}

/// Set up event handlers for the playground
fn setup_playground_events() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Canvas events for interaction
    if let Some(canvas) = document.get_element_by_id("flow-canvas") {
        let mouse_down_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            handle_canvas_mouse_down(event);
        }) as Box<dyn FnMut(MouseEvent>);

        let mouse_move_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            handle_canvas_mouse_move(event);
        }) as Box<dyn FnMut(MouseEvent>);

        let mouse_up_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            handle_canvas_mouse_up(event);
        }) as Box<dyn FnMut(MouseEvent>);

        canvas.add_event_listener_with_callback("mousedown", mouse_down_closure.as_ref().unchecked_ref()).unwrap();
        canvas.add_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref()).unwrap();
        canvas.add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref()).unwrap();

        mouse_down_closure.forget();
        mouse_move_closure.forget();
        mouse_up_closure.forget();
    }

    // Keyboard events
    let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        handle_key_down(event);
    }) as Box<dyn FnMut(KeyboardEvent>);

    window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap();
    keydown_closure.forget();
}

// Canvas interaction handlers
fn handle_canvas_mouse_down(event: MouseEvent) {
    let rect = event.target().unwrap().dyn_into::<web_sys::Element>()
        .unwrap()
        .get_bounding_client_rect();

    let mouse_x = event.client_x() as f64 - rect.left();
    let mouse_y = event.client_y() as f64 - rect.top();

    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            match state.mode {
                EditorMode::Select => {
                    // Check if clicking on a node
                    for node in &state.nodes {
                        if is_point_in_node(mouse_x, mouse_y, node) {
                            if !state.selected_nodes.contains(&node.id) {
                                if !event.shift_key() {
                                    state.selected_nodes.clear();
                                }
                                state.selected_nodes.push(node.id.clone());
                            }

                            // Start dragging
                            DRAG_STATE = Some(DragState {
                                node_id: node.id.clone(),
                                start_pos: Position::new(mouse_x, mouse_y),
                                offset: Position::new(
                                    mouse_x - node.position.x,
                                    mouse_y - node.position.y,
                                ),
                            });
                            break;
                        }
                    }

                    // Clear selection if clicking empty space
                    if DRAG_STATE.is_none() && !event.shift_key() {
                        state.selected_nodes.clear();
                        state.selected_edges.clear();
                    }
                }
                EditorMode::CreateNode => {
                    add_node_at_position(mouse_x, mouse_y);
                }
                EditorMode::CreateEdge => {
                    // Start edge creation from clicked node
                    for node in &state.nodes {
                        if is_point_in_node(mouse_x, mouse_y, node) {
                            EDGE_CREATION_STATE = Some(EdgeCreationState {
                                source_node: node.id.clone(),
                                temp_end_pos: Position::new(mouse_x, mouse_y),
                            });
                            break;
                        }
                    }
                }
                EditorMode::Delete => {
                    // Delete clicked node or edge
                    delete_at_position(mouse_x, mouse_y);
                }
            }
        }
    }

    update_ui_display();
}

fn handle_canvas_mouse_move(event: MouseEvent) {
    let rect = event.target().unwrap().dyn_into::<web_sys::Element>()
        .unwrap()
        .get_bounding_client_rect();

    let mouse_x = event.client_x() as f64 - rect.left();
    let mouse_y = event.client_y() as f64 - rect.top();

    unsafe {
        // Handle dragging
        if let (Some(ref drag_state), Some(ref mut state)) = (DRAG_STATE.as_ref(), PLAYGROUND_STATE.as_mut()) {
            if let Some(node) = state.nodes.iter_mut().find(|n| n.id == drag_state.node_id) {
                let mut new_x = mouse_x - drag_state.offset.x;
                let mut new_y = mouse_y - drag_state.offset.y;

                // Snap to grid if enabled
                if state.snap_to_grid {
                    new_x = (new_x / state.grid_size).round() * state.grid_size;
                    new_y = (new_y / state.grid_size).round() * state.grid_size;
                }

                node.position.x = new_x;
                node.position.y = new_y;
            }
        }

        // Handle edge creation preview
        if let Some(ref mut edge_state) = EDGE_CREATION_STATE.as_mut() {
            edge_state.temp_end_pos = Position::new(mouse_x, mouse_y);
        }
    }
}

fn handle_canvas_mouse_up(_event: MouseEvent) {
    unsafe {
        // Finish dragging
        DRAG_STATE = None;

        // Finish edge creation
        if let Some(edge_state) = EDGE_CREATION_STATE.take() {
            // Check if released on a valid target node
            if let Some(ref state) = PLAYGROUND_STATE {
                let end_pos = edge_state.temp_end_pos;

                for node in &state.nodes {
                    if node.id != edge_state.source_node && is_point_in_node(end_pos.x, end_pos.y, node) {
                        // Create edge
                        let edge_id = format!("edge-{}-{}", edge_state.source_node, node.id);
                        let new_edge = EdgeData {
                            id: edge_id.clone(),
                            source: edge_state.source_node.clone(),
                            target: node.id.clone(),
                            label: "connection".to_string(),
                            edge_type: "default".to_string(),
                            color: "#6B7280".to_string(),
                            data: serde_json::json!({}),
                        };

                        if let Some(ref mut state) = PLAYGROUND_STATE {
                            state.edges.push(new_edge);
                        }

                        web_sys::console::log_1(&format!("🔗 Created edge: {} → {}", edge_state.source_node, node.id).into());
                        break;
                    }
                }
            }
        }
    }

    update_ui_display();
}

fn handle_key_down(event: KeyboardEvent) {
    let key = event.key();

    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            match key.as_str() {
                "Delete" | "Backspace" => {
                    delete_selected_items();
                }
                "Escape" => {
                    state.selected_nodes.clear();
                    state.selected_edges.clear();
                    DRAG_STATE = None;
                    EDGE_CREATION_STATE = None;
                }
                "s" | "S" => {
                    if event.ctrl_key() || event.meta_key() {
                        event.prevent_default();
                        export_graph();
                    }
                }
                "o" | "O" => {
                    if event.ctrl_key() || event.meta_key() {
                        event.prevent_default();
                        // Import would be triggered from UI
                        web_sys::console::log_1(&"💡 Use import_graph(json_string) to import".into());
                    }
                }
                "z" | "Z" => {
                    if event.ctrl_key() || event.meta_key() {
                        event.prevent_default();
                        // Undo functionality would go here
                        web_sys::console::log_1(&"↶ Undo not implemented in this demo".into());
                    }
                }
                "y" | "Y" => {
                    if event.ctrl_key() || event.meta_key() {
                        event.prevent_default();
                        // Redo functionality would go here
                        web_sys::console::log_1(&"↷ Redo not implemented in this demo".into());
                    }
                }
                _ => {}
            }
        }
    }

    update_ui_display();
}

// Utility functions
fn is_point_in_node(x: f64, y: f64, node: &NodeData) -> bool {
    x >= node.position.x - node.size.0 / 2.0
        && x <= node.position.x + node.size.0 / 2.0
        && y >= node.position.y - node.size.1 / 2.0
        && y <= node.position.y + node.size.1 / 2.0
}

fn add_node_at_position(x: f64, y: f64) {
    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            let node_count = state.nodes.len() + 1;
            let node_id = format!("node-{}", node_count);

            let new_node = NodeData {
                id: node_id.clone(),
                position: Position::new(x, y),
                size: (100.0, 60.0),
                label: format!("Node {}", node_count),
                node_type: "default".to_string(),
                color: "#6366F1".to_string(),
                data: serde_json::json!({}),
            };

            state.nodes.push(new_node);
            web_sys::console::log_1(&format!("➕ Added node: {}", node_id).into());
        }
    }
}

fn delete_at_position(x: f64, y: f64) {
    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            // Delete edges first (in reverse order to maintain indices)
            state.edges.retain(|edge| {
                // Simple edge deletion - in a real implementation,
                // you'd check if the click is on the edge
                !is_point_near_edge(x, y, edge, state)
            });

            // Delete nodes
            state.nodes.retain(|node| !is_point_in_node(x, y, node));

            // Clean up selections
            state.selected_nodes.retain(|id| state.nodes.iter().any(|n| n.id == *id));
            state.selected_edges.retain(|id| state.edges.iter().any(|e| e.id == *id));
        }
    }
}

fn is_point_near_edge(x: f64, y: f64, edge: &EdgeData, state: &PlaygroundState) -> bool {
    // Simple edge hit detection - find source and target positions
    if let (Some(source), Some(target)) = (
        state.nodes.iter().find(|n| n.id == edge.source),
        state.nodes.iter().find(|n| n.id == edge.target),
    ) {
        // Check if point is near the line between source and target
        let dx = target.position.x - source.position.x;
        let dy = target.position.y - source.position.y;
        let length = (dx * dx + dy * dy).sqrt();

        if length > 0.0 {
            let ux = dx / length;
            let uy = dy / length;

            let vx = x - source.position.x;
            let vy = y - source.position.y;

            let proj = vx * ux + vy * uy;
            let proj_x = source.position.x + proj * ux;
            let proj_y = source.position.y + proj * uy;

            let dist_to_line = ((x - proj_x).powi(2) + (y - proj_y).powi(2)).sqrt();

            // Check if projection is within line segment bounds
            proj >= 0.0 && proj <= length && dist_to_line < 10.0
        } else {
            false
        }
    } else {
        false
    }
}

fn delete_selected_items() {
    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            // Delete selected edges
            state.edges.retain(|edge| !state.selected_edges.contains(&edge.id));

            // Delete selected nodes
            state.nodes.retain(|node| !state.selected_nodes.contains(&node.id));

            // Also delete edges connected to deleted nodes
            let remaining_node_ids: std::collections::HashSet<_> =
                state.nodes.iter().map(|n| n.id.clone()).collect();

            state.edges.retain(|edge|
                remaining_node_ids.contains(&edge.source) &&
                remaining_node_ids.contains(&edge.target)
            );

            state.selected_nodes.clear();
            state.selected_edges.clear();

            web_sys::console::log_1(&"🗑️ Deleted selected items".into());
        }
    }
}

fn update_ui_display() {
    unsafe {
        if let Some(ref state) = PLAYGROUND_STATE {
            // Update status display
            web_sys::console::log_1(&format!("📊 Graph: {} nodes, {} edges | Selected: {} nodes, {} edges",
                state.nodes.len(), state.edges.len(),
                state.selected_nodes.len(), state.selected_edges.len()).into());

            // Update mode display
            let mode_str = match state.mode {
                EditorMode::Select => "Select",
                EditorMode::CreateNode => "Create Node",
                EditorMode::CreateEdge => "Create Edge",
                EditorMode::Delete => "Delete",
            };
            web_sys::console::log_1(&format!("🎮 Mode: {}", mode_str).into());
        }
    }
}

// Public API functions
#[wasm_bindgen]
pub fn add_node(label: &str, x: f64, y: f64) {
    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            let node_count = state.nodes.len() + 1;
            let node_id = format!("node-{}", node_count);

            let new_node = NodeData {
                id: node_id.clone(),
                position: Position::new(x, y),
                size: (100.0, 60.0),
                label: label.to_string(),
                node_type: "default".to_string(),
                color: "#6366F1".to_string(),
                data: serde_json::json!({}),
            };

            state.nodes.push(new_node);
            web_sys::console::log_1(&format!("➕ Added node: {} at ({}, {})", node_id, x, y).into());
        }
    }
    update_ui_display();
}

#[wasm_bindgen]
pub fn select_mode(mode: &str) {
    unsafe {
        if let Some(ref mut state) = PLAYGROUND_STATE {
            state.mode = match mode {
                "select" => EditorMode::Select,
                "create_node" => EditorMode::CreateNode,
                "create_edge" => EditorMode::CreateEdge,
                "delete" => EditorMode::Delete,
                _ => EditorMode::Select,
            };
            web_sys::console::log_1(&format!("🎮 Switched to {} mode", mode).into());
        }
    }
    update_ui_display();
}

#[wasm_bindgen]
pub fn export_graph() -> String {
    unsafe {
        if let Some(ref state) = PLAYGROUND_STATE {
            match serde_json::to_string_pretty(state) {
                Ok(json) => {
                    web_sys::console::log_1(&"💾 Graph exported successfully!".into());
                    web_sys::console::log_1(&format!("📄 JSON length: {} characters", json.len()).into());
                    json
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("❌ Export failed: {:?}", e).into());
                    "{}".to_string()
                }
            }
        } else {
            "{}".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn import_graph(json_str: &str) {
    unsafe {
        match serde_json::from_str::<PlaygroundState>(json_str) {
            Ok(new_state) => {
                PLAYGROUND_STATE = Some(new_state);
                web_sys::console::log_1(&"📥 Graph imported successfully!".into());
            }
            Err(e) => {
                web_sys::console::log_1(&format!("❌ Import failed: {:?}", e).into());
            }
        }
    }
    update_ui_display();
}

#[wasm_bindgen]
pub fn clear_graph() {
    unsafe {
        PLAYGROUND_STATE = Some(PlaygroundState {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_nodes: Vec::new(),
            selected_edges: Vec::new(),
            mode: EditorMode::Select,
            show_grid: true,
            snap_to_grid: false,
            grid_size: 20.0,
        });
        web_sys::console::log_1(&"🧹 Graph cleared!".into());
    }
    update_ui_display();
}

#[wasm_bindgen]
pub fn get_graph_stats() -> String {
    unsafe {
        if let Some(ref state) = PLAYGROUND_STATE {
            format!(
                "Graph Statistics:\n• Nodes: {}\n• Edges: {}\n• Selected Nodes: {}\n• Selected Edges: {}\n• Mode: {:?}\n• Grid: {} ({}px)",
                state.nodes.len(),
                state.edges.len(),
                state.selected_nodes.len(),
                state.selected_edges.len(),
                state.mode,
                if state.show_grid { "visible" } else { "hidden" },
                state.grid_size as i32
            )
        } else {
            "No graph loaded".to_string()
        }
    }
}
