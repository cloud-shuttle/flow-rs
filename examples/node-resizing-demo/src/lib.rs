//! Node Resizing Demo Example
//!
//! Demonstrates node resizing functionality in Flow-RS:
//! - Resize handles on node corners and edges
//! - Visual feedback during resize operations
//! - Minimum and maximum size constraints
//! - Aspect ratio preservation options

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Node, Position, Graph};

#[component]
pub fn NodeResizingDemo() -> impl IntoView {
    // Create a sample graph with resizable nodes
    let graph = Graph::new();

    // Add some nodes with different sizes
    let node1 = Node::new("resizable-1".to_string(), Position::new(150.0, 100.0), "Resizable Node 1".to_string());
    let node2 = Node::new("resizable-2".to_string(), Position::new(400.0, 200.0), "Resizable Node 2".to_string());
    let node3 = Node::new("resizable-3".to_string(), Position::new(200.0, 350.0), "Resizable Node 3".to_string());

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    // Add some edges
    let edge1 = flow_rs_core::Edge::new("edge1".to_string(), "resizable-1".to_string(), "resizable-2".to_string(), "connection".to_string());
    let edge2 = flow_rs_core::Edge::new("edge2".to_string(), "resizable-2".to_string(), "resizable-3".to_string(), "connection".to_string());

    graph.add_edge(edge1).unwrap();
    graph.add_edge(edge2).unwrap();

    view! {
        <div class="resizing-demo" style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Flow-RS Node Resizing Demo"</h1>
            <p>"Try resizing nodes by dragging the handles on corners and edges:"</p>
            <ul>
                <li>"Corner handles: Resize in both width and height"</li>
                <li>"Edge handles: Resize in one dimension"</li>
                <li>"Minimum size: 50x30 pixels"</li>
                <li>"Visual feedback: Resize handles appear on hover"</li>
            </ul>

            <div class="demo-container" style="border: 2px solid #007bff; border-radius: 8px; overflow: hidden; margin: 20px 0;">
                <FlowCanvas
                    width=700
                    height=500
                    enable_node_resizing=true
                    enable_touch=true
                />
            </div>

            <div class="info-panel" style="margin-top: 20px; padding: 15px; background: #f8f9fa; border-radius: 8px;">
                <h3>"Resize Instructions"</h3>
                <p>"1. Hover over a node to see resize handles"</p>
                <p>"2. Click and drag a handle to resize the node"</p>
                <p>"3. Release to confirm the new size"</p>
                <p>"4. Nodes have minimum size constraints to remain readable"</p>
            </div>
        </div>
    }
}
