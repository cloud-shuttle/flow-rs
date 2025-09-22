# Leptos 0.8.9 Migration Design Document

## Executive Summary

This document outlines the comprehensive plan for migrating Flow-RS from Leptos 0.6.15 to Leptos 0.8.9. The migration involves significant API changes and requires careful planning to maintain functionality while leveraging new features and improvements.

## Current State Analysis

### Current Leptos Usage in Flow-RS

**Package**: `flow-leptos` (Leptos integration layer)

**Key Leptos Features Used**:
- **Components**: 3 main components (`FlowEditor`, `MiniMap`, `FlowControls`)
- **Signals**: Extensive use of reactive signals (`RwSignal`, `ReadSignal`, `WriteSignal`)
- **Effects**: `create_effect` for reactive updates and side effects
- **Hooks**: Custom hooks for canvas interactions and graph operations
- **Macros**: `#[component]` and `#[prop]` for component definition

**Usage Statistics**:
- **82 Leptos API calls** across 12 files
- **19 direct `leptos::` imports** across 14 files
- **3 main components** with complex reactive state management
- **Multiple custom hooks** for mouse, keyboard, and drag interactions

### Current Architecture

```
flow-leptos/
├── components/          # Leptos components
│   ├── FlowEditor      # Main editor component
│   ├── MiniMap         # Minimap component  
│   └── FlowControls    # Control panel component
├── hooks/              # Custom Leptos hooks
├── signals/            # Reactive state management
├── events/             # Event handling
├── drag/               # Drag and drop functionality
└── interactions/       # User interaction handling
```

## Migration Benefits

### New Features in Leptos 0.8.9

1. **Enhanced Security**
   - Live reload over secure WebSockets (`wss`) for HTTPS
   - Improved event listener context preservation

2. **Better Navigation**
   - Smarter history management (prevents duplicate entries)
   - Improved lazy route navigation

3. **Improved Error Handling**
   - Better server function error responses with content-type headers
   - Enhanced debugging capabilities

4. **Performance Improvements**
   - Standardized function implementations
   - Better attribute management (prevents conflicts)

5. **Developer Experience**
   - Enhanced development server features
   - Better error messages and debugging tools

## Migration Challenges

### Breaking Changes Analysis

Based on research and analysis, the main breaking changes between 0.6.x and 0.8.x include:

1. **Component Macro Changes**
   - `#[component]` macro signature updates
   - `#[prop]` attribute changes
   - Component prop handling modifications

2. **Signal API Changes**
   - Signal creation and management updates
   - Effect system modifications
   - Reactive context changes

3. **Import Path Changes**
   - Some Leptos types moved to different modules
   - Re-export structure modifications

4. **Type System Updates**
   - Generic parameter handling changes
   - Lifetime management updates
   - Trait bound modifications

### Risk Assessment

**High Risk Areas**:
- Component definitions and prop handling
- Signal management and reactive effects
- Custom hooks implementation
- Event handling and context preservation

**Medium Risk Areas**:
- Import statements and re-exports
- Type annotations and generic parameters
- Test implementations

**Low Risk Areas**:
- Core business logic (flow-core, flow-renderer)
- WASM bindings (flow-wasm)
- Build configuration

## Migration Strategy

### Phase 1: Preparation and Analysis (1-2 days)

1. **Create Migration Branch**
   ```bash
   git checkout -b feature/leptos-0.8-migration
   ```

2. **Update Dependencies**
   ```toml
   # Cargo.toml
   [workspace.dependencies]
   leptos = "0.8.9"
   leptos_dom = "0.8.9"
   leptos_reactive = "0.8.9"
   ```

3. **Initial Compilation Test**
   - Update dependencies
   - Run `cargo check` to identify immediate compilation errors
   - Document all breaking changes

### Phase 2: Core API Updates (2-3 days)

1. **Update Import Statements**
   - Fix import paths for moved types
   - Update re-export statements in `lib.rs`
   - Resolve naming conflicts

2. **Component Macro Updates**
   - Update `#[component]` macro usage
   - Fix `#[prop]` attribute syntax
   - Update component prop handling

3. **Signal System Updates**
   - Update signal creation and management
   - Fix effect system usage
   - Update reactive context handling

### Phase 3: Custom Hooks and Effects (2-3 days)

1. **Hook Implementation Updates**
   - Update `use_canvas_mouse` hook
   - Fix `use_graph_operations` hook
   - Update drag and interaction hooks

2. **Effect System Updates**
   - Update `create_effect` usage
   - Fix reactive dependencies
   - Update side effect handling

### Phase 4: Component Updates (2-3 days)

1. **FlowEditor Component**
   - Update component definition
   - Fix prop handling
   - Update reactive state management

2. **MiniMap Component**
   - Update component structure
   - Fix signal usage
   - Update rendering logic

3. **FlowControls Component**
   - Update control panel logic
   - Fix event handling
   - Update state management

### Phase 5: Testing and Validation (2-3 days)

1. **Unit Tests**
   - Update test implementations
   - Fix test-specific Leptos usage
   - Validate test coverage

2. **Integration Tests**
   - Test component interactions
   - Validate reactive behavior
   - Test event handling

3. **E2E Tests**
   - Update Playwright tests
   - Validate user interactions
   - Test complete workflows

## Implementation Plan

### Detailed Migration Steps

#### Step 1: Dependency Update
```toml
# Cargo.toml
[workspace.dependencies]
leptos = "0.8.9"
leptos_dom = "0.8.9" 
leptos_reactive = "0.8.9"
```

#### Step 2: Import Updates
```rust
// Before (0.6.15)
use leptos::{
    component, create_effect, create_memo, create_resource, create_signal, 
    Children, ChildrenFn, IntoView, Memo, ReadSignal, RwSignal, Signal, WriteSignal,
};

// After (0.8.9) - Update based on actual API changes
use leptos::{
    component, create_effect, create_memo, create_resource, create_signal,
    Children, ChildrenFn, IntoView, Memo, ReadSignal, RwSignal, Signal, WriteSignal,
};
```

#### Step 3: Component Updates
```rust
// Before
#[component]
pub fn FlowEditor<N, E>(
    graph: RwSignal<Graph<N, E>>,
    #[prop(default = 800)] width: u32,
    #[prop(default = 600)] height: u32,
    #[prop(optional)] _renderer_type: Option<RendererType>,
) -> impl IntoView

// After - Update based on actual 0.8.9 API
#[component]
pub fn FlowEditor<N, E>(
    graph: RwSignal<Graph<N, E>>,
    #[prop(default = 800)] width: u32,
    #[prop(default = 600)] height: u32,
    #[prop(optional)] _renderer_type: Option<RendererType>,
) -> impl IntoView
```

#### Step 4: Signal Updates
```rust
// Before
let viewport = create_rw_signal(ViewportState::default());
let flow_state = create_rw_signal(FlowState::default());

// After - Update based on actual 0.8.9 API
let viewport = create_rw_signal(ViewportState::default());
let flow_state = create_rw_signal(FlowState::default());
```

### Testing Strategy

#### Unit Tests
- Update all Leptos-specific test code
- Validate signal behavior
- Test component prop handling

#### Integration Tests
- Test component interactions
- Validate reactive updates
- Test event propagation

#### E2E Tests
- Update Playwright tests for new behavior
- Test complete user workflows
- Validate performance characteristics

## Risk Mitigation

### Rollback Plan
1. **Branch Strategy**: Keep original branch intact
2. **Incremental Commits**: Small, focused commits for easy rollback
3. **Feature Flags**: Use feature flags to toggle between versions
4. **Testing**: Comprehensive testing at each phase

### Quality Assurance
1. **Code Review**: Peer review of all changes
2. **Automated Testing**: CI/CD pipeline validation
3. **Performance Testing**: Benchmark before/after performance
4. **User Testing**: Validate user experience

## Timeline and Resources

### Estimated Timeline: 10-14 days

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1: Preparation | 1-2 days | None |
| Phase 2: Core API | 2-3 days | Phase 1 complete |
| Phase 3: Hooks/Effects | 2-3 days | Phase 2 complete |
| Phase 4: Components | 2-3 days | Phase 3 complete |
| Phase 5: Testing | 2-3 days | Phase 4 complete |

### Resource Requirements
- **Developer**: 1 senior Rust developer
- **Testing**: Access to multiple browsers and devices
- **Review**: Code review from Leptos experts (if available)

## Success Criteria

### Technical Success
- [ ] All packages compile without errors
- [ ] All tests pass (unit, integration, E2E)
- [ ] Performance maintained or improved
- [ ] No regression in functionality

### Quality Success
- [ ] Code quality maintained
- [ ] Documentation updated
- [ ] Examples working
- [ ] CI/CD pipeline green

### User Success
- [ ] No breaking changes for end users
- [ ] Improved development experience
- [ ] Better error messages
- [ ] Enhanced debugging capabilities

## Decision Matrix

### Proceed with Migration
**Pros**:
- Access to latest features and improvements
- Better security and performance
- Enhanced developer experience
- Future-proofing the codebase

**Cons**:
- Significant development effort (10-14 days)
- Risk of introducing bugs
- Potential breaking changes for users
- Opportunity cost of other features

### Defer Migration
**Pros**:
- No immediate risk
- Focus on other priorities
- Current version is stable and working
- Lower resource commitment

**Cons**:
- Missing out on improvements
- Technical debt accumulation
- Future migration will be harder
- Security and performance gaps

## Recommendation

### **RECOMMENDATION: PROCEED WITH MIGRATION**

**Rationale**:
1. **Current State**: The project is in excellent condition with all tests passing
2. **Migration Complexity**: Moderate complexity, well-defined scope
3. **Benefits**: Significant improvements in security, performance, and DX
4. **Risk Management**: Comprehensive plan with rollback strategy
5. **Timing**: Good time to upgrade while project is stable

**Next Steps**:
1. **Approve Migration Plan**: Get stakeholder approval
2. **Create Migration Branch**: Start Phase 1
3. **Begin Implementation**: Follow phased approach
4. **Monitor Progress**: Daily standups and progress tracking

## Conclusion

The Leptos 0.8.9 migration is a significant but manageable undertaking that will provide substantial benefits to the Flow-RS project. With proper planning, risk mitigation, and execution, this migration will enhance the project's capabilities while maintaining its stability and quality.

The comprehensive plan outlined in this document provides a clear roadmap for successful migration, with appropriate safeguards and quality assurance measures to ensure a smooth transition to the latest Leptos version.
