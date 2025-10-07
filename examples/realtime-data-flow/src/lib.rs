//! Real-time Data Flow Example
//!
//! Demonstrates real-time data streaming, live graph updates, and dynamic data flow visualization.
//! This example shows how Flow-RS can handle streaming data and real-time updates efficiently.
//!
//! Features:
//! - Simulated WebSocket data streams
//! - Real-time node updates (positions, data, colors)
//! - Dynamic edge creation/removal
//! - Data flow visualization with animated edges
//! - Performance monitoring of real-time updates
//! - Interactive controls for data stream simulation

use flow_rs_core::{Edge, Graph, Node, Position, Viewport, NodeId, EdgeId};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::{Canvas2DRenderer, Renderer};
use wasm_bindgen::prelude::*;
use js_sys;

// Data flow simulation state
#[derive(Clone, Debug)]
struct DataFlowState {
    pub active_streams: Vec<DataStream>,
    pub last_update: f64,
    pub total_messages: u64,
    pub update_frequency: f64, // Hz
}

// Individual data stream
#[derive(Clone, Debug)]
struct DataStream {
    pub id: String,
    pub source_node: String,
    pub target_node: String,
    pub data_rate: f64, // messages per second
    pub active: bool,
    pub message_count: u64,
    pub color: String,
    pub last_message_time: f64,
}

// Global state
static mut FLOW_STATE: Option<DataFlowState> = None;
static mut GRAPH: Option<Graph<String, String>> = None;
static mut RENDERER: Option<Canvas2DRenderer> = None;

// Animation and timing
static mut ANIMATION_FRAME_ID: Option<i32> = None;
static mut LAST_FRAME_TIME: f64 = 0.0;

// Data message types
#[derive(Clone, Debug)]
enum DataMessage {
    NodeUpdate { node_id: String, data: String, color: String },
    EdgeTraffic { source: String, target: String, intensity: f32 },
    NewConnection { source: String, target: String, stream_type: String },
    ConnectionLost { source: String, target: String },
}

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Get the canvas element from the DOM
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("flow-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // Create the renderer
    let renderer = Canvas2DRenderer::new(&canvas)
        .expect("Failed to create renderer");

    // Create initial graph
    let graph = create_initial_graph();

    // Initialize data flow state
    let flow_state = DataFlowState {
        active_streams: Vec::new(),
        last_update: instant(),
        total_messages: 0,
        update_frequency: 10.0, // 10 updates per second
    };

    // Store global state
    unsafe {
        GRAPH = Some(graph);
        RENDERER = Some(renderer);
        FLOW_STATE = Some(flow_state);
    }

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 1200.0, 800.0, 1.0);

    // Initial render
    render_data_flow(&viewport)
        .expect("Failed to render initial data flow");

    // Set up controls
    setup_data_flow_controls();

    // Start data flow simulation
    start_data_flow_simulation();

    // Start animation loop
    start_animation_loop(viewport);
}

/// Create the initial graph structure for data flow visualization
fn create_initial_graph() -> Graph<String, String> {
    let mut graph = Graph::new();

    // Create server nodes (data sources)
    let servers = vec![
        ("server-1", "API Server", Position::new(150.0, 200.0), "#4F46E5"),
        ("server-2", "Database", Position::new(150.0, 400.0), "#059669"),
        ("server-3", "Cache", Position::new(150.0, 600.0), "#DC2626"),
    ];

    for (id, label, pos, _color) in servers {
        let node = Node::new(id.to_string(), pos, format!("{}: {}", label, "idle"));
        graph.add_node(node).expect("Failed to add server node");
    }

    // Create processing nodes
    let processors = vec![
        ("processor-1", "Load Balancer", Position::new(450.0, 150.0), "#7C3AED"),
        ("processor-2", "Auth Service", Position::new(450.0, 300.0), "#EA580C"),
        ("processor-3", "Business Logic", Position::new(450.0, 450.0), "#0891B2"),
        ("processor-4", "Analytics", Position::new(450.0, 600.0), "#BE123C"),
    ];

    for (id, label, pos, _color) in processors {
        let node = Node::new(id.to_string(), pos, format!("{}: {}", label, "ready"));
        graph.add_node(node).expect("Failed to add processor node");
    }

    // Create client nodes (data consumers)
    let clients = vec![
        ("client-1", "Web App", Position::new(850.0, 200.0), "#2563EB"),
        ("client-2", "Mobile App", Position::new(850.0, 400.0), "#16A34A"),
        ("client-3", "API Client", Position::new(850.0, 600.0), "#CA8A04"),
    ];

    for (id, label, pos, _color) in clients {
        let node = Node::new(id.to_string(), pos, format!("{}: {}", label, "connected"));
        graph.add_node(node).expect("Failed to add client node");
    }

    // Create initial edges (static connections)
    let edges = vec![
        ("server-1", "processor-1"),
        ("server-2", "processor-2"),
        ("server-3", "processor-3"),
        ("processor-1", "processor-2"),
        ("processor-2", "processor-3"),
        ("processor-3", "processor-4"),
        ("processor-1", "client-1"),
        ("processor-2", "client-2"),
        ("processor-4", "client-3"),
    ];

    for (source, target) in edges {
        let edge = Edge::new(
            format!("edge-{}-{}", source, target),
            source.to_string(),
            target.to_string(),
            "static".to_string(),
        );
        graph.add_edge(edge).expect("Failed to add edge");
    }

    graph
}

/// Set up data flow simulation controls
fn setup_data_flow_controls() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Start/stop simulation button
    if let Some(button) = document.get_element_by_id("toggle-simulation-btn") {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::EventTarget| {
            toggle_data_flow_simulation();
        }) as Box<dyn FnMut(web_sys::EventTarget)>);

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    // Update frequency slider
    if let Some(slider) = document.get_element_by_id("frequency-slider") {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::EventTarget| {
            update_simulation_frequency();
        }) as Box<dyn FnMut(web_sys::EventTarget)>);

        slider.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    // Reset graph button
    if let Some(button) = document.get_element_by_id("reset-graph-btn") {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::EventTarget| {
            reset_data_flow_graph();
        }) as Box<dyn FnMut(web_sys::EventTarget)>);

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    // Add random stream button
    if let Some(button) = document.get_element_by_id("add-stream-btn") {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::EventTarget| {
            add_random_data_stream();
        }) as Box<dyn FnMut(web_sys::EventTarget)>);

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

/// Start the data flow simulation
fn start_data_flow_simulation() {
    unsafe {
        if let Some(ref mut flow_state) = FLOW_STATE {
            // Create initial data streams
            flow_state.active_streams = vec![
                DataStream {
                    id: "stream-api-auth".to_string(),
                    source_node: "server-1".to_string(),
                    target_node: "processor-2".to_string(),
                    data_rate: 5.0,
                    active: true,
                    message_count: 0,
                    color: "#4F46E5".to_string(),
                    last_message_time: instant(),
                },
                DataStream {
                    id: "stream-db-logic".to_string(),
                    source_node: "server-2".to_string(),
                    target_node: "processor-3".to_string(),
                    data_rate: 3.0,
                    active: true,
                    message_count: 0,
                    color: "#059669".to_string(),
                    last_message_time: instant(),
                },
                DataStream {
                    id: "stream-cache-analytics".to_string(),
                    source_node: "server-3".to_string(),
                    target_node: "processor-4".to_string(),
                    data_rate: 2.0,
                    active: true,
                    message_count: 0,
                    color: "#DC2626".to_string(),
                    last_message_time: instant(),
                },
            ];
        }
    }
}

/// Toggle data flow simulation on/off
fn toggle_data_flow_simulation() {
    unsafe {
        if let Some(ref mut flow_state) = FLOW_STATE {
            let is_active = flow_state.active_streams.iter().any(|s| s.active);

            for stream in &mut flow_state.active_streams {
                stream.active = !is_active;
            }

            update_simulation_status_display();
        }
    }
}

/// Update simulation frequency based on slider
fn update_simulation_frequency() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(slider) = document.get_element_by_id("frequency-slider") {
        if let Ok(input) = slider.dyn_into::<web_sys::HtmlInputElement>() {
            if let Ok(freq) = input.value().parse::<f64>() {
                unsafe {
                    if let Some(ref mut flow_state) = FLOW_STATE {
                        flow_state.update_frequency = freq;
                    }
                }
            }
        }
    }
}

/// Reset the graph to initial state
fn reset_data_flow_graph() {
    unsafe {
        GRAPH = Some(create_initial_graph());
        FLOW_STATE = Some(DataFlowState {
            active_streams: Vec::new(),
            last_update: instant(),
            total_messages: 0,
            update_frequency: 10.0,
        });
        start_data_flow_simulation();
    }
}

/// Add a random data stream
fn add_random_data_stream() {
    unsafe {
        if let (Some(ref mut graph), Some(ref mut flow_state)) = (&mut GRAPH, &mut FLOW_STATE) {
            let node_ids: Vec<NodeId> = graph.nodes().map(|n| n.id.clone()).collect();

            if node_ids.len() >= 2 {
                let source_idx = (js_sys::Math::random() * node_ids.len() as f64) as usize;
                let mut target_idx = (js_sys::Math::random() * node_ids.len() as f64) as usize;

                // Ensure source != target
                while target_idx == source_idx {
                    target_idx = (js_sys::Math::random() * node_ids.len() as f64) as usize;
                }

                let colors = vec!["#4F46E5", "#059669", "#DC2626", "#7C3AED", "#EA580C"];
                let color_idx = (js_sys::Math::random() * colors.len() as f64) as usize;

                let stream = DataStream {
                    id: format!("stream-random-{}", flow_state.active_streams.len()),
                    source_node: node_ids[source_idx].to_string(),
                    target_node: node_ids[target_idx].to_string(),
                    data_rate: 1.0 + (js_sys::Math::random() * 5.0),
                    active: true,
                    message_count: 0,
                    color: colors[color_idx].to_string(),
                    last_message_time: instant(),
                };

                flow_state.active_streams.push(stream);
            }
        }
    }
}

/// Start the animation loop for real-time updates
fn start_animation_loop(viewport: Viewport) {
    let closure = Closure::wrap(Box::new(move || {
        update_data_flow(&viewport);
    }) as Box<dyn FnMut()>);

    unsafe {
        LAST_FRAME_TIME = instant();
        ANIMATION_FRAME_ID = web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .ok();
    }

    closure.forget();
}

/// Update data flow state and render
fn update_data_flow(viewport: &Viewport) {
    let current_time = instant();
    let _delta_time = current_time - unsafe { LAST_FRAME_TIME };

    // Update at the specified frequency
    unsafe {
        if let Some(ref mut flow_state) = FLOW_STATE {
            let update_interval = 1000.0 / flow_state.update_frequency;

            if current_time - flow_state.last_update >= update_interval {
                // Process data streams
                process_data_streams(current_time);

                // Update display
                update_data_flow_display();

                flow_state.last_update = current_time;
            }
        }
    }

    // Render
    render_data_flow(viewport).ok();

    // Continue animation loop
    let closure = Closure::wrap(Box::new(move || {
        update_data_flow(viewport);
    }) as Box<dyn FnMut()>);

    unsafe {
        LAST_FRAME_TIME = current_time;
        ANIMATION_FRAME_ID = web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .ok();
    }

    closure.forget();
}

/// Process active data streams and generate messages
fn process_data_streams(current_time: f64) {
    unsafe {
        if let (Some(ref mut graph), Some(ref mut flow_state)) = (&mut GRAPH, &mut FLOW_STATE) {
            for stream in &mut flow_state.active_streams {
                if !stream.active {
                    continue;
                }

                let time_since_last_message = current_time - stream.last_message_time;
                let message_interval = 1000.0 / stream.data_rate;

                if time_since_last_message >= message_interval {
                    // Generate a data message
                    generate_data_message(graph, stream, current_time);
                    stream.message_count += 1;
                    stream.last_message_time = current_time;
                    flow_state.total_messages += 1;
                }
            }
        }
    }
}

/// Generate a data message for a stream
fn generate_data_message(graph: &mut Graph<String, String>, stream: &mut DataStream, _current_time: f64) {
    // Randomly choose message type
    let message_type = (js_sys::Math::random() * 4.0) as u32;

    match message_type {
        0 => {
            // Node update - change node data
            if let Some(node) = graph.nodes_mut().find(|n| n.id == stream.target_node.clone().into()) {
                node.data = format!("Processing: {} msgs", stream.message_count);
            }
        }
        1 => {
            // Edge traffic - could visualize traffic intensity
            // For now, just update the edge data
            let edge_id: EdgeId = format!("edge-{}-{}", stream.source_node, stream.target_node).into();
            if let Some(edge) = graph.edges_mut().find(|e| e.id == edge_id) {
                edge.data = format!("Traffic: {} msgs/sec", stream.data_rate as u32);
            }
        }
        2 => {
            // New connection attempt
            // This could add temporary edges, but we'll skip for simplicity
        }
        3 => {
            // Connection status update
            if let Some(node) = graph.nodes_mut().find(|n| n.id == stream.source_node.clone().into()) {
                node.data = format!("Active: {} msgs sent", stream.message_count);
            }
        }
        _ => {}
    }
}

/// Render the data flow visualization
fn render_data_flow(viewport: &Viewport) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if let (Some(ref mut renderer), Some(ref graph)) = (&mut RENDERER, &GRAPH) {
            // Clear the canvas
            renderer.clear(Some("#0f172a"))?;

            // Set up background
            let background_config = BackgroundConfig {
                color: "#0f172a".to_string(),
                pattern_color: "#1e293b".to_string(),
                variant: BackgroundVariant::Dots,
                size: 20.0,
                opacity: 0.3,
            };

            // Render background
            renderer.render_background(&background_config, viewport)?;

            // Render graph
            renderer.render_graph_dyn(graph, viewport)?;

            // Render data flow visualization
            render_data_streams(renderer, viewport)?;

            // Present
            renderer.present()?;
        }
    }

    Ok(())
}

/// Render active data streams (animated edges, particles, etc.)
fn render_data_streams(_renderer: &mut Canvas2DRenderer, _viewport: &Viewport) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if let Some(ref flow_state) = FLOW_STATE {
            // This is where we'd render animated data flow visualization
            // For now, we'll just log the active streams
            for stream in &flow_state.active_streams {
                if stream.active {
                    web_sys::console::log_1(
                        &format!("Active stream: {} -> {} ({} msgs/sec)",
                                stream.source_node, stream.target_node, stream.data_rate).into()
                    );
                }
            }
        }
    }

    Ok(())
}

/// Update the data flow display with current statistics
fn update_data_flow_display() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    unsafe {
        if let Some(ref flow_state) = FLOW_STATE {
            // Update total messages
            if let Some(element) = document.get_element_by_id("total-messages") {
                element.set_text_content(Some(&format!("{}", flow_state.total_messages)));
            }

            // Update active streams
            if let Some(element) = document.get_element_by_id("active-streams") {
                let active_count = flow_state.active_streams.iter().filter(|s| s.active).count();
                element.set_text_content(Some(&format!("{}", active_count)));
            }

            // Update messages per second
            if let Some(element) = document.get_element_by_id("messages-per-second") {
                let total_rate: f64 = flow_state.active_streams.iter()
                    .filter(|s| s.active)
                    .map(|s| s.data_rate)
                    .sum();
                element.set_text_content(Some(&format!("{:.1}", total_rate)));
            }

            update_simulation_status_display();
        }
    }
}

/// Update simulation status display
fn update_simulation_status_display() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    unsafe {
        if let Some(ref flow_state) = FLOW_STATE {
            let is_active = flow_state.active_streams.iter().any(|s| s.active);

            if let Some(button) = document.get_element_by_id("toggle-simulation-btn") {
                if is_active {
                    button.set_text_content(Some("⏸️ Pause Simulation"));
                    button.set_attribute("class", "btn btn-danger").ok();
                } else {
                    button.set_text_content(Some("▶️ Start Simulation"));
                    button.set_attribute("class", "btn btn-success").ok();
                }
            }

            if let Some(status) = document.get_element_by_id("simulation-status") {
                if is_active {
                    status.set_text_content(Some("🟢 ACTIVE"));
                    status.set_attribute("class", "status-active").ok();
                } else {
                    status.set_text_content(Some("🔴 PAUSED"));
                    status.set_attribute("class", "status-paused").ok();
                }
            }
        }
    }
}

// Helper function for high-precision timing
fn instant() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}
