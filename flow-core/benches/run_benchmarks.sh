#!/bin/bash

# Comprehensive benchmark runner for leptos-flow-core
# This script runs all benchmarks and generates reports

set -e

echo "🚀 Starting comprehensive benchmark suite for leptos-flow-core"
echo "=============================================================="

# Create results directory
mkdir -p benchmark_results
cd benchmark_results

# Function to run benchmarks with different configurations
run_benchmark() {
    local name=$1
    local features=$2
    local description=$3

    echo ""
    echo "📊 Running $description"
    echo "Features: $features"
    echo "----------------------------------------"

    if [ -n "$features" ]; then
        cargo bench --features "$features" --bench "$name" 2>&1 | tee "${name}_${features//,/_}.log"
    else
        cargo bench --bench "$name" 2>&1 | tee "${name}.log"
    fi

    echo "✅ Completed $description"
}

# Run graph operations benchmarks
run_benchmark "graph_operations" "" "Graph Operations Benchmarks"
run_benchmark "graph_operations" "serde" "Graph Operations Benchmarks (with Serde)"

# Run algorithm benchmarks
run_benchmark "algorithms" "" "Algorithm Performance Benchmarks"
run_benchmark "algorithms" "serde" "Algorithm Performance Benchmarks (with Serde)"

# Run configuration benchmark
run_benchmark "criterion" "" "Criterion Configuration Benchmark"

echo ""
echo "🎯 Running quick performance validation"
echo "======================================"

# Quick validation run
cargo bench --bench graph_operations -- --quick 2>&1 | tee quick_validation.log

echo ""
echo "📈 Generating benchmark summary"
echo "=============================="

# Create a summary report
cat > benchmark_summary.md << EOF
# Benchmark Results Summary

Generated on: $(date)

## Benchmarks Run

1. **Graph Operations** - Core graph data structure performance
2. **Algorithm Performance** - Layout and spatial indexing algorithms
3. **Configuration** - Criterion setup and validation

## Key Metrics

- Graph creation and operations
- Spatial index query performance
- Layout algorithm execution time
- Memory usage patterns
- Scaling characteristics

## Files Generated

- \`graph_operations.log\` - Graph operations benchmark results
- \`algorithms.log\` - Algorithm performance results
- \`criterion.log\` - Configuration benchmark results
- \`quick_validation.log\` - Quick performance validation

## Next Steps

1. Review benchmark results for performance regressions
2. Identify optimization opportunities
3. Set up continuous benchmarking in CI/CD
4. Monitor performance trends over time

EOF

echo "✅ Benchmark summary generated: benchmark_summary.md"

echo ""
echo "🎉 All benchmarks completed successfully!"
echo "Results are available in the benchmark_results/ directory"
echo ""
echo "To view results:"
echo "  cd benchmark_results"
echo "  cat benchmark_summary.md"
echo ""
echo "To run specific benchmarks:"
echo "  cargo bench --bench graph_operations"
echo "  cargo bench --bench algorithms"
echo ""
echo "To run with different features:"
echo "  cargo bench --features serde --bench graph_operations"
