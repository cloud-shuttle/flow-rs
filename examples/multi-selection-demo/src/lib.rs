//! Multi-Selection Demo Example
//!
//! Demonstrates multi-selection functionality in Flow-RS:
//! - Rectangle (marquee) selection
//! - Shift-click and Ctrl-click modifiers
//! - Visual selection feedback
//! - Selection state management
//! - Bulk operations on selected nodes

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Node, Position, Graph};

#[component]
pub fn MultiSelectionDemo() -> impl IntoView {
    // Create a sample graph with multiple nodes for selection testing
    let graph = Graph::new();

    // Create a grid of nodes for selection testing
    let node_positions = vec![
        (50.0, 50.0), (150.0, 50.0), (250.0, 50.0), (350.0, 50.0),
        (50.0, 150.0), (150.0, 150.0), (250.0, 150.0), (350.0, 150.0),
        (50.0, 250.0), (150.0, 250.0), (250.0, 250.0), (350.0, 250.0),
        (50.0, 350.0), (150.0, 350.0), (250.0, 350.0), (350.0, 350.0),
    ];

    for (i, (x, y)) in node_positions.iter().enumerate() {
        let node = Node::new(
            format!("node-{}", i + 1),
            Position::new(*x, *y),
            format!("Node {}", i + 1)
        );
        graph.add_node(node).unwrap();
    }

    // Add some edges to connect nodes
    let edges = vec![
        ("node-1", "node-2"), ("node-2", "node-3"), ("node-3", "node-4"),
        ("node-5", "node-6"), ("node-6", "node-7"), ("node-7", "node-8"),
        ("node-9", "node-10"), ("node-10", "node-11"), ("node-11", "node-12"),
        ("node-13", "node-14"), ("node-14", "node-15"), ("node-15", "node-16"),
    ];

    for (source, target) in edges {
        let edge = flow_rs_core::Edge::new(
            format!("edge-{}-{}", source, target),
            source.to_string(),
            target.to_string(),
            "connection".to_string()
        );
        graph.add_edge(edge).unwrap();
    }

    view! {
        <div class="selection-demo" style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Flow-RS Multi-Selection Demo"</h1>
            <p>"Test various selection methods:"</p>
            <ul>
                <li>"Click and drag to create a rectangle selection"</li>
                <li>"Shift+Click to add to selection"</li>
                <li>"Ctrl+Click (or Cmd+Click on Mac) to toggle selection"</li>
                <li>"Selected nodes show blue highlight borders"</li>
                <li>"Selection rectangle shows during drag"</li>
            </ul>

            <div class="demo-container" style="border: 2px solid #28a745; border-radius: 8px; overflow: hidden; margin: 20px 0;">
                <FlowCanvas
                    width=500
                    height=450
                    enable_selection=true
                    enable_touch=true
                />
            </div>

            <div class="instructions" style="background: #f8f9fa; padding: 15px; border-radius: 8px; margin-top: 20px;">
                <h3>"Selection Instructions"</h3>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px;">
                    <div>
                        <h4>"Rectangle Selection"</h4>
                        <p>"1. Click and hold the mouse button</p>
                        <p>"2. Drag to create a selection rectangle</p>
                        <p>"3. Release to select all nodes in the rectangle"</p>
                    </div>
                    <div>
                        <h4>"Modifier Keys"</h4>
                        <p>"<strong>Shift:</strong> Add to current selection</p>
                        <p>"<strong>Ctrl/Cmd:</strong> Toggle node selection</p>
                        <p>"<strong>No modifier:</strong> Replace selection</p>
                    </div>
                </div>
            </div>

            <div class="tips" style="background: #e3f2fd; padding: 15px; border-radius: 8px; margin-top: 15px;">
                <h4>"💡 Pro Tips"</h4>
                <ul>
                    <li>"Selection rectangles appear as you drag"</li>
                    <li>"Selected nodes have blue highlight borders"</li>
                    <li>"You can select multiple nodes and perform bulk operations"</li>
                    <li>"Try selecting nodes in different patterns to test the selection logic"</li>
                </ul>
            </div>
        </div>
    }
}
