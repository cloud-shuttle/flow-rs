# Drag Handler Architecture Design

## Overview

The drag handler system manages all drag interactions within Flow-RS graphs, providing a unified interface for mouse/touch-based node and selection manipulation. This design document outlines the refactored architecture that addresses the original monolithic 860-line implementation.

## Current State Analysis

### Problems with Original Implementation
1. **Single monolithic file** (860+ lines) - difficult to maintain and test
2. **Tight coupling** between event handling, state management, and calculations
3. **Mixed responsibilities** - event processing, coordinate transformations, collision detection
4. **Limited testability** - complex interdependencies make unit testing difficult
5. **Poor extensibility** - adding new drag behaviors requires modifying core logic

### Design Goals
- **Modular Architecture**: Break down into focused, single-responsibility modules
- **Testability**: Each module should be independently testable
- **Extensibility**: Easy to add new drag behaviors and constraints
- **Performance**: Efficient algorithms with minimal allocations
- **Type Safety**: Leverage Rust's type system for correctness

## Architecture Overview

```
flow-leptos/src/drag/
├── mod.rs                 # Public API and re-exports
├── handler.rs            # Main DragHandler struct and lifecycle
├── events.rs             # Event processing and gesture recognition
├── calculations.rs       # Mathematical computations and transformations
├── state.rs              # Drag state management and history
└── constraints.rs        # Movement limitations and validation
```

## Module Specifications

### 1. Handler Module (`handler.rs`)

#### Purpose
Core drag handler managing the drag lifecycle and coordinating between subsystems.

#### Key Components

```rust
#[derive(Clone, Debug)]
pub struct DragHandler {
    config: DragConfig,
    current_drag: Option<DragState>,
    constraints: DragConstraints,
    history: DragHistory,
}

#[derive(Clone, Debug)]
pub struct DragConfig {
    pub enable_snap_to_grid: bool,
    pub grid_size: f64,
    pub enable_constraints: bool,
    pub momentum_enabled: bool,
    pub selection_drag_enabled: bool,
}

impl DragHandler {
    pub fn new(config: DragConfig) -> Self { ... }

    pub fn handle_mouse_down(&mut self, event: &MouseEvent, graph: &Graph<N, E>, viewport: &Viewport) -> DragResult { ... }
    pub fn handle_mouse_move(&mut self, event: &MouseEvent, graph: &mut Graph<N, E>, viewport: &mut Viewport) -> DragResult { ... }
    pub fn handle_mouse_up(&mut self, event: &MouseEvent, graph: &mut Graph<N, E>, viewport: &mut Viewport) -> DragResult { ... }

    pub fn handle_touch_start(&mut self, event: &TouchEvent, graph: &Graph<N, E>, viewport: &Viewport) -> DragResult { ... }
    pub fn handle_touch_move(&mut self, event: &TouchEvent, graph: &mut Graph<N, E>, viewport: &mut Viewport) -> DragResult { ... }
    pub fn handle_touch_end(&mut self, event: &TouchEvent, graph: &mut Graph<N, E>, viewport: &mut Viewport) -> DragResult { ... }

    pub fn cancel_drag(&mut self) -> DragResult { ... }
    pub fn apply_momentum(&mut self, delta_time: f64) -> DragResult { ... }
}
```

#### Public API Contract

```rust
pub enum DragResult {
    NoAction,
    DragStarted { node_ids: Vec<NodeId> },
    DragMoved { delta: Position },
    DragEnded { final_positions: HashMap<NodeId, Position> },
    DragCancelled,
    ConstraintViolated { reason: String },
    CollisionDetected { collisions: Vec<(NodeId, NodeId)> },
}
```

### 2. Events Module (`events.rs`)

#### Purpose
Handle mouse and touch event processing, gesture recognition, and input abstraction.

#### Key Components

```rust
pub struct EventProcessor {
    gesture_recognizer: GestureRecognizer,
    modifier_tracker: ModifierTracker,
}

#[derive(Clone, Debug)]
pub struct ModifierKeys {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

pub enum DragBehavior {
    Select,
    DragNodes,
    DragSelection,
    PanViewport,
    CreateSelection,
    AddToSelection,
    ToggleSelection,
}

impl EventProcessor {
    pub fn process_mouse_event(&self, event: &MouseEvent, modifiers: ModifierKeys) -> ProcessedEvent { ... }
    pub fn process_touch_event(&self, event: &TouchEvent) -> ProcessedEvent { ... }
    pub fn determine_drag_behavior(&self, event: &ProcessedEvent, hit_test: &HitTestResult) -> DragBehavior { ... }
}

#[derive(Clone, Debug)]
pub struct ProcessedEvent {
    pub position: Position,
    pub button: MouseButton,
    pub modifiers: ModifierKeys,
    pub timestamp: u64,
    pub event_type: EventType,
}

#[derive(Clone, Debug)]
pub enum EventType {
    MouseDown,
    MouseMove,
    MouseUp,
    TouchStart,
    TouchMove,
    TouchEnd,
    GestureStart,
    GestureChange,
    GestureEnd,
}
```

### 3. Calculations Module (`calculations.rs`)

#### Purpose
Mathematical computations for drag operations, coordinate transformations, and physics.

#### Key Components

```rust
pub struct DragCalculator {
    snap_threshold: f64,
    momentum_decay: f64,
}

impl DragCalculator {
    pub fn screen_to_world(&self, screen_pos: Position, viewport: &Viewport) -> Position { ... }
    pub fn world_to_screen(&self, world_pos: Position, viewport: &Viewport) -> Position { ... }
    pub fn calculate_drag_delta(&self, start_pos: Position, current_pos: Position) -> Position { ... }
    pub fn apply_snap_to_grid(&self, position: Position, grid_size: f64) -> Position { ... }
    pub fn calculate_momentum(&self, velocity: Position, delta_time: f64) -> Position { ... }
    pub fn detect_collisions(&self, moving_nodes: &[(NodeId, Position, Size)], static_nodes: &[(NodeId, Position, Size)]) -> Vec<Collision> { ... }
    pub fn resolve_collisions(&self, collisions: &[Collision]) -> HashMap<NodeId, Position> { ... }
}

#[derive(Clone, Debug)]
pub struct Collision {
    pub node_a: NodeId,
    pub node_b: NodeId,
    pub overlap: Position,
    pub resolution: Position,
}
```

### 4. State Module (`state.rs`)

#### Purpose
Manage drag state, history, and undo/redo functionality.

#### Key Components

```rust
#[derive(Clone, Debug)]
pub struct DragState {
    pub drag_id: String,
    pub start_time: u64,
    pub initial_positions: HashMap<NodeId, Position>,
    pub current_positions: HashMap<NodeId, Position>,
    pub velocity: Position,
    pub accumulated_delta: Position,
    pub is_momentum_active: bool,
    pub constraint_violations: Vec<ConstraintViolation>,
}

#[derive(Clone, Debug)]
pub struct DragHistory {
    operations: VecDeque<DragOperation>,
    max_history_size: usize,
    current_index: usize,
}

impl DragHistory {
    pub fn record_operation(&mut self, operation: DragOperation) { ... }
    pub fn undo_last_operation(&mut self) -> Option<DragOperation> { ... }
    pub fn redo_next_operation(&mut self) -> Option<DragOperation> { ... }
    pub fn can_undo(&self) -> bool { ... }
    pub fn can_redo(&self) -> bool { ... }
}

#[derive(Clone, Debug)]
pub struct DragOperation {
    pub operation_id: String,
    pub timestamp: u64,
    pub node_changes: HashMap<NodeId, PositionChange>,
    pub viewport_changes: Option<ViewportChange>,
}

#[derive(Clone, Debug)]
pub struct PositionChange {
    pub from: Position,
    pub to: Position,
    pub duration: u64,
}
```

### 5. Constraints Module (`constraints.rs`)

#### Purpose
Handle movement limitations, bounds checking, and constraint validation.

#### Key Components

```rust
#[derive(Clone, Debug)]
pub struct DragConstraints {
    pub bounds: Option<Rect>,
    pub axis_lock: Option<Axis>,
    pub distance_limits: Option<DistanceLimits>,
    pub snap_targets: Vec<SnapTarget>,
    pub collision_avoidance: bool,
    pub custom_constraints: Vec<Box<dyn CustomConstraint>>,
}

#[derive(Clone, Debug)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub struct DistanceLimits {
    pub min_distance: f64,
    pub max_distance: f64,
    pub origin: Position,
}

pub trait CustomConstraint: Send + Sync {
    fn validate(&self, node_id: &NodeId, from: Position, to: Position, graph: &Graph) -> ConstraintResult;
}

#[derive(Clone, Debug)]
pub enum ConstraintResult {
    Valid,
    Invalid { reason: String, suggested_position: Option<Position> },
}

impl DragConstraints {
    pub fn validate_move(&self, node_id: &NodeId, from: Position, to: Position, graph: &Graph) -> ConstraintResult { ... }
    pub fn apply_constraints(&self, proposed_position: Position, current_position: Position) -> Position { ... }
    pub fn find_snap_target(&self, position: Position, threshold: f64) -> Option<SnapTarget> { ... }
}
```

## Integration with Flow-RS Ecosystem

### Leptos Integration
```rust
// In flow-leptos/src/components.rs
pub struct FlowDragIntegration {
    drag_handler: DragHandler,
    event_processor: EventProcessor,
}

impl FlowDragIntegration {
    pub fn handle_drag_event(&mut self, event: DragEvent, flow_state: &mut FlowState) -> DragResult {
        match event {
            DragEvent::MouseDown(e) => self.drag_handler.handle_mouse_down(&e, &flow_state.graph, &flow_state.viewport),
            DragEvent::MouseMove(e) => self.drag_handler.handle_mouse_move(&e, &mut flow_state.graph, &mut flow_state.viewport),
            DragEvent::MouseUp(e) => self.drag_handler.handle_mouse_up(&e, &mut flow_state.graph, &mut flow_state.viewport),
            // ... touch events
        }
    }
}
```

### Plugin System Integration
```rust
// Allow plugins to customize drag behavior
pub trait DragPlugin: Plugin {
    fn customize_constraints(&self, constraints: &mut DragConstraints, graph: &Graph) { ... }
    fn on_drag_start(&self, drag_state: &DragState, graph: &Graph) { ... }
    fn on_drag_move(&self, drag_state: &DragState, graph: &Graph) { ... }
    fn on_drag_end(&self, drag_state: &DragState, graph: &mut Graph) { ... }
}
```

## Performance Considerations

### Memory Management
- **Object Pooling**: Reuse collision detection objects
- **Lazy Evaluation**: Only calculate expensive operations when needed
- **Efficient Data Structures**: Use HashMap for O(1) lookups, Vec for iteration

### Computational Optimizations
- **Spatial Partitioning**: Use spatial indices for collision detection
- **Incremental Updates**: Only recalculate affected nodes
- **Batching**: Group similar operations to reduce overhead

### WASM-Specific Optimizations
- **Minimal Allocations**: Prefer stack allocation over heap
- **Efficient Math**: Use WebAssembly SIMD where available
- **Memory Pooling**: Reuse objects across frames

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_calculator_screen_to_world() {
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 2.0 };
        let screen_pos = Position::new(100.0, 50.0);

        let calculator = DragCalculator::new();
        let world_pos = calculator.screen_to_world(screen_pos, &viewport);

        assert_eq!(world_pos, Position::new(50.0, 25.0));
    }

    #[test]
    fn test_constraint_validation() {
        let constraints = DragConstraints {
            bounds: Some(Rect::new(0.0, 0.0, 100.0, 100.0)),
            ..Default::default()
        };

        let result = constraints.validate_move(
            &NodeId::new(),
            Position::new(50.0, 50.0),
            Position::new(150.0, 50.0), // Outside bounds
            &Graph::new()
        );

        assert!(matches!(result, ConstraintResult::Invalid { .. }));
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use flow_rs_core::test_utils::*;

    #[test]
    fn test_complete_drag_workflow() {
        let mut graph = TestGraphBuilder::new().with_nodes(3).build();
        let mut viewport = Viewport::default();
        let mut handler = DragHandler::new(DragConfig::default());

        // Simulate mouse down on node
        let mouse_down = create_mouse_event(100.0, 100.0, MouseButton::Left);
        let result = handler.handle_mouse_down(&mouse_down, &graph, &viewport);
        assert!(matches!(result, DragResult::DragStarted { .. }));

        // Simulate drag
        let mouse_move = create_mouse_event(150.0, 150.0, MouseButton::Left);
        let result = handler.handle_mouse_move(&mouse_move, &mut graph, &mut viewport);
        assert!(matches!(result, DragResult::DragMoved { .. }));

        // Simulate mouse up
        let mouse_up = create_mouse_event(150.0, 150.0, MouseButton::Left);
        let result = handler.handle_mouse_up(&mouse_up, &mut graph, &mut viewport);
        assert!(matches!(result, DragResult::DragEnded { .. }));
    }
}
```

### Performance Tests
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use criterion::{black_box, Criterion};

    fn bench_collision_detection(c: &mut Criterion) {
        let nodes = generate_test_nodes(1000);
        let calculator = DragCalculator::new();

        c.bench_function("collision_detection_1000_nodes", |b| {
            b.iter(|| {
                black_box(calculator.detect_collisions(&moving_nodes, &static_nodes));
            });
        });
    }
}
```

## Error Handling

### Comprehensive Error Types
```rust
#[derive(Clone, Debug)]
pub enum DragError {
    InvalidState(String),
    ConstraintViolation(String),
    CollisionResolutionFailed(String),
    EventProcessingError(String),
    CalculationError(String),
}

impl std::fmt::Display for DragError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DragError::InvalidState(msg) => write!(f, "Invalid drag state: {}", msg),
            DragError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            DragError::CollisionResolutionFailed(msg) => write!(f, "Collision resolution failed: {}", msg),
            DragError::EventProcessingError(msg) => write!(f, "Event processing error: {}", msg),
            DragError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}
```

### Error Recovery Strategies
- **Graceful Degradation**: Continue operation with reduced functionality
- **Automatic Correction**: Suggest valid alternatives when constraints violated
- **Rollback Capability**: Undo failed operations
- **Logging and Monitoring**: Comprehensive error reporting for debugging

## Future Extensions

### Advanced Features
- **Multi-touch Gestures**: Pinch-to-zoom, rotate
- **Magnetic Snapping**: Intelligent alignment to other elements
- **Physics-based Animation**: Spring physics for natural movement
- **Collaborative Dragging**: Multi-user drag operations

### Plugin Architecture
- **Custom Constraints**: Plugin-defined movement rules
- **Drag Modifiers**: Plugins can modify drag behavior
- **Visual Feedback**: Custom drag indicators and previews
- **Post-processing**: Plugins can modify final positions

## Migration Strategy

### From Monolithic Implementation
1. **Extract Core Logic**: Move calculation functions to `calculations.rs`
2. **Split Event Handling**: Create `events.rs` with event processing
3. **Isolate State Management**: Extract state tracking to `state.rs`
4. **Modularize Constraints**: Create `constraints.rs` for validation logic
5. **Refactor Handler**: Simplify main handler to coordinate modules

### Backward Compatibility
- **API Preservation**: Maintain existing public interfaces
- **Configuration Migration**: Support old configuration format
- **Gradual Rollout**: Feature flags for new vs old implementation

## Success Metrics

### Code Quality
- ✅ **File sizes <300 lines** for all modules
- ✅ **Test coverage >90%** for drag functionality
- ✅ **Zero panics** in normal operation
- ✅ **Clear error messages** for all failure modes

### Performance
- ✅ **60 FPS** drag operations for 1000+ nodes
- ✅ **<16ms** response time for drag events
- ✅ **Memory efficient** with bounded allocations
- ✅ **Smooth momentum** animations

### Maintainability
- ✅ **Modular architecture** with clear boundaries
- ✅ **Comprehensive documentation** for all APIs
- ✅ **Easy extensibility** for new drag behaviors
- ✅ **Independent testing** of all modules