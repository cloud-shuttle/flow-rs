# P1: Stub Implementation Completion

## Issue Summary
Critical components are marked as implemented but contain placeholder code or empty functions, making production claims misleading.

## Missing Core Implementations

### Layout Algorithms (flow-core/src/layout.rs)
**Status**: Function signatures exist but return `Ok(())` without positioning nodes

#### CircularLayout::apply()
- **Current**: Returns immediately without computation
- **Required**: Arrange nodes in circle based on graph connectivity
- **Complexity**: ~200 lines of trigonometry and node spacing

#### GridLayout::apply()
- **Current**: Empty implementation
- **Required**: Grid-based positioning with collision detection
- **Complexity**: ~150 lines with spatial partitioning

### Renderer Backends (flow-renderer/src/)
**Status**: Feature flags compile but runtime methods are `todo!()`

#### WebGL2 Backend
- **Current**: Struct definitions only
- **Required**: Shader compilation, buffer management, draw calls
- **Complexity**: ~800 lines (vertex/fragment shaders, uniforms)

#### WebGPU Backend
- **Current**: Feature gated but unimplemented
- **Required**: Full rendering pipeline, compute shader support
- **Complexity**: ~1200 lines (most complex backend)

### WASM Bindings (flow-wasm/src/bindings.rs)
**Status**: Only exports `greet()` function

#### Missing Exports
- [ ] Graph constructor and manipulation methods
- [ ] Node/Edge creation and deletion
- [ ] Event handling (mouse, keyboard, drag)
- [ ] Renderer initialization and canvas binding
- [ ] Layout algorithm invocation

## Implementation Priority

### Phase 1: Core Layout Algorithms (5-7 days)
**Acceptance Criteria**: Layouts actually move nodes to meaningful positions

```rust
// CircularLayout - arrange nodes in concentric circles
impl LayoutAlgorithm for CircularLayout {
    fn apply(&mut self, graph: &mut Graph) -> Result<()> {
        let nodes: Vec<_> = graph.nodes().collect();
        let center = Position::new(self.center_x, self.center_y);
        let radius = self.radius;

        for (i, node) in nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / nodes.len() as f64;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();

            graph.update_node_position(node.id(), Position::new(x, y))?;
        }
        Ok(())
    }
}
```

### Phase 2: WASM API Surface (3-4 days)
**Acceptance Criteria**: JS can create and manipulate graphs without recompiling Rust

```rust
#[wasm_bindgen]
impl WasmGraph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGraph { /* ... */ }

    #[wasm_bindgen]
    pub fn add_node(&mut self, id: &str, x: f64, y: f64) -> Result<(), JsValue> { /* ... */ }

    #[wasm_bindgen]
    pub fn add_edge(&mut self, source: &str, target: &str) -> Result<(), JsValue> { /* ... */ }

    #[wasm_bindgen]
    pub fn apply_layout(&mut self, algorithm: &str) -> Result<(), JsValue> { /* ... */ }
}
```

### Phase 3: Renderer Backend Choice (2-3 days)
**Decision**: Implement ONE complete backend rather than three partial ones

**Recommendation**: Complete Canvas2D backend with performance optimizations
- Simpler than WebGL/WebGPU
- Better browser compatibility
- Sufficient for 1000+ node performance target
- Can add GPU backends later

## Testing Requirements

### Layout Algorithm Tests
```rust
#[test]
fn circular_layout_positions_nodes_in_circle() {
    let mut graph = Graph::new();
    // Add 8 nodes at origin
    // Apply circular layout
    // Verify nodes are positioned on circle circumference
    // Verify no overlapping positions
}

#[test]
fn grid_layout_avoids_collisions() {
    let mut graph = Graph::new();
    // Add 100 nodes at origin
    // Apply grid layout
    // Verify no two nodes occupy same grid cell
    // Verify reasonable spacing between nodes
}
```

### WASM Integration Tests
```rust
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn js_can_create_graph() {
        let graph = WasmGraph::new();
        assert!(graph.add_node("node1", 100.0, 100.0).is_ok());
    }
}
```

## Risk Assessment

**High Risk**: WebGL/WebGPU backends
- Complex shader management
- Browser compatibility issues
- Performance debugging difficulties

**Medium Risk**: Layout algorithms
- Mathematical correctness
- Performance with large graphs
- Edge case handling (empty graphs, single nodes)

**Low Risk**: WASM bindings
- Straightforward delegation to Rust API
- Good wasm-bindgen documentation
- Easy to test incrementally

## Success Criteria
- [ ] CircularLayout produces visually correct circular arrangements
- [ ] GridLayout handles 1000+ nodes without overlaps
- [ ] WASM exports allow basic graph manipulation from JS
- [ ] Performance targets met (layout <100ms for 1000 nodes)
- [ ] Property-based tests validate layout invariants
- [ ] Browser demo works without recompiling Rust

## Dependencies
- Requires completion of [02-panic-audit.md](02-panic-audit.md)
- Blocks [06-integration-tests.md](06-integration-tests.md)
