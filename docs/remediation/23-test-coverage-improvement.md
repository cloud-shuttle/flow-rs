# Test Coverage Improvement Plan (Priority 2)

## Status: INADEQUATE - Multiple test gaps identified

### Current Test Status

**Overall**: ⚠️ **Partial coverage** - Core functionality works but gaps exist
**API Contracts**: ✅ **Implemented** - 12 contract test modules exist
**Unit Tests**: ⚠️ **Incomplete** - Many functions untested
**Integration Tests**: ❌ **Missing** - No cross-crate integration tests
**E2E Tests**: ✅ **Implemented** - Playwright tests exist
**Property Tests**: ⚠️ **Limited** - Some proptest usage
**Benchmark Tests**: ❌ **Broken** - Compilation errors prevent running

### Critical Test Gaps

#### 1. Benchmark Tests (CRITICAL - Currently Broken)
**Issue**: `leptos_flow_core` imports don't exist
**Location**: `flow-core/benches/algorithms.rs`, `flow-core/benches/graph_operations.rs`
**Impact**: Cannot run performance tests
**Status**: ❌ **BROKEN**

#### 2. API Contract Tests (HIGH - Partially Broken)
**Issue**: Type mismatch in serialization contract
**Location**: `flow-core/src/api_contracts/serialization.rs`
**Impact**: Cannot verify serialization API contracts
**Status**: ❌ **BROKEN**

#### 3. Integration Test Coverage (MEDIUM)
**Issue**: No tests for cross-crate interactions
**Missing Tests**:
- `flow-core` ↔ `flow-renderer` integration
- `flow-core` ↔ `flow-leptos` integration
- `flow-renderer` ↔ `flow-leptos` integration
**Status**: ❌ **MISSING**

#### 4. Error Handling Tests (MEDIUM)
**Issue**: Limited error condition testing
**Missing Coverage**:
- Invalid node/edge operations
- Renderer failure scenarios
- WASM binding edge cases
**Status**: ⚠️ **INCOMPLETE**

#### 5. Property-Based Testing (LOW)
**Issue**: Limited proptest usage
**Current**: Basic graph operations
**Missing**: Complex graph algorithms, layout computations
**Status**: ⚠️ **MINIMAL**

### Test Coverage by Module

#### flow-core (Core Data Structures)
- ✅ **Graph operations**: Well tested
- ✅ **Spatial indexing**: Well tested
- ⚠️ **Layout algorithms**: Partial coverage
- ❌ **Auto-layout**: Minimal coverage
- ⚠️ **Serialization**: Broken contract tests

#### flow-renderer (Rendering)
- ⚠️ **Canvas2D**: Basic functionality tested
- ❌ **WebGL**: No tests
- ❌ **Performance**: No automated tests
- ❌ **Error handling**: No failure scenario tests

#### flow-leptos (Framework Integration)
- ⚠️ **Component rendering**: Basic tests
- ❌ **Event handling**: Minimal coverage
- ❌ **Signal management**: Untested
- ❌ **Hook functionality**: Untested
- ⚠️ **Drag operations**: Partial coverage

#### flow-wasm (WASM Bindings)
- ❌ **WASM compilation**: No tests
- ❌ **JS interop**: No tests
- ❌ **Memory management**: No tests

### Implementation Plan

#### Phase 1: Fix Broken Tests (Priority 1)
1. **Fix benchmark imports** (`leptos_flow_core` → `flow_rs_core`)
2. **Fix API contract serialization test**
3. **Fix benchmark type annotations**
4. **Verify all tests pass**

#### Phase 2: Integration Test Suite (Priority 2)
```rust
// New test structure
tests/
├── integration/
│   ├── core_renderer.rs     // flow-core ↔ flow-renderer
│   ├── core_leptos.rs       // flow-core ↔ flow-leptos
│   ├── renderer_leptos.rs   // flow-renderer ↔ flow-leptos
│   └── wasm_bindings.rs      // WASM integration tests
```

#### Phase 3: Error Handling Tests (Priority 3)
```rust
// Error condition testing
#[cfg(test)]
mod error_handling {
    mod invalid_operations;
    mod renderer_failures;
    mod wasm_edge_cases;
}
```

#### Phase 4: Property-Based Testing Expansion (Priority 4)
```rust
// Enhanced proptest coverage
proptest! {
    #[test]
    fn graph_operations_maintain_integrity(graph in arb_graph()) {
        // Complex graph operation properties
    }

    #[test]
    fn layout_algorithms_converge(nodes in arb_nodes()) {
        // Layout algorithm properties
    }
}
```

### Test Organization Improvements

#### Current Issues
- Large test files (>500 lines)
- Mixed unit/integration tests
- Poor test organization
- Limited documentation

#### Proposed Structure
```
flow-core/tests/
├── unit/           # Pure unit tests
├── integration/    # Cross-module tests
├── contracts/      # API contract verification
└── property/       # Property-based tests

flow-leptos/tests/
├── components/     # Component tests
├── hooks/          # Hook tests
├── integration/    # Framework integration
└── e2e/            # End-to-end tests
```

### Coverage Metrics Target

**Current**: ~70% coverage (estimated)
**Target**: 90%+ coverage

**Breakdown**:
- **Unit Tests**: 85% coverage
- **Integration Tests**: 95% coverage
- **API Contracts**: 100% coverage
- **Error Paths**: 90% coverage

### Timeline

- **Week 1**: Fix broken tests, establish baseline
- **Week 2**: Implement integration test suite
- **Week 3**: Add error handling tests
- **Week 4**: Expand property testing, reach coverage targets

## Priority: HIGH
## Estimated Time: 3-4 weeks
## Risk Level: MEDIUM (test improvements generally low risk)
