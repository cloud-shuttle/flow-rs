#!/bin/bash

# Build script for Flow-RS Real-time Data Flow example
echo "Building Flow-RS Real-time Data Flow example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/realtime-data-flow/pkg/"
    echo "🌐 Serve with: cd examples/realtime-data-flow && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "⚡ This example showcases:"
    echo "   • Real-time data streaming simulation"
    echo "   • Live graph updates with WebSocket-like behavior"
    echo "   • Interactive controls (start/stop, frequency, add streams)"
    echo "   • Performance monitoring and statistics"
    echo "   • Dynamic node and edge state changes"
    echo "   • WebAssembly real-time processing capabilities"
else
    echo "❌ Build failed!"
    exit 1
fi
