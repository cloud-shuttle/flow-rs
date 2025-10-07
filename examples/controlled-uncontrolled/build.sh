#!/bin/bash

# Build script for Flow-RS Controlled vs Uncontrolled example
echo "Building Flow-RS Controlled vs Uncontrolled example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/controlled-uncontrolled/pkg/"
    echo "🌐 Serve with: cd examples/controlled-uncontrolled && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🎛️ This example showcases:"
    echo "   • Controlled mode (reactive state)"
    echo "   • Uncontrolled mode (internal state)"
    echo "   • Leptos signal integration"
    echo "   • Dynamic node addition"
    echo "   • State management patterns"
else
    echo "❌ Build failed!"
    exit 1
fi
