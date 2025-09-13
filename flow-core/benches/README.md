# Performance Benchmarks

This directory contains comprehensive performance benchmarks for the `leptos-flow-core` crate using [Criterion.rs](https://github.com/bheisler/criterion.rs).

## Overview

The benchmark suite is designed to measure and track the performance of all major operations in the core library, providing insights into:

- **Graph Operations**: Node/edge creation, retrieval, and iteration
- **Spatial Indexing**: Query performance, updates, and scaling characteristics
- **Layout Algorithms**: Performance comparison across different layout types
- **Memory Usage**: Memory efficiency and allocation patterns
- **Scaling Behavior**: How performance changes with data size

## Benchmark Files

### `graph_operations.rs`
Comprehensive benchmarks for core graph data structures and operations:

- **Graph Creation**: Performance of creating graphs with varying node counts
- **Graph Operations**: Node/edge retrieval, bounds calculation, iteration
- **Spatial Index Operations**: Query performance, insert/remove/update operations
- **Layout Algorithms**: All 4 layout types (Force-directed, Grid, Circular, Hierarchical)
- **Memory Usage**: Memory consumption patterns
- **Scaling Characteristics**: Performance vs. graph density

### `algorithms.rs`
Focused benchmarks for algorithmic performance:

- **Force-Directed Convergence**: Performance vs. iteration count
- **Layout Algorithm Comparison**: Direct performance comparison
- **Spatial Index Scaling**: Performance across different data sizes
- **Graph Density Impact**: How edge density affects performance
- **Memory Efficiency**: Memory usage patterns
- **Edge Cases**: Performance with extreme values

### `criterion.rs`
Configuration benchmark for setting up consistent benchmark parameters.

## Running Benchmarks

### Basic Usage

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark file
cargo bench --bench graph_operations
cargo bench --bench algorithms

# Run with specific features
cargo bench --features serde --bench graph_operations
```

### Advanced Usage

```bash
# Run with custom configuration
cargo bench --bench graph_operations -- --sample-size 100 --measurement-time 30

# Run specific benchmark groups
cargo bench --bench graph_operations -- graph_creation
cargo bench --bench algorithms -- force_directed_convergence

# Generate HTML reports
cargo bench --bench graph_operations
# Reports are saved to target/criterion/
```

### Using the Benchmark Runner Script

```bash
# Run comprehensive benchmark suite
./benches/run_benchmarks.sh

# This will:
# - Run all benchmarks with different feature combinations
# - Generate summary reports
# - Create performance validation
```

## Benchmark Categories

### 1. Graph Operations
- **Creation**: Building graphs with 10-5000 nodes
- **Retrieval**: Node/edge lookup performance
- **Iteration**: Traversing all nodes/edges
- **Bounds**: Calculating graph boundaries

### 2. Spatial Indexing
- **Queries**: Rectangular and radius-based queries
- **Updates**: Node position updates
- **Scaling**: Performance with 100-50,000 nodes
- **Memory**: Memory usage patterns

### 3. Layout Algorithms
- **Force-Directed**: Performance vs. iterations (10-200)
- **Grid**: Grid-based positioning
- **Circular**: Circular arrangement
- **Hierarchical**: Tree-based layouts

### 4. Memory Efficiency
- **Allocation**: Memory usage for different graph sizes
- **Growth**: How memory scales with data
- **Comparison**: Memory efficiency across algorithms

### 5. Edge Cases
- **Empty Graphs**: Performance with no data
- **Single Nodes**: Minimal case performance
- **Large Coordinates**: Extreme position values
- **Small Coordinates**: Precision edge cases

## Performance Targets

Based on the benchmarks, we aim for:

- **Graph Creation**: < 1ms for 1000 nodes
- **Node Retrieval**: < 1μs per lookup
- **Spatial Queries**: < 10μs for 1000 nodes
- **Layout Algorithms**: < 100ms for 100 nodes
- **Memory Usage**: < 1MB for 10,000 nodes

## Interpreting Results

### Key Metrics

- **Time**: Execution time in nanoseconds
- **Throughput**: Operations per second
- **Memory**: Memory allocations and usage
- **Scaling**: How performance changes with size

### Performance Regression Detection

Criterion automatically detects performance regressions by comparing against previous runs. Look for:

- **Significant changes** in execution time
- **Memory usage increases**
- **Scaling behavior changes**

### HTML Reports

After running benchmarks, detailed HTML reports are generated in `target/criterion/`:

- **Performance graphs** showing trends over time
- **Statistical analysis** of results
- **Comparison tables** between different configurations
- **Regression detection** highlighting changes

## Continuous Benchmarking

### CI Integration

The benchmarks are designed to be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Performance Benchmarks
  run: |
    cargo bench --bench graph_operations
    cargo bench --bench algorithms
```

### Performance Monitoring

- **Baseline Establishment**: Run benchmarks on main branch
- **Regression Detection**: Compare PR performance against baseline
- **Trend Analysis**: Track performance over time
- **Alert System**: Notify on significant regressions

## Best Practices

### Benchmark Development

1. **Isolate Operations**: Each benchmark should test one specific operation
2. **Use Realistic Data**: Generate data similar to real-world usage
3. **Test Edge Cases**: Include boundary conditions and extreme values
4. **Measure Consistently**: Use the same measurement methodology
5. **Document Assumptions**: Clearly state what each benchmark measures

### Performance Optimization

1. **Profile First**: Use benchmarks to identify bottlenecks
2. **Measure Changes**: Always benchmark before/after optimizations
3. **Test Scaling**: Ensure optimizations work across different sizes
4. **Validate Results**: Verify that optimizations don't break correctness
5. **Document Trade-offs**: Record any performance vs. functionality trade-offs

## Troubleshooting

### Common Issues

1. **High Variance**: Increase sample size or measurement time
2. **Slow Benchmarks**: Reduce data size or use `--test` flag
3. **Memory Issues**: Check for memory leaks in benchmark setup
4. **Inconsistent Results**: Ensure system is not under load

### Debugging

```bash
# Run with debug output
RUST_LOG=debug cargo bench --bench graph_operations

# Profile with perf (Linux)
perf record cargo bench --bench graph_operations
perf report

# Memory profiling with valgrind
valgrind --tool=massif cargo bench --bench graph_operations
```

## Future Enhancements

- **WebAssembly Benchmarks**: Performance in WASM environment
- **Concurrent Operations**: Multi-threaded performance testing
- **Real-world Workloads**: Benchmarks based on actual usage patterns
- **Cross-platform Comparison**: Performance across different platforms
- **Automated Performance Testing**: Integration with CI/CD for regression detection

## Contributing

When adding new benchmarks:

1. **Follow Naming Conventions**: Use descriptive names for benchmark groups
2. **Include Documentation**: Document what each benchmark measures
3. **Test Edge Cases**: Include boundary conditions
4. **Update This README**: Keep documentation current
5. **Validate Results**: Ensure benchmarks produce meaningful data

## Resources

- [Criterion.rs Documentation](https://docs.rs/criterion/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Benchmarking Best Practices](https://github.com/bheisler/criterion.rs/wiki/Benchmarking-best-practices)
