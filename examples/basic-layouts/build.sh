#!/bin/bash

# Build script for Flow-RS Basic Layouts example
echo "Building Flow-RS Basic Layouts example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/basic-layouts/pkg/"
    echo "🌐 Serve with: cd examples/basic-layouts && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "📐 This example showcases:"
    echo "   • Force-directed layout algorithm"
    echo "   • Grid-based structured layout"
    echo "   • Hierarchical tree layout"
    echo "   • Interactive algorithm switching"
    echo "   • Real-time layout recalculation"
else
    echo "❌ Build failed!"
    exit 1
fi
