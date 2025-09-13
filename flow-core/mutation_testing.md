# Mutation Testing for leptos-flow-core

## Overview

Mutation testing is a powerful technique that helps identify gaps in test coverage by introducing small changes (mutations) to the code and checking if our tests can detect these changes. We've set up `cargo-mutants` to perform mutation testing on our codebase.

## Setup

We've installed and configured `cargo-mutants` with the following configuration in `.cargo/mutants.toml`:

```toml
# Cargo-mutants configuration for leptos-flow-core
# Focused configuration for faster, more targeted mutation testing

# Skip certain function calls that are commonly mutated but not critical
skip_calls = [
    "Default::default",
    "Ok",
    "Err",
    "vec!",
    "String::new",
    "format!",
    "println!",
    "eprintln!",
    "dbg!",
    "assert!",
    "assert_eq!",
    "assert_ne!",
    "panic!",
    "unreachable!",
    "todo!",
    "unimplemented!",
]

# Focus on specific files for initial testing
file = [
    "src/types.rs",      # Core data structures only
]

# Exclude certain patterns
exclude = [
    "**/tests/**",
    "**/benches/**",
    "**/examples/**",
    "**/target/**",
]

# Timeout settings - much shorter for faster iteration
timeout = 15
build_timeout = 10

# Parallel execution
jobs = 1

# Output settings
output = "mutants.out"
colors = "auto"
```

## Key Findings

From our initial mutation testing run, we discovered several important insights:

### 1. Test Coverage Gaps

Cargo-mutants found many mutations that our tests are **not catching** (marked as "MISSED"), including:

- **Arithmetic operations**: Changes to `+`, `-`, `*`, `/` operators in mathematical calculations
- **Comparison operations**: Changes to `<`, `>`, `<=`, `>=`, `==`, `!=` operators
- **Boolean logic**: Changes to `&&`, `||` operators
- **Assignment operations**: Changes to `+=`, `-=`, `*=`, `/=` operators
- **Return value mutations**: Functions returning different values than expected

### 2. Critical Areas Needing Better Test Coverage

Based on the mutations found, these areas need more comprehensive testing:

#### Layout Algorithms (`src/layout.rs`)
- **ForceDirectedLayout**: Mathematical calculations for forces, convergence detection
- **GridLayout**: Position calculations, grid cell assignments
- **HierarchicalLayout**: Tree traversal, position calculations
- **CircularLayout**: Angle calculations, position assignments

#### Spatial Indexing (`src/spatial.rs`)
- **SpatialIndex**: Grid cell calculations, query operations
- **Radius queries**: Distance calculations
- **Nearest neighbor**: Distance comparisons

#### Core Types (`src/types.rs`)
- **Position**: Distance calculations, arithmetic operations
- **Rect**: Intersection logic, bounds calculations
- **Viewport**: Coordinate transformations, bounds checking

### 3. Performance Considerations

Our current test suite is comprehensive but slow, causing timeouts in mutation testing. This suggests we need:

- **Faster unit tests** for core operations
- **Focused integration tests** for complex algorithms
- **Property-based tests** (which we've already implemented) for invariant validation

## Recommended Next Steps

### 1. Add Targeted Unit Tests

Create focused unit tests for the most critical mathematical operations:

```rust
#[cfg(test)]
mod mutation_tests {
    use super::*;

    #[test]
    fn test_position_distance_calculation() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        assert_eq!(p1.distance_to(p2), 5.0);
    }

    #[test]
    fn test_rect_intersection_logic() {
        let rect1 = Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(10.0, 10.0));
        let rect2 = Rect::from_pos_size(Position::new(5.0, 5.0), Size::new(10.0, 10.0));
        assert!(rect1.intersects(&rect2));
    }
}
```

### 2. Add Edge Case Tests

Focus on boundary conditions and edge cases that mutations might reveal:

```rust
#[test]
fn test_zero_distance() {
    let p1 = Position::new(1.0, 1.0);
    let p2 = Position::new(1.0, 1.0);
    assert_eq!(p1.distance_to(p2), 0.0);
}

#[test]
fn test_negative_coordinates() {
    let p1 = Position::new(-1.0, -1.0);
    let p2 = Position::new(1.0, 1.0);
    assert_eq!(p1.distance_to(p2), 2.0_f64.sqrt() * 2.0);
}
```

### 3. Optimize Test Performance

- Reduce test data sizes for faster execution
- Use `cargo test --release` for performance-critical tests
- Consider using `cargo nextest` for parallel test execution

### 4. Continuous Integration

Set up mutation testing in CI with:
- Limited scope (e.g., only critical functions)
- Longer timeouts
- Parallel execution
- Regular runs (e.g., nightly)

## Usage

### Running Mutation Tests

```bash
# List all possible mutations
cargo mutants --list

# Run mutation tests on specific file
cargo mutants --file "src/types.rs" --timeout 30

# Run mutation tests on specific function
cargo mutants --re "Position::distance_to" --timeout 15

# Run with custom configuration
cargo mutants --config .cargo/mutants.toml
```

### Interpreting Results

- **OK**: Mutation was caught by tests (good!)
- **MISSED**: Mutation was not caught by tests (needs better test coverage)
- **TIMEOUT**: Test execution timed out (needs optimization)
- **UNVIABLE**: Mutation caused compilation failure (expected)

## Conclusion

Mutation testing has revealed significant gaps in our test coverage, particularly around mathematical operations and edge cases. While our property-based tests provide excellent invariant validation, we need more targeted unit tests to catch the specific mutations that cargo-mutants identified.

The next priority should be adding focused unit tests for the most critical mathematical operations, especially in the layout algorithms and spatial indexing components.
