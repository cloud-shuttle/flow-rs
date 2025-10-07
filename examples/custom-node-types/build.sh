#!/bin/bash

# Build script for Flow-RS Custom Node Types example
echo "Building Flow-RS Custom Node Types example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/custom-node-types/pkg/"
    echo "🌐 Serve with: cd examples/custom-node-types && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "⚡ This example showcases:"
    echo "   • Custom node components with specialized behaviors"
    echo "   • Interactive buttons, sliders, toggles, progress bars"
    echo "   • Custom rendering for different node types"
    echo "   • Real-time animations and state changes"
    echo "   • Plugin-like architecture for extensibility"
    echo "   • Mouse interactions (hover, click, drag)"
else
    echo "❌ Build failed!"
    exit 1
fi
