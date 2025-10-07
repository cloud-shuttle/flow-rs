#!/bin/bash

# Build script for Flow-RS Interactive Playground example
echo "Building Flow-RS Interactive Playground example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/interactive-playground/pkg/"
    echo "🌐 Serve with: cd examples/interactive-playground && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "⚡ This example showcases:"
    echo "   • Complete graph editing interface with toolbar and sidebar"
    echo "   • Multiple editing modes (Select, Create Node, Create Edge, Delete)"
    echo "   • Drag-and-drop node manipulation with optional grid snapping"
    echo "   • Property panels for editing node and edge attributes"
    echo "   • Export/import functionality with JSON serialization"
    echo "   • Keyboard shortcuts (Delete, Ctrl+S, Ctrl+Z, Esc)"
    echo "   • Real-time statistics and console logging"
    echo "   • JavaScript API for programmatic control"
    echo "   • Professional UI with responsive design"
    echo "   • Multi-selection and bulk operations"
    echo "   • Extensible architecture for custom node types"
else
    echo "❌ Build failed!"
    exit 1
fi
