use leptos::*;
use leptos_flow_core::{Graph, Node, Edge, Position, Size};
use leptos_flow_leptos::components::{FlowEditor, FlowCanvas};

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
    let graph = create_sample_graph();
    let graph_signal = create_rw_signal(graph);

    view! {
        <div style="border: 1px solid #ccc; border-radius: 8px; overflow: hidden;">
            <FlowEditor
                graph=graph_signal
                width=800
                height=600
            />
        </div>

        <div style="margin-top: 20px; display: flex; gap: 10px;">
            <button
                on:click=move |_| {
                    add_random_node(&graph_signal);
                }
                style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                "Add Random Node"
            </button>

            <button
                on:click=move |_| {
                    clear_graph(&graph_signal);
                }
                style="padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                "Clear Graph"
            </button>

            <button
                on:click=move |_| {
                    add_sample_edges(&graph_signal);
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

fn add_random_node(graph_signal: &RwSignal<Graph<(), ()>>) {
    graph_signal.update(|graph| {
        let node_count = graph.node_count();
        let x = 100.0 + (node_count as f64 * 150.0) % 600.0;
        let y = 100.0 + (node_count as f64 * 100.0) % 400.0;

        let node = Node::simple(
            format!("node_{}", node_count),
            Position::new(x, y),
        );
        let _ = graph.add_node(node);
    });
}

fn clear_graph(graph_signal: &RwSignal<Graph<(), ()>>) {
    graph_signal.set(Graph::new());
}

fn add_sample_edges(graph_signal: &RwSignal<Graph<(), ()>>) {
    graph_signal.update(|graph| {
        let nodes: Vec<_> = graph.nodes().collect();

        if nodes.len() >= 2 {
            // Connect the first two nodes
            let source_id = nodes[0].id.clone();
            let target_id = nodes[1].id.clone();

            let edge = Edge::simple(
                format!("{}-{}", source_id.as_str(), target_id.as_str()),
                source_id,
                target_id,
            );
            let _ = graph.add_edge(edge);
        }
    });
}
