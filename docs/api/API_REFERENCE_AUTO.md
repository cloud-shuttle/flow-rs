# Leptos Flow Core - Auto-Generated API Reference

> **Note**: This file is auto-generated. For the complete API reference, see [REFERENCE.md](REFERENCE.md)

## Auto-Generated API Summary

This document provides an automatically generated summary of all public APIs in `leptos-flow-core`.

### Generation Info
- **Generated**: $(date)
- **Version**: $(grep '^version = ' Cargo.toml | cut -d'"' -f2)
- **Rust Version**: $(rustc --version)

## Public Modules

pub mod error;
pub mod graph;
pub mod spatial;
pub mod layout;
pub mod auto_layout;
pub mod types;
pub mod selection;
pub mod groups;
pub mod handle;
pub mod drag_operations;
pub mod edge_creator;
pub use error::{FlowError, Result};
pub use graph::{Graph, Node, Edge};
pub use types::{Position, Size, Rect, Viewport, NodeId, EdgeId, GroupId};
pub use selection::{SelectionManager, SelectionMode, NavigationDirection, KeyboardShortcut, VisualFeedback};
pub use groups::{Group, GroupManager};
pub use handle::{Handle, HandleId, HandleType, HandlePosition, HandleManager};
pub use drag_operations::DragOperation;
pub use edge_creator::{EdgeCreator, PreviewEdge, ConnectionFeedback};
pub use auto_layout::{AutoLayoutManager, AutoLayoutStrategy, AutoLayoutConfig, AutoLayoutConfigBuilder};
pub mod prelude {

## Core Types

### Geometric Types
- `Position` - 2D position in flow coordinates
- `Size` - Width and height dimensions
- `Rect` - Rectangular area with position and size
- `Viewport` - Viewport for coordinate transformations

### Graph Types
- `Graph<N, E>` - Main graph data structure
- `Node<N>` - Node in the graph
- `Edge<E>` - Edge connecting nodes
- `NodeId` - Unique identifier for nodes
- `EdgeId` - Unique identifier for edges

### Management Types
- `SelectionManager` - Manages node selection
- `GroupManager` - Manages node groups
- `HandleManager` - Manages connection handles
- `AutoLayoutManager` - Automatic layout selection

### Layout Types
- `LayoutAlgorithm<N, E>` - Trait for layout algorithms
- `ForceDirectedLayout` - Force-directed layout
- `GridLayout` - Grid-based layout
- `CircularLayout` - Circular layout

### Error Types
- `FlowError` - Main error type
- `SpatialError` - Spatial indexing errors
- `LayoutError` - Layout algorithm errors

## Builder Patterns

### NodeBuilder
```rust
let node = Node::<()>::builder("id")
    .position(100.0, 200.0)
    .size(150.0, 75.0)
    .node_type("custom")
    .selectable(true)
    .build();
```

### EdgeBuilder
```rust
let edge = Edge::<()>::builder()
    .id("edge1")
    .connect("node1", "node2")
    .build()
    .unwrap();
```

## Common Usage Patterns

### Creating a Graph
```rust
use leptos_flow_core::{Graph, Node, Edge, Position};

let mut graph: Graph<(), ()> = Graph::new();
let node = Node::new("node1", Position::new(100.0, 100.0), ());
graph.add_node(node).unwrap();
```

### Selection Management
```rust
use leptos_flow_core::{SelectionManager, NodeId};

let mut selection = SelectionManager::new();
selection.select_node(&NodeId::new("node1"));
```

### Layout Algorithms
```rust
use leptos_flow_core::layout::ForceDirectedLayout;

let mut layout = ForceDirectedLayout::new();
layout.apply(&mut graph).unwrap();
```

## Performance Characteristics

- **Graph Operations**: O(1) for most operations
- **Spatial Queries**: O(log n) with spatial indexing
- **Layout Algorithms**: O(n²) for force-directed, O(n) for grid/circular
- **Memory**: Zero-copy operations where possible

## Error Handling

All operations that can fail return `Result<T, FlowError>`:

```rust
match graph.add_node(node) {
    Ok(()) => println!("Node added successfully"),
    Err(e) => println!("Error: {}", e),
}
```

## See Also

- [Complete API Reference](REFERENCE.md) - Comprehensive documentation
- [API Design Principles](API_DESIGN.md) - Design philosophy
- [Examples](../examples/README.md) - Working examples
