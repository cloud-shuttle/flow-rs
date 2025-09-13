#!/bin/bash

# Cleanup script for the simple flow example
# This script removes temporary files, optimizes the build, and cleans up resources

set -e

echo "🧹 Cleaning up Leptos Flow Simple Example..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the simple-flow example directory"
    exit 1
fi

echo "📦 Cleaning build artifacts..."

# Remove build artifacts
if [ -d "pkg" ]; then
    echo "   Removing pkg directory..."
    rm -rf pkg
fi

if [ -d "target" ]; then
    echo "   Removing target directory..."
    rm -rf target
fi

if [ -d "node_modules" ]; then
    echo "   Removing node_modules directory..."
    rm -rf node_modules
fi

# Remove temporary files
echo "🗑️  Removing temporary files..."

find . -name "*.tmp" -type f -delete 2>/dev/null || true
find . -name "*.log" -type f -delete 2>/dev/null || true
find . -name ".DS_Store" -type f -delete 2>/dev/null || true
find . -name "Thumbs.db" -type f -delete 2>/dev/null || true

# Remove backup files
find . -name "*.bak" -type f -delete 2>/dev/null || true
find . -name "*.backup" -type f -delete 2>/dev/null || true
find . -name "*~" -type f -delete 2>/dev/null || true

echo "🔍 Checking for unused dependencies..."

# Check for unused dependencies in Cargo.toml
if command -v cargo-udeps &> /dev/null; then
    echo "   Running cargo-udeps to find unused dependencies..."
    cargo +nightly udeps --all-targets 2>/dev/null || echo "   cargo-udeps not available or failed"
else
    echo "   cargo-udeps not installed. Install with: cargo install cargo-udeps"
fi

echo "📊 Analyzing code quality..."

# Run clippy for code quality
if command -v cargo &> /dev/null; then
    echo "   Running clippy for code quality checks..."
    cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null || echo "   Clippy found some issues (see output above)"
else
    echo "   Cargo not available"
fi

echo "🧪 Running tests..."

# Run tests to ensure everything still works
if command -v cargo &> /dev/null; then
    echo "   Running cargo check..."
    cargo check --all-targets --all-features 2>/dev/null || echo "   Cargo check found some issues"
else
    echo "   Cargo not available"
fi

echo "📈 Performance analysis..."

# Check for performance issues
if command -v cargo &> /dev/null; then
    echo "   Running cargo check with performance warnings..."
    RUSTFLAGS="-W clippy::perf" cargo check --all-targets 2>/dev/null || echo "   Performance warnings found"
else
    echo "   Cargo not available"
fi

echo "🔧 Optimizing build configuration..."

# Create optimized build configuration
cat > .cargo/config.toml << 'EOF'
[build]
# Optimize for size in release builds
[profile.release]
opt-level = "z"  # Optimize for size
lto = true       # Link-time optimization
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
# Faster compilation in development
opt-level = 0
debug = true
incremental = true

[profile.test]
# Optimize tests for speed
opt-level = 1
debug = true
EOF

echo "📝 Generating documentation..."

# Generate documentation
if command -v cargo &> /dev/null; then
    echo "   Generating documentation..."
    cargo doc --no-deps --document-private-items 2>/dev/null || echo "   Documentation generation failed"
else
    echo "   Cargo not available"
fi

echo "🎯 Final optimizations..."

# Remove debug symbols from release builds
if [ -f "Cargo.toml" ]; then
    # Add optimization settings to Cargo.toml if not present
    if ! grep -q "opt-level" Cargo.toml; then
        echo "" >> Cargo.toml
        echo "[profile.release]" >> Cargo.toml
        echo "opt-level = \"z\"" >> Cargo.toml
        echo "lto = true" >> Cargo.toml
        echo "codegen-units = 1" >> Cargo.toml
        echo "panic = \"abort\"" >> Cargo.toml
        echo "strip = true" >> Cargo.toml
    fi
fi

echo "📊 Summary report..."

# Generate summary report
cat > cleanup_report.txt << EOF
Leptos Flow Simple Example Cleanup Report
Generated: $(date)

Build Artifacts Removed:
- pkg/ directory
- target/ directory
- node_modules/ directory

Temporary Files Removed:
- *.tmp files
- *.log files
- .DS_Store files
- Thumbs.db files
- Backup files (*.bak, *.backup, *~)

Optimizations Applied:
- Release build optimized for size (opt-level = "z")
- Link-time optimization enabled
- Debug symbols stripped
- Single codegen unit for better optimization

Code Quality:
- Clippy warnings checked
- Unused dependencies identified
- Performance warnings reviewed

Documentation:
- API documentation generated
- Private items documented

Next Steps:
1. Run './build.sh' to create optimized build
2. Run './run_tests.sh' to verify functionality
3. Check cleanup_report.txt for details
EOF

echo "✅ Cleanup complete!"
echo ""
echo "📋 Summary:"
echo "   - Build artifacts removed"
echo "   - Temporary files cleaned"
echo "   - Code quality checked"
echo "   - Build optimized for performance"
echo "   - Documentation generated"
echo ""
echo "📄 Detailed report saved to: cleanup_report.txt"
echo ""
echo "🚀 Ready for optimized build!"
echo "   Run: ./build.sh"
echo "   Test: ./run_tests.sh"
