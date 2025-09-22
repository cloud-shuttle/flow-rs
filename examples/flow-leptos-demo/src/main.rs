use flow_rs_core::{Edge, Graph, Node, Position};
use flow_rs_leptos::components::FlowEditor;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div style="padding: 20px; font-family: Arial, sans-serif;">
                <h1>"Leptos Flow Demo"</h1>
                <p>"A simple demonstration of the Leptos Flow editor with drag and drop, edge connections, and minimap."</p>

                <div style="margin: 20px 0;">
                    <h2>"Interactive Flow Editor"</h2>
                    <FlowDemo />
                </div>
            </div>
        }
    })
}

#[component]
fn FlowDemo() -> impl IntoView {
    // Create a sample graph with some nodes and edges
    let _graph = create_sample_graph();

    view! {
        <div style="border: 1px solid #ccc; border-radius: 8px; overflow: hidden;">
            <FlowEditor
                width=800
                height=600
            />
        </div>

        <div style="margin-top: 20px; display: flex; gap: 10px;">
            <button
                onclick=move || {
                    // add_random_node(&graph_signal);
                }
                style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                "Add Random Node"
            </button>

            <button
                onclick=move || {
                    // clear_graph(&graph_signal);
                }
                style="padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                "Clear Graph"
            </button>

            <button
                onclick=move || {
                    // add_sample_edges(&graph_signal);
                }
                style="padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                "Add Sample Edges"
            </button>
        </div>
    }
}

fn create_sample_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Create some sample nodes
    let nodes = vec![
        ("start", Position::new(100.0, 100.0)),
        ("process1", Position::new(300.0, 100.0)),
        ("process2", Position::new(500.0, 100.0)),
        ("decision", Position::new(300.0, 250.0)),
        ("end", Position::new(500.0, 250.0)),
    ];

    for (id, pos) in nodes {
        let node = Node::simple(id.to_string(), pos);
        let _ = graph.add_node(node);
    }

    // Add some sample edges
    let edges = vec![
        ("start", "process1"),
        ("process1", "process2"),
        ("process2", "decision"),
        ("decision", "end"),
    ];

    for (source, target) in edges {
        let edge = Edge::simple(
            format!("{}-{}", source, target),
            source.to_string(),
            target.to_string(),
        );
        let _ = graph.add_edge(edge);
    }

    graph
}

// Helper functions for future use
// fn add_random_node(graph_signal: &RwSignal<Graph<(), ()>>) { ... }
// fn clear_graph(graph_signal: &RwSignal<Graph<(), ()>>) { ... }
// fn add_sample_edges(graph_signal: &RwSignal<Graph<(), ()>>) { ... }
