//! Background Variants Example
//!
//! Demonstrates Flow-RS's background customization capabilities:
//! - Different pattern types (dots, lines, crosshatch)
//! - Customizable colors and spacing
//! - Interactive pattern switching
//! - Performance optimized rendering
//!
//! This example shows how to create visually appealing and functional backgrounds.

use flow_rs_core::{Edge, Graph, Node, Position, Viewport};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::Canvas2DRenderer;
use wasm_bindgen::prelude::*;

// Global state for background switching
static mut CURRENT_BACKGROUND: BackgroundVariant = BackgroundVariant::Dots;

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
    let mut renderer = Canvas2DRenderer::new(&canvas)
        .expect("Failed to create renderer");

    // Create a sample graph
    let graph = create_sample_graph();

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 1000.0, 600.0, 1.0);

    // Initial render with dots background
    render_with_background(&mut renderer, &graph, &viewport, BackgroundVariant::Dots)
        .expect("Failed to render initial background");

    // Set up background switcher controls
    setup_background_controls();
}

/// Set up interactive background controls
fn setup_background_controls() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Get the select element
    if let Some(select) = document.get_element_by_id("background-select") {
        let select = select.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

        // Create a closure to handle changes
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Get the select element from the event
            if let Ok(target) = event.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>() {
                let value = target.value();

                // Parse the selected background variant
                let variant = match value.as_str() {
                    "dots" => BackgroundVariant::Dots,
                    "lines" => BackgroundVariant::Lines,
                    "crosshatch" => BackgroundVariant::Crosshatch,
                    _ => BackgroundVariant::Dots,
                };

                // Update global state
                unsafe {
                    CURRENT_BACKGROUND = variant;
                }

                // Re-render with new background
                rerender_with_current_background();
            }
        }) as Box<dyn FnMut(web_sys::Event)>);

        select.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up color picker
    if let Some(color_input) = document.get_element_by_id("background-color") {
        let color_input = color_input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            // Re-render when color changes
            rerender_with_current_background();
        }) as Box<dyn FnMut(web_sys::Event)>);

        color_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up size slider
    if let Some(size_input) = document.get_element_by_id("background-size") {
        let size_input = size_input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            // Re-render when size changes
            rerender_with_current_background();
        }) as Box<dyn FnMut(web_sys::Event)>);

        size_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Set up opacity slider
    if let Some(opacity_input) = document.get_element_by_id("background-opacity") {
        let opacity_input = opacity_input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            // Re-render when opacity changes
            rerender_with_current_background();
        }) as Box<dyn FnMut(web_sys::Event)>);

        opacity_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
}

/// Re-render the canvas with the current background settings
fn rerender_with_current_background() {
    // Get current settings from DOM
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let variant = unsafe { CURRENT_BACKGROUND };

    // Get color
    let color = document
        .get_element_by_id("background-color")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_else(|| "#f8fafc".to_string());

    // Get size
    let size = document
        .get_element_by_id("background-size")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value().parse::<f64>().unwrap_or(20.0))
        .unwrap_or(20.0);

    // Get opacity
    let opacity = document
        .get_element_by_id("background-opacity")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value().parse::<f64>().unwrap_or(0.5) / 100.0)
        .unwrap_or(0.5);

    // Get canvas and create renderer
    if let Some(canvas) = document.get_element_by_id("flow-canvas") {
        if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
            if let Ok(mut renderer) = Canvas2DRenderer::new(&canvas) {
                let graph = create_sample_graph();
                let viewport = Viewport::new(0.0, 0.0, 1000.0, 600.0, 1.0);

                let _ = render_with_background(&mut renderer, &graph, &viewport, variant);
            }
        }
    }
}

/// Render the graph with a specific background variant
fn render_with_background(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<(), ()>,
    viewport: &Viewport,
    variant: BackgroundVariant,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get current settings from DOM
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let color = document
        .get_element_by_id("background-color")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_else(|| "#f8fafc".to_string());

    let pattern_color = document
        .get_element_by_id("pattern-color")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_else(|| "#e2e8f0".to_string());

    let size = document
        .get_element_by_id("background-size")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value().parse::<f64>().unwrap_or(20.0))
        .unwrap_or(20.0);

    let opacity = document
        .get_element_by_id("background-opacity")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value().parse::<f64>().unwrap_or(50.0) / 100.0)
        .unwrap_or(0.5);

    // Clear the canvas
    renderer.clear(Some(&color))?;

    // Create background configuration
    let background_config = BackgroundConfig {
        color,
        pattern_color,
        variant,
        size,
        opacity,
    };

    // Render the background
    renderer.render_background(&background_config, viewport)?;

    // Render the graph
    renderer.render_graph_dyn(graph, viewport)?;

    // Present the frame
    renderer.present()?;

    Ok(())
}

/// Create a sample graph for demonstration
fn create_sample_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Create a nice arrangement of nodes
    let positions = vec![
        (150.0, 150.0), (350.0, 100.0), (550.0, 150.0), (750.0, 200.0),
        (150.0, 350.0), (350.0, 400.0), (550.0, 350.0), (750.0, 300.0),
        (450.0, 250.0), // Center node
    ];

    // Add nodes
    for (i, (x, y)) in positions.iter().enumerate() {
        let node = Node::simple(&format!("node{}", i + 1), Position::new(*x, *y));
        graph.add_node(node).expect("Failed to add node");
    }

    // Add some edges to create an interesting flow
    let edges = vec![
        ("node1", "node2"), ("node2", "node3"), ("node3", "node4"),
        ("node5", "node6"), ("node6", "node7"), ("node7", "node8"),
        ("node1", "node5"), ("node4", "node8"),
        ("node2", "node9"), ("node6", "node9"), ("node9", "node3"),
    ];

    for (source, target) in edges {
        let edge = Edge::simple(&format!("{}-{}", source, target), source, target);
        graph.add_edge(edge).expect("Failed to add edge");
    }

    graph
}
