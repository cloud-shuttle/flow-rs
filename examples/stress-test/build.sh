#!/bin/bash

# Build script for Flow-RS Stress Test example
echo "Building Flow-RS Stress Test example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/stress-test/pkg/"
    echo "🌐 Serve with: cd examples/stress-test && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🎯 This example showcases:"
    echo "   • 100-1500+ node graph handling"
    echo "   • Real-time performance monitoring"
    echo "   • WebAssembly performance advantages"
    echo "   • Automated benchmarking suite"
    echo "   • Interactive graph size controls"
else
    echo "❌ Build failed!"
    exit 1
fi
