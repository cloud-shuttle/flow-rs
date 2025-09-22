# P0: Panic Audit and Error Handling

## Issue Summary
Library code contains numerous `unwrap()`, `expect()`, and `panic!()` calls that can crash the application. This violates the principle that libraries should never panic and makes the codebase unsuitable for production use.

## Critical Problems

### Library Code Panics
- **Current**: Many `unwrap()` calls throughout the codebase
- **Risk**: Application crashes from library panics
- **Impact**: Unreliable production behavior

### Missing Error Handling
- **Current**: Functions return `Result<T>` but callers use `unwrap()`
- **Required**: Proper error propagation and handling
- **Impact**: Silent failures and crashes

## Implementation Plan

### Phase 1: Audit and Catalog (1-2 days)
**Goal**: Identify all panic points in library code

```bash
# Find all unwrap() calls
grep -r "unwrap()" flow-core/src/ flow-renderer/src/ flow-wasm/src/

# Find all expect() calls  
grep -r "expect(" flow-core/src/ flow-renderer/src/ flow-wasm/src/

# Find all panic!() calls
grep -r "panic!" flow-core/src/ flow-renderer/src/ flow-wasm/src/
```

### Phase 2: Replace with Proper Error Handling (3-5 days)
**Goal**: Replace all panics with proper error handling

#### Example Transformations

**Before (Panic-prone)**:
```rust
fn get_node(&self, id: &NodeId) -> &Node {
    self.nodes.get(id).unwrap() // PANIC if node not found
}
```

**After (Safe)**:
```rust
fn get_node(&self, id: &NodeId) -> Result<&Node, FlowError> {
    self.nodes.get(id)
        .ok_or_else(|| FlowError::node_not_found(id.as_str()))
}
```

**Before (Panic-prone)**:
```rust
fn parse_config(config_str: &str) -> Config {
    serde_json::from_str(config_str).expect("Invalid config") // PANIC on parse error
}
```

**After (Safe)**:
```rust
fn parse_config(config_str: &str) -> Result<Config, FlowError> {
    serde_json::from_str(config_str)
        .map_err(|e| FlowError::config_parse_error(e.to_string()))
}
```

### Phase 3: Update Callers (2-3 days)
**Goal**: Update all callers to handle errors properly

#### Example Caller Updates

**Before (Panic-prone)**:
```rust
let node = graph.get_node(&node_id).unwrap(); // PANIC if not found
```

**After (Safe)**:
```rust
let node = graph.get_node(&node_id)?; // Propagate error
// or
let node = match graph.get_node(&node_id) {
    Ok(node) => node,
    Err(e) => {
        log::warn!("Node not found: {}", e);
        return Err(e);
    }
};
```

## Testing Requirements

### Panic Detection Tests
```rust
#[test]
fn no_panics_on_invalid_input() {
    let mut graph = Graph::new();
    
    // These should not panic
    assert!(graph.get_node(&NodeId::new("nonexistent")).is_err());
    assert!(graph.remove_node(&NodeId::new("nonexistent")).is_err());
    assert!(graph.get_edge(&EdgeId::new("nonexistent")).is_err());
}

#[test]
fn no_panics_on_malformed_data() {
    let mut graph = Graph::new();
    
    // These should not panic
    assert!(graph.add_node(Node::builder(NodeId::new("")).build()).is_err());
    assert!(graph.add_edge(Edge::builder().id(EdgeId::new("")).build()).is_err());
}
```

### Error Propagation Tests
```rust
#[test]
fn errors_propagate_correctly() {
    let mut graph = Graph::new();
    let result = graph.get_node(&NodeId::new("nonexistent"));
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FlowError::NodeNotFound(_)));
}
```

## Risk Assessment

**High Risk**: Breaking changes to public API
- **Mitigation**: Use `Result<T>` return types, maintain backward compatibility where possible
- **Testing**: Comprehensive integration tests

**Medium Risk**: Performance impact of error handling
- **Mitigation**: Use `Result<T>` which has zero-cost abstractions
- **Testing**: Benchmark critical paths

**Low Risk**: Increased code complexity
- **Mitigation**: Clear error handling patterns, good documentation
- **Testing**: Code review, documentation tests

## Success Criteria

- [ ] Zero `unwrap()` calls in library code
- [ ] Zero `expect()` calls in library code  
- [ ] Zero `panic!()` calls in library code
- [ ] All public APIs return `Result<T, FlowError>`
- [ ] Comprehensive error handling tests
- [ ] Error messages are user-friendly and actionable
- [ ] Performance benchmarks show no regression
- [ ] All existing functionality works with error handling

## Dependencies

- Requires completion of [01-dependency-vulnerabilities.md](01-dependency-vulnerabilities.md)
- Blocks [03-error-handling.md](03-error-handling.md)
- Enables [06-integration-tests.md](06-integration-tests.md)

## Implementation Checklist

### Phase 1: Audit
- [ ] Scan all library code for panic points
- [ ] Categorize panics by severity and impact
- [ ] Create prioritized list of fixes

### Phase 2: Core Library (flow-core)
- [ ] Fix all panics in `graph.rs`
- [ ] Fix all panics in `layout/` algorithms
- [ ] Fix all panics in `spatial/` indexing
- [ ] Fix all panics in `selection/` management
- [ ] Fix all panics in `groups/` management
- [ ] Fix all panics in `handle/` management

### Phase 3: Renderer (flow-renderer)
- [ ] Fix all panics in Canvas2D renderer
- [ ] Fix all panics in renderer traits
- [ ] Fix all panics in performance monitoring

### Phase 4: WASM Bindings (flow-wasm)
- [ ] Fix all panics in JavaScript interop
- [ ] Fix all panics in WASM-specific code
- [ ] Ensure proper error propagation to JavaScript

### Phase 5: Testing & Validation
- [ ] Add panic detection tests
- [ ] Add error propagation tests
- [ ] Run comprehensive test suite
- [ ] Performance benchmarking
- [ ] Integration testing

## Notes

- **Library Principle**: Libraries should never panic - only applications should panic
- **Error Types**: Use `FlowError` enum for all library errors
- **Logging**: Add appropriate logging for error conditions
- **Documentation**: Update all API documentation to reflect error handling
