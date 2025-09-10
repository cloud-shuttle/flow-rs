# Testing Guide for Simple Flow Example

This document describes the testing infrastructure and how to run tests for the Simple Flow Example.

## Test Structure

The testing infrastructure is organized into several layers:

### 1. Unit Tests (`src/interaction_tests.rs`)
- **Purpose**: Test individual components and functions
- **Scope**: Interaction handlers, state management, utility functions
- **Framework**: `wasm-bindgen-test`
- **Location**: `src/interaction_tests.rs`

### 2. Integration Tests (`tests/integration_tests.rs`)
- **Purpose**: Test complete workflows and component interactions
- **Scope**: End-to-end rendering, graph operations, viewport management
- **Framework**: `wasm-bindgen-test`
- **Location**: `tests/integration_tests.rs`

### 3. Performance Tests (`tests/performance_tests.rs`)
- **Purpose**: Verify performance characteristics and benchmarks
- **Scope**: Rendering speed, memory usage, operation timing
- **Framework**: `wasm-bindgen-test`
- **Location**: `tests/performance_tests.rs`

### 4. Common Test Utilities (`tests/common/mod.rs`)
- **Purpose**: Shared test helpers and utilities
- **Scope**: Test data creation, assertion helpers, performance measurement
- **Location**: `tests/common/mod.rs`

## Running Tests

### Prerequisites

1. **Install wasm-pack**:
   ```bash
   cargo install wasm-pack
   ```

2. **Install Firefox** (for headless testing):
   - macOS: `brew install firefox`
   - Ubuntu: `sudo apt install firefox`
   - Windows: Download from Mozilla

### Running All Tests

Use the provided test runner script:

```bash
./run_tests.sh
```

This will:
- Build all test dependencies
- Run unit tests
- Run integration tests  
- Run performance tests
- Generate a test summary

### Running Specific Test Suites

#### Unit Tests Only
```bash
wasm-pack test --headless --firefox --lib
```

#### Integration Tests Only
```bash
wasm-pack test --headless --firefox --test integration_tests
```

#### Performance Tests Only
```bash
wasm-pack test --headless --firefox --test performance_tests
```

#### Interactive Testing (Browser)
```bash
wasm-pack test --firefox
```

## Test Categories

### Unit Tests

| Test | Purpose | Expected Behavior |
|------|---------|-------------------|
| `test_node_selection_on_click` | Node selection | Clicking on node selects it |
| `test_node_drag_functionality` | Node dragging | Dragging moves node to new position |
| `test_multiple_node_selection` | Multi-selection | Ctrl+click selects multiple nodes |
| `test_canvas_pan_functionality` | Canvas panning | Dragging empty space pans viewport |
| `test_node_visual_feedback` | Visual feedback | Selected nodes show highlight |
| `test_edge_creation_on_drag` | Edge creation | Dragging between nodes creates edge |

### Integration Tests

| Test | Purpose | Expected Behavior |
|------|---------|-------------------|
| `test_canvas2d_renderer_initialization` | Renderer setup | Canvas2D renderer initializes successfully |
| `test_graph_rendering` | Graph rendering | Graph with nodes and edges renders correctly |
| `test_background_rendering` | Background patterns | Different background patterns render |
| `test_viewport_operations` | Viewport management | Panning and zooming work correctly |
| `test_graph_operations` | Graph manipulation | Adding/removing nodes and edges works |
| `test_canvas_resize` | Canvas resizing | Canvas can be resized dynamically |
| `test_renderer_capabilities` | Capability reporting | Renderer reports correct capabilities |

### Performance Tests

| Test | Purpose | Performance Target |
|------|---------|-------------------|
| `test_small_graph_rendering_performance` | Small graph rendering | ≤ 100ms |
| `test_large_graph_rendering_performance` | Large graph rendering | ≤ 500ms |
| `test_background_rendering_performance` | Background rendering | ≤ 50ms |
| `test_canvas_resize_performance` | Canvas resize | ≤ 10ms |
| `test_viewport_operations_performance` | Viewport operations | ≤ 10ms for 100 ops |
| `test_graph_operations_performance` | Graph operations | ≤ 50ms for 100 nodes |
| `test_memory_usage` | Memory consumption | ≤ 50MB |
| `test_renderer_initialization_performance` | Renderer init | ≤ 100ms |
| `test_concurrent_operations` | Multiple operations | ≤ 200ms |

## Test Data

### Test Graphs

- **Simple Graph**: 3 nodes, 2 edges (for basic testing)
- **Complex Graph**: 25 nodes, 40 edges (for performance testing)

### Test Viewports

- **Default Viewport**: Standard settings
- **Custom Viewport**: Configurable position, size, zoom

### Test Backgrounds

- **Dots Pattern**: Default dotted background
- **Custom Patterns**: Configurable colors, sizes, opacity

## Continuous Integration

The test suite is designed to run in CI environments:

```yaml
# Example GitHub Actions workflow
- name: Run WASM Tests
  run: |
    cargo install wasm-pack
    cd examples/simple-flow
    ./run_tests.sh
```

## Debugging Tests

### Common Issues

1. **Firefox not found**: Install Firefox and ensure it's in PATH
2. **Memory issues**: Increase Node.js memory limit: `NODE_OPTIONS="--max-old-space-size=4096"`
3. **Timeout issues**: Increase test timeout in `wasm-bindgen-test` configuration

### Debug Mode

Run tests with debug output:

```bash
RUST_LOG=debug wasm-pack test --headless --firefox
```

### Interactive Debugging

Run tests in browser for interactive debugging:

```bash
wasm-pack test --firefox
```

## Adding New Tests

### Unit Test Template

```rust
#[wasm_bindgen_test]
fn test_new_functionality() {
    // Arrange
    let setup = create_test_setup();
    
    // Act
    let result = perform_operation(&setup);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

### Integration Test Template

```rust
#[wasm_bindgen_test]
fn test_end_to_end_workflow() {
    let canvas = create_test_canvas();
    let mut renderer = Canvas2DRenderer::new(&canvas).unwrap();
    let graph = create_test_graph();
    let viewport = create_test_viewport();
    
    let result = renderer.render_graph(&graph, &viewport);
    assert!(result.is_ok());
}
```

### Performance Test Template

```rust
#[wasm_bindgen_test]
fn test_operation_performance() {
    let setup = create_test_setup();
    
    let (_, duration) = measure_execution_time(|| {
        perform_operation(&setup);
    });
    
    assert!(duration <= max_time_ms);
}
```

## Test Coverage

The test suite aims for comprehensive coverage:

- ✅ **Functionality**: All public APIs tested
- ✅ **Error Handling**: Error cases and edge cases covered
- ✅ **Performance**: Performance benchmarks established
- ✅ **Integration**: End-to-end workflows tested
- ✅ **Browser Compatibility**: WASM compatibility verified

## Contributing

When adding new features:

1. Write tests first (TDD approach)
2. Ensure all tests pass
3. Add performance benchmarks if applicable
4. Update this documentation
5. Run the full test suite before submitting

## Troubleshooting

### Test Failures

1. **Check browser compatibility**: Ensure Firefox is up to date
2. **Verify dependencies**: Run `cargo check` to ensure compilation
3. **Check memory usage**: Monitor memory consumption during tests
4. **Review test logs**: Use debug mode for detailed output

### Performance Issues

1. **Profile test execution**: Use browser dev tools
2. **Check memory leaks**: Monitor memory usage over time
3. **Optimize test data**: Use appropriate test data sizes
4. **Review performance targets**: Adjust if hardware limitations exist
