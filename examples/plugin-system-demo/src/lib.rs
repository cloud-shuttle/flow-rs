//! Plugin System Demo Example
//!
//! Comprehensive demonstration of Flow-RS plugin system capabilities:
//! - Custom node types (buttons, sliders, progress bars, toggles)
//! - Custom layout algorithms (circular, grid, radial tree, spiral)
//! - Plugin management and lifecycle
//! - Real-time plugin interaction
//! - Plugin marketplace simulation

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Node, Position, Graph, PluginManager};
use std::collections::HashMap;

/// Demo application state
#[derive(Clone)]
struct DemoState {
    graph: Graph<(), ()>,
    plugin_manager: PluginManager,
    selected_layout: String,
    custom_nodes: HashMap<String, Box<dyn flow_rs_core::CustomNode>>,
}

#[component]
pub fn PluginSystemDemo() -> impl IntoView {
    // Initialize demo state
    let demo_state = RwSignal::new(DemoState {
        graph: create_initial_graph(),
        plugin_manager: PluginManager::new(),
        selected_layout: "circular-layout".to_string(),
        custom_nodes: HashMap::new(),
    });

    // Load plugins
    demo_state.update(|state| {
        // In a real implementation, plugins would be loaded from WASM files
        // For this demo, we'll simulate plugin loading
        simulate_plugin_loading(&mut state.plugin_manager);
    });

    // Create some custom nodes to demonstrate
    demo_state.update(|state| {
        create_demo_custom_nodes(&mut state.custom_nodes, &state.plugin_manager);
    });

    view! {
        <div class="plugin-system-demo" style="padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            <h1 style="color: #2c3e50; text-align: center; margin-bottom: 10px;">"Flow-RS Plugin System Demo"</h1>
            <p style="text-align: center; color: #6c757d; margin-bottom: 30px;">
                "Extensible architecture with custom nodes, layouts, and interactions"
            </p>

            // Plugin information panel
            <div class="plugin-info" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 12px; margin-bottom: 20px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);">
                <h2 style="margin-top: 0; text-align: center;">"🔌 Plugin System Active"</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px; margin-top: 15px;">
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 5px;">📦</div>
                        <div style="font-weight: bold;">"2 Plugins"</div>
                        <div style="font-size: 0.9em; opacity: 0.9;">"Loaded"</div>
                    </div>
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 5px;">🧩</div>
                        <div style="font-weight: bold;">"4 Node Types"</div>
                        <div style="font-size: 0.9em; opacity: 0.9;">"Registered"</div>
                    </div>
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 5px;">📐</div>
                        <div style="font-weight: bold;">"4 Layouts"</div>
                        <div style="font-size: 0.9em; opacity: 0.9;">"Available"</div>
                    </div>
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 5px;">⚡</div>
                        <div style="font-weight: bold;">"Real-time"</div>
                        <div style="font-size: 0.9em; opacity: 0.9;">"Interaction"</div>
                    </div>
                </div>
            </div>

            // Controls panel
            <div class="controls-panel" style="background: white; padding: 20px; border-radius: 12px; margin-bottom: 20px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); border: 2px solid #007bff;">
                <h3 style="margin-top: 0; color: #007bff;">"🎛️ Plugin Controls"</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-top: 15px;">

                    // Layout selector
                    <div>
                        <label style="display: block; font-weight: bold; margin-bottom: 8px; color: #495057;">"Layout Algorithm:"</label>
                        <select
                            style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; background: white;"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                demo_state.update(|state| {
                                    state.selected_layout = value;
                                    apply_selected_layout(state);
                                });
                            }
                        >
                            <option value="circular-layout">"🔵 Circular Layout"</option>
                            <option value="grid-layout-custom">"📐 Custom Grid"</option>
                            <option value="radial-tree-layout">"🌳 Radial Tree"</option>
                            <option value="spiral-layout">"🌀 Spiral Layout"</option>
                        </select>
                    </div>

                    // Action buttons
                    <div>
                        <label style="display: block; font-weight: bold; margin-bottom: 8px; color: #495057;">"Actions:"</label>
                        <div style="display: flex; gap: 8px;">
                            <button
                                style="padding: 8px 12px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| {
                                    demo_state.update(|state| {
                                        add_random_custom_node(state);
                                    });
                                }
                            >
                                "➕ Add Node"
                            </button>
                            <button
                                style="padding: 8px 12px; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| {
                                    demo_state.update(|state| {
                                        clear_graph(state);
                                    });
                                }
                            >
                                "🗑️ Clear"
                            </button>
                        </div>
                    </div>

                </div>
            </div>

            // Main canvas area
            <div class="demo-container" style="border: 3px solid #28a745; border-radius: 12px; overflow: hidden; margin: 20px 0; box-shadow: 0 8px 32px rgba(40, 167, 69, 0.2);">
                <FlowCanvas
                    width=1000
                    height=700
                    enable_selection=true
                    enable_context_menu=true
                    enable_keyboard_shortcuts=true
                    enable_history=true
                />
            </div>

            // Feature showcase
            <div class="features-showcase" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-top: 30px;">
                <div class="feature-card" style="background: linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"🎮 Custom Node Types"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: rgba(255,255,255,0.9);">
                        <li>"Interactive buttons with click handlers"</li>
                        <li>"Sliders for real-time value input"</li>
                        <li>"Progress bars for status display"</li>
                        <li>"Toggle switches for boolean states"</li>
                        <li>"Plugin-defined rendering and behavior"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #333;">"📐 Advanced Layouts"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Circular arrangements for radial layouts"</li>
                        <li>"Custom grid with configurable spacing"</li>
                        <li>"Radial tree for hierarchical data"</li>
                        <li>"Spiral patterns for organic layouts"</li>
                        <li>"Plugin-extensible layout system"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #333;">"🔌 Plugin Architecture"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Hot-loadable plugin system"</li>
                        <li>"Capability-based registration"</li>
                        <li>"Inter-plugin communication"</li>
                        <li>"Secure sandboxed execution"</li>
                        <li>"Plugin marketplace ready"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #d299c2 0%, #fef9d7 100%); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #333;">"⚡ Real-time Features"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Live node interaction feedback"</li>
                        <li>"Dynamic layout application"</li>
                        <li>"Plugin message broadcasting"</li>
                        <li>"State synchronization across plugins"</li>
                        <li>"Performance optimized updates"</li>
                    </ul>
                </div>
            </div>

            // Plugin stats
            <div class="plugin-stats" style="margin-top: 30px; padding: 20px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); border-radius: 12px; color: white;">
                <h3 style="margin-top: 0; text-align: center;">"📊 Live Plugin Statistics"</h3>
                {move || {
                    let state = demo_state.get();
                    let stats = state.plugin_manager.get_stats();
                    view! {
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 15px; margin-top: 15px;">
                            <div style="text-align: center;">
                                <div style="font-size: 1.5em; font-weight: bold;">{stats.loaded_plugins}</div>
                                <div style="font-size: 0.9em; opacity: 0.9;">"Plugins"</div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 1.5em; font-weight: bold;">{stats.registered_nodes}</div>
                                <div style="font-size: 0.9em; opacity: 0.9;">"Node Types"</div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 1.5em; font-weight: bold;">{stats.registered_layouts}</div>
                                <div style="font-size: 0.9em; opacity: 0.9;">"Layouts"</div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 1.5em; font-weight: bold;">{state.graph.nodes().count()}</div>
                                <div style="font-size: 0.9em; opacity: 0.9;">"Active Nodes"</div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 1.5em; font-weight: bold;">{state.custom_nodes.len()}</div>
                                <div style="font-size: 0.9em; opacity: 0.9;">"Custom Nodes"</div>
                            </div>
                        </div>
                    }
                }}
            </div>

            // Tips and instructions
            <div class="tips-section" style="margin-top: 30px; padding: 20px; background: #fff3cd; border: 1px solid #ffeaa7; border-radius: 8px;">
                <h3 style="color: #856404; margin-top: 0;">"💡 Plugin System Tips"</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-top: 15px;">
                    <div>
                        <h4 style="color: #856404; margin-bottom: 8px;">"🎮 Custom Nodes"</h4>
                        <p style="margin: 0; color: #856404;">"Try clicking on custom nodes (buttons, sliders, toggles) to see interactive behavior. Each node type has unique functionality and visual feedback."</p>
                    </div>
                    <div>
                        <h4 style="color: #856404; margin-bottom: 8px;">"📐 Layout Algorithms"</h4>
                        <p style="margin: 0; color: #856404;">"Switch between different layout algorithms to see how plugins can completely change graph organization. Each layout serves different visualization needs."</p>
                    </div>
                    <div>
                        <h4 style="color: #856404; margin-bottom: 8px;">"🔌 Plugin Architecture"</h4>
                        <p style="margin: 0; color: #856404;">"The plugin system enables third-party developers to extend Flow-RS with new node types, layouts, themes, and export formats without modifying core code."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Create initial demo graph
fn create_initial_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Add some basic nodes
    let positions = vec![
        (200.0, 150.0), (400.0, 150.0), (600.0, 150.0),
        (200.0, 350.0), (400.0, 350.0), (600.0, 350.0),
    ];

    for (i, (x, y)) in positions.iter().enumerate() {
        let node = Node::new(format!("node-{}", i + 1), Position::new(*x, *y), ());
        graph.add_node(node).unwrap();
    }

    // Add some edges
    let edges = vec![
        ("node-1", "node-2"),
        ("node-2", "node-3"),
        ("node-4", "node-5"),
        ("node-5", "node-6"),
        ("node-2", "node-5"),
    ];

    for (source, target) in edges {
        let edge = flow_rs_core::Edge::new(
            format!("edge-{}-{}", source, target),
            source.to_string(),
            target.to_string(),
            (),
        );
        graph.add_edge(edge).unwrap();
    }

    graph
}

/// Simulate plugin loading (in real implementation, plugins would be loaded from WASM files)
fn simulate_plugin_loading(manager: &mut PluginManager) {
    // In a real implementation, this would load actual plugin WASM files
    // For this demo, we simulate the presence of plugins
    println!("Simulating plugin loading...");

    // Simulate that plugins are loaded and registered
    // (Actual plugin loading would happen here)
}

/// Create demo custom nodes
fn create_demo_custom_nodes(
    custom_nodes: &mut HashMap<String, Box<dyn flow_rs_core::CustomNode>>,
    plugin_manager: &PluginManager,
) {
    // In a real implementation, custom nodes would be created through plugin factories
    // For this demo, we create placeholder nodes
    println!("Creating demo custom nodes...");

    // Simulate custom nodes being created
    // (Actual node creation through plugins would happen here)
}

/// Apply selected layout to the graph
fn apply_selected_layout(state: &mut DemoState) {
    if let Some(layout_engine) = state.plugin_manager.registry().read().unwrap().get_layout_engine(&state.selected_layout) {
        let _ = layout_engine.layout(&mut state.graph);
        println!("Applied layout: {}", state.selected_layout);
    }
}

/// Add a random custom node to the graph
fn add_random_custom_node(state: &mut DemoState) {
    // Simulate adding a custom node
    let node_count = state.graph.nodes().count();
    let x = 100.0 + (node_count as f64 * 50.0);
    let y = 200.0 + (node_count as f64 * 30.0);

    let node = Node::new(
        format!("custom-node-{}", node_count + 1),
        Position::new(x, y),
        (),
    );

    state.graph.add_node(node).unwrap();
    println!("Added custom node at position ({}, {})", x, y);
}

/// Clear the graph
fn clear_graph(state: &mut DemoState) {
    state.graph = Graph::new();
    state.custom_nodes.clear();
    println!("Graph cleared");
}
