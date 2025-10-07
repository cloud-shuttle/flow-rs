#!/bin/bash

# Build script for Flow-RS Custom Edges example
echo "Building Flow-RS Custom Edges example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/custom-edges/pkg/"
    echo "🌐 Serve with: cd examples/custom-edges && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🔗 This example showcases:"
    echo "   • 4 edge routing algorithms"
    echo "   • 7 different color schemes"
    echo "   • 3 thickness variations"
    echo "   • Dashed and animated edges"
    echo "   • Multiple connection patterns"
else
    echo "❌ Build failed!"
    exit 1
fi
