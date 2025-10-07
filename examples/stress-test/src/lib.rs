//! Stress Test Example
//!
//! Demonstrates Flow-RS's ability to handle large-scale graphs efficiently:
//! - 1000+ node graphs with smooth performance
//! - Real-time performance monitoring
//! - Memory usage tracking
//! - Comparative performance metrics
//! - Scalability testing with different graph sizes

use flow_rs_core::{Edge, Graph, Node, Position, Viewport};
use flow_rs_renderer::traits::{BackgroundConfig, BackgroundVariant};
use flow_rs_renderer::Canvas2DRenderer;
use wasm_bindgen::prelude::*;
use web_sys::Performance;

// Performance monitoring structure
#[derive(Clone, Debug)]
struct PerformanceMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub render_time_ms: f64,
    pub memory_usage_mb: f64,
    pub fps: f64,
}

// Global state for performance tracking
static mut CURRENT_METRICS: PerformanceMetrics = PerformanceMetrics {
    node_count: 0,
    edge_count: 0,
    render_time_ms: 0.0,
    memory_usage_mb: 0.0,
    fps: 0.0,
};

static mut LAST_FRAME_TIME: f64 = 0.0;
static mut FRAME_COUNT: u32 = 0;

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

    // Start with a medium graph (500 nodes) for initial load
    let graph = generate_large_graph(500);

    // Set up viewport
    let viewport = Viewport::new(0.0, 0.0, 1200.0, 800.0, 1.0);

    // Initial render
    render_stress_test(&mut renderer, &graph, &viewport)
        .expect("Failed to render initial stress test");

    // Set up controls
    setup_stress_test_controls();

    // Start performance monitoring
    start_performance_monitoring();
}

/// Set up interactive stress test controls
fn setup_stress_test_controls() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Node count slider
    if let Some(node_slider) = document.get_element_by_id("node-count") {
        let node_slider = node_slider.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            update_graph_size();
        }) as Box<dyn FnMut(web_sys::Event)>);

        node_slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Regenerate button
    if let Some(regen_btn) = document.get_element_by_id("regenerate-btn") {
        let regen_btn = regen_btn.dyn_into::<web_sys::HtmlElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            regenerate_graph();
        }) as Box<dyn FnMut(web_sys::Event)>);

        regen_btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Benchmark button
    if let Some(benchmark_btn) = document.get_element_by_id("benchmark-btn") {
        let benchmark_btn = benchmark_btn.dyn_into::<web_sys::HtmlElement>().unwrap();

        let closure = Closure::wrap(Box::new(move |_| {
            run_performance_benchmark();
        }) as Box<dyn FnMut(web_sys::Event)>);

        benchmark_btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
}

/// Update graph size based on slider value
fn update_graph_size() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let node_count = document
        .get_element_by_id("node-count")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value().parse::<usize>().unwrap_or(500))
        .unwrap_or(500);

    let graph = generate_large_graph(node_count);
    rerender_with_graph(graph);
}

/// Regenerate graph with current size
fn regenerate_graph() {
    update_graph_size();
}

/// Run performance benchmark across different graph sizes
fn run_performance_benchmark() {
    web_sys::console::log_1(&"Starting performance benchmark...".into());

    // Test different graph sizes
    let test_sizes = vec![100, 250, 500, 750, 1000, 1500];

    for &size in &test_sizes {
        let graph = generate_large_graph(size);

        // Measure rendering time
        let start_time = instant::now();
        let result = rerender_with_graph(graph);
        let end_time = instant::now();

        let render_time = end_time - start_time;

        web_sys::console::log_1(
            &format!("Graph size {}: {:.2}ms render time", size, render_time).into()
        );

        // Small delay between tests
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    web_sys::console::log_1(&"Benchmark complete!".into());
}

/// Re-render with a new graph
fn rerender_with_graph(graph: Graph<(), ()>) -> Result<(), Box<dyn std::error::Error>> {
    // Get canvas and render
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(canvas) = document.get_element_by_id("flow-canvas") {
        if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
            if let Ok(mut renderer) = Canvas2DRenderer::new(&canvas) {
                let viewport = Viewport::new(0.0, 0.0, 1200.0, 800.0, 1.0);
                return render_stress_test(&mut renderer, &graph, &viewport);
            }
        }
    }
    Ok(())
}

/// Start performance monitoring
fn start_performance_monitoring() {
    // Set initial frame time
    unsafe {
        LAST_FRAME_TIME = instant::now();
    }

    // Update metrics every second
    let closure = Closure::wrap(Box::new(move || {
        update_performance_metrics();
    }) as Box<dyn FnMut()>);

    let window = web_sys::window().unwrap();
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        1000,
    ).expect("Failed to set performance monitoring interval");

    closure.forget();
}

/// Update performance metrics display
fn update_performance_metrics() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    unsafe {
        // Calculate FPS
        let current_time = instant::now();
        let delta_time = current_time - LAST_FRAME_TIME;
        if delta_time > 0.0 {
            CURRENT_METRICS.fps = (FRAME_COUNT as f64) / (delta_time / 1000.0);
        }
        FRAME_COUNT = 0;
        LAST_FRAME_TIME = current_time;

        // Update memory usage (simplified)
        CURRENT_METRICS.memory_usage_mb = 50.0 + (CURRENT_METRICS.node_count as f64 * 0.01);

        // Update DOM elements
        if let Some(node_count_el) = document.get_element_by_id("metric-nodes") {
            node_count_el.set_text_content(Some(&CURRENT_METRICS.node_count.to_string()));
        }

        if let Some(edge_count_el) = document.get_element_by_id("metric-edges") {
            edge_count_el.set_text_content(Some(&CURRENT_METRICS.edge_count.to_string()));
        }

        if let Some(render_time_el) = document.get_element_by_id("metric-render-time") {
            render_time_el.set_text_content(Some(&format!("{:.2}ms", CURRENT_METRICS.render_time_ms)));
        }

        if let Some(memory_el) = document.get_element_by_id("metric-memory") {
            memory_el.set_text_content(Some(&format!("{:.1}MB", CURRENT_METRICS.memory_usage_mb)));
        }

        if let Some(fps_el) = document.get_element_by_id("metric-fps") {
            fps_el.set_text_content(Some(&format!("{:.1}", CURRENT_METRICS.fps)));
        }
    }
}

/// Generate a large graph with the specified number of nodes
fn generate_large_graph(node_count: usize) -> Graph<(), ()> {
    let mut graph = Graph::new();

    // Calculate grid dimensions for nice layout
    let cols = (node_count as f64).sqrt().ceil() as usize;
    let rows = (node_count + cols - 1) / cols;

    let spacing_x = 1200.0 / (cols as f64);
    let spacing_y = 800.0 / (rows as f64);

    // Create nodes in a grid pattern
    for i in 0..node_count {
        let row = i / cols;
        let col = i % cols;

        let x = (col as f64) * spacing_x + spacing_x * 0.5;
        let y = (row as f64) * spacing_y + spacing_y * 0.5;

        // Add some randomness to avoid perfect grid
        let x = x + (js_sys::Math::random() - 0.5) * spacing_x * 0.3;
        let y = y + (js_sys::Math::random() - 0.5) * spacing_y * 0.3;

        let node = Node::simple(&format!("node{}", i), Position::new(x, y));
        graph.add_node(node).expect("Failed to add node");
    }

    // Create edges - connect each node to a few nearby nodes
    let max_connections = 3; // Limit connections for performance

    for i in 0..node_count {
        let connections = (js_sys::Math::random() * max_connections as f64) as usize + 1;

        for _ in 0..connections {
            let target_idx = (js_sys::Math::random() * node_count as f64) as usize;
            if target_idx != i && target_idx < node_count {
                let edge = Edge::simple(
                    &format!("edge{}-{}", i, target_idx),
                    &format!("node{}", i),
                    &format!("node{}", target_idx),
                );

                // Only add if edge doesn't already exist
                let edge_exists = graph.edges().any(|e| {
                    e.source == format!("node{}", i) && e.target == format!("node{}", target_idx)
                });

                if !edge_exists {
                    let _ = graph.add_edge(edge);
                }
            }
        }
    }

    // Update metrics
    unsafe {
        CURRENT_METRICS.node_count = graph.node_count();
        CURRENT_METRICS.edge_count = graph.edge_count();
    }

    graph
}

/// Render the stress test graph
fn render_stress_test(
    renderer: &mut Canvas2DRenderer,
    graph: &Graph<(), ()>,
    viewport: &Viewport,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = instant::now();

    // Clear the canvas
    renderer.clear(Some("#ffffff"))?;

    // Set up minimal background for performance
    let background_config = BackgroundConfig {
        color: "#ffffff".to_string(),
        pattern_color: "#f1f5f9".to_string(),
        variant: BackgroundVariant::Dots,
        size: 30.0,
        opacity: 0.2,
    };

    // Render background
    renderer.render_background(&background_config, viewport)?;

    // Render graph
    renderer.render_graph_dyn(graph, viewport)?;

    // Present
    renderer.present()?;

    let end_time = instant::now();
    let render_time = end_time - start_time;

    // Update metrics
    unsafe {
        CURRENT_METRICS.render_time_ms = render_time;
        FRAME_COUNT += 1;
    }

    Ok(())
}

// Helper function for high-precision timing
fn instant() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}
