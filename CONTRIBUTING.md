# Contributing to Leptos Flow

## Welcome Contributors

We're excited you want to contribute to Leptos Flow. This guide will help you get started with development, understand our processes, and make meaningful contributions.

## Quick Start

### Prerequisites

**Required:**

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+ (for development tools)
- Git

**Install Rust targets:**

```bash
rustup target add wasm32-unknown-unknown
```

**Install development tools:**

```bash
# Essential tools
cargo install trunk
cargo install wasm-pack
cargo install cargo-watch
cargo install cargo-nextest  # Fast test runner

# Code quality tools
cargo install cargo-clippy
rustup component add rustfmt
rustup component add clippy

# Optional but recommended
cargo install cargo-expand      # Macro expansion
cargo install cargo-audit       # Security auditing
cargo install cargo-deny        # License/dependency checking
```

### Development Setup

1. **Fork and clone the repository:**

```bash
git clone https://github.com/YOUR_USERNAME/leptos-flow.git
cd leptos-flow
```

2. **Install dependencies:**

```bash
# Install npm dependencies for examples
cd examples/basic
npm install
cd ../..
```

3. **Run the test suite:**

```bash
# Run all tests with timeout protection (recommended)
make test

# Run specific test suites
make test-quick      # Quick tests (10s timeout)
make test-spatial    # Spatial tests (30s timeout)
make test-proptest   # Property-based tests (45s timeout)

# Traditional cargo test commands
cargo test --all-features
cargo test --target wasm32-unknown-unknown --all-features
```

4. **Start development server:**

```bash
cd examples/basic
trunk serve --open
```

## Project Structure

```
leptos-flow/
├── leptos-flow-core/           # Core graph logic (no dependencies on UI)
│   ├── src/
│   │   ├── graph/              # Graph data structures
│   │   ├── layout/             # Layout algorithms
│   │   ├── spatial/            # Spatial indexing
│   │   └── events/             # Event system
│   └── tests/
├── leptos-flow-renderer/       # Rendering abstractions
│   ├── src/
│   │   ├── traits.rs           # Renderer trait definitions
│   │   ├── canvas2d/           # Canvas2D renderer
│   │   ├── webgl2/             # WebGL2 renderer
│   │   └── webgpu/             # WebGPU renderer
│   └── tests/
├── leptos-flow-leptos/         # Leptos integration
│   ├── src/
│   │   ├── components/         # Leptos components
│   │   ├── hooks/              # Custom hooks
│   │   └── context/            # Context providers
│   └── tests/
├── leptos-flow-wasm/           # WASM bindings
├── examples/                   # Example applications
└── docs/                       # Documentation
```

## Development Workflow

### 1. Branch Strategy

We use a simple branching model:

- `main` - Stable, releasable code
- `develop` - Integration branch for features
- `feature/feature-name` - Feature development
- `bugfix/bug-description` - Bug fixes
- `docs/improvement-description` - Documentation updates

**Create a feature branch:**

```bash
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name
```

### 2. Code Standards

#### Rust Code Style

We follow the standard Rust style with these additional guidelines:

**Format your code:**

```bash
cargo fmt --all
```

**Run clippy:**

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Our clippy configuration:**

```toml
# Cargo.toml
[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
cargo = "warn"

# Allow these specific lints
module_name_repetitions = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
```

#### Code Organization

**Module structure:**

```rust
// Good: Clear module hierarchy
pub mod graph {
    pub mod node;
    pub mod edge;
    pub mod algorithms;
}

// Bad: Flat structure
pub mod node;
pub mod edge;
pub mod graph_algorithms;
```

**Error handling:**

```rust
// Good: Use proper error types
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Invalid node ID: {id}")]
    InvalidNodeId { id: String },

    #[error("Renderer error: {0}")]
    Renderer(#[from] RendererError),
}

// Bad: Generic error types
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

**Documentation:**

```rust
// Good: Comprehensive documentation
/// Adds a new node to the graph.
///
/// # Arguments
/// * `node` - The node to add
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(FlowError::DuplicateId)` if node ID already exists
///
/// # Example
/// ```
/// let mut graph = Graph::new();
/// let node = Node::new("1", Position::new(100.0, 100.0));
/// graph.add_node(node)?;
/// ```
pub fn add_node(&mut self, node: Node) -> Result<(), FlowError> {
    // Implementation
}
```

### 3. Testing Requirements

#### Test Coverage Requirements

| Component | Minimum Coverage |
|-----------|------------------|
| Core logic | 90% |
| Renderers | 80% |
| Leptos integration | 85% |
| WASM bindings | 75% |

#### Test Categories

**1. Unit Tests**

```bash
# Run unit tests with timeout protection
make test

# Run specific test suites with appropriate timeouts
make test-quick      # 10s timeout for fast tests
make test-spatial    # 30s timeout for spatial tests
make test-proptest   # 45s timeout for property-based tests

# Traditional unit tests
cargo test --lib

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --all-features --out Html
```

**Current Test Status:**
- ✅ **32/32 spatial tests passing** (previously 7 failing)
- ✅ **0 hanging tests** (previously multiple)
- ✅ **100% spatial indexing coverage** with comprehensive edge case handling
- ✅ **Property-based testing** with Proptest integration
- ✅ **Infinite loop prevention** with bounds checking and safety limits

**2. Integration Tests**

```bash
# Run integration tests
cargo test --test integration

# Run WASM tests
wasm-pack test --headless --firefox --chrome
```

**3. Property-Based Tests**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn node_addition_is_idempotent(
        nodes in prop::collection::vec(any::<Node>(), 0..100)
    ) {
        let mut graph = Graph::new();
        for node in &nodes {
            let _ = graph.add_node(node.clone());
        }

        // Adding the same nodes again should not change the graph
        for node in &nodes {
            let _ = graph.add_node(node.clone());
        }

        prop_assert_eq!(graph.node_count(), nodes.len());
    }
}
```

**4. Visual Regression Tests**

```bash
# Setup visual tests (requires Chrome/Chromium)
cd tests/visual
npm install

# Run visual tests
npm run test:visual

# Update baselines
npm run test:visual -- --update-snapshots
```

**5. Performance Benchmarks**

```bash
# Run benchmarks
cargo bench

# Compare with baseline
cargo bench -- --save-baseline main
git checkout your-feature-branch
cargo bench -- --baseline main
```

### 4. Commit Guidelines

We follow [Conventional Commits](https://conventionalcommits.org/):

```bash
# Format: type(scope): description
feat(core): add spatial indexing for large graphs
fix(renderer): resolve WebGL context loss
docs(api): update node creation examples
test(integration): add layout algorithm tests
perf(wasm): optimize memory allocation
refactor(leptos): simplify hook implementation
```

**Commit types:**

- `feat` - New features
- `fix` - Bug fixes
- `docs` - Documentation changes
- `test` - Test additions/changes
- `perf` - Performance improvements
- `refactor` - Code refactoring
- `style` - Code style changes
- `ci` - CI/CD changes
- `chore` - Maintenance tasks

### 5. Pull Request Process

#### Before Creating a PR

**1. Ensure tests pass:**

```bash
cargo test --all-features
cargo clippy --all-targets --all-features
cargo fmt --all -- --check
```

**2. Update documentation:**

```bash
cargo doc --no-deps --all-features
```

**3. Run the full test suite:**

```bash
# Run all tests including WASM
./scripts/test-all.sh
```

#### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change (fix or feature that breaks existing API)
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] WASM tests pass
- [ ] Visual regression tests pass
- [ ] Manual testing completed

## Performance Impact
- [ ] No performance impact
- [ ] Performance improvement
- [ ] Acceptable performance trade-off
- [ ] Performance impact requires discussion

## Breaking Changes
List any breaking changes and migration path

## Documentation
- [ ] Documentation updated
- [ ] Examples updated
- [ ] API documentation updated

## Screenshots (for UI changes)
[Add screenshots or videos if applicable]
```

#### Review Process

1. **Automated checks** must pass (CI/CD, tests, linting)
2. **Code review** by at least one maintainer
3. **Performance review** for changes affecting core algorithms
4. **Documentation review** for public API changes
5. **Final approval** and merge

## Development Guidelines

### 1. Performance Considerations

**Memory efficiency:**

```rust
// Good: Use references where possible
pub fn render_nodes(&self, nodes: &[Node], viewport: &Viewport) {
    // Implementation
}

// Bad: Unnecessary cloning
pub fn render_nodes(&self, nodes: Vec<Node>, viewport: Viewport) {
    // Implementation
}
```

**WASM optimization:**

```rust
// Good: Minimize allocations
#[wasm_bindgen]
pub struct NodeBuilder {
    inner: Node,
}

// Bad: Frequent allocations
#[wasm_bindgen]
pub fn create_node(id: String) -> Node {
    Node::new(id) // Allocates on every call
}
```

### 2. API Design Principles

**Consistency:**

```rust
// Good: Consistent naming
pub fn add_node(&mut self, node: Node) -> Result<(), FlowError>;
pub fn remove_node(&mut self, id: &str) -> Result<Node, FlowError>;
pub fn update_node(&mut self, id: &str, data: NodeData) -> Result<(), FlowError>;

// Bad: Inconsistent naming
pub fn add_node(&mut self, node: Node) -> Result<(), FlowError>;
pub fn delete_node(&mut self, id: &str) -> Result<Node, FlowError>;
pub fn change_node(&mut self, id: &str, data: NodeData) -> Result<(), FlowError>;
```

**Type safety:**

```rust
// Good: Strong typing
pub struct NodeId(String);
pub struct EdgeId(String);

// Bad: Primitive obsession
pub fn connect_nodes(source: String, target: String) -> Edge;
```

### 3. Error Handling

**Comprehensive error types:**

```rust
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Node with ID '{id}' not found")]
    NodeNotFound { id: String },

    #[error("Cannot connect node to itself")]
    SelfConnection,

    #[error("Renderer error: {source}")]
    Renderer {
        #[from]
        source: RendererError,
    },

    #[error("Layout error: {message}")]
    Layout { message: String },
}
```

**Graceful degradation:**

```rust
// Good: Fallback behavior
pub fn select_renderer() -> Box<dyn Renderer> {
    if webgpu_available() {
        Box::new(WebGPURenderer::new())
    } else if webgl2_available() {
        Box::new(WebGL2Renderer::new())
    } else {
        Box::new(Canvas2DRenderer::new())
    }
}
```

## Architecture Decision Records (ADRs)

When making significant architectural decisions, create an ADR:

```markdown
# ADR-001: Use R-tree for Spatial Indexing

## Status
Accepted

## Context
We need efficient spatial queries for large graphs (10k+ nodes).

## Decision
Use R-tree data structure for spatial indexing.

## Consequences
- Pros: O(log n) query time, proven performance
- Cons: Additional memory overhead, complex implementation

## Alternatives Considered
- Quadtree: Simpler but worse performance on non-uniform distributions
- Grid: Very fast but high memory usage
```

## Release Process

### 1. Version Strategy

We follow [Semantic Versioning](https://semver.org/):

- `MAJOR.MINOR.PATCH` (e.g., 1.2.3)
- Breaking changes increment MAJOR
- New features increment MINOR
- Bug fixes increment PATCH
- Pre-release versions: `1.0.0-alpha.1`, `1.0.0-beta.2`, `1.0.0-rc.1`

### 2. Release Checklist

**Pre-release:**

- [ ] All tests passing
- [ ] Performance benchmarks run
- [ ] Documentation updated
- [ ] Examples tested
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml

**Release:**

- [ ] Git tag created
- [ ] Crates published to crates.io
- [ ] GitHub release created
- [ ] Documentation deployed
- [ ] Release notes published

## Getting Help

### Resources

- **Documentation**: [docs.rs/leptos-flow](https://docs.rs/leptos-flow)
- **Examples**: `examples/` directory
- **API Reference**: Generated from rustdoc
- **Architecture**: `docs/architecture/ARCHITECTURE.md`

### Communication

- **GitHub Discussions**: General questions and ideas
- **GitHub Issues**: Bug reports and feature requests
- **Pull Requests**: Code contributions and discussions
- **Discord**: Real-time chat (link in README)

### Common Issues

**Build failures:**

```bash
# Clear cache and rebuild
cargo clean
cargo build --all-features
```

**WASM test failures:**

```bash
# Install wasm-pack if not available
cargo install wasm-pack

# Run WASM tests with verbose output
wasm-pack test --headless --chrome -- --test-threads=1
```

**Performance regressions:**

```bash
# Run benchmarks and compare
cargo bench -- --save-baseline main
git checkout your-branch
cargo bench -- --baseline main
```

Thank you for contributing to Leptos Flow! Your contributions help make reactive flow editing better for everyone.
