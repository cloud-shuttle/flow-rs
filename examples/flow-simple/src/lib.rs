//! Simple Flow Example
//!
//! A basic demonstration of the Leptos Flow Canvas2D renderer
//! showing nodes and edges in a simple flow diagram.

use flow_core::{Graph, Node, Edge, Position, Viewport};
use flow_renderer::{Canvas2DRenderer, Renderer};
use flow_renderer::traits::{BackgroundConfig, BackgroundVariant};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[cfg(test)]
mod interaction_tests;

mod interactions;
mod performance_monitor;

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();

    // Get the canvas element from the DOM
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("flow-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Create the renderer
    let mut renderer = match Canvas2DRenderer::new(&canvas) {
        Ok(r) => r,
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to create renderer: {:?}", e).into());
            return;
        }
    };

    // Create a sample graph
    let graph = create_sample_graph();
    let viewport = Viewport::default();

    // Create interaction handler
    let mut interaction_handler = interactions::InteractionHandler::new(
        canvas.clone(),
        renderer,
        graph,
        viewport,
    );

    // Render the initial graph
    if let Err(e) = interaction_handler.render() {
        web_sys::console::error_1(&format!("Failed to render: {:?}", e).into());
    }

    // Set up event handlers
    setup_event_handlers(&canvas);
}

fn render_graph(renderer: &mut Canvas2DRenderer, graph: &Graph<(), ()>, viewport: &Viewport) {
    // Clear canvas
    if let Err(e) = renderer.clear(Some("#ffffff")) {
        web_sys::console::error_1(&format!("Failed to clear canvas: {:?}", e).into());
        return;
    }

    // Render background
    let bg_config = BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#e2e8f0".to_string(),
        variant: BackgroundVariant::Dots,
        size: 20.0,
        opacity: 0.5,
    };

    if let Err(e) = renderer.render_background(&bg_config, viewport) {
        web_sys::console::error_1(&format!("Failed to render background: {:?}", e).into());
    }

    // Render graph
    if let Err(e) = renderer.render_graph_dyn(graph, viewport) {
        web_sys::console::error_1(&format!("Failed to render graph: {:?}", e).into());
    }

    // Present frame
    if let Err(e) = renderer.present() {
        web_sys::console::error_1(&format!("Failed to present frame: {:?}", e).into());
    }
}

fn create_sample_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Create sample nodes
    let node1 = Node::simple("node1", Position::new(100.0, 100.0));
    let node2 = Node::simple("node2", Position::new(300.0, 200.0));
    let node3 = Node::simple("node3", Position::new(500.0, 150.0));

    // Add nodes to graph
    if let Err(e) = graph.add_node(node1) {
        web_sys::console::error_1(&format!("Failed to add node1: {:?}", e).into());
    }
    if let Err(e) = graph.add_node(node2) {
        web_sys::console::error_1(&format!("Failed to add node2: {:?}", e).into());
    }
    if let Err(e) = graph.add_node(node3) {
        web_sys::console::error_1(&format!("Failed to add node3: {:?}", e).into());
    }

    // Create sample edges
    let edge1 = Edge::simple("edge1", "node1", "node2");
    let edge2 = Edge::simple("edge2", "node2", "node3");

    // Add edges to graph
    if let Err(e) = graph.add_edge(edge1) {
        web_sys::console::error_1(&format!("Failed to add edge1: {:?}", e).into());
    }
    if let Err(e) = graph.add_edge(edge2) {
        web_sys::console::error_1(&format!("Failed to add edge2: {:?}", e).into());
    }

    graph
}

fn setup_event_handlers(canvas: &HtmlCanvasElement) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Set up basic mouse event handlers (simplified for now)
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        web_sys::console::log_1(&format!("Mouse down at: {}, {}", event.client_x(), event.client_y()).into());
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);
    canvas.set_onmousedown(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    // Add node button
    if let Some(add_btn) = document.get_element_by_id("add-node-btn") {
        let add_btn = add_btn.dyn_into::<web_sys::HtmlElement>().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"Add node clicked!".into());
            // TODO: Implement add node functionality
        }) as Box<dyn FnMut()>);
        add_btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Clear canvas button
    if let Some(clear_btn) = document.get_element_by_id("clear-btn") {
        let clear_btn = clear_btn.dyn_into::<web_sys::HtmlElement>().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            web_sys::console::log_1(&"Clear canvas clicked!".into());
            // TODO: Implement clear canvas functionality
        }) as Box<dyn FnMut()>);
        clear_btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
}
