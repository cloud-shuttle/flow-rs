#!/bin/bash

# Build script for Flow-RS Hello World example
echo "Building Flow-RS Hello World example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/hello-world/pkg/"
    echo "🌐 Serve with: cd examples/hello-world && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
else
    echo "❌ Build failed!"
    exit 1
fi
