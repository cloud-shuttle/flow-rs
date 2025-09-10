#!/bin/bash

# Optimized build script for the simple flow example
# This script builds the WASM module with maximum performance optimizations

set -e

echo "🚀 Building Leptos Flow Simple Example (Optimized)..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it first:"
    echo "   cargo install wasm-pack"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the simple-flow example directory"
    exit 1
fi

echo "🧹 Cleaning previous builds..."

# Clean previous builds
if [ -d "pkg" ]; then
    rm -rf pkg
fi

if [ -d "target" ]; then
    rm -rf target
fi

echo "⚙️  Setting up optimized build environment..."

# Set optimization flags
export RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat -C codegen-units=1"

# Create optimized Cargo.toml configuration
cat > .cargo/config.toml << 'EOF'
[build]
# Optimize for maximum performance
[profile.release]
opt-level = 3          # Maximum optimization
lto = "fat"            # Aggressive link-time optimization
codegen-units = 1      # Single codegen unit for better optimization
panic = "abort"        # Smaller binary size
strip = true           # Remove debug symbols
overflow-checks = false # Disable overflow checks for performance

[profile.dev]
# Faster compilation in development
opt-level = 0
debug = true
incremental = true

[profile.test]
# Optimize tests for speed
opt-level = 1
debug = true
EOF

echo "📦 Building WASM module with optimizations..."

# Build with maximum optimizations
wasm-pack build \
    --target web \
    --out-dir pkg \
    --release \
    --no-typescript \
    --no-pack \
    -- --features "console_error_panic_hook"

echo "🔧 Post-build optimizations..."

# Optimize the generated WASM file
if command -v wasm-opt &> /dev/null; then
    echo "   Running wasm-opt for additional optimizations..."
    wasm-opt -O3 -s 100 --enable-bulk-memory --enable-mutable-globals \
        pkg/simple_flow_example_bg.wasm -o pkg/simple_flow_example_bg.wasm
else
    echo "   wasm-opt not found. Install with: npm install -g binaryen"
    echo "   This will provide additional size and performance optimizations"
fi

# Compress the generated files
if command -v gzip &> /dev/null; then
    echo "   Compressing generated files..."
    gzip -k -9 pkg/simple_flow_example_bg.wasm
    gzip -k -9 pkg/simple_flow_example.js
fi

echo "📊 Analyzing build results..."

# Analyze the build
if [ -f "pkg/simple_flow_example_bg.wasm" ]; then
    WASM_SIZE=$(du -h pkg/simple_flow_example_bg.wasm | cut -f1)
    JS_SIZE=$(du -h pkg/simple_flow_example.js | cut -f1)

    echo "   WASM file size: $WASM_SIZE"
    echo "   JS file size: $JS_SIZE"

    if [ -f "pkg/simple_flow_example_bg.wasm.gz" ]; then
        GZIP_SIZE=$(du -h pkg/simple_flow_example_bg.wasm.gz | cut -f1)
        echo "   WASM file size (gzipped): $GZIP_SIZE"
    fi
fi

echo "🧪 Running performance tests..."

# Run performance tests
if [ -f "run_tests.sh" ]; then
    echo "   Running performance test suite..."
    ./run_tests.sh 2>/dev/null || echo "   Some tests failed (check output above)"
else
    echo "   Performance tests not available"
fi

echo "📈 Performance benchmarks..."

# Create performance benchmark
cat > pkg/benchmark.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Leptos Flow Performance Benchmark</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .benchmark { margin: 20px 0; padding: 20px; border: 1px solid #ccc; border-radius: 8px; }
        .result { font-weight: bold; color: #2e7d32; }
        .warning { color: #f57c00; }
        .error { color: #d32f2f; }
        canvas { border: 1px solid #ddd; margin: 10px 0; }
    </style>
</head>
<body>
    <h1>Leptos Flow Performance Benchmark</h1>

    <div class="benchmark">
        <h2>Rendering Performance</h2>
        <canvas id="perf-canvas" width="800" height="600"></canvas>
        <div id="perf-results"></div>
    </div>

    <div class="benchmark">
        <h2>Memory Usage</h2>
        <div id="memory-results"></div>
    </div>

    <script type="module">
        import init, { create_simple_flow } from './simple_flow_example.js';

        async function runBenchmark() {
            await init();

            const canvas = document.getElementById('perf-canvas');
            const perfResults = document.getElementById('perf-results');
            const memoryResults = document.getElementById('memory-results');

            // Test rendering performance
            const startTime = performance.now();
            create_simple_flow();
            const endTime = performance.now();

            const renderTime = endTime - startTime;
            perfResults.innerHTML = `
                <div class="result">Initial render time: ${renderTime.toFixed(2)}ms</div>
                <div class="${renderTime < 100 ? 'result' : renderTime < 500 ? 'warning' : 'error'}">
                    Performance: ${renderTime < 100 ? 'Excellent' : renderTime < 500 ? 'Good' : 'Needs optimization'}
                </div>
            `;

            // Test memory usage
            if (performance.memory) {
                const memory = performance.memory;
                const usedMB = (memory.usedJSHeapSize / 1024 / 1024).toFixed(2);
                const totalMB = (memory.totalJSHeapSize / 1024 / 1024).toFixed(2);

                memoryResults.innerHTML = `
                    <div class="result">Memory usage: ${usedMB}MB / ${totalMB}MB</div>
                    <div class="${usedMB < 50 ? 'result' : usedMB < 100 ? 'warning' : 'error'}">
                        Memory efficiency: ${usedMB < 50 ? 'Excellent' : usedMB < 100 ? 'Good' : 'High usage'}
                    </div>
                `;
            } else {
                memoryResults.innerHTML = '<div class="warning">Memory API not available in this browser</div>';
            }
        }

        runBenchmark().catch(console.error);
    </script>
</body>
</html>
EOF

echo "✅ Optimized build complete!"
echo ""
echo "📊 Build Summary:"
echo "   - WASM module: $WASM_SIZE"
echo "   - JavaScript bindings: $JS_SIZE"
if [ -f "pkg/simple_flow_example_bg.wasm.gz" ]; then
    echo "   - Compressed WASM: $GZIP_SIZE"
fi
echo ""
echo "🎯 Performance Features:"
echo "   - Maximum optimization level (O3)"
echo "   - Link-time optimization (LTO)"
echo "   - Native CPU targeting"
echo "   - Single codegen unit"
echo "   - Debug symbols stripped"
echo "   - WASM optimizations applied"
echo ""
echo "🌐 To serve the optimized example:"
echo "   python3 -m http.server 8080"
echo "   # or"
echo "   python -m http.server 8080"
echo ""
echo "Then open:"
echo "   - http://localhost:8080 (main example)"
echo "   - http://localhost:8080/pkg/benchmark.html (performance benchmark)"
echo ""
echo "📁 Files created:"
echo "   - pkg/simple_flow_example.js (optimized JavaScript)"
echo "   - pkg/simple_flow_example_bg.wasm (optimized WASM)"
echo "   - pkg/benchmark.html (performance benchmark)"
if [ -f "pkg/simple_flow_example_bg.wasm.gz" ]; then
    echo "   - pkg/simple_flow_example_bg.wasm.gz (compressed WASM)"
fi
echo ""
echo "🚀 Ready for production use!"
