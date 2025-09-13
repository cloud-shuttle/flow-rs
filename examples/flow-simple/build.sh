#!/bin/bash

# Simple Flow Example Build Script
# This script builds the WASM module and serves it locally

set -e

echo "🚀 Building Leptos Flow Simple Example..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it first:"
    echo "   cargo install wasm-pack"
    exit 1
fi

# Build the WASM module
echo "📦 Building WASM module..."
wasm-pack build --target web --out-dir pkg --dev

echo "✅ Build complete!"
echo ""
echo "🌐 To serve the example:"
echo "   python3 -m http.server 8080"
echo "   # or"
echo "   python -m http.server 8080"
echo ""
echo "Then open: http://localhost:8080"
echo ""
echo "📁 Files created:"
echo "   - pkg/simple_flow_example.js"
echo "   - pkg/simple_flow_example_bg.wasm"
echo "   - pkg/simple_flow_example.d.ts"
