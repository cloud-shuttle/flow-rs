# Migration Guide: leptos-flow → flow-rs

This guide helps you migrate from the old `leptos-flow` project to the new `flow-rs` project structure.

## Overview

The project has been renamed from `leptos-flow` to `flow-rs` to better reflect its framework-agnostic nature and broader ecosystem potential.

## Package Name Changes

| Old Name | New Name | Description |
|----------|----------|-------------|
| `leptos-flow-core` | `flow-core` | Core data structures and algorithms |
| `leptos-flow-renderer` | `flow-renderer` | Rendering backends |
| `leptos-flow-leptos` | `flow-leptos` | Leptos framework integration |
| `leptos-flow-wasm` | `flow-wasm` | WASM bindings and utilities |
| `leptos-flow` | `flow-rs` | Main package (re-exports) |

## Cargo.toml Updates

### Dependencies

Update your `Cargo.toml` dependencies:

```toml
# Old
[dependencies]
leptos-flow-core = "0.1.0"
leptos-flow-renderer = "0.1.0"
leptos-flow-leptos = "0.1.0"

# New
[dependencies]
flow-core = "0.1.0"
flow-renderer = "0.1.0"
flow-leptos = "0.1.0"
```

### Features

Update feature names:

```toml
# Old
[features]
default = ["canvas2d", "leptos"]
canvas2d = ["leptos-flow-renderer/canvas2d"]
leptos = ["leptos-flow-leptos"]

# New
[features]
default = ["canvas2d", "leptos"]
canvas2d = ["flow-renderer/canvas2d"]
leptos = ["flow-leptos"]
```

## Import Statement Updates

### Rust Code

Update your import statements:

```rust
// Old imports
use flow_core::{Graph, Node, Edge, Position};
use flow_renderer::{Canvas2DRenderer, Renderer};
use flow_leptos::components::{FlowEditor, FlowCanvas};

// New imports
use flow_core::{Graph, Node, Edge, Position};
use flow_renderer::{Canvas2DRenderer, Renderer};
use flow_leptos::components::{FlowEditor, FlowCanvas};
```

### Main Package Usage

```rust
// Old
use leptos_flow::core::{Graph, Node, Edge};
use leptos_flow::renderer::Canvas2DRenderer;
use leptos_flow::leptos::components::FlowEditor;

// New
use flow_rs::core::{Graph, Node, Edge};
use flow_rs::renderer::Canvas2DRenderer;
use flow_rs::leptos::components::FlowEditor;
```

## JavaScript/TypeScript (WASM)

If you're using the WASM bindings:

```javascript
// Old
import { Graph, Node, Edge } from 'leptos-flow-wasm';

// New
import { Graph, Node, Edge } from 'flow-wasm';
```

## Breaking Changes

### API Changes

- All public APIs remain the same
- Only package names and import paths have changed
- No functional changes to the core API

### Crate Features

- Feature names remain the same
- Only the underlying crate names have changed

## Migration Steps

1. **Update Cargo.toml**: Change all `leptos-flow-*` dependencies to `flow-*`
2. **Update imports**: Change all import statements to use new package names
3. **Update features**: Update feature dependencies to use new crate names
4. **Test**: Run your tests to ensure everything works correctly
5. **Update documentation**: Update any documentation that references the old names

## Backward Compatibility

- The old `leptos-flow` packages will continue to work
- No immediate deprecation timeline
- New features will only be available in `flow-rs`

## Getting Help

If you encounter issues during migration:

1. Check this migration guide
2. Review the [API Reference](docs/api/REFERENCE.md)
3. Look at the [examples](examples/) for updated usage patterns
4. Open an issue on the [GitHub repository](https://github.com/flow-rs/flow-rs)

## Future Plans

The `flow-rs` project is designed to be framework-agnostic, with plans for:

- `flow-yew` - Yew framework integration
- `flow-dioxus` - Dioxus framework integration
- `flow-tauri` - Tauri desktop app integration
- Additional rendering backends (WebGL2, WebGPU)

This migration positions you to take advantage of these future integrations.
