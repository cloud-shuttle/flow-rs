# File Size Refactoring Plan (Priority 2)

## Status: HIGH PRIORITY - Large Files Need Splitting

### Target: Keep all files under 300 lines (max 500 lines for complex modules)

### Critical Files Requiring Immediate Refactoring

#### 1. `flow-leptos/src/drag.rs` - **860 lines** ❌
**Issue**: Massive drag handling implementation
**Current Structure**:
- DragHandler impl (400+ lines)
- Mouse event handling (200+ lines)
- Position calculations (150+ lines)
- State management (100+ lines)

**Refactoring Plan**:
```
flow-leptos/src/drag/
├── mod.rs (50 lines) - Main exports
├── handler.rs (250 lines) - DragHandler impl
├── calculations.rs (200 lines) - Position/delta calculations
├── events.rs (150 lines) - Mouse event processing
├── state.rs (100 lines) - Drag state management
└── types.rs (80 lines) - Drag-specific types
```

#### 2. `flow-renderer/src/traits.rs` - **825 lines** ❌
**Issue**: All renderer traits in one massive file
**Current Structure**:
- BackgroundConfig trait (150 lines)
- Renderer trait (200 lines)
- Canvas2DRenderer impl (300 lines)
- WebGLRenderer impl (175 lines)

**Refactoring Plan**:
```
flow-renderer/src/
├── traits/
│   ├── mod.rs (50 lines)
│   ├── renderer.rs (150 lines) - Core Renderer trait
│   ├── background.rs (100 lines) - BackgroundConfig trait
│   └── canvas.rs (100 lines) - Canvas-specific traits
├── canvas2d/
│   ├── mod.rs (50 lines)
│   ├── renderer.rs (200 lines)
│   └── impls.rs (150 lines)
└── webgl/
    ├── mod.rs (50 lines)
    ├── renderer.rs (200 lines)
    └── impls.rs (150 lines)
```

#### 3. `flow-leptos/src/hooks.rs` - **716 lines** ❌
**Issue**: All Leptos hooks in single file
**Current Structure**:
- use_canvas hook (150 lines)
- use_mouse_events hook (120 lines)
- use_keyboard_events hook (100 lines)
- use_viewport hook (100 lines)
- use_nodes hook (120 lines)
- use_edges hook (126 lines)

**Refactoring Plan**:
```
flow-leptos/src/hooks/
├── mod.rs (50 lines)
├── canvas.rs (150 lines) - use_canvas hook
├── mouse.rs (120 lines) - use_mouse_events hook
├── keyboard.rs (100 lines) - use_keyboard_events hook
├── viewport.rs (100 lines) - use_viewport hook
├── nodes.rs (120 lines) - use_nodes hook
└── edges.rs (126 lines) - use_edges hook
```

#### 4. `flow-leptos/src/signals.rs` - **705 lines** ❌
**Issue**: All signal management logic
**Current Structure**:
- FlowSignalManager impl (300 lines)
- Signal creation logic (150 lines)
- State synchronization (150 lines)
- Update handling (105 lines)

**Refactoring Plan**:
```
flow-leptos/src/signals/
├── mod.rs (50 lines)
├── manager.rs (200 lines) - FlowSignalManager core
├── creation.rs (150 lines) - Signal creation logic
├── sync.rs (150 lines) - State synchronization
├── updates.rs (105 lines) - Update handling
└── types.rs (80 lines) - Signal types
```

### Medium Priority Files (300-500 lines)

#### 5. `examples/flow-simple/src/interaction_tests.rs` - **597 lines** ⚠️
**Refactoring Plan**: Split into multiple test files by feature

#### 6. `flow-renderer/src/performance.rs` - **596 lines** ⚠️
**Refactoring Plan**: Split performance monitoring into separate concerns

#### 7. `flow-core/src/layout/tests.rs` - **592 lines** ⚠️
**Refactoring Plan**: Split by layout algorithm type

### Implementation Strategy

1. **Phase 1**: Create directory structures and move files
2. **Phase 2**: Update all import statements
3. **Phase 3**: Update module declarations
4. **Phase 4**: Run tests to verify no breaking changes
5. **Phase 5**: Update documentation

### Benefits

- **Maintainability**: Easier to locate and modify specific functionality
- **Testing**: Smaller files are easier to test in isolation
- **Code Reviews**: Smaller diffs and focused changes
- **LLM Compatibility**: Files under 300 lines work better with AI assistants
- **Team Collaboration**: Reduced merge conflicts

### Timeline

- **Week 1**: Critical files (drag.rs, traits.rs)
- **Week 2**: High priority files (hooks.rs, signals.rs)
- **Week 3**: Medium priority files and testing
- **Week 4**: Documentation updates and final verification

## Priority: HIGH
## Estimated Time: 2-3 weeks
## Risk Level: MEDIUM (structural changes but preserves functionality)
