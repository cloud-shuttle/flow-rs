# WASM Bindings Design

## Purpose
Provides a JavaScript API for Flow-RS components, enabling browser applications to create and manipulate flow graphs without recompiling Rust code. Optimized for minimal bundle size and efficient JS/WASM boundary crossings.

## Public API

### Core Graph API
```typescript
export class WasmGraph {
    constructor();

    // Node management
    add_node(id: string, x: number, y: number, width?: number, height?: number): void;
    remove_node(id: string): boolean;
    update_node_position(id: string, x: number, y: number): void;
    get_node(id: string): WasmNode | undefined;

    // Edge management
    add_edge(source: string, target: string, id?: string): void;
    remove_edge(id: string): boolean;
    get_edge(id: string): WasmEdge | undefined;

    // Layout operations
    apply_layout(algorithm: string, config?: any): void;

    // Serialization
    to_json(): string;
    from_json(json: string): void;
}
```

### Renderer Integration
```typescript
export class WasmRenderer {
    constructor(canvas: HTMLCanvasElement);

    render_graph(graph: WasmGraph, viewport: WasmViewport): void;
    set_theme(theme: RenderTheme): void;

    // Event handling
    handle_mouse_event(event: MouseEvent, graph: WasmGraph): boolean;
    handle_keyboard_event(event: KeyboardEvent, graph: WasmGraph): boolean;
}
```

### Event System
```typescript
export interface FlowEventHandler {
    on_node_selected?(node_id: string): void;
    on_node_dragged?(node_id: string, x: number, y: number): void;
    on_edge_created?(source: string, target: string): void;
    on_viewport_changed?(viewport: WasmViewport): void;
}

export function setup_event_handlers(
    canvas: HTMLCanvasElement,
    graph: WasmGraph,
    handlers: FlowEventHandler
): void;
```

## Internal Architecture

### Rust Implementation

#### Core Wrapper Types
```rust
#[wasm_bindgen]
pub struct WasmGraph {
    inner: Graph<WasmNodeData, WasmEdgeData>,
    event_handlers: Vec<js_sys::Function>,
}

#[wasm_bindgen]
impl WasmGraph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGraph {
        WasmGraph {
            inner: Graph::new(),
            event_handlers: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_node(&mut self, id: &str, x: f64, y: f64) -> Result<(), JsValue> {
        let node = Node::builder(id)
            .position(x, y)
            .data(WasmNodeData::default())
            .build();

        self.inner.add_node(node)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
```

#### Memory Management
```rust
// Use weak references to avoid cycles between JS and Rust
#[wasm_bindgen]
pub struct WasmEventManager {
    callbacks: HashMap<String, js_sys::WeakRef>,
}

impl WasmEventManager {
    pub fn emit_event(&self, event_type: &str, data: &JsValue) {
        if let Some(weak_ref) = self.callbacks.get(event_type) {
            if let Some(callback) = weak_ref.deref() {
                let _ = callback.call1(&JsValue::NULL, data);
            }
        }
    }
}
```

### JavaScript Integration

#### TypeScript Definitions
```typescript
// Generated automatically from wasm-bindgen
declare module "flow-rs-wasm" {
    export class WasmGraph {
        constructor();
        add_node(id: string, x: number, y: number): void;
        // ... rest of API
    }
}
```

#### Bundle Optimization
```javascript
// Use dynamic imports for code splitting
export async function loadFlowRS() {
    const wasm = await import("flow-rs-wasm");
    await wasm.default(); // Initialize WASM module
    return wasm;
}

// Tree-shake unused features
export { WasmGraph, WasmRenderer } from "flow-rs-wasm";
// Don't export advanced layout algorithms unless needed
```

### Error Handling Strategy

#### Rust Error Mapping
```rust
impl From<FlowError> for JsValue {
    fn from(error: FlowError) -> JsValue {
        let error_obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &error_obj,
            &"type".into(),
            &error.error_type().into()
        ).unwrap();
        js_sys::Reflect::set(
            &error_obj,
            &"message".into(),
            &error.to_string().into()
        ).unwrap();
        error_obj.into()
    }
}
```

#### JavaScript Error Handling
```typescript
try {
    graph.add_node("duplicate", 0, 0);
} catch (error: any) {
    if (error.type === "NodeAlreadyExists") {
        console.warn(`Node already exists: ${error.message}`);
    } else {
        throw error; // Re-throw unexpected errors
    }
}
```

## Dependencies

### Rust Crates
- `wasm-bindgen` - Core WASM bindings generation
- `js-sys` - JavaScript API bindings
- `web-sys` - DOM and Web API access
- `serde-wasm-bindgen` - Efficient serialization
- `console_error_panic_hook` - Better error messages

### JavaScript Dependencies
- Modern browser with WASM support
- Optional: TypeScript for type safety
- Optional: Bundler supporting WASM (Webpack 5+, Vite, Rollup)

## Performance Characteristics

### Bundle Size Optimization
| Component | Size (gzipped) | Notes |
|-----------|----------------|-------|
| Core graph ops | ~45KB | Essential functionality |
| Canvas renderer | ~25KB | 2D rendering only |
| Layout algorithms | ~35KB | All basic layouts |
| Advanced layouts | ~60KB | Force-directed, hierarchical |
| **Total (basic)** | **~105KB** | Reasonable for web apps |

### Runtime Performance
- **Graph operations**: O(1) for add/remove, O(log n) for queries
- **JS/WASM boundary**: <1ms overhead per operation
- **Memory usage**: ~100 bytes per node, ~50 bytes per edge
- **GC pressure**: Minimal due to WASM linear memory

### Optimization Strategies
```rust
// Batch operations to reduce boundary crossings
#[wasm_bindgen]
impl WasmGraph {
    pub fn add_nodes_batch(&mut self, nodes_json: &str) -> Result<(), JsValue> {
        let nodes: Vec<NodeData> = serde_json::from_str(nodes_json)?;
        for node_data in nodes {
            self.inner.add_node(node_data.into_node())?;
        }
        Ok(())
    }
}
```

## Testing Strategy

### Unit Tests (Rust)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn can_create_and_manipulate_graph() {
        let mut graph = WasmGraph::new();
        graph.add_node("node1", 100.0, 100.0).unwrap();

        assert!(graph.get_node("node1").is_some());
        assert!(graph.get_node("nonexistent").is_none());
    }
}
```

### Integration Tests (JavaScript)
```javascript
import { loadFlowRS } from './flow-rs-wasm';

describe('WASM Graph Integration', () => {
    let wasm;

    beforeAll(async () => {
        wasm = await loadFlowRS();
    });

    test('can create graph with nodes and edges', () => {
        const graph = new wasm.WasmGraph();

        graph.add_node('A', 0, 0);
        graph.add_node('B', 100, 100);
        graph.add_edge('A', 'B');

        expect(graph.get_node('A')).toBeTruthy();
        expect(graph.get_edge('A', 'B')).toBeTruthy();
    });
});
```

### Browser Tests
```javascript
// Playwright/Puppeteer tests
test('WASM loads and renders in browser', async ({ page }) => {
    await page.goto('/examples/basic-flow');

    // Wait for WASM to load
    await page.waitForFunction(() => window.FlowRS !== undefined);

    // Interact with rendered graph
    await page.click('[data-node-id="node1"]');
    await page.dragAndDrop('[data-node-id="node1"]', { x: 200, y: 200 });

    // Verify state changes
    const nodePosition = await page.evaluate(() => {
        return window.graph.get_node('node1').position;
    });
    expect(nodePosition.x).toBeCloseTo(200, 1);
});
```

## Current Implementation Status

### ✅ Complete
- Basic wasm-bindgen setup and build configuration
- WasmGraph struct with minimal API surface

### 🚧 In Progress
- Node and edge manipulation methods
- Error handling and JS exception mapping

### ❓ Missing
- Layout algorithm bindings
- Renderer integration
- Event system implementation
- Performance optimization
- TypeScript definition generation

## Future Considerations

### Advanced Features
```rust
// Streaming large graphs to avoid memory spikes
#[wasm_bindgen]
impl WasmGraph {
    pub fn load_graph_stream(&mut self, reader: &mut dyn BufRead) -> Result<(), JsValue> {
        // Process graph data in chunks to avoid blocking event loop
    }
}

// WebWorker support for background processing
#[wasm_bindgen]
pub struct WasmWorkerGraph {
    // Thread-safe graph operations for WebWorker
}
```

### Performance Enhancements
- **Incremental serialization**: Only serialize changed data
- **Memory pools**: Reuse WASM memory allocations
- **SIMD optimizations**: Use WebAssembly SIMD for layout math
- **Shared memory**: Use SharedArrayBuffer where available

### Developer Experience
- **Hot reload**: Update WASM module without page refresh
- **Debug mode**: Additional validation and logging
- **Profiling hooks**: Integration with browser dev tools

## Migration Path

### Phase 1: Core API (Week 1)
- Implement WasmGraph with basic node/edge operations
- Set up error handling and TypeScript definitions
- Create simple browser example

### Phase 2: Renderer Integration (Week 2)
- Bind Canvas2D renderer to WASM
- Implement basic event handling (mouse, keyboard)
- Add viewport management

### Phase 3: Advanced Features (Week 3)
- Layout algorithm bindings
- Performance optimizations (batching, memory management)
- Comprehensive test suite

### Breaking Changes
- Current `greet()` function will be removed
- New error types may require updated JS error handling
- Bundle size will increase significantly from current stub

The WASM bindings design balances ease of use for JavaScript developers with the performance constraints of the WASM runtime, providing a clean API surface while maintaining the performance benefits of the Rust implementation.
