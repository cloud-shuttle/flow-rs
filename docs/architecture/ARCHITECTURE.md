# Leptos Flow Architecture Specification

## Overview

Leptos Flow is a high-performance, reactive flow-based node editor built on Rust and WebAssembly, designed specifically for the Leptos framework. This document outlines the core architectural decisions, design principles, and system boundaries.

## Core Design Principles

### 1. Zero-Cost Abstractions
- Leverage Rust's ownership system for memory safety without runtime overhead
- Compile-time optimizations through trait specialization
- WASM-optimized code generation with minimal bundle size

### 2. Framework-Agnostic Core
- Pure Rust logic layer independent of UI frameworks
- Clean separation between computation and presentation
- Pluggable renderer architecture supporting multiple backends

### 3. Progressive Enhancement
- Graceful degradation from WebGPU → WebGL2 → Canvas2D
- Feature detection and automatic fallback selection
- Adaptive performance based on device capabilities

### 4. Reactive State Management
- Signal-based reactivity aligned with Leptos patterns
- Efficient dirty checking and selective updates
- Minimal re-renders through dependency tracking

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Leptos Integration Layer                  │
├─────────────────────────────────────────────────────────────┤
│  Components  │  Hooks  │  Context  │  Signal Integration    │
├─────────────────────────────────────────────────────────────┤
│                     Core Engine (WASM)                      │
├─────────────────────────────────────────────────────────────┤
│   Graph    │  Layout   │  Events   │   Spatial Index       │
│   State    │  Engine   │  System   │   (R-tree/Quadtree)   │
├─────────────────────────────────────────────────────────────┤
│                    Rendering Abstraction                    │
├─────────────────────────────────────────────────────────────┤
│  WebGPU    │  WebGL2   │ Canvas2D  │    SVG Export         │
│ Renderer   │ Renderer  │ Renderer  │    (Optional)         │
└─────────────────────────────────────────────────────────────┘
```

## Module Boundaries

### leptos-flow-core
**Purpose**: Framework-agnostic core logic
**Responsibilities**:
- Graph data structures (Node, Edge, Graph)
- Spatial indexing and collision detection
- Layout algorithms (Force-directed, Hierarchical, Manual)
- Event handling and interaction state
- Memory management and object pooling

**Key Types**:
```rust
pub struct Graph<N, E> {
    pub nodes: HashMap<NodeId, Node<N>>,
    pub edges: HashMap<EdgeId, Edge<E>>,
    spatial_index: SpatialIndex,
}

pub struct Node<T> {
    pub id: NodeId,
    pub position: Position,
    pub data: T,
    pub size: Size,
}

pub struct Edge<T> {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub data: T,
}
```

### leptos-flow-renderer
**Purpose**: Rendering abstraction and implementations
**Responsibilities**:
- Renderer trait definition
- WebGPU/WebGL2/Canvas2D implementations
- Viewport management and camera controls
- Performance monitoring and metrics

**Key Traits**:
```rust
pub trait Renderer {
    fn render(&mut self, graph: &Graph, viewport: &Viewport) -> Result<()>;
    fn capabilities(&self) -> RendererCapabilities;
    fn set_viewport(&mut self, viewport: Viewport);
}
```

### leptos-flow-leptos
**Purpose**: Leptos framework integration
**Responsibilities**:
- Leptos component wrappers
- Signal integration and reactivity
- Event handling bridge
- Context providers

**Key Components**:
```rust
#[component]
pub fn FlowEditor<N, E>(
    nodes: ReadSignal<Vec<Node<N>>>,
    edges: ReadSignal<Vec<Edge<E>>>,
    on_nodes_change: WriteSignal<Vec<Node<N>>>,
) -> impl IntoView;
```

### leptos-flow-wasm
**Purpose**: WebAssembly bindings and optimization
**Responsibilities**:
- WASM interface generation
- JavaScript interop
- Memory management across WASM boundary
- Performance profiling hooks

## Data Flow Architecture

### 1. Reactive State Management
```rust
// Leptos signals drive the reactive system
let (nodes, set_nodes) = create_signal(Vec::new());
let (edges, set_edges) = create_signal(Vec::new());

// Core engine reacts to signal changes
let graph = create_memo(move |_| {
    Graph::new(nodes.get(), edges.get())
});
```

### 2. Event Propagation System
```
User Interaction → DOM Event → Leptos Event Handler → 
Core Engine → State Update → Signal Update → Re-render
```

### 3. Render Pipeline Stages
```
1. Dirty Detection   → Identify changed elements
2. Spatial Query     → Query visible nodes/edges
3. Culling          → Remove off-screen elements
4. Batching         → Group similar draw calls
5. Rendering        → Execute draw commands
6. Present          → Display frame buffer
```

### 4. Memory Management Strategy
- **Object Pooling**: Reuse Node/Edge instances to reduce allocations
- **Incremental Updates**: Only process changed elements
- **Spatial Partitioning**: Efficient spatial queries using R-tree
- **WASM Optimization**: Minimize WASM heap allocations

## Performance Characteristics

### Target Performance Metrics
- **10,000 nodes** at **60 FPS** (smooth interaction)
- **Sub-millisecond** spatial queries for viewport operations  
- **<50MB memory** usage for 1000-node graphs
- **<500KB** WASM bundle size (gzipped)

### Optimization Strategies
- **Spatial Indexing**: R-tree for O(log n) spatial queries
- **Object Pooling**: Reduce GC pressure through reuse
- **Dirty Rectangle Rendering**: Only redraw changed regions
- **Level-of-Detail (LOD)**: Reduce complexity at distance
- **Instanced Rendering**: Batch similar nodes/edges
- **Web Workers**: Offload layout calculations

## Error Handling & Recovery

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Renderer initialization failed: {0}")]
    RendererInit(String),
    
    #[error("Invalid graph structure: {0}")]
    InvalidGraph(String),
    
    #[error("Spatial index error: {0}")]
    SpatialIndex(String),
    
    #[error("WASM boundary error: {0}")]
    WasmBoundary(String),
}
```

### Recovery Strategies
- **Graceful Degradation**: Fall back to simpler renderer
- **State Recovery**: Restore from last known good state
- **Progressive Loading**: Load large graphs incrementally
- **Error Boundaries**: Isolate failures to specific components

## Extensibility Points

### Custom Node Types
```rust
#[derive(Clone, Debug)]
pub struct CustomNodeData {
    pub label: String,
    pub node_type: NodeType,
    pub style: NodeStyle,
}

impl NodeData for CustomNodeData {
    fn render_content(&self, renderer: &mut dyn Renderer) {
        // Custom rendering logic
    }
}
```

### Custom Layout Algorithms
```rust
pub trait LayoutAlgorithm {
    fn layout(&mut self, graph: &mut Graph, options: LayoutOptions) -> Result<()>;
}

pub struct ForceDirectedLayout {
    iterations: usize,
    spring_strength: f32,
    repulsion_strength: f32,
}
```

### Custom Renderers
```rust
pub struct CustomRenderer {
    context: CustomRenderContext,
}

impl Renderer for CustomRenderer {
    fn render(&mut self, graph: &Graph, viewport: &Viewport) -> Result<()> {
        // Custom rendering implementation
    }
}
```

## Security Considerations

### WASM Sandbox
- All core logic runs within WASM security sandbox
- No direct file system access
- Limited network capabilities
- Memory isolation from host environment

### Input Validation
- Sanitize all user inputs at WASM boundary
- Validate graph structure integrity
- Prevent excessive resource consumption
- Rate limiting for interaction events

## Testing Strategy

### Unit Tests
- Core data structures and algorithms
- Renderer implementations
- Layout algorithms
- Spatial indexing

### Integration Tests  
- Leptos component integration
- Event handling workflows
- State synchronization
- WASM boundary operations

### Performance Tests
- Rendering benchmarks
- Memory usage profiling
- Large graph handling
- Interaction responsiveness

### Visual Regression Tests
- Automated screenshot comparison
- Cross-browser compatibility
- Rendering accuracy validation

This architecture provides a solid foundation for building a high-performance, maintainable flow editor that leverages Rust's strengths while integrating seamlessly with the Leptos ecosystem.