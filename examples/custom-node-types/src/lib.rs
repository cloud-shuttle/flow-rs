//! Custom Node Types Example
//!
//! Demonstrates custom node components with specialized behaviors and properties.
//! This example shows how to extend Flow-RS with custom node types that have unique functionality.
//!
//! Features:
//! - Custom node types (Button, Slider, Progress, Toggle, Data Display)
//! - Node-specific properties and behaviors
//! - Plugin-like architecture for extensibility
//! - Interactive console demonstrations

use flow_rs_core::{Graph, Node, Position};
use flow_rs_renderer::Canvas2DRenderer;
use wasm_bindgen::prelude::*;
use js_sys;

// Node types and their behaviors
#[derive(Clone, Debug, PartialEq)]
enum CustomNodeType {
    Button,
    Slider,
    Progress,
    Toggle,
    DataDisplay,
}

// Custom node data structure
#[derive(Clone, Debug)]
struct CustomNodeData {
    pub node_type: CustomNodeType,
    pub label: String,
    pub value: f64,
    pub is_active: bool,
    pub last_interaction: f64,
}

// Global state
static mut GRAPH: Option<Graph<CustomNodeData, String>> = None;
static mut RENDERER: Option<Canvas2DRenderer> = None;
static mut SIMULATION_STEP: u32 = 0;

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"🎨 Flow-RS Custom Node Types Example Started!".into());

    // Create initial graph with custom node types
    let graph = create_custom_nodes_graph();

    // Store global state
    unsafe {
        GRAPH = Some(graph);
    }

    // Demonstrate custom node functionality
    demonstrate_custom_nodes();

    // Set up periodic updates
    setup_periodic_updates();
}

#[wasm_bindgen]
pub fn trigger_button_click(button_id: &str) {
    unsafe {
        if let Some(ref mut graph) = GRAPH {
            for node in graph.nodes_mut() {
                if node.id == button_id.into() && node.data.node_type == CustomNodeType::Button {
                    handle_button_interaction(node);
                    web_sys::console::log_1(&format!("🔘 Button '{}' clicked!", button_id).into());
                    break;
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn toggle_switch(toggle_id: &str) {
    unsafe {
        if let Some(ref mut graph) = GRAPH {
            for node in graph.nodes_mut() {
                if node.id == toggle_id.into() && node.data.node_type == CustomNodeType::Toggle {
                    node.data.value = if node.data.value > 0.0 { 0.0 } else { 1.0 };
                    node.data.last_interaction = instant();
                    web_sys::console::log_1(&format!("🔄 Toggle '{}' set to: {}",
                        toggle_id, if node.data.value > 0.0 { "ON" } else { "OFF" }).into());
                    break;
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn adjust_slider(slider_id: &str, value: f64) {
    unsafe {
        if let Some(ref mut graph) = GRAPH {
            for node in graph.nodes_mut() {
                if node.id == slider_id.into() && node.data.node_type == CustomNodeType::Slider {
                    node.data.value = value.clamp(0.0, 1.0);
                    node.data.last_interaction = instant();
                    web_sys::console::log_1(&format!("🎚️ Slider '{}' set to: {:.1}%",
                        slider_id, value * 100.0).into());
                    break;
                }
            }
        }
    }
}

/// Create a graph with various custom node types
fn create_custom_nodes_graph() -> Graph<CustomNodeData, String> {
    let mut graph = Graph::new();

    // Create different types of custom nodes
    let nodes_data = vec![
        ("button-1", "Click Me!", CustomNodeType::Button, Position::new(200.0, 150.0)),
        ("slider-1", "Volume", CustomNodeType::Slider, Position::new(500.0, 150.0)),
        ("progress-1", "Loading", CustomNodeType::Progress, Position::new(800.0, 150.0)),
        ("toggle-1", "Power", CustomNodeType::Toggle, Position::new(200.0, 350.0)),
        ("data-1", "CPU Usage", CustomNodeType::DataDisplay, Position::new(500.0, 350.0)),
        ("button-2", "Reset", CustomNodeType::Button, Position::new(800.0, 350.0)),
    ];

    for (id, label, node_type, pos) in nodes_data {
        let value = match node_type {
            CustomNodeType::Slider => 0.5,
            CustomNodeType::Progress => 0.3,
            CustomNodeType::Toggle => 0.0,
            CustomNodeType::DataDisplay => 45.0,
            _ => 0.0,
        };

        let custom_data = CustomNodeData {
            node_type,
            label: label.to_string(),
            value,
            is_active: false,
            last_interaction: 0.0,
        };

        let node = Node::new(id.to_string(), pos, custom_data);
        graph.add_node(node).expect("Failed to add custom node");
    }

    graph
}

/// Demonstrate custom node functionality
fn demonstrate_custom_nodes() {
    unsafe {
        if let Some(ref graph) = GRAPH {
            web_sys::console::log_1(&"📊 Custom Node Types in Graph:".into());

            for node in graph.nodes() {
                let node_type_str = match node.data.node_type {
                    CustomNodeType::Button => "Button",
                    CustomNodeType::Slider => "Slider",
                    CustomNodeType::Progress => "Progress",
                    CustomNodeType::Toggle => "Toggle",
                    CustomNodeType::DataDisplay => "Data Display",
                };

                web_sys::console::log_1(&format!("  • {}: {} ({}) - Value: {:.1}",
                    node.id, node.data.label, node_type_str, node.data.value).into());
            }

            web_sys::console::log_1(&"🎮 Try these interactions:".into());
            web_sys::console::log_1(&"  • trigger_button_click('button-1') - Click the button".into());
            web_sys::console::log_1(&"  • toggle_switch('toggle-1') - Toggle the switch".into());
            web_sys::console::log_1(&"  • adjust_slider('slider-1', 0.8) - Set slider to 80%".into());
        }
    }
}

/// Handle button interaction
fn handle_button_interaction(node: &mut Node<CustomNodeData>) {
    node.data.last_interaction = instant();

    match node.id.as_str() {
        "button-1" => {
            web_sys::console::log_1(&"✨ Button clicked! Triggering random value changes...".into());
            // Trigger random value changes on other nodes
            unsafe {
                if let Some(ref mut graph) = GRAPH {
                    for other_node in graph.nodes_mut() {
                        if other_node.id != node.id {
                            other_node.data.value = js_sys::Math::random();
                            other_node.data.last_interaction = instant();
                        }
                    }
                }
            }
        }
        "button-2" => {
            web_sys::console::log_1(&"🔄 Resetting all node values...".into());
            // Reset all nodes
            unsafe {
                if let Some(ref mut graph) = GRAPH {
                    for other_node in graph.nodes_mut() {
                        other_node.data.value = match other_node.data.node_type {
                            CustomNodeType::Slider => 0.5,
                            CustomNodeType::Progress => 0.3,
                            CustomNodeType::Toggle => 0.0,
                            CustomNodeType::DataDisplay => 45.0,
                            _ => 0.0,
                        };
                        other_node.data.last_interaction = instant();
                    }
                }
            }
        }
        _ => {}
    }
}

/// Set up periodic updates for animated elements
fn setup_periodic_updates() {
    let closure = Closure::wrap(Box::new(move || {
        update_custom_nodes();
    }) as Box<dyn FnMut()>);

    // Update every 2 seconds
    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            2000,
        )
        .expect("Failed to set periodic update interval");

    closure.forget();
}

/// Update custom nodes periodically
fn update_custom_nodes() {
    unsafe {
        SIMULATION_STEP += 1;

        if let Some(ref mut graph) = GRAPH {
            let current_time = instant();

            // Update progress bars and data displays
            for node in graph.nodes_mut() {
                match node.data.node_type {
                    CustomNodeType::Progress => {
                        // Animate progress bars
                        node.data.value = (current_time * 0.001).sin() * 0.5 + 0.5;
                    }
                    CustomNodeType::DataDisplay => {
                        // Simulate changing data every few updates
                        if SIMULATION_STEP % 3 == 0 {
                            node.data.value = 20.0 + js_sys::Math::random() * 60.0;
                            node.data.last_interaction = current_time;
                        }
                    }
                    _ => {}
                }
            }

            // Log current state occasionally
            if SIMULATION_STEP % 5 == 0 {
                web_sys::console::log_1(&"🔄 Simulation Step: Updating animated nodes...".into());
                for node in graph.nodes() {
                    if node.data.node_type == CustomNodeType::Progress ||
                       node.data.node_type == CustomNodeType::DataDisplay {
                        web_sys::console::log_1(&format!("  • {}: {:.1}",
                            node.data.label, node.data.value).into());
                    }
                }
            }
        }
    }
}

/// Get current timestamp
fn instant() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}
