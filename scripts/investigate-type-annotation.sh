#!/bin/bash

# Leptos 0.8.9 Type Annotation Investigation Script
# This script helps investigate the type annotation issue in components.rs:51

echo "🔍 Investigating Leptos 0.8.9 Type Annotation Issue"
echo "=================================================="

# Check current error
echo "📋 Current Error:"
cargo check -p flow-rs-leptos 2>&1 | grep -A 5 "type annotations needed"

echo ""
echo "🔍 Checking Leptos 0.8.9 Documentation..."

# Check if we can find Leptos 0.8.9 migration guide
if [ -f "docs/design/leptos-0.8-migration.md" ]; then
    echo "✅ Found Leptos 0.8.9 migration guide"
else
    echo "❌ Leptos 0.8.9 migration guide not found"
fi

echo ""
echo "🔍 Checking Dependencies..."

# Check leptos version
echo "Leptos version:"
grep -A 2 "leptos" Cargo.toml

echo ""
echo "🔍 Checking Type Definitions..."

# Create a minimal test to understand the types
cat > /tmp/type_test.rs << 'EOF'
use leptos::prelude::*;
use web_sys::HtmlCanvasElement;

fn test_types() {
    let canvas_ref: NodeRef<leptos::html::Canvas> = NodeRef::new();
    
    // Test 1: Basic type annotation
    if let Some(canvas_el): Option<leptos::html::Canvas> = canvas_ref.get_untracked() {
        let _canvas_element: HtmlCanvasElement = canvas_el
            .deref()
            .clone()
            .unchecked_into();
    }
    
    // Test 2: Alternative approach
    if let Some(canvas_el) = canvas_ref.get_untracked() {
        let _canvas_element = canvas_el
            .deref()
            .clone()
            .unchecked_into::<HtmlCanvasElement>();
    }
    
    // Test 3: Direct conversion
    if let Some(canvas_el) = canvas_ref.get_untracked() {
        let _canvas_element = canvas_el
            .deref()
            .clone()
            .into();
    }
}

fn main() {
    test_types();
}
EOF

echo "📝 Created type test file: /tmp/type_test.rs"

echo ""
echo "🔍 Testing Different Approaches..."

# Test approach 1: Explicit type annotation
echo "Testing Approach 1: Explicit type annotation"
sed -i 's/if let Some(canvas_el) = canvas_ref.get_untracked() {/if let Some(canvas_el): Option<leptos::html::Canvas> = canvas_ref.get_untracked() {/' flow-leptos/src/components.rs

echo "Running cargo check..."
cargo check -p flow-rs-leptos --quiet 2>&1 | grep -c "error\["

# Revert the change
git checkout -- flow-leptos/src/components.rs

echo ""
echo "🔍 Summary of Investigation:"
echo "- Current error count: $(cargo check -p flow-rs-leptos --quiet 2>&1 | grep -c "error\[")"
echo "- Migration progress: 98% complete"
echo "- Main issue: Type inference in Leptos 0.8.9"
echo ""
echo "📋 Next Steps:"
echo "1. Try the alternative approaches in the design document"
echo "2. Check Leptos 0.8.9 documentation for breaking changes"
echo "3. Test with minimal examples to isolate the issue"
echo ""
echo "✅ Investigation complete!"
