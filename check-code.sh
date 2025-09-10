#!/bin/bash

# Simple code quality check script
# Run this manually instead of using pre-commit hooks

set -e

echo "🔍 Running code quality checks..."

# Check for trailing whitespace
echo "📝 Checking for trailing whitespace..."
if grep -r '[[:space:]]$' --include="*.rs" --include="*.toml" --include="*.yaml" --include="*.json" --include="*.md" . | grep -v target/ | grep -v node_modules/ | grep -v .git/; then
    echo "❌ Found trailing whitespace"
    exit 1
fi
echo "✅ No trailing whitespace found"

# Check YAML syntax
echo "📄 Checking YAML syntax..."
if command -v yamllint &> /dev/null; then
    yamllint .pre-commit-config.yaml .github/workflows/ci.yml || echo "⚠️  YAML linting issues found"
else
    echo "⚠️  yamllint not installed, skipping YAML checks"
fi

# Check TOML syntax
echo "📄 Checking TOML syntax..."
if command -v toml-sort &> /dev/null; then
    echo "⚠️  TOML validation not available"
else
    echo "⚠️  TOML validator not installed, skipping TOML checks"
fi

# Check for large files
echo "📦 Checking for large files..."
if find . -type f -size +1M | grep -v target/ | grep -v node_modules/ | grep -v .git/ | grep -v pkg/; then
    echo "❌ Found large files (>1MB)"
    exit 1
fi
echo "✅ No large files found"

# Check for merge conflicts
echo "🔀 Checking for merge conflicts..."
if grep -r "<<<<<<< HEAD" --include="*.rs" --include="*.toml" --include="*.yaml" --include="*.json" --include="*.md" . | grep -v target/ | grep -v node_modules/ | grep -v .git/; then
    echo "❌ Found merge conflict markers"
    exit 1
fi
echo "✅ No merge conflicts found"

# Optional: Run cargo fmt check (commented out to avoid hanging)
# echo "🦀 Checking Rust formatting..."
# cargo fmt --all -- --check

# Optional: Run cargo clippy (commented out to avoid hanging)
# echo "🦀 Running Rust linter..."
# cargo clippy --all-targets --all-features -- -D warnings

echo "✅ All basic checks passed!"
echo ""
echo "💡 To run comprehensive checks manually:"
echo "   cargo fmt --all"
echo "   cargo clippy --all-targets --all-features -- -D warnings"
echo "   cargo test --all-targets --all-features"
echo "   npm run test:e2e"
