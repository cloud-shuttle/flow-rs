# Agents Configuration for Flow-RS

## Build/Test Commands
- **Test single crate**: `cargo test -p flow-rs-core`
- **Test with timeout**: `./scripts/test-with-timeout.sh flow-core 60 1 all`
- **Quick tests**: `make test-quick` (10s timeout)
- **Specific test suites**: `make test-spatial`, `make test-proptest`, `make test-layout`
- **Format**: `cargo fmt --all`
- **Lint**: `cargo clippy --all-targets --all-features`
- **E2E tests**: `npm run test:e2e`

## Architecture
- **Workspace structure**: flow-core (data/algorithms), flow-leptos (Leptos integration), flow-renderer (Canvas2D/WebGL), flow-wasm (WASM bindings)
- **Core types**: Graph, Node, Edge, Position, Size, Viewport in `flow-core/src/types.rs`
- **Spatial indexing**: Grid-based spatial queries in `flow-core/src/spatial.rs`
- **Selection system**: Multi-selection with groups in `flow-core/src/selection.rs`

## Code Style
- **Imports**: Use full paths from crate root (`use crate::types::Position`)
- **Error handling**: Use `thiserror::Error` and `Result<T, FlowError>` pattern
- **Builder pattern**: Node/Edge builders with fluent API (`Node::builder("id").position(x, y).build()`)
- **Serialization**: Optional serde support with workspace feature flags
- **Tests**: Inline `#[cfg(test)]` modules, proptest for edge cases, nextest runner preferred
- **Dependencies**: Leptos 0.6.15, nalgebra for math, spade for spatial data structures
