#!/bin/bash

# Build script for Flow-RS Performance Comparison example
echo "Building Flow-RS Performance Comparison example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/performance-comparison/pkg/"
    echo "🌐 Serve with: cd examples/performance-comparison && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "⚡ This example showcases:"
    echo "   • Side-by-side Flow-RS vs xyflow performance"
    echo "   • Real-time benchmark metrics (render time, FPS, memory)"
    echo "   • Interactive graph size testing (100-1500 nodes)"
    echo "   • Automated benchmark suite with comparative analysis"
    echo "   • WebAssembly performance advantages quantification"
else
    echo "❌ Build failed!"
    exit 1
fi
