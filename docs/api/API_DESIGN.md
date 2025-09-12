# Leptos Flow Core - API Design Principles

## Overview

This document outlines the design principles, patterns, and guidelines that shape the `leptos-flow-core` API. These principles ensure consistency, usability, and maintainability across the entire API surface.

## Design Principles

### 1. Type Safety

**Principle**: Leverage Rust's type system to prevent runtime errors and provide clear contracts.

**Implementation**:
- Strong typing for all IDs (`NodeId`, `EdgeId`, `GroupId`, `HandleId`)
- Generic types for node and edge data (`Graph<N, E>`)
- Result types for operations that can fail
- Newtype patterns for domain-specific values

```rust
// Strong typing prevents mixing different ID types
let node_id = NodeId::new("node1");
let edge_id = EdgeId::new("edge1");
// node_id == edge_id; // Compile-time error

// Generic types allow custom data
let graph: Graph<MyNodeData, MyEdgeData> = Graph::new();
```

### 2. Consistency

**Principle**: Similar operations should have similar interfaces and behavior.

**Implementation**:
- Consistent naming conventions across all types
- Uniform error handling with `Result<T, FlowError>`
- Standardized builder patterns for complex objects
- Consistent method signatures for similar operations

```rust
// Consistent builder patterns
let node = Node::builder("id").position(0.0, 0.0).build();
let edge = Edge::builder().id("id").connect("src", "tgt").build();

// Consistent error handling
let result1 = graph.add_node(node);
let result2 = graph.add_edge(edge);
// Both return Result<(), FlowError>
```

### 3. Performance

**Principle**: Optimize for common use cases while maintaining API clarity.

**Implementation**:
- Zero-copy operations where possible
- Efficient data structures (HashMap for O(1) lookups)
- Spatial indexing for fast queries
- Minimal allocations in hot paths

```rust
// Zero-copy position updates
node.set_position(new_position); // Moves, doesn't copy

// Efficient lookups
let node = graph.nodes().get(&node_id); // O(1) HashMap lookup
```

### 4. Usability

**Principle**: Make common operations simple and intuitive.

**Implementation**:
- Builder patterns for complex object creation
- Sensible defaults for optional parameters
- Clear method names that describe intent
- Comprehensive documentation with examples

```rust
// Simple creation with defaults
let node = Node::new("id", Position::new(0.0, 0.0), ());

// Complex creation with builder
let complex_node = Node::builder("id")
    .position(100.0, 200.0)
    .size(150.0, 75.0)
    .node_type("custom")
    .build();
```

### 5. Extensibility

**Principle**: Design for future growth and customization.

**Implementation**:
- Trait-based architecture for algorithms
- Generic types for custom data
- Pluggable components (renderers, layouts)
- Versioned API evolution

```rust
// Extensible layout system
trait LayoutAlgorithm<N, E> {
    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<(), FlowError>;
}

// Custom data types
struct MyNodeData { /* custom fields */ }
struct MyEdgeData { /* custom fields */ }
let graph: Graph<MyNodeData, MyEdgeData> = Graph::new();
```

## API Patterns

### Builder Pattern

Used for complex object creation with optional parameters.

```rust
// Node builder
let node = Node::<()>::builder("id")
    .position(100.0, 200.0)
    .size(150.0, 75.0)
    .node_type("custom")
    .selectable(true)
    .build();

// Edge builder
let edge = Edge::<()>::builder()
    .id("edge1")
    .connect("node1", "node2")
    .build()
    .unwrap();
```

**Benefits**:
- Clear, readable object creation
- Optional parameters with sensible defaults
- Compile-time validation of required fields
- Fluent interface for method chaining

### Manager Pattern

Used for managing collections of related objects.

```rust
// Selection management
let mut selection = SelectionManager::new();
selection.select_node(&node_id);
selection.clear_selection();

// Group management
let mut groups = GroupManager::new();
groups.create_group(group_id, node_ids)?;
```

**Benefits**:
- Encapsulated state management
- Consistent operations across related objects
- Clear separation of concerns
- Easy testing and mocking

### Trait-Based Architecture

Used for pluggable algorithms and behaviors.

```rust
// Layout algorithms
trait LayoutAlgorithm<N, E> {
    fn name(&self) -> &'static str;
    fn apply(&mut self, graph: &mut Graph<N, E>) -> Result<(), FlowError>;
    fn can_interrupt(&self) -> bool;
}

// Implementations
struct ForceDirectedLayout { /* ... */ }
struct GridLayout { /* ... */ }
struct CircularLayout { /* ... */ }
```

**Benefits**:
- Pluggable implementations
- Easy testing with mock implementations
- Clear contracts for algorithm behavior
- Extensible without breaking existing code

### Error Handling

Consistent error handling with Result types and custom error enums.

```rust
// Custom error types
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Node not found: {0}")]
    NodeNotFound(NodeId),

    #[error("Edge not found: {0}")]
    EdgeNotFound(EdgeId),

    #[error("Invalid connection: {source} -> {target}")]
    InvalidConnection { source: NodeId, target: NodeId },
}

// Consistent Result usage
fn add_node(&mut self, node: Node<N>) -> Result<(), FlowError> {
    // Implementation
}
```

**Benefits**:
- Explicit error handling
- Rich error information
- Compile-time error checking
- Easy error propagation

## Naming Conventions

### Types

- **Structs**: PascalCase (`Graph`, `Node`, `Edge`)
- **Enums**: PascalCase (`SelectionMode`, `HandleType`)
- **Traits**: PascalCase with descriptive names (`LayoutAlgorithm`)
- **Type aliases**: PascalCase (`NodeId`, `EdgeId`)

### Methods

- **Constructors**: `new()` for simple creation, `builder()` for complex creation
- **Getters**: Property names (`position()`, `size()`, `id()`)
- **Setters**: `set_*` prefix (`set_position()`, `set_size()`)
- **Actions**: Verb names (`add_node()`, `remove_edge()`, `select_node()`)
- **Queries**: Question words (`is_selected()`, `contains()`)

### Parameters

- **IDs**: `id` for generic IDs, `node_id`/`edge_id` for specific types
- **Coordinates**: `x`, `y` for positions, `width`, `height` for dimensions
- **Data**: `data` for generic data, `node_data`/`edge_data` for specific types

## Versioning Strategy

### Semantic Versioning

We follow [Semantic Versioning](https://semver.org/) principles:

- **Major (X.0.0)**: Breaking API changes
- **Minor (X.Y.0)**: New features, backward compatible
- **Patch (X.Y.Z)**: Bug fixes, backward compatible

### Backward Compatibility

**Guaranteed**:
- Adding new methods to existing types
- Adding new variants to enums (with default handling)
- Adding new optional parameters to methods
- Adding new types and traits

**Not Guaranteed**:
- Removing or renaming public APIs
- Changing method signatures
- Changing behavior of existing methods
- Removing enum variants

### Migration Strategy

For breaking changes:

1. **Deprecation**: Mark old APIs as deprecated with clear migration path
2. **Transition Period**: Maintain deprecated APIs for at least one minor version
3. **Migration Guide**: Provide detailed migration instructions
4. **Tooling**: Provide automated migration tools where possible

## Performance Guidelines

### Memory Management

- **Zero-copy**: Use move semantics instead of copying where possible
- **Efficient Storage**: Use appropriate data structures (HashMap for O(1) lookups)
- **Minimal Allocations**: Avoid unnecessary allocations in hot paths
- **Spatial Optimization**: Use spatial indexing for large datasets

### Algorithm Complexity

- **Graph Operations**: O(1) for most operations with HashMap storage
- **Spatial Queries**: O(log n) with spatial indexing
- **Layout Algorithms**: Document complexity (O(n²) for force-directed, O(n) for grid)
- **Selection Operations**: O(1) for single operations, O(n) for bulk operations

### Best Practices

1. **Batch Operations**: Group related operations for better performance
2. **Lazy Evaluation**: Defer expensive operations until needed
3. **Caching**: Cache expensive computations when appropriate
4. **Profiling**: Use benchmarks to validate performance improvements

## Testing Strategy

### Unit Tests

- **API Contracts**: Test all public APIs with comprehensive test coverage
- **Edge Cases**: Test boundary conditions and error cases
- **Property-Based Testing**: Use proptest for randomized testing
- **Performance Tests**: Benchmark critical operations

### Integration Tests

- **End-to-End**: Test complete workflows
- **Cross-Browser**: Validate behavior across different browsers
- **Large Datasets**: Test with realistic data sizes
- **Stress Testing**: Test under high load conditions

### Documentation Tests

- **Examples**: Ensure all code examples compile and run
- **Accuracy**: Validate documentation matches implementation
- **Completeness**: Ensure all public APIs are documented
- **Consistency**: Maintain consistent style and format

## Evolution Strategy

### API Stability

**Stable APIs** (guaranteed backward compatibility):
- Core types (`Graph`, `Node`, `Edge`)
- Basic operations (CRUD operations)
- Error types and handling
- Builder patterns

**Experimental APIs** (may change):
- Advanced algorithms
- Performance optimizations
- Integration features
- Platform-specific functionality

### Feature Flags

Use feature flags for experimental or platform-specific functionality:

```rust
#[cfg(feature = "webgl-renderer")]
pub mod webgl_renderer;

#[cfg(feature = "performance-monitoring")]
pub mod performance_monitor;
```

### Feedback Integration

- **RFC Process**: Use Request for Comments for major changes
- **Community Input**: Gather feedback from users and contributors
- **Usage Analytics**: Monitor API usage patterns (with privacy considerations)
- **Breaking Change Policy**: Clear process for handling breaking changes

## Conclusion

These design principles and patterns ensure that `leptos-flow-core` provides a consistent, performant, and maintainable API that serves both current needs and future growth. The focus on type safety, consistency, and usability makes the API approachable for new users while providing the power and flexibility needed for complex applications.

The trait-based architecture and generic design allow for extensive customization while maintaining a stable core API. The comprehensive testing strategy and clear versioning policy provide confidence in the API's reliability and evolution.

**Status**: Production Ready
**Version**: 0.1.0-alpha
**Last Updated**: January 2024
