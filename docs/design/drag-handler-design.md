# Drag Handler Design Document

## Overview
**Component**: DragHandler (`flow-leptos/src/drag.rs` - 860 lines)
**Status**: Needs refactoring into smaller modules
**Complexity**: High - Handles mouse events, position calculations, state management

## Current Architecture Problems

### Issues
1. **Single massive file** (860 lines) - violates 300-line limit
2. **Mixed responsibilities** - event handling, calculations, state management
3. **Tight coupling** - direct DOM manipulation mixed with business logic
4. **Poor testability** - large functions hard to unit test

### Current Structure Analysis
```rust
pub struct DragHandler {        // State management
    // ... 20+ fields
}

impl DragHandler {              // 400+ lines
    // Mouse event handlers (200+ lines)
    // Position calculations (150+ lines)
    // State updates (100+ lines)
}
```

## Proposed Refactored Architecture

### Directory Structure
```
flow-leptos/src/drag/
├── mod.rs              (50 lines) - Main exports and types
├── handler.rs          (200 lines) - Core DragHandler struct and lifecycle
├── events.rs           (150 lines) - Mouse/touch event processing
├── calculations.rs     (120 lines) - Position and delta calculations
├── state.rs            (80 lines) - Drag state management
├── constraints.rs      (60 lines) - Movement constraints and boundaries
└── types.rs            (40 lines) - Drag-specific types and enums
```

### Component Breakdown

#### 1. Core Handler (`handler.rs`)
**Responsibility**: Main DragHandler struct and public API

**Key Methods**:
```rust
impl DragHandler {
    pub fn new(config: DragConfig) -> Self
    pub fn start_drag(&mut self, node_id: NodeId, start_pos: Position) -> Result<(), DragError>
    pub fn update_drag(&mut self, current_pos: Position) -> Result<(), DragError>
    pub fn end_drag(&mut self) -> Result<DragResult, DragError>
    pub fn cancel_drag(&mut self) -> Result<(), DragError>
}
```

**State Management**:
```rust
pub struct DragHandler {
    state: DragState,
    config: DragConfig,
    constraints: DragConstraints,
    calculator: PositionCalculator,
}
```

#### 2. Event Processing (`events.rs`)
**Responsibility**: Translate DOM events to drag operations

**Key Functions**:
```rust
pub fn process_mouse_down(event: &MouseEvent, position: Position) -> Result<DragStart, EventError>
pub fn process_mouse_move(event: &MouseEvent, position: Position) -> DragUpdate
pub fn process_mouse_up(event: &MouseEvent) -> DragEnd
pub fn process_touch_events(events: &[TouchEvent]) -> Result<DragOperation, EventError>
```

#### 3. Position Calculations (`calculations.rs`)
**Responsibility**: Mathematical operations for drag positioning

**Key Functions**:
```rust
pub fn calculate_drag_delta(start: Position, current: Position) -> Position
pub fn apply_snap_to_grid(position: Position, grid_size: f64) -> Position
pub fn constrain_to_bounds(position: Position, bounds: Rect) -> Position
pub fn calculate_velocity(positions: &[Position], timestamps: &[f64]) -> Velocity
```

#### 4. State Management (`state.rs`)
**Responsibility**: Internal drag state tracking

**Key Types**:
```rust
#[derive(Debug, Clone)]
pub struct DragState {
    pub node_id: NodeId,
    pub start_position: Position,
    pub current_position: Position,
    pub velocity: Velocity,
    pub is_active: bool,
    pub snap_enabled: bool,
}

pub struct DragHistory {
    positions: Vec<Position>,
    timestamps: Vec<f64>,
    max_history: usize,
}
```

#### 5. Constraints (`constraints.rs`)
**Responsibility**: Movement limitations and validation

**Key Types**:
```rust
pub struct DragConstraints {
    pub bounds: Option<Rect>,
    pub snap_grid: Option<GridConfig>,
    pub axis_lock: Option<Axis>,
    pub min_distance: f64,
    pub max_distance: Option<f64>,
}
```

## Interface Design

### Public API
```rust
// Main drag handler interface
pub struct DragHandler;

impl DragHandler {
    pub fn new(config: DragConfig) -> Self;
    pub fn handle_event(&mut self, event: DragEvent) -> Result<DragResponse, DragError>;
    pub fn get_drag_state(&self) -> Option<&DragState>;
}

// Event types
pub enum DragEvent {
    Start { node_id: NodeId, position: Position },
    Move { position: Position },
    End,
    Cancel,
}

// Response types
pub enum DragResponse {
    Started { node_id: NodeId },
    Updated { node_id: NodeId, position: Position },
    Ended { node_id: NodeId, final_position: Position },
    Cancelled { node_id: NodeId },
}
```

### Configuration
```rust
#[derive(Debug, Clone)]
pub struct DragConfig {
    pub enable_snap: bool,
    pub grid_size: f64,
    pub constraints: DragConstraints,
    pub smooth_dragging: bool,
    pub momentum: bool,
}
```

## Error Handling

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum DragError {
    #[error("Invalid drag state transition")]
    InvalidState,

    #[error("Position calculation failed")]
    CalculationError,

    #[error("Constraint violation: {constraint}")]
    ConstraintViolation { constraint: String },

    #[error("Event processing failed")]
    EventError,
}
```

## Testing Strategy

### Unit Tests (per module)
- **handler.rs**: State transitions and public API
- **events.rs**: Event processing and validation
- **calculations.rs**: Mathematical operations
- **state.rs**: State management logic
- **constraints.rs**: Constraint validation

### Integration Tests
- Full drag workflow from start to finish
- Event sequence validation
- Constraint interactions

### Property Tests
```rust
proptest! {
    #[test]
    fn drag_positions_are_consistent(start in position(), moves in vec(position(), 1..10)) {
        // Property: drag position calculations maintain consistency
    }

    #[test]
    fn constraints_always_respected(bounds in rect(), positions in vec(position(), 1..10)) {
        // Property: constrained positions never violate bounds
    }
}
```

## Performance Considerations

### Optimizations
1. **Lazy calculations** - Only compute when needed
2. **Minimal allocations** - Reuse buffers where possible
3. **Early returns** - Skip unnecessary work
4. **SIMD operations** - For bulk position calculations

### Benchmarks
```rust
// Benchmark drag performance
#[bench]
fn bench_drag_calculation(b: &mut Bencher) {
    // Measure drag position calculation performance
}

// Benchmark constraint validation
#[bench]
fn bench_constraint_checking(b: &mut Bencher) {
    // Measure constraint validation performance
}
```

## Migration Plan

### Phase 1: Extract Types and Constants
1. Create `types.rs` and `mod.rs`
2. Move type definitions and constants
3. Update imports

### Phase 2: Extract Calculations
1. Create `calculations.rs`
2. Move mathematical functions
3. Update references

### Phase 3: Extract State Management
1. Create `state.rs`
2. Move state tracking logic
3. Update state handling

### Phase 4: Extract Event Processing
1. Create `events.rs`
2. Move event handlers
3. Update event routing

### Phase 5: Refactor Core Handler
1. Simplify main `DragHandler` impl
2. Use composition over inheritance
3. Final cleanup and testing

## Success Metrics

- **File sizes**: All modules < 300 lines
- **Test coverage**: > 90% for each module
- **Performance**: No regression in drag operations
- **Maintainability**: Clear separation of concerns
- **Extensibility**: Easy to add new drag features

## Timeline: 2-3 weeks
## Risk Level: Medium
## Dependencies: None (pure refactoring)
