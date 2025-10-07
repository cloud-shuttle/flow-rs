#!/bin/bash

# Build script for Flow-RS Custom Styles example
echo "Building Flow-RS Custom Styles example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/custom-styles/pkg/"
    echo "🌐 Serve with: cd examples/custom-styles && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🎨 This example showcases:"
    echo "   • 5 different node styles"
    echo "   • 3 shape types (rectangle, rounded, circle)"
    echo "   • 3 size variants (small, medium, large)"
    echo "   • Custom colors and labels"
else
    echo "❌ Build failed!"
    exit 1
fi
