# Testing Strategy for Leptos Flow

## Overview

Leptos Flow employs a comprehensive testing strategy covering unit tests, integration tests, property-based tests, visual regression tests, performance benchmarks, and browser compatibility tests. This document outlines our testing approach and best practices.

## Test Categories

### 1. Unit Tests

#### Core Logic Tests (90% coverage required)

**Graph Operations:**

```rust
#[cfg(test)]
mod graph_tests {
    use super::*;
    use leptos_flow_core::graph::*;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        let node = Node::new("1", Position::new(100.0, 100.0));

        assert!(graph.add_node(node).is_ok());
        assert_eq!(graph.node_count(), 1);
        assert!(graph.get_node("1").is_some());
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut graph = Graph::new();
        let node1 = Node::new("1", Position::new(100.0, 100.0));
        let node2 = Node::new("1", Position::new(200.0, 200.0));

        assert!(graph.add_node(node1).is_ok());
        assert_eq!(graph.add_node(node2), Err(FlowError::DuplicateNodeId { id: "1".to_string() }));
    }

    #[test]
    fn test_remove_node_cascades_edges() {
        let mut graph = Graph::new();
        graph.add_node(Node::new("1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::new("2", Position::new(200.0, 200.0))).unwrap();
        graph.add_edge(Edge::new("e1", "1", "2")).unwrap();

        assert_eq!(graph.edge_count(), 1);
        graph.remove_node("1").unwrap();
        assert_eq!(graph.edge_count(), 0); // Edge should be removed
    }
}
```

**Spatial Indexing Tests:**

```rust
#[cfg(test)]
mod spatial_tests {
    use super::*;
    use leptos_flow_core::spatial::*;

    #[test]
    fn test_rtree_insertion_and_query() {
        let mut index = RTreeIndex::new();

        // Insert test nodes
        for i in 0..100 {
            let node = Node::new(
                &format!("node_{}", i),
                Position::new(i as f64 * 10.0, i as f64 * 10.0)
            );
            index.insert(&node);
        }

        // Query viewport
        let viewport = Viewport::new(0.0, 0.0, 500.0, 500.0);
        let results = index.query_viewport(&viewport);

        assert!(results.len() <= 50); // Should return ~50 nodes in viewport
        assert!(results.len() > 0);   // Should return some nodes
    }

    #[test]
    fn test_rtree_update_performance() {
        let mut index = RTreeIndex::new();
        let start = std::time::Instant::now();

        // Insert 10k nodes
        for i in 0..10000 {
            let node = Node::new(
                &format!("node_{}", i),
                Position::new(
                    (i as f64 % 100.0) * 10.0,
                    (i as f64 / 100.0) * 10.0
                )
            );
            index.insert(&node);
        }

        let insertion_time = start.elapsed();
        assert!(insertion_time < Duration::from_millis(100)); // Should be fast

        // Query performance
        let start = std::time::Instant::now();
        let viewport = Viewport::new(0.0, 0.0, 200.0, 200.0);
        let _results = index.query_viewport(&viewport);
        let query_time = start.elapsed();

        assert!(query_time < Duration::from_micros(1000)); // Sub-millisecond queries
    }
}
```

**Layout Algorithm Tests:**

```rust
#[cfg(test)]
mod layout_tests {
    use super::*;
    use leptos_flow_core::layout::*;

    #[test]
    fn test_force_directed_layout_convergence() {
        let mut graph = create_test_graph(50, 75); // 50 nodes, 75 edges
        let mut layout = ForceDirectedLayout::new();

        let initial_energy = layout.calculate_energy(&graph);
        layout.apply(&mut graph, 100); // 100 iterations
        let final_energy = layout.calculate_energy(&graph);

        assert!(final_energy < initial_energy); // Energy should decrease
        assert!(layout.has_converged()); // Should converge within 100 iterations
    }

    #[test]
    fn test_hierarchical_layout_levels() {
        let mut graph = create_dag(20); // Create DAG with 20 nodes
        let mut layout = HierarchicalLayout::new();

        layout.apply(&mut graph, 50);

        // Verify nodes are arranged in levels
        let levels = layout.get_node_levels(&graph);
        assert!(levels.len() > 1); // Should have multiple levels

        // Verify edges flow downward
        for edge in graph.edges() {
            let source_level = levels[&edge.source];
            let target_level = levels[&edge.target];
            assert!(target_level > source_level); // Target should be lower level
        }
    }
}
```

#### Renderer Tests (80% coverage required)

**Canvas2D Renderer:**

```rust
#[cfg(test)]
mod canvas2d_tests {
    use super::*;
    use leptos_flow_renderer::canvas2d::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_canvas2d_renderer_creation() {
        let canvas = create_test_canvas(800, 600);
        let renderer = Canvas2DRenderer::new(&canvas);

        assert!(renderer.is_ok());

        let renderer = renderer.unwrap();
        assert_eq!(renderer.width(), 800);
        assert_eq!(renderer.height(), 600);
    }

    #[wasm_bindgen_test]
    fn test_node_rendering() {
        let canvas = create_test_canvas(800, 600);
        let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();

        let node = Node::new("1", Position::new(100.0, 100.0))
            .with_size(Size::new(80.0, 40.0))
            .with_style(NodeStyle::default());

        let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0);

        assert!(renderer.render_node(&node, &viewport).is_ok());

        // Verify rendering occurred (check canvas state)
        let image_data = get_canvas_image_data(&canvas, 90, 90, 20, 20);
        assert!(has_non_white_pixels(&image_data)); // Node should be visible
    }
}
```

**WebGL2 Renderer:**

```rust
#[cfg(test)]
mod webgl2_tests {
    use super::*;
    use leptos_flow_renderer::webgl2::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_webgl2_context_creation() {
        if !webgl2_available() {
            return; // Skip if WebGL2 not available
        }

        let canvas = create_test_canvas(800, 600);
        let renderer = WebGL2Renderer::new(&canvas);

        assert!(renderer.is_ok());

        let renderer = renderer.unwrap();
        assert!(renderer.context().is_some());
    }

    #[wasm_bindgen_test]
    fn test_shader_compilation() {
        if !webgl2_available() {
            return;
        }

        let canvas = create_test_canvas(800, 600);
        let renderer = WebGL2Renderer::new(&canvas).unwrap();

        // Test vertex shader compilation
        let vertex_shader = renderer.compile_vertex_shader(DEFAULT_VERTEX_SHADER);
        assert!(vertex_shader.is_ok());

        // Test fragment shader compilation
        let fragment_shader = renderer.compile_fragment_shader(DEFAULT_FRAGMENT_SHADER);
        assert!(fragment_shader.is_ok());

        // Test shader program linking
        let program = renderer.link_program(&vertex_shader.unwrap(), &fragment_shader.unwrap());
        assert!(program.is_ok());
    }
}
```

### 2. Integration Tests

#### Cross-Module Integration Tests (85% coverage required)

**Graph + Spatial Index Integration:**

```rust
#[test]
fn test_graph_spatial_integration() {
    let mut graph = Graph::with_spatial_index();

    // Add nodes and verify spatial index is updated
    for i in 0..100 {
        let node = Node::new(
            &format!("node_{}", i),
            Position::new((i % 10) as f64 * 100.0, (i / 10) as f64 * 100.0)
        );
        graph.add_node(node).unwrap();
    }

    // Test viewport query through graph
    let viewport = Viewport::new(0.0, 0.0, 300.0, 300.0);
    let visible_nodes = graph.get_nodes_in_viewport(&viewport);

    assert!(visible_nodes.len() > 0);
    assert!(visible_nodes.len() < 100); // Should be subset

    // Verify all returned nodes are actually in viewport
    for node in visible_nodes {
        assert!(viewport.contains_point(&node.position));
    }
}
```

**Renderer + Graph Integration:**

```rust
#[wasm_bindgen_test]
fn test_full_rendering_pipeline() {
    let canvas = create_test_canvas(800, 600);
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_test_graph(20, 30);
    let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0);

    // Test full rendering pipeline
    renderer.clear().unwrap();
    renderer.render_graph(&graph, &viewport).unwrap();
    renderer.present().unwrap();

    // Verify rendering occurred
    let image_data = get_full_canvas_image_data(&canvas);
    assert!(has_rendered_content(&image_data));
}
```

#### Leptos Component Integration Tests

```rust
use leptos::*;
use leptos_testing::*;

#[test]
fn test_flow_editor_component() {
    let runtime = create_runtime();

    let (nodes, set_nodes) = create_signal(vec![
        Node::new("1", Position::new(100.0, 100.0)),
        Node::new("2", Position::new(300.0, 200.0)),
    ]);

    let (edges, set_edges) = create_signal(vec![
        Edge::new("e1", "1", "2"),
    ]);

    let view = view! {
        <FlowEditor
            nodes=nodes
            edges=edges
            on_nodes_change=set_nodes
            on_edges_change=set_edges
        />
    };

    let document = mount_to_body(view);

    // Test component mounting
    let flow_container = document.query_selector(".flow-editor").unwrap().unwrap();
    assert!(flow_container.is_some());

    // Test node rendering
    let node_elements = document.query_selector_all(".flow-node").unwrap();
    assert_eq!(node_elements.length(), 2);

    // Test edge rendering
    let edge_elements = document.query_selector_all(".flow-edge").unwrap();
    assert_eq!(edge_elements.length(), 1);

    runtime.dispose();
}
```

### 3. Property-Based Tests

#### Using PropTest for Comprehensive Testing

```rust
use proptest::prelude::*;

// Custom generators for graph structures
fn arb_position() -> impl Strategy<Value = Position> {
    (-10000.0..10000.0, -10000.0..10000.0)
        .prop_map(|(x, y)| Position::new(x, y))
}

fn arb_node() -> impl Strategy<Value = Node> {
    (r"node_[0-9]+", arb_position())
        .prop_map(|(id, position)| Node::new(id, position))
}

fn arb_graph() -> impl Strategy<Value = Graph> {
    prop::collection::vec(arb_node(), 0..100)
        .prop_map(|nodes| {
            let mut graph = Graph::new();
            for node in nodes {
                let _ = graph.add_node(node);
            }
            graph
        })
}

proptest! {
    #[test]
    fn test_graph_invariants(graph in arb_graph()) {
        // Graph invariants that should always hold
        prop_assert!(graph.node_count() >= 0);
        prop_assert!(graph.edge_count() >= 0);
        prop_assert!(graph.edge_count() <= graph.node_count() * (graph.node_count() - 1));

        // All edges should reference existing nodes
        for edge in graph.edges() {
            prop_assert!(graph.get_node(&edge.source).is_some());
            prop_assert!(graph.get_node(&edge.target).is_some());
        }
    }

    #[test]
    fn test_spatial_index_consistency(
        nodes in prop::collection::vec(arb_node(), 0..1000)
    ) {
        let mut graph = Graph::with_spatial_index();

        // Add nodes
        for node in &nodes {
            let _ = graph.add_node(node.clone());
        }

        // Test that spatial queries are consistent
        let viewport = Viewport::new(-1000.0, -1000.0, 2000.0, 2000.0);
        let spatial_results = graph.get_nodes_in_viewport(&viewport);
        let linear_results: Vec<_> = graph.nodes()
            .filter(|node| viewport.contains_point(&node.position))
            .collect();

        prop_assert_eq!(spatial_results.len(), linear_results.len());
    }

    #[test]
    fn test_layout_algorithms_preserve_connectivity(
        mut graph in arb_connected_graph()
    ) {
        let original_edges = graph.edges().cloned().collect::<Vec<_>>();

        // Apply layout
        let mut layout = ForceDirectedLayout::new();
        layout.apply(&mut graph, 50);

        // Verify connectivity is preserved
        let new_edges = graph.edges().cloned().collect::<Vec<_>>();
        prop_assert_eq!(original_edges.len(), new_edges.len());

        for original_edge in &original_edges {
            prop_assert!(new_edges.iter().any(|e|
                e.source == original_edge.source && e.target == original_edge.target
            ));
        }
    }
}
```

### 4. Visual Regression Tests

#### Automated Screenshot Testing

```typescript
// tests/visual/visual-tests.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Visual Regression Tests', () => {
  test('basic flow rendering', async ({ page }) => {
    await page.goto('/examples/basic');

    // Wait for flow to load
    await page.waitForSelector('.flow-editor');
    await page.waitForTimeout(1000); // Allow animations to settle

    // Take screenshot
    await expect(page.locator('.flow-editor')).toHaveScreenshot('basic-flow.png');
  });

  test('large graph performance', async ({ page }) => {
    await page.goto('/examples/large-graph');

    // Wait for all nodes to render
    await page.waitForSelector('.flow-node', { timeout: 10000 });
    await page.waitForFunction(() =>
      document.querySelectorAll('.flow-node').length >= 1000
    );

    // Test viewport interaction
    await page.mouse.wheel(0, -500); // Zoom in
    await page.waitForTimeout(500);

    await expect(page.locator('.flow-editor')).toHaveScreenshot('large-graph-zoomed.png');
  });

  test('node selection states', async ({ page }) => {
    await page.goto('/examples/interactive');

    // Single node selection
    await page.click('.flow-node[data-id="node-1"]');
    await expect(page.locator('.flow-editor')).toHaveScreenshot('single-selection.png');

    // Multi-selection with Shift
    await page.keyboard.down('Shift');
    await page.click('.flow-node[data-id="node-2"]');
    await page.keyboard.up('Shift');
    await expect(page.locator('.flow-editor')).toHaveScreenshot('multi-selection.png');
  });

  test('edge creation animation', async ({ page }) => {
    await page.goto('/examples/connection');

    // Start connection
    const sourceHandle = page.locator('.handle[data-id="node-1-output"]');
    await sourceHandle.dragTo(page.locator('.handle[data-id="node-2-input"]'));

    // Capture the new edge
    await page.waitForTimeout(500); // Allow animation to complete
    await expect(page.locator('.flow-editor')).toHaveScreenshot('new-edge.png');
  });
});
```

#### Custom Visual Testing Helper

```rust
// Helper for Rust-based visual tests
pub struct VisualTestRunner {
    headless_browser: HeadlessBrowser,
    baseline_dir: PathBuf,
    output_dir: PathBuf,
}

impl VisualTestRunner {
    pub async fn compare_rendering(&self, test_name: &str, html: &str) -> Result<bool, TestError> {
        // Render HTML in headless browser
        let screenshot = self.headless_browser.render_html(html).await?;

        // Compare with baseline
        let baseline_path = self.baseline_dir.join(format!("{}.png", test_name));
        let output_path = self.output_dir.join(format!("{}.png", test_name));

        screenshot.save(&output_path)?;

        if baseline_path.exists() {
            let baseline = image::open(&baseline_path)?;
            let diff = image_diff::diff(&baseline, &screenshot);

            if diff.score > 0.99 { // 99% similarity threshold
                Ok(true)
            } else {
                // Save diff image for inspection
                let diff_path = self.output_dir.join(format!("{}-diff.png", test_name));
                diff.image.save(&diff_path)?;
                Ok(false)
            }
        } else {
            // No baseline exists, save current as baseline
            screenshot.save(&baseline_path)?;
            Ok(true)
        }
    }
}
```

### 5. Performance Benchmarks

#### Criterion-based Benchmarks

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_node_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("node_creation");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("create_nodes", size), size, |b, &size| {
            b.iter(|| {
                let nodes: Vec<_> = (0..size)
                    .map(|i| Node::new(format!("node_{}", i), Position::new(i as f64, i as f64)))
                    .collect();
                nodes
            });
        });
    }
    group.finish();
}

fn bench_spatial_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_queries");

    // Setup
    let mut index = RTreeIndex::new();
    for i in 0..10000 {
        let node = Node::new(
            format!("node_{}", i),
            Position::new((i % 100) as f64 * 10.0, (i / 100) as f64 * 10.0)
        );
        index.insert(&node);
    }

    let viewport = Viewport::new(0.0, 0.0, 500.0, 500.0);

    group.bench_function("query_viewport_10k_nodes", |b| {
        b.iter(|| index.query_viewport(&viewport));
    });

    group.finish();
}

fn bench_layout_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_algorithms");

    for &node_count in [100, 500, 1000].iter() {
        let graph = create_test_graph(node_count, node_count * 2);

        group.bench_with_input(
            BenchmarkId::new("force_directed", node_count),
            &graph,
            |b, graph| {
                b.iter(|| {
                    let mut graph = graph.clone();
                    let mut layout = ForceDirectedLayout::new();
                    layout.apply(&mut graph, 100);
                });
            }
        );
    }

    group.finish();
}

criterion_group!(benches, bench_node_creation, bench_spatial_queries, bench_layout_algorithms);
criterion_main!(benches);
```

#### Memory Usage Benchmarks

```rust
#[cfg(test)]
mod memory_benchmarks {
    use super::*;
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Custom allocator to track memory usage
    struct TrackingAllocator;

    static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = System.alloc(layout);
            if !ptr.is_null() {
                ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
            System.dealloc(ptr, layout);
        }
    }

    #[global_allocator]
    static GLOBAL: TrackingAllocator = TrackingAllocator;

    #[test]
    fn test_memory_usage_graph() {
        let initial_memory = ALLOCATED.load(Ordering::SeqCst);

        {
            let mut graph = Graph::new();

            // Add 1000 nodes
            for i in 0..1000 {
                let node = Node::new(
                    format!("node_{}", i),
                    Position::new(i as f64, i as f64)
                );
                graph.add_node(node).unwrap();
            }

            let peak_memory = ALLOCATED.load(Ordering::SeqCst);
            let graph_memory = peak_memory - initial_memory;

            // Should use less than 50MB for 1000 nodes
            assert!(graph_memory < 50 * 1024 * 1024);

            println!("Memory usage for 1000 nodes: {} bytes", graph_memory);
        }

        // Check for memory leaks
        let final_memory = ALLOCATED.load(Ordering::SeqCst);
        assert_eq!(final_memory, initial_memory);
    }
}
```

### 6. Browser Compatibility Tests

#### Cross-Browser Testing Matrix

```yaml
# .github/workflows/browser-tests.yml
name: Browser Compatibility Tests

on: [push, pull_request]

jobs:
  browser-tests:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        browser: [chrome, firefox, safari, edge]
        version: [latest, previous]

    steps:
      - uses: actions/checkout@v2

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - name: Build WASM
        run: wasm-pack build --target web

      - name: Install Playwright
        run: |
          npm ci
          npx playwright install ${{ matrix.browser }}

      - name: Run browser tests
        run: |
          npx playwright test --browser=${{ matrix.browser }}
```

#### Feature Detection Tests

```rust
#[wasm_bindgen_test]
fn test_webgpu_feature_detection() {
    let webgpu_available = detect_webgpu_support();

    if webgpu_available {
        // Test WebGPU functionality
        let renderer = WebGPURenderer::new();
        assert!(renderer.is_ok());
    } else {
        // Test graceful fallback
        let renderer = select_fallback_renderer();
        assert!(matches!(renderer, RendererType::WebGL2 | RendererType::Canvas2D));
    }
}

#[wasm_bindgen_test]
fn test_touch_device_support() {
    let has_touch = detect_touch_support();

    if has_touch {
        // Test touch-specific interactions
        test_touch_gestures();
        test_pinch_zoom();
    } else {
        // Test mouse interactions
        test_mouse_interactions();
    }
}
```

## Test Execution

### Local Development

```bash
# Run all tests
cargo test --all-features

# Run specific test categories
cargo test --lib                    # Unit tests only
cargo test --test integration       # Integration tests only
cargo test --target wasm32-unknown-unknown  # WASM tests

# Run benchmarks
cargo bench

# Run visual tests
cd tests/visual
npm test

# Run browser compatibility tests
npx playwright test --headed
```

### Continuous Integration

```bash
# Full test suite (used in CI)
./scripts/test-all.sh

# Performance regression testing
cargo bench -- --save-baseline main
git checkout feature-branch
cargo bench -- --baseline main
```

### Coverage Reporting

```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --all-features --out Html --output-dir coverage

# View coverage report
open coverage/tarpaulin-report.html
```

## Test Data and Utilities

### Test Graph Generators

```rust
pub fn create_test_graph(node_count: usize, edge_count: usize) -> Graph {
    let mut graph = Graph::new();

    // Add nodes
    for i in 0..node_count {
        let node = Node::new(
            format!("node_{}", i),
            Position::new(
                (i % 10) as f64 * 100.0,
                (i / 10) as f64 * 100.0
            )
        );
        graph.add_node(node).unwrap();
    }

    // Add random edges
    let mut rng = rand::thread_rng();
    for _ in 0..edge_count {
        let source = format!("node_{}", rng.gen_range(0..node_count));
        let target = format!("node_{}", rng.gen_range(0..node_count));

        if source != target {
            let edge = Edge::new(
                format!("edge_{}_{}", source, target),
                source,
                target
            );
            let _ = graph.add_edge(edge); // Ignore duplicates
        }
    }

    graph
}

pub fn create_dag(node_count: usize) -> Graph {
    let mut graph = Graph::new();

    // Create nodes in levels
    for i in 0..node_count {
        let node = Node::new(
            format!("node_{}", i),
            Position::new((i % 5) as f64 * 150.0, (i / 5) as f64 * 100.0)
        );
        graph.add_node(node).unwrap();
    }

    // Add edges that respect DAG constraints
    for i in 0..node_count {
        for j in (i + 1)..std::cmp::min(i + 3, node_count) {
            let edge = Edge::new(
                format!("edge_{}_{}", i, j),
                format!("node_{}", i),
                format!("node_{}", j)
            );
            graph.add_edge(edge).unwrap();
        }
    }

    graph
}
```

This comprehensive testing strategy ensures high quality and reliability across all aspects of Leptos Flow, from core algorithms to user interfaces and cross-browser compatibility.
