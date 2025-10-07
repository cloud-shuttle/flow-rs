# Critical Bugs and Fixes (Priority 1)

## Status: CRITICAL - Must Fix Immediately

### 1. Benchmark Compilation Errors
**Location**: `flow-core/benches/algorithms.rs`, `flow-core/benches/graph_operations.rs`

**Issue**: Benchmarks reference non-existent `leptos_flow_core` crate instead of `flow_rs_core`

**Error**:
```
use leptos_flow_core::{...}  // Should be flow_rs_core
```

**Impact**: Benchmarks completely broken, cannot run performance tests

**Fix Required**:
```rust
// Change all imports from:
use leptos_flow_core::{Graph, Node, Position, Rect};

// To:
use flow_rs_core::{Graph, Node, Position, Rect};
```

### 2. API Contract Test Compilation Error
**Location**: `flow-core/src/api_contracts/serialization.rs:54`

**Issue**: Type mismatch in Node constructor call

**Error**:
```rust
graph.add_node(Node::new("node2", Position::new(30.0, 40.0), 42)).unwrap();
//                                            Expected &str --^ found integer
```

**Impact**: API contract tests broken, cannot verify serialization contracts

**Fix Required**:
```rust
// Change to proper Node::new signature:
Node::new("node2", Position::new(30.0, 40.0), "node data").unwrap()
```

### 3. Benchmark Type Annotation Issues
**Location**: All benchmark files

**Issue**: Missing type annotations for closure parameters

**Error**:
```rust
|b, graph| {  // Missing type annotation
```

**Fix Required**:
```rust
|b, graph: &Graph<(), ()>| {  // Add explicit type
```

## Immediate Actions Required

1. **Fix benchmark imports** - Replace `leptos_flow_core` with `flow_rs_core`
2. **Fix API contract test** - Correct Node constructor call
3. **Fix benchmark type annotations** - Add missing type parameters
4. **Test compilation** - Ensure all tests pass after fixes

## Testing Verification

After fixes, run:
```bash
cargo test --workspace --all-targets --all-features
cargo bench --all-targets
```

## Priority: CRITICAL
## Estimated Time: 2-4 hours
## Risk Level: HIGH (affects CI/CD and performance testing)
