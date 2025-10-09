# File Size Refactoring Plan

## Overview
This document outlines the refactoring of large files (>300 lines) into smaller, more maintainable modules.

## Current File Size Analysis

### Files Requiring Refactoring

#### 1. `flow-core/src/collaboration.rs` - 685 lines
**Status**: 🚨 NEEDS REFACTORING
**Modules to Extract**:
- `operational_transform.rs` - OT algorithm implementation
- `collaborative_session.rs` - Session management
- `p2p_synchronization.rs` - WebRTC/P2P networking
- `change_tracking.rs` - Undo/redo across users
- `conflict_resolution.rs` - CRDT-style conflict resolution

#### 2. `flow-core/src/framework_abstractions.rs` - 600+ lines
**Status**: 🚨 NEEDS REFACTORING
**Modules to Extract**:
- `reactive_state.rs` - Reactive state traits and implementations
- `framework_elements.rs` - Element abstraction layer
- `framework_registry.rs` - Adapter management
- `framework_events.rs` - Event handling abstractions
- `optimizations.rs` - Performance optimizations

#### 3. `flow-core/src/plugins.rs` - 592 lines
**Status**: ⚠️ SHOULD REFACTOR
**Modules to Extract**:
- `plugin_manager.rs` - Plugin lifecycle management
- `plugin_registry.rs` - Plugin discovery and loading
- `message_passing.rs` - Inter-plugin communication
- `plugin_traits.rs` - Plugin interface definitions

#### 4. `flow-leptos/src/selection.rs` - 733 lines
**Status**: 🚨 NEEDS IMMEDIATE REFACTORING
**Modules to Extract**:
- `rectangle_selection.rs` - Rectangular selection logic
- `lasso_selection.rs` - Free-form selection
- `selection_state.rs` - State management
- `selection_ui.rs` - Visual feedback
- `selection_events.rs` - Event handling

#### 5. `flow-leptos/src/hooks.rs` - 716 lines
**Status**: 🚨 NEEDS REFACTORING
**Modules to Extract**:
- `graph_hooks.rs` - Graph state management hooks
- `viewport_hooks.rs` - Viewport/camera controls
- `interaction_hooks.rs` - User interaction handling
- `layout_hooks.rs` - Auto-layout integration

#### 6. `flow-leptos/src/signals.rs` - 705 lines
**Status**: 🚨 NEEDS REFACTORING
**Modules to Extract**:
- `flow_signals.rs` - Main Flow state signals
- `node_signals.rs` - Node-specific reactive state
- `edge_signals.rs` - Edge-specific reactive state
- `selection_signals.rs` - Selection state management
- `viewport_signals.rs` - Camera/viewport state

## Refactoring Strategy

### Phase 1: Core Infrastructure (Week 1-2)

#### 1.1 Extract Operational Transform (collaboration.rs)
```rust
// New file: flow-core/src/collaboration/operational_transform.rs
pub struct OperationalTransform { ... }
impl OperationalTransform {
    pub fn apply_operation(&mut self, operation: Operation) -> Result<(), OTError> { ... }
    pub fn transform_operation(&self, op1: &GraphOperation, op2: &GraphOperation) -> Result<GraphOperation, OTError> { ... }
    // ... OT-specific methods
}
```

#### 1.2 Extract Reactive State (framework_abstractions.rs)
```rust
// New file: flow-core/src/framework_abstractions/reactive_state.rs
pub trait ReactiveStateRead<T: Clone + 'static>: Send + Sync {
    fn get(&self) -> T;
}

pub trait ReactiveStateWrite<T: Clone + 'static>: Send + Sync {
    fn set(&mut self, value: T);
    fn update<F>(&mut self, f: F) where F: FnOnce(&mut T);
    fn subscribe<F>(&self, callback: F) where F: Fn(&T) + Send + Sync + 'static;
}

pub struct MockReactiveState<T> { ... }
```

#### 1.3 Extract Framework Registry
```rust
// New file: flow-core/src/framework_abstractions/registry.rs
#[derive(Clone, Debug)]
pub enum FrameworkAdapter {
    Leptos,
    Yew,
    Dioxus,
}

pub struct FrameworkRegistry {
    adapters: HashMap<String, FrameworkAdapter>,
    default_adapter: Option<String>,
}
```

### Phase 2: Leptos Integration (Week 3-4)

#### 2.1 Refactor Selection System
```rust
// New structure:
flow-leptos/src/selection/
├── mod.rs
├── rectangle.rs
├── lasso.rs
├── state.rs
├── ui.rs
└── events.rs
```

#### 2.2 Refactor Signal Management
```rust
// New structure:
flow-leptos/src/signals/
├── mod.rs
├── flow.rs
├── nodes.rs
├── edges.rs
├── selection.rs
└── viewport.rs
```

#### 2.3 Refactor Hooks
```rust
// New structure:
flow-leptos/src/hooks/
├── mod.rs
├── graph.rs
├── viewport.rs
├── interaction.rs
└── layout.rs
```

### Phase 3: Plugin System (Week 5)

#### 3.1 Extract Plugin Components
```rust
// New structure:
flow-core/src/plugins/
├── mod.rs
├── manager.rs
├── registry.rs
├── messaging.rs
└── traits.rs
```

## Module Interface Design

### Each Module Must Provide:
1. **Clear Public API**: Well-documented public functions and types
2. **Minimal Dependencies**: Avoid circular dependencies
3. **Comprehensive Tests**: Unit tests for all public APIs
4. **Documentation**: Module-level and function-level docs

### Example Module Structure:
```rust
// mod.rs
//! Module documentation
pub mod sub_module;

// Re-exports for convenience
pub use sub_module::{MainType, MainTrait};

// sub_module.rs
//! Sub-module documentation

/// Main type documentation
#[derive(Clone, Debug)]
pub struct MainType {
    // ...
}

impl MainType {
    /// Constructor documentation
    pub fn new() -> Self {
        // ...
    }

    /// Method documentation
    pub fn do_something(&mut self) -> Result<(), Error> {
        // ...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_type_creation() {
        let instance = MainType::new();
        assert!(instance.is_valid());
    }

    #[test]
    fn test_do_something() {
        let mut instance = MainType::new();
        assert!(instance.do_something().is_ok());
    }
}
```

## Quality Assurance

### Pre-Refactoring Checklist:
- [ ] All tests pass
- [ ] Public API documented
- [ ] No breaking changes to public interfaces
- [ ] Performance benchmarks established

### Post-Refactoring Checklist:
- [ ] All tests still pass
- [ ] No functionality regressions
- [ ] Performance maintained or improved
- [ ] All modules under 300 lines
- [ ] Clear module boundaries
- [ ] Comprehensive documentation

## Risk Mitigation

### Technical Risks:
- **API Breaking Changes**: Use feature flags during transition
- **Circular Dependencies**: Plan module hierarchy carefully
- **Performance Impact**: Profile before and after refactoring
- **Test Coverage Gaps**: Maintain 100% coverage during refactoring

### Process Risks:
- **Timeline Slippage**: Break into smaller, manageable chunks
- **Integration Issues**: Test integrations continuously
- **Knowledge Transfer**: Document design decisions

## Success Metrics

### Code Quality:
- ✅ All files under 300 lines
- ✅ Clear module separation of concerns
- ✅ Comprehensive documentation
- ✅ Zero circular dependencies

### Maintainability:
- ✅ Easier to understand individual components
- ✅ Faster compilation times
- ✅ Reduced merge conflicts
- ✅ Improved testability

### Developer Experience:
- ✅ Faster onboarding for new contributors
- ✅ Easier to locate and fix bugs
- ✅ Better code navigation
- ✅ Improved IDE support

## Timeline and Milestones

### Week 1: Planning and Infrastructure
- [ ] Complete module design documents
- [ ] Set up new directory structures
- [ ] Establish coding standards for modules
- [ ] Create refactoring checklists

### Week 2: Core Refactoring
- [ ] Refactor collaboration.rs into 5 modules
- [ ] Refactor framework_abstractions.rs into 4 modules
- [ ] Update all imports and dependencies
- [ ] Run full test suite after each module

### Week 3: Leptos Integration Refactoring
- [ ] Refactor selection.rs into 5 modules
- [ ] Refactor signals.rs into 5 modules
- [ ] Refactor hooks.rs into 4 modules
- [ ] Update Leptos-specific integrations

### Week 4: Plugin System and Testing
- [ ] Refactor plugins.rs into 4 modules
- [ ] Add integration tests for all modules
- [ ] Performance testing and optimization
- [ ] Documentation updates

### Week 5: Final Integration and Polish
- [ ] End-to-end testing of refactored codebase
- [ ] Performance benchmarking
- [ ] Documentation completion
- [ ] Final code review and cleanup

## Dependencies and Prerequisites

- [ ] All compilation errors resolved
- [ ] Core functionality stable
- [ ] Test suite passing (378+ tests)
- [ ] API contracts documented
- [ ] Performance baselines established