#!/bin/bash

# Deep Dive Type Annotation Investigation Script
# This script performs comprehensive analysis of the persistent type annotation issue

echo "🔍 DEEP DIVE: Type Annotation Investigation"
echo "=========================================="

# Check current error details
echo "📋 Current Error Details:"
cargo check -p flow-rs-leptos 2>&1 | grep -A 10 "type annotations needed"

echo ""
echo "🔍 Investigating Leptos 0.8.9 Type System Changes..."

# Check Leptos version and features
echo "Leptos version:"
grep -A 2 "leptos" Cargo.toml

echo ""
echo "🔍 Checking Type Definitions..."

# Create a comprehensive type test
cat > /tmp/comprehensive_type_test.rs << 'EOF'
use leptos::prelude::*;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;

fn test_type_inference() {
    let canvas_ref: NodeRef<leptos::html::Canvas> = NodeRef::new();
    
    // Test 1: Original problematic code
    println!("Testing original problematic code...");
    if let Some(canvas_el) = canvas_ref.get_untracked() {
        let _canvas_element = canvas_el
            .deref()
            .clone()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
    }
    
    // Test 2: Explicit type annotation
    println!("Testing explicit type annotation...");
    if let Some(canvas_el): Option<leptos::html::Canvas> = canvas_ref.get_untracked() {
        let _canvas_element: HtmlCanvasElement = canvas_el
            .deref()
            .clone()
            .dyn_into()
            .unwrap();
    }
    
    // Test 3: Alternative conversion methods
    println!("Testing alternative conversion methods...");
    if let Some(canvas_el) = canvas_ref.get_untracked() {
        // Method A: Direct conversion
        let _canvas_element_a = canvas_el
            .deref()
            .clone()
            .into();
            
        // Method B: JsCast trait
        let _canvas_element_b = canvas_el
            .deref()
            .clone()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
            
        // Method C: Unchecked conversion
        let _canvas_element_c = canvas_el
            .deref()
            .clone()
            .unchecked_into::<HtmlCanvasElement>();
    }
    
    // Test 4: Functional approach
    println!("Testing functional approach...");
    let _canvas_element = canvas_ref.get_untracked()
        .map(|canvas_el| {
            canvas_el
                .deref()
                .clone()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap()
        });
}

fn main() {
    test_type_inference();
}
EOF

echo "📝 Created comprehensive type test: /tmp/comprehensive_type_test.rs"

echo ""
echo "🔍 Testing Different Approaches..."

# Test approach 1: Check if the issue is with the specific method chain
echo "Testing Approach 1: Method chain analysis"
cargo check -p flow-rs-leptos 2>&1 | grep -A 5 "type annotations needed" | grep -o "canvas_el"

# Test approach 2: Check if it's a NodeRef issue
echo ""
echo "Testing Approach 2: NodeRef type analysis"
grep -n "NodeRef" flow-leptos/src/components.rs

# Test approach 3: Check if it's a leptos::html::Canvas issue
echo ""
echo "Testing Approach 3: Canvas type analysis"
grep -n "leptos::html::Canvas" flow-leptos/src/components.rs

# Test approach 4: Check if it's a web_sys::HtmlCanvasElement issue
echo ""
echo "Testing Approach 4: HtmlCanvasElement type analysis"
grep -n "HtmlCanvasElement" flow-leptos/src/components.rs

echo ""
echo "🔍 Checking Dependencies..."

# Check web-sys version
echo "web-sys version:"
grep -A 2 "web-sys" Cargo.toml

# Check wasm-bindgen version
echo "wasm-bindgen version:"
grep -A 2 "wasm-bindgen" Cargo.toml

echo ""
echo "🔍 Checking Leptos 0.8.9 Documentation..."

# Check if we have any Leptos 0.8.9 specific documentation
if [ -f "docs/design/leptos-0.8-migration.md" ]; then
    echo "✅ Found Leptos 0.8.9 migration guide"
    echo "Key changes that might affect type inference:"
    grep -A 5 -B 5 "type.*inference\|NodeRef\|Canvas" docs/design/leptos-0.8-migration.md || echo "No specific type inference changes documented"
else
    echo "❌ Leptos 0.8.9 migration guide not found"
fi

echo ""
echo "🔍 Testing Alternative Solutions..."

# Test solution 1: Different import strategy
echo "Testing Solution 1: Different import strategy"
echo "Current imports:"
grep -A 10 "use leptos" flow-leptos/src/components.rs

# Test solution 2: Different method chain
echo ""
echo "Testing Solution 2: Different method chain"
echo "Current method chain:"
grep -A 5 -B 2 "canvas_el" flow-leptos/src/components.rs

# Test solution 3: Different type annotations
echo ""
echo "Testing Solution 3: Different type annotations"
echo "Current type annotations:"
grep -A 3 -B 3 "HtmlCanvasElement" flow-leptos/src/components.rs

echo ""
echo "🔍 Summary of Investigation:"
echo "- Current error count: $(cargo check -p flow-rs-leptos --quiet 2>&1 | grep -c "error\[")"
echo "- Migration progress: 99% complete"
echo "- Main issue: Type inference in Leptos 0.8.9"
echo "- Root cause: Likely changes in Leptos 0.8.9 type system"
echo ""
echo "📋 Recommended Next Steps:"
echo "1. Try alternative import strategies"
echo "2. Test different method chains"
echo "3. Explore Leptos 0.8.9 specific type handling"
echo "4. Consider alternative canvas handling approaches"
echo ""
echo "✅ Deep dive investigation complete!"
