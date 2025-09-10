# Leptos Flow Performance Specification

## Overview

This document defines performance requirements, optimization strategies, and benchmarking approaches for Leptos Flow. Our goal is to achieve desktop-class performance in the browser while maintaining smooth 60 FPS interactions even with large graphs.

## Performance Targets

### Primary Benchmarks

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| **Large Graph Rendering** | 10,000 nodes at 60 FPS | Frame time < 16.67ms |
| **Spatial Queries** | Sub-millisecond response | Query 1000 nodes < 1ms |
| **Memory Efficiency** | <50MB for 1000 nodes | DevTools memory profiler |
| **Bundle Size** | <500KB WASM (gzipped) | wasm-pack build output |
| **Initial Load** | <2s first render | Time to interactive |
| **Node Creation** | 1000 nodes/second | Batch creation benchmark |
| **Edge Creation** | 500 edges/second | Connection performance |
| **Zoom/Pan Response** | <50ms viewport update | Interaction latency |

### Secondary Benchmarks

| Metric | Target | Notes |
|--------|--------|-------|
| Layout algorithm (Force-directed) | 1000 nodes < 5s | Single-threaded |
| Export to SVG | 5000 nodes < 3s | String generation |
| Export to PNG | 5000 nodes < 10s | Canvas rendering |
| Undo/Redo operation | <100ms | State restoration |
| Selection of 1000 nodes | <200ms | Bulk selection |
| Search through 10k nodes | <50ms | Text-based search |

## Optimization Strategies

### 1. Spatial Indexing

#### R-tree Implementation
```rust
// Efficient spatial queries using R-tree data structure
pub struct SpatialIndex {
    rtree: RTree<NodeHandle>,
}

impl SpatialIndex {
    // O(log n) insertion
    pub fn insert(&mut self, node: &Node, handle: NodeHandle) {
        let rect = node.bounding_rect();
        self.rtree.insert(Entry::new(rect, handle));
    }
    
    // O(log n + k) viewport query where k = results
    pub fn query_viewport(&self, viewport: &Viewport) -> Vec<NodeHandle> {
        self.rtree.locate_in_envelope(&viewport.bounds())
            .map(|entry| entry.data)
            .collect()
    }
    
    // O(log n) nearest neighbor
    pub fn nearest_node(&self, point: Point) -> Option<NodeHandle> {
        self.rtree.nearest_neighbor(&point)
            .map(|entry| entry.data)
    }
}
```

#### Performance Characteristics
- **Query Time**: O(log n) for point queries, O(log n + k) for range queries
- **Memory Overhead**: ~40 bytes per node
- **Update Cost**: O(log n) for insert/remove/update operations

### 2. Object Pooling

#### Node Pool Implementation
```rust
// Reuse Node instances to reduce GC pressure
pub struct NodePool<T> {
    available: Vec<Box<Node<T>>>,
    in_use: HashMap<NodeId, Box<Node<T>>>,
    total_allocated: usize,
}

impl<T> NodePool<T> 
where 
    T: Default + Clone 
{
    pub fn acquire(&mut self, id: NodeId) -> &mut Node<T> {
        let node = if let Some(mut node) = self.available.pop() {
            node.reset();
            node.id = id;
            node
        } else {
            self.total_allocated += 1;
            Box::new(Node::new(id))
        };
        
        let node_ref = unsafe { 
            // Safety: We maintain exclusive access through HashMap
            &mut *(node.as_mut() as *mut Node<T>)
        };
        
        self.in_use.insert(id, node);
        node_ref
    }
    
    pub fn release(&mut self, id: &NodeId) -> Option<Box<Node<T>>> {
        if let Some(node) = self.in_use.remove(id) {
            self.available.push(node);
            Some(self.available.last().unwrap().clone())
        } else {
            None
        }
    }
}
```

#### Memory Benefits
- **Allocation Reduction**: Up to 90% fewer allocations for dynamic graphs
- **GC Pressure**: Reduced garbage collection overhead
- **Memory Locality**: Better cache performance through reuse

### 3. Dirty Rectangle Rendering

#### Change Detection System
```rust
// Track which screen regions need re-rendering
pub struct RenderRegions {
    dirty_rects: Vec<Rect>,
    merged_threshold: f64,
}

impl RenderRegions {
    pub fn mark_dirty(&mut self, rect: Rect) {
        // Merge overlapping rectangles to reduce draw calls
        let mut merged = false;
        for existing in &mut self.dirty_rects {
            if existing.overlaps(&rect) {
                *existing = existing.union(&rect);
                merged = true;
                break;
            }
        }
        
        if !merged {
            self.dirty_rects.push(rect);
        }
        
        // Merge adjacent rectangles if beneficial
        if self.should_merge() {
            self.merge_adjacent_rects();
        }
    }
    
    pub fn render_dirty_regions(&mut self, renderer: &mut dyn Renderer) {
        for rect in &self.dirty_rects {
            renderer.set_clip_rect(*rect);
            renderer.render_viewport_region(*rect);
        }
        self.dirty_rects.clear();
    }
}
```

#### Performance Impact
- **Rendering Cost**: Only redraw changed regions (5-50x improvement)
- **GPU Utilization**: Reduced fragment shader workload
- **Power Efficiency**: Lower battery usage on mobile devices

### 4. Level-of-Detail (LOD) System

#### Adaptive Detail Rendering
```rust
// Reduce visual complexity based on zoom level
pub struct LodRenderer {
    zoom_levels: Vec<ZoomLevel>,
}

#[derive(Clone)]
pub struct ZoomLevel {
    min_zoom: f64,
    max_zoom: f64,
    node_detail: NodeDetailLevel,
    edge_detail: EdgeDetailLevel,
    text_threshold: f64,
}

impl LodRenderer {
    pub fn render_node(&self, node: &Node, zoom: f64, renderer: &mut dyn Renderer) {
        let lod = self.get_lod_for_zoom(zoom);
        
        match lod.node_detail {
            NodeDetailLevel::Full => {
                renderer.render_node_full(node);
                if zoom >= lod.text_threshold {
                    renderer.render_node_text(node);
                }
            },
            NodeDetailLevel::Shape => {
                renderer.render_node_shape_only(node);
            },
            NodeDetailLevel::Point => {
                renderer.render_node_as_point(node);
            },
            NodeDetailLevel::Hidden => {
                // Skip rendering entirely
            }
        }
    }
}
```

#### LOD Levels
- **Zoom > 1.0**: Full detail with text, icons, handles
- **Zoom 0.5-1.0**: Simplified shapes, no text
- **Zoom 0.1-0.5**: Points or very simple shapes
- **Zoom < 0.1**: Culled entirely

### 5. Instanced Rendering

#### GPU-Optimized Batch Rendering
```rust
// Render similar nodes in a single draw call
pub struct InstancedRenderer {
    node_instances: HashMap<String, Vec<NodeInstance>>,
    instance_buffers: HashMap<String, InstanceBuffer>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NodeInstance {
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    transform: [f32; 4], // 2D transform matrix
}

impl InstancedRenderer {
    pub fn add_node_instance(&mut self, node_type: &str, instance: NodeInstance) {
        self.node_instances
            .entry(node_type.to_string())
            .or_default()
            .push(instance);
    }
    
    pub fn render_all_instances(&mut self, renderer: &mut dyn Renderer) {
        for (node_type, instances) in &self.node_instances {
            if instances.len() > INSTANCING_THRESHOLD {
                // Use instanced rendering for large batches
                renderer.render_instanced(node_type, instances);
            } else {
                // Use individual rendering for small batches
                for instance in instances {
                    renderer.render_individual(node_type, instance);
                }
            }
        }
        
        // Clear for next frame
        for instances in self.node_instances.values_mut() {
            instances.clear();
        }
    }
}
```

### 6. Web Workers for Layout

#### Offload CPU-Intensive Operations
```rust
// Run layout algorithms in Web Workers
pub struct LayoutWorker {
    worker: web_sys::Worker,
    pending_layouts: HashMap<LayoutId, LayoutRequest>,
}

impl LayoutWorker {
    pub fn start_layout(&mut self, layout: LayoutRequest) -> LayoutId {
        let layout_id = self.generate_layout_id();
        
        // Serialize graph data for worker
        let message = WorkerMessage::StartLayout {
            id: layout_id,
            algorithm: layout.algorithm,
            nodes: layout.nodes,
            edges: layout.edges,
            options: layout.options,
        };
        
        self.worker.post_message(&JsValue::from_serde(&message).unwrap());
        self.pending_layouts.insert(layout_id, layout);
        
        layout_id
    }
    
    pub fn handle_worker_message(&mut self, event: MessageEvent) {
        if let Ok(message) = event.data().into_serde::<WorkerResponse>() {
            match message {
                WorkerResponse::LayoutProgress { id, positions } => {
                    // Update node positions incrementally
                    self.update_node_positions(id, positions);
                },
                WorkerResponse::LayoutComplete { id, final_positions } => {
                    // Apply final positions and cleanup
                    self.complete_layout(id, final_positions);
                    self.pending_layouts.remove(&id);
                }
            }
        }
    }
}
```

## Memory Management Strategy

### 1. WASM Memory Optimization

#### Memory Layout
```rust
// Optimize struct layout for cache efficiency
#[repr(C)]
pub struct Node<T> {
    // Hot fields (frequently accessed) first
    pub position: Position,     // 16 bytes
    pub size: Size,            // 16 bytes
    pub id: NodeId,            // 8 bytes (interned string)
    
    // Warm fields
    pub selected: bool,        // 1 byte
    pub dragging: bool,        // 1 byte  
    pub visible: bool,         // 1 byte
    pub _padding: [u8; 5],     // Align to 8 bytes
    
    // Cold fields (accessed less frequently)
    pub data: T,               // Variable size
    pub style: Option<Style>,  // 8 bytes (Option)
    pub handles: Vec<Handle>,  // 24 bytes
}
```

#### Memory Pool Management
```rust
// Custom allocator for WASM heap
pub struct WasmAllocator {
    free_blocks: BTreeMap<usize, Vec<*mut u8>>,
    allocated_blocks: HashMap<*mut u8, usize>,
    total_allocated: usize,
    peak_usage: usize,
}

impl WasmAllocator {
    pub fn allocate(&mut self, size: usize, align: usize) -> *mut u8 {
        let aligned_size = (size + align - 1) & !(align - 1);
        
        // Try to reuse existing block
        if let Some(blocks) = self.free_blocks.get_mut(&aligned_size) {
            if let Some(ptr) = blocks.pop() {
                self.allocated_blocks.insert(ptr, aligned_size);
                return ptr;
            }
        }
        
        // Allocate new block
        let ptr = unsafe { 
            std::alloc::alloc(Layout::from_size_align(aligned_size, align).unwrap()) 
        };
        
        self.allocated_blocks.insert(ptr, aligned_size);
        self.total_allocated += aligned_size;
        self.peak_usage = self.peak_usage.max(self.total_allocated);
        
        ptr
    }
}
```

### 2. Garbage Collection Avoidance

#### String Interning
```rust
// Reduce string allocation overhead
pub struct StringInterner {
    strings: Vec<String>,
    indices: HashMap<String, StringId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(u32);

impl StringInterner {
    pub fn intern(&mut self, s: &str) -> StringId {
        if let Some(&id) = self.indices.get(s) {
            return id;
        }
        
        let id = StringId(self.strings.len() as u32);
        self.strings.push(s.to_string());
        self.indices.insert(s.to_string(), id);
        id
    }
    
    pub fn get(&self, id: StringId) -> &str {
        &self.strings[id.0 as usize]
    }
}
```

## Benchmarking Framework

### 1. Performance Testing Suite

#### Benchmark Categories
```rust
#[cfg(test)]
mod benchmarks {
    use criterion::{criterion_group, criterion_main, Criterion};
    
    // Rendering benchmarks
    fn bench_render_large_graph(c: &mut Criterion) {
        let graph = create_test_graph(10000, 15000);
        c.bench_function("render_10k_nodes", |b| {
            b.iter(|| graph.render(&mut test_renderer()))
        });
    }
    
    // Spatial query benchmarks  
    fn bench_spatial_queries(c: &mut Criterion) {
        let index = create_spatial_index(10000);
        let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0);
        
        c.bench_function("viewport_query", |b| {
            b.iter(|| index.query_viewport(&viewport))
        });
    }
    
    // Memory allocation benchmarks
    fn bench_node_creation(c: &mut Criterion) {
        c.bench_function("create_1000_nodes", |b| {
            b.iter(|| {
                let nodes: Vec<_> = (0..1000)
                    .map(|i| Node::new(format!("node_{}", i)))
                    .collect();
                nodes
            })
        });
    }
    
    // Layout algorithm benchmarks
    fn bench_force_directed_layout(c: &mut Criterion) {
        let mut graph = create_test_graph(1000, 1500);
        let mut layout = ForceDirectedLayout::new();
        
        c.bench_function("force_layout_1000_nodes", |b| {
            b.iter(|| layout.apply(&mut graph))
        });
    }
    
    criterion_group!(
        benches,
        bench_render_large_graph,
        bench_spatial_queries,
        bench_node_creation,
        bench_force_directed_layout
    );
}
```

### 2. Real-World Performance Testing

#### Interactive Performance Monitoring
```rust
// Runtime performance monitoring
pub struct PerformanceMonitor {
    frame_times: RingBuffer<f64>,
    memory_usage: RingBuffer<usize>,
    render_stats: RenderStats,
}

impl PerformanceMonitor {
    pub fn start_frame(&mut self) {
        self.frame_start = performance::now();
    }
    
    pub fn end_frame(&mut self) {
        let frame_time = performance::now() - self.frame_start;
        self.frame_times.push(frame_time);
        
        if frame_time > 16.67 {
            warn!("Frame time exceeded 60 FPS target: {:.2}ms", frame_time);
        }
    }
    
    pub fn get_fps(&self) -> f64 {
        let avg_frame_time = self.frame_times.iter().sum::<f64>() 
            / self.frame_times.len() as f64;
        1000.0 / avg_frame_time
    }
    
    pub fn get_memory_usage(&self) -> MemoryStats {
        MemoryStats {
            wasm_heap: wasm_heap_size(),
            js_heap: js_heap_size(),
            gpu_memory: gpu_memory_usage(),
        }
    }
}
```

### 3. Continuous Performance Regression Testing

#### CI/CD Integration
```yaml
# .github/workflows/performance.yml
name: Performance Tests
on: [push, pull_request]

jobs:
  performance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      # Run benchmark suite
      - name: Run benchmarks
        run: |
          cargo bench --features="benchmark" -- --output-format json > benchmarks.json
      
      # Compare with baseline
      - name: Compare performance
        run: |
          python scripts/compare_benchmarks.py benchmarks.json baseline.json
      
      # Fail if regression > 10%
      - name: Check regression
        run: |
          if [ "$REGRESSION_PERCENT" -gt 10 ]; then
            echo "Performance regression detected: $REGRESSION_PERCENT%"
            exit 1
          fi
```

## Performance Monitoring Dashboard

### Real-Time Metrics
- Frame rate (FPS) with 1-second moving average  
- Memory usage (WASM heap, JS heap, GPU memory)
- Render statistics (nodes/edges rendered, culled, cached)
- Layout performance (iterations/second, convergence time)
- User interaction latency (click-to-response time)

### Performance Alerts
- Frame drops below 45 FPS for >1 second
- Memory usage increases >50% in 10 seconds  
- Any operation takes >500ms to complete
- GPU memory usage exceeds browser limits

This performance specification ensures Leptos Flow delivers desktop-class performance while maintaining the flexibility needed for complex interactive applications.