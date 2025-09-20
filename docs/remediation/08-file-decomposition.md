# P2: File Decomposition - Breaking Down Large Files

## Issue Summary
Multiple files exceed 300 lines, making them difficult to test, review, and understand for both humans and LLMs.

## Files Requiring Decomposition

### Critical (1000+ lines)
| File | Lines | Main Issues |
|------|-------|-------------|
| flow-core/src/layout.rs | 1840 | Multiple algorithms + tests + config |
| flow-core/src/graph.rs | 1675 | God object with mixed concerns |
| flow-core/src/proptest.rs | 1509 | Property tests scattered across domains |
| flow-core/src/spatial.rs | 1351 | Grid implementation + tests + benchmarks |
| flow-core/src/selection.rs | 1253 | Multiple selection modes + group management |

### High Priority (500-1000 lines)
| File | Lines | Main Issues |
|------|-------|-------------|
| flow-core/src/groups.rs | 1020 | Group management + layout integration |
| flow-core/src/types.rs | 1007 | All type definitions in single file |
| flow-core/src/auto_layout.rs | 888 | Multiple layout strategies |
| flow-leptos/src/drag.rs | 859 | Drag handling + selection + validation |
| flow-renderer/src/traits.rs | 825 | All renderer abstractions |

## Decomposition Strategy

### Phase 1: layout.rs → layout/ module (Priority: High)

```
flow-core/src/layout/
├── mod.rs                    # Public API + LayoutAlgorithm trait
├── algorithms/
│   ├── mod.rs
│   ├── circular.rs          # CircularLayout implementation
│   ├── force_directed.rs    # ForceDirectedLayout implementation
│   └── grid.rs              # GridLayout implementation
├── config.rs                # Layout configuration types
└── tests.rs                 # Integration tests
```

**Benefits**: Each algorithm <200 lines, easier to implement missing functions

### Phase 2: graph.rs → graph/ module (Priority: High)

```
flow-core/src/graph/
├── mod.rs                   # Graph struct + core methods
├── node.rs                  # Node, NodeBuilder
├── edge.rs                  # Edge, EdgeBuilder
├── traversal.rs             # DFS, BFS, cycle detection
├── validation.rs            # Schema validation, constraints
└── handle_integration.rs    # Handle management methods
```

**Benefits**: Separates data model from algorithms, reduces God object

### Phase 3: types.rs → types/ module (Priority: Medium)

```
flow-core/src/types/
├── mod.rs                   # Re-exports
├── identifiers.rs           # NodeId, EdgeId, GroupId
├── geometry.rs              # Position, Size, Rect, Viewport
├── builders.rs              # Builder pattern implementations
└── serde_impls.rs           # Serialization (feature-gated)
```

**Benefits**: Logical grouping, easier to find specific type definitions

### Phase 4: Component-Specific Decomposition

#### spatial.rs → spatial/ module
```
flow-core/src/spatial/
├── mod.rs                   # SpatialIndex trait
├── grid.rs                  # Grid-based implementation
├── rtree.rs                 # Future R-tree implementation
└── benchmarks.rs            # Performance tests
```

#### selection.rs → selection/ module
```
flow-core/src/selection/
├── mod.rs                   # SelectionManager
├── modes.rs                 # Single, Multi, Rectangle modes
├── group_integration.rs     # Group-aware selection
└── events.rs                # Selection event handling
```

## Implementation Guidelines

### Module Structure Template
```rust
// mod.rs - Public API only
pub use self::implementation::*;
pub use self::config::*;

mod implementation;
mod config;
#[cfg(test)]
mod tests;
```

### File Size Targets
- **Implementation files**: <200 lines
- **Test files**: <150 lines
- **Config/types files**: <100 lines
- **mod.rs files**: <50 lines (re-exports only)

### Migration Process
1. **Create module directory**
2. **Extract logical components** (one at a time)
3. **Update imports** in dependent modules
4. **Verify tests pass** after each extraction
5. **Update documentation** links

## Testing Strategy

### Before Decomposition
```bash
# Establish baseline
cargo test --lib
cargo clippy --all -- -D warnings
cargo doc --no-deps
```

### During Each Module Split
```bash
# After each file extraction
cargo check --all
cargo test $MODULE_NAME
cargo clippy --all -- -D warnings
```

### After Completion
```bash
# Verify no functionality lost
cargo test --all-features
cargo bench                # Performance regression check
wc -l src/**/*.rs | sort -n # Verify no files >300 lines
```

## Specific Implementation Plans

### layout.rs Decomposition (2-3 days)

**Step 1**: Extract CircularLayout to `layout/algorithms/circular.rs`
- Move struct definition and implementation
- Update imports in layout.rs
- Test compilation

**Step 2**: Extract ForceDirectedLayout to `layout/algorithms/force_directed.rs`
- Include physics simulation code
- Separate configuration from algorithm

**Step 3**: Create `layout/config.rs` for shared configuration types
- LayoutConfig, AlgorithmConfig
- Validation methods

**Step 4**: Create `layout/mod.rs` with clean public API
- Re-export all public types
- Document usage examples
- Hide implementation details

### graph.rs Decomposition (3-4 days)

**Step 1**: Extract traversal algorithms to `graph/traversal.rs`
- DFS, BFS, topological sort
- Cycle detection logic
- Keep as pure functions where possible

**Step 2**: Extract Node/Edge builders to `graph/node.rs` and `graph/edge.rs`
- Include validation logic
- Builder pattern implementations

**Step 3**: Create `graph/validation.rs` for constraint checking
- Schema validation
- Relationship constraints
- Data consistency checks

## Success Criteria
- [ ] No source files >300 lines (excluding generated code)
- [ ] Module structure follows Rust conventions
- [ ] All existing tests pass without modification
- [ ] No performance regression (>5% slowdown)
- [ ] Clippy warnings do not increase
- [ ] Documentation builds without broken links
- [ ] Each module has single responsibility
- [ ] Public API surface unchanged

## Risk Mitigation
- **Incremental approach**: One module at a time
- **Automated testing**: CI runs on every change
- **Rollback plan**: Git branch per module decomposition
- **API compatibility**: Preserve all public exports in mod.rs files

## Dependencies
- Should complete after [01-dependency-vulnerabilities.md](01-dependency-vulnerabilities.md)
- Can run in parallel with [04-stub-implementations.md](04-stub-implementations.md)
