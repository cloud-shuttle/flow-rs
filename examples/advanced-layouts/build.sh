#!/bin/bash

# Build script for Flow-RS Advanced Layouts example
echo "Building Flow-RS Advanced Layouts example..."

# Build with wasm-pack for web deployment
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📁 Output in: examples/advanced-layouts/pkg/"
    echo "🌐 Serve with: cd examples/advanced-layouts && python3 -m http.server 8000"
    echo "🔗 Open: http://localhost:8000"
    echo ""
    echo "🔬 This example showcases:"
    echo "   • 6 advanced layout algorithms (constrained force-directed, circular, radial, layered, spiral, grid)"
    echo "   • Interactive constraint management (pinning, grouping, alignment)"
    echo "   • Real-time layout performance benchmarking and comparison"
    echo "   • Complex demo graph with 14 nodes and 24 edges"
    echo "   • Layout quality metrics and convergence analysis"
    echo "   • Export/import functionality for layout data"
    echo "   • JavaScript API for programmatic layout control"
    echo "   • Constraint satisfaction algorithms and spatial optimization"
    echo ""
    echo "🎯 Advanced Layout Features:"
    echo "   • Constrained Force-Directed: Pinning and grouping with spatial constraints"
    echo "   • Circular Layout: Perfect circle arrangement for symmetric data"
    echo "   • Radial Layout: Hierarchical radial positioning from center"
    echo "   • Layered Layout: Automatic topological layering for flowcharts"
    echo "   • Custom Spiral: Artistic spiral arrangement for visual appeal"
    echo "   • Constrained Grid: Grid layout respecting pinning constraints"
    echo ""
    echo "📌 Constraint System:"
    echo "   • Node pinning: Lock nodes at specific positions"
    echo "   • Node grouping: Keep related nodes together"
    echo "   • Alignment rules: Horizontal/vertical alignment constraints"
    echo "   • Spacing rules: Minimum/maximum distance constraints"
    echo "   • Boundary constraints: Confine nodes to regions"
else
    echo "❌ Build failed!"
    exit 1
fi
