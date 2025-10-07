#!/bin/bash

# Build script for Flow-RS Background Variants example
echo "Building Flow-RS Background Variants example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/background-variants/pkg/"
    echo "🌐 Serve with: cd examples/background-variants && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🎨 This example showcases:"
    echo "   • 3 background pattern types"
    echo "   • Interactive real-time controls"
    echo "   • Full color customization"
    echo "   • Size and opacity adjustments"
    echo "   • Live preview of changes"
else
    echo "❌ Build failed!"
    exit 1
fi
