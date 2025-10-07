#!/bin/bash

# Build script for Flow-RS Framework Integrations example
echo "Building Flow-RS Framework Integrations example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/framework-integrations/pkg/"
    echo "🌐 Serve with: cd examples/framework-integrations && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🔧 This example showcases:"
    echo "   • Integration patterns for 5 different Rust web frameworks"
    echo "   • Interactive framework comparison and selection"
    echo "   • Code generation for each integration pattern"
    echo "   • Detailed setup guides and best practices"
    echo "   • Framework-specific performance and feature analysis"
    echo "   • JavaScript API for programmatic exploration"
    echo "   • Educational resources for framework integration"
    echo "   • Troubleshooting guides and migration patterns"
    echo ""
    echo "🎯 Framework Integrations:"
    echo "   • Leptos: Reactive signals and fine-grained updates"
    echo "   • Yew: Component composition and enterprise features"
    echo "   • Dioxus: React-like hooks and cross-platform support"
    echo "   • Sycamore: High-performance reactive system"
    echo "   • Generic: Minimal overhead direct DOM integration"
else
    echo "❌ Build failed!"
    exit 1
fi
