# Performance Optimization Guide

This guide provides comprehensive information about optimizing the performance of the Leptos Flow Simple Example.

## Table of Contents

- [Performance Overview](#performance-overview)
- [Built-in Optimizations](#built-in-optimizations)
- [Rendering Optimizations](#rendering-optimizations)
- [Memory Management](#memory-management)
- [Performance Monitoring](#performance-monitoring)
- [Optimization Strategies](#optimization-strategies)
- [Benchmarking](#benchmarking)
- [Troubleshooting](#troubleshooting)

## Performance Overview

The Leptos Flow Simple Example is designed for high performance with the following targets:

### Performance Targets

| Metric | Target | Excellent | Good | Acceptable |
|--------|--------|-----------|------|------------|
| **FPS** | 60 FPS | ≥ 60 | ≥ 30 | ≥ 15 |
| **Frame Time** | ≤ 16.67ms | ≤ 16.67ms | ≤ 33.33ms | ≤ 66.67ms |
| **Memory Usage** | ≤ 50MB | ≤ 50MB | ≤ 100MB | ≤ 200MB |
| **Node Rendering** | ≤ 100ms | ≤ 50ms | ≤ 100ms | ≤ 200ms |
| **Edge Rendering** | ≤ 50ms | ≤ 25ms | ≤ 50ms | ≤ 100ms |

### Performance Features

- ✅ **Spatial Indexing**: Efficient culling of off-screen elements
- ✅ **Batching**: Grouped rendering operations for better performance
- ✅ **Level of Detail (LOD)**: Reduced detail at high zoom levels
- ✅ **Memory Pooling**: Efficient memory allocation and reuse
- ✅ **Performance Monitoring**: Real-time performance tracking
- ✅ **Optimized Builds**: Size and performance optimized WASM

## Built-in Optimizations

### 1. Spatial Indexing

The renderer uses spatial indexing to efficiently cull elements outside the viewport:

```rust
// Automatic viewport culling
let viewport_rect = viewport.bounds().expand(cull_margin);
if !viewport_rect.intersects(&node_rect) {
    return; // Skip rendering off-screen nodes
}
```

### 2. Batching System

Rendering operations are batched for better performance:

```rust
// Batch multiple nodes for efficient rendering
let mut batch = RenderBatch::new();
for node in nodes {
    if should_render_node(node, viewport) {
        batch.add_node(node);
    }
}
batch.sort_by_z_index();
render_batch(&batch);
```

### 3. Level of Detail (LOD)

Detail is reduced at high zoom levels to maintain performance:

```rust
// LOD system reduces detail based on zoom level
let detail_level = lod_system.get_detail_level(viewport.zoom);
if node_size < detail_level {
    return; // Skip small nodes at high zoom
}
```

### 4. Memory Pooling

Efficient memory allocation using object pools:

```rust
// Reuse allocated objects to reduce garbage collection
let node_id = node_pool.allocate();
// ... use node ...
node_pool.deallocate(node_id);
```

## Rendering Optimizations

### 1. Efficient Canvas Operations

```rust
// Batch canvas operations
context.save();
context.set_fill_style(&color);
context.fill_rect(x, y, width, height);
context.restore();
```

### 2. Minimize State Changes

```rust
// Group operations by state to minimize context changes
let mut current_style = None;
for node in nodes {
    if current_style != Some(&node.style) {
        apply_node_style(&node.style);
        current_style = Some(&node.style);
    }
    render_node(node);
}
```

### 3. Use Efficient Shapes

```rust
// Prefer simple shapes for better performance
match node.variant {
    NodeVariant::Rectangle => render_rectangle(node), // Fastest
    NodeVariant::Circle => render_circle(node),       // Fast
    NodeVariant::Diamond => render_diamond(node),     // Slower
    NodeVariant::Custom => render_custom(node),       // Slowest
}
```

### 4. Optimize Edge Rendering

```rust
// Use straight lines when possible
match edge.variant {
    EdgeVariant::Straight => render_straight_line(edge),     // Fastest
    EdgeVariant::Curved => render_curved_line(edge),         // Slower
    EdgeVariant::Bezier => render_bezier_curve(edge),        // Slowest
}
```

## Memory Management

### 1. Object Pooling

```rust
// Use object pools for frequently allocated objects
let mut node_pool = MemoryPool::new(1000);
let node_id = node_pool.allocate();
// ... use node ...
node_pool.deallocate(node_id);
```

### 2. Efficient Data Structures

```rust
// Use appropriate data structures
use std::collections::HashMap; // For lookups
use std::collections::VecDeque; // For queues
use std::collections::BTreeSet; // For ordered sets
```

### 3. Memory Monitoring

```rust
// Monitor memory usage
let memory_monitor = MemoryMonitor::new();
memory_monitor.init();
// ... perform operations ...
memory_monitor.update_peak_memory();
let stats = memory_monitor.get_memory_stats();
```

### 4. Garbage Collection Optimization

```rust
// Minimize allocations in hot paths
let mut temp_vec = Vec::with_capacity(100); // Pre-allocate
// ... use temp_vec ...
temp_vec.clear(); // Reuse instead of dropping
```

## Performance Monitoring

### 1. Built-in Performance Monitor

```rust
use crate::performance_monitor::PerformanceMonitor;

let mut monitor = PerformanceMonitor::new(100);
monitor.start_monitoring();

// ... perform operations ...

monitor.record_frame();
let fps = monitor.get_fps();
let stats = monitor.get_stats();
```

### 2. Rendering Analyzer

```rust
use crate::performance_monitor::RenderingAnalyzer;

let mut analyzer = RenderingAnalyzer::new(100);
analyzer.record_render(render_time, node_count, edge_count);
let metrics = analyzer.get_metrics();
```

### 3. Performance Benchmark

```rust
use crate::performance_monitor::PerformanceBenchmark;

let mut benchmark = PerformanceBenchmark::new();
let result = benchmark.run_benchmark(5000.0); // 5 second benchmark
```

### 4. Real-time Monitoring

```javascript
// JavaScript integration
import { PerformanceMonitor } from './pkg/simple_flow_example.js';

const monitor = new PerformanceMonitor(100);
monitor.startMonitoring();

// In your render loop
function renderLoop() {
    // ... rendering code ...
    monitor.recordFrame();

    if (monitor.getFps() < 30) {
        console.warn('Performance degraded:', monitor.getStats());
    }

    requestAnimationFrame(renderLoop);
}
```

## Optimization Strategies

### 1. Node Count Optimization

```rust
// Limit the number of visible nodes
const MAX_VISIBLE_NODES: usize = 1000;

if visible_nodes.len() > MAX_VISIBLE_NODES {
    // Use LOD or culling to reduce visible nodes
    visible_nodes = apply_lod_culling(visible_nodes, viewport);
}
```

### 2. Edge Count Optimization

```rust
// Limit the number of visible edges
const MAX_VISIBLE_EDGES: usize = 2000;

if visible_edges.len() > MAX_VISIBLE_EDGES {
    // Use LOD or culling to reduce visible edges
    visible_edges = apply_lod_culling(visible_edges, viewport);
}
```

### 3. Viewport Optimization

```rust
// Only render what's visible
let viewport_bounds = viewport.bounds().expand(100.0); // Add margin
let visible_nodes: Vec<_> = nodes
    .iter()
    .filter(|node| viewport_bounds.intersects(&node.bounds()))
    .collect();
```

### 4. Animation Optimization

```rust
// Use efficient animation techniques
if animation_frame % 2 == 0 {
    // Skip every other frame for non-critical animations
    render_animations();
}
```

### 5. Texture Optimization

```rust
// Use efficient texture formats
let texture_format = if supports_webgl2 {
    TextureFormat::RGBA8 // High quality
} else {
    TextureFormat::RGB565 // Lower quality, better performance
};
```

## Benchmarking

### 1. Performance Benchmark Suite

```bash
# Run the built-in benchmark
./run_tests.sh

# Run performance-specific tests
wasm-pack test --headless --firefox --test performance_tests
```

### 2. Custom Benchmarks

```rust
#[wasm_bindgen_test]
fn benchmark_large_graph_rendering() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_large_test_graph(1000); // 1000 nodes
    let viewport = create_test_viewport();

    let start_time = web_sys::js_sys::Date::now();
    renderer.render_graph(&graph, &viewport).unwrap();
    let end_time = web_sys::js_sys::Date::now();

    let render_time = end_time - start_time;
    assert!(render_time <= 500.0, "Large graph rendering took {}ms, expected <= 500ms", render_time);
}
```

### 3. Memory Benchmarks

```rust
#[wasm_bindgen_test]
fn benchmark_memory_usage() {
    let canvas = create_test_canvas();
    let renderer = Canvas2DRenderer::new(&canvas).unwrap();

    // Check memory usage after initialization
    if let Some(performance) = web_sys::window().unwrap().performance() {
        let memory = performance.memory();
        if !memory.is_undefined() {
            let used_js_heap_size = js_sys::Reflect::get(&memory, &"usedJSHeapSize".into()).unwrap();
            let used_js_heap_size = used_js_heap_size.as_f64().unwrap();

            let max_memory_mb = 50.0 * 1024.0 * 1024.0; // 50MB
            assert!(used_js_heap_size <= max_memory_mb,
                   "Memory usage {}MB exceeds limit of 50MB",
                   used_js_heap_size / (1024.0 * 1024.0));
        }
    }
}
```

### 4. Interactive Benchmarking

```html
<!-- benchmark.html -->
<!DOCTYPE html>
<html>
<head>
    <title>Performance Benchmark</title>
</head>
<body>
    <canvas id="benchmark-canvas" width="800" height="600"></canvas>
    <div id="results"></div>

    <script type="module">
        import init, { PerformanceBenchmark } from './pkg/simple_flow_example.js';

        async function runBenchmark() {
            await init();

            const benchmark = new PerformanceBenchmark();
            const result = benchmark.runBenchmark(5000); // 5 seconds

            document.getElementById('results').innerHTML = `
                <h2>Benchmark Results</h2>
                <p>Average FPS: ${result.average_fps.toFixed(2)}</p>
                <p>Average Frame Time: ${result.average_frame_time.toFixed(2)}ms</p>
                <p>Performance Grade: ${result.performance_grade}</p>
            `;
        }

        runBenchmark();
    </script>
</body>
</html>
```

## Troubleshooting

### Common Performance Issues

#### 1. Low FPS (< 30 FPS)

**Symptoms:**

- Choppy animations
- Slow response to user input
- High frame times (> 33ms)

**Solutions:**

```rust
// Reduce node count
const MAX_NODES: usize = 500;

// Enable aggressive culling
let settings = PerformanceSettings {
    enable_culling: true,
    cull_margin: 200.0, // Larger cull margin
    ..Default::default()
};

// Use LOD system
let settings = PerformanceSettings {
    enable_lod: true,
    ..Default::default()
};
```

#### 2. High Memory Usage (> 100MB)

**Symptoms:**

- Browser becomes unresponsive
- Memory usage keeps growing
- Garbage collection pauses

**Solutions:**

```rust
// Use object pooling
let mut node_pool = MemoryPool::new(1000);

// Clear unused data
graph.clear_unused_nodes();
renderer.clear_batch_cache();

// Monitor memory usage
let memory_monitor = MemoryMonitor::new();
memory_monitor.init();
```

#### 3. Slow Initial Rendering (> 200ms)

**Symptoms:**

- Long delay before first render
- Blank canvas for several seconds
- High initial frame time

**Solutions:**

```rust
// Optimize initial graph creation
let graph = create_optimized_graph();

// Use progressive rendering
renderer.enable_progressive_rendering(true);

// Pre-warm the renderer
renderer.pre_warm();
```

#### 4. Memory Leaks

**Symptoms:**

- Memory usage keeps growing
- Performance degrades over time
- Browser crashes after extended use

**Solutions:**

```rust
// Proper cleanup
impl Drop for FlowRenderer {
    fn drop(&mut self) {
        self.clear_all_caches();
        self.remove_event_listeners();
    }
}

// Regular cleanup
setInterval(|| {
    renderer.cleanup_unused_resources();
}, 30000); // Every 30 seconds
```

### Performance Debugging

#### 1. Enable Performance Logging

```rust
// Enable detailed performance logging
let monitor = PerformanceMonitor::new(100);
monitor.start_monitoring();

// Log performance every second
setInterval(|| {
    monitor.log_stats();
}, 1000);
```

#### 2. Profile Rendering Operations

```rust
// Profile individual operations
let start_time = web_sys::js_sys::Date::now();
renderer.render_background(&bg_config, &viewport)?;
let bg_time = web_sys::js_sys::Date::now() - start_time;

let start_time = web_sys::js_sys::Date::now();
renderer.render_graph(&graph, &viewport)?;
let graph_time = web_sys::js_sys::Date::now() - start_time;

console::log_1(&format!("Background: {}ms, Graph: {}ms", bg_time, graph_time).into());
```

#### 3. Monitor Resource Usage

```rust
// Monitor resource usage
let stats = renderer.get_stats();
console::log_1(&format!("Nodes: {}, Edges: {}, Draw calls: {}",
    stats.nodes_rendered,
    stats.edges_rendered,
    stats.draw_calls
).into());
```

## Best Practices

### 1. Development

- Use performance monitoring during development
- Set up automated performance tests
- Profile regularly with different data sizes
- Test on various devices and browsers

### 2. Production

- Use optimized builds (`./build_optimized.sh`)
- Enable all performance optimizations
- Monitor performance in production
- Set up performance alerts

### 3. Maintenance

- Regular performance audits
- Update dependencies for performance improvements
- Monitor for performance regressions
- Optimize based on real usage patterns

## Conclusion

The Leptos Flow Simple Example provides comprehensive performance optimization features. By following this guide and using the built-in performance monitoring tools, you can ensure your flow diagrams perform well across all devices and use cases.

For more information, see:

- [API Examples](./API_EXAMPLES.md) for detailed API usage
- [Usage Examples](./USAGE_EXAMPLES.md) for practical examples
- [Testing Guide](./TESTING.md) for performance testing
