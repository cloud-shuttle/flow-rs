//! Touch Demo Example
//!
//! Demonstrates touch gestures and mobile support in Flow-RS:
//! - Touch panning and zooming
//! - Touch-optimized UI
//! - Mobile device detection
//! - Touch feedback

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Node, Position, Graph};

#[component]
pub fn TouchDemo() -> impl IntoView {
    // Create a sample graph
    let graph = Graph::new();

    // Add some nodes
    let node1 = Node::new("node1".to_string(), Position::new(100.0, 100.0), "Touch Node 1".to_string());
    let node2 = Node::new("node2".to_string(), Position::new(300.0, 200.0), "Touch Node 2".to_string());
    let node3 = Node::new("node3".to_string(), Position::new(200.0, 300.0), "Touch Node 3".to_string());

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    // Add an edge
    let edge1 = flow_rs_core::Edge::new("edge1".to_string(), "node1".to_string(), "node2".to_string(), "connection".to_string());
    graph.add_edge(edge1).unwrap();

    view! {
        <div class="touch-demo" style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Flow-RS Touch Demo"</h1>
            <p>"Try touch gestures on mobile devices or touch-enabled displays:"</p>
            <ul>
                <li>"Single finger: Pan the canvas"</li>
                <li>"Two fingers: Pinch to zoom"</li>
                <li>"Tap: Select nodes"</li>
                <li>"Long press: Context menu (future feature)"</li>
            </ul>

            <div class="demo-container" style="border: 2px solid #ccc; border-radius: 8px; overflow: hidden;">
                <FlowCanvas
                    width=600
                    height=400
                    enable_touch=true
                    touch_throttle_ms=16.0
                />
            </div>

            <div class="info-panel" style="margin-top: 20px; padding: 15px; background: #f5f5f5; border-radius: 8px;">
                <h3>"Device Info"</h3>
                <p>"Touch capable: " {move || if TouchUIUtils::is_touch_device() { "Yes" } else { "No" }}</p>
                <p>"Minimum touch target: " {TouchUIUtils::MIN_TOUCH_TARGET} "px"</p>
            </div>
        </div>
    }
}
