//! Controlled vs Uncontrolled Example
//!
//! Demonstrates the two main state management patterns in Flow-RS:
//!
//! 1. **Controlled Mode**: Reactive state managed by Leptos signals
//!    - Graph state is stored in reactive signals
//!    - Changes flow through the component hierarchy
//!    - Full control over state mutations
//!
//! 2. **Uncontrolled Mode**: Internal state management
//!    - Flow component manages its own state
//!    - Changes happen internally
//!    - Simpler API, less boilerplate
//!
//! This example shows both patterns side-by-side for comparison.

use flow_rs_core::{Edge, Graph, Node, Position};
use flow_rs_leptos::{Flow, FlowProps};
use leptos::prelude::*;

// Example data structure for our nodes
#[derive(Clone, Debug, Default)]
struct ProcessData {
    pub name: String,
    pub status: ProcessStatus,
}

#[derive(Clone, Debug, Default)]
enum ProcessStatus {
    #[default]
    Idle,
    Running,
    Complete,
    Error,
}

impl ProcessData {
    fn new(name: &str, status: ProcessStatus) -> Self {
        Self {
            name: name.to_string(),
            status,
        }
    }
}

// Controlled Flow Component
// State is managed externally via Leptos signals
#[component]
pub fn ControlledFlowExample() -> impl IntoView {
    // Reactive state for the controlled flow
    let (nodes, set_nodes) = create_signal(create_initial_nodes());
    let (edges, set_edges) = create_signal(create_initial_edges());

    // Handler for adding new nodes
    let add_node = move |_| {
        set_nodes.update(|nodes| {
            let new_id = format!("node{}", nodes.len() + 1);
            let x = 100.0 + (nodes.len() as f64 * 150.0);
            let y = 150.0;

            nodes.push(Node::simple(&new_id, Position::new(x, y)));
        });

        set_edges.update(|edges| {
            if edges.len() > 0 {
                let last_node = format!("node{}", edges.len() + 1);
                let new_node = format!("node{}", edges.len() + 2);
                edges.push(Edge::simple(
                    &format!("edge{}", edges.len() + 1),
                    &last_node,
                    &new_node,
                ));
            }
        });
    };

    // Handler for resetting the flow
    let reset_flow = move |_| {
        set_nodes.set(create_initial_nodes());
        set_edges.set(create_initial_edges());
    };

    view! {
        <div class="controlled-section">
            <div class="section-header">
                <h3>"🎛️ Controlled Mode"</h3>
                <p>"State managed by Leptos signals - full reactive control"</p>
            </div>

            <div class="controls">
                <button class="btn primary" on:click=add_node>
                    "➕ Add Node"
                </button>
                <button class="btn secondary" on:click=reset_flow>
                    "🔄 Reset"
                </button>
            </div>

            <div class="flow-container">
                <Flow
                    nodes=nodes
                    edges=edges
                    class="controlled-flow"
                />
            </div>

            <div class="info">
                <p><strong>Controlled Mode Benefits:</strong></p>
                <ul>
                    <li>Full control over state changes</li>
                    <li>Reactive updates throughout your app</li>
                    <li>Easy integration with other state</li>
                    <li>Predictable data flow</li>
                </ul>
            </div>
        </div>
    }
}

// Uncontrolled Flow Component
// State is managed internally by the Flow component
#[component]
pub fn UncontrolledFlowExample() -> impl IntoView {
    // For uncontrolled mode, we just provide initial data
    // The Flow component manages state internally
    let initial_nodes = create_initial_nodes();
    let initial_edges = create_initial_edges();

    view! {
        <div class="uncontrolled-section">
            <div class="section-header">
                <h3>"🎯 Uncontrolled Mode"</h3>
                <p>"State managed internally - simpler API"</p>
            </div>

            <div class="flow-container">
                <Flow
                    nodes=initial_nodes
                    edges=initial_edges
                    class="uncontrolled-flow"
                />
            </div>

            <div class="info">
                <p><strong>Uncontrolled Mode Benefits:</strong></p>
                <ul>
                    <li>Simpler API - less boilerplate</li>
                    <li>Internal state management</li>
                    <li>Good for simple use cases</li>
                    <li>Less coupling with parent components</li>
                </ul>
            </div>
        </div>
    }
}

// Main app component that shows both patterns
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <div class="header">
                <h1>"Flow-RS: Controlled vs Uncontrolled"</h1>
                <p>"Explore the two state management patterns in Flow-RS"</p>
            </div>

            <div class="comparison-grid">
                <ControlledFlowExample />
                <UncontrolledFlowExample />
            </div>

            <div class="explanation">
                <h2>"Understanding State Management"</h2>
                <div class="pattern-comparison">
                    <div class="pattern">
                        <h3>"Controlled Pattern"</h3>
                        <div class="code-snippet">
                            <pre><code>// State managed by parent component
let (nodes, set_nodes) = create_signal(nodes);
let (edges, set_edges) = create_signal(edges);

// Pass to Flow component
<Flow nodes=nodes edges=edges /></code></pre>
                        </div>
                        <p>"Best for complex applications where you need full control over state changes and want to coordinate with other parts of your application."</p>
                    </div>

                    <div class="pattern">
                        <h3>"Uncontrolled Pattern"</h3>
                        <div class="code-snippet">
                            <pre><code>// Just provide initial data
let initial_nodes = create_nodes();
let initial_edges = create_edges();

// Flow manages its own state
<Flow nodes=initial_nodes edges=initial_edges /></code></pre>
                        </div>
                        <p>"Best for simpler use cases where you don't need to track or modify the flow state from outside the component."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Helper functions to create initial data
fn create_initial_nodes() -> Vec<Node<()>> {
    vec![
        Node::simple("start", Position::new(150.0, 150.0)),
        Node::simple("process", Position::new(400.0, 150.0)),
        Node::simple("end", Position::new(650.0, 150.0)),
    ]
}

fn create_initial_edges() -> Vec<Edge<()>> {
    vec![
        Edge::simple("start-process", "start", "process"),
        Edge::simple("process-end", "process", "end"),
    ]
}

// WASM entry point
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();

    // Mount the Leptos app
    leptos::mount_to_body(App);
}
