#!/bin/bash

# Leptos 0.8 Migration Assessment Script
# This script analyzes the current Leptos usage and estimates migration complexity

set -e

echo "🔍 Leptos 0.8 Migration Assessment"
echo "=================================="

# Check current Leptos version
echo "📦 Current Leptos Version:"
grep -A 3 "leptos.*=" Cargo.toml | head -3

echo ""
echo "📊 Leptos Usage Analysis:"
echo "------------------------"

# Count Leptos API usage
echo "🔢 Leptos API Usage Statistics:"
echo "  - Direct leptos:: imports: $(grep -r "leptos::" flow-leptos/src --include="*.rs" | wc -l)"
echo "  - Component macros: $(grep -r "#\[component\]" flow-leptos/src --include="*.rs" | wc -l)"
echo "  - Signal usage: $(grep -r "create_signal\|RwSignal\|ReadSignal\|WriteSignal" flow-leptos/src --include="*.rs" | wc -l)"
echo "  - Effect usage: $(grep -r "create_effect" flow-leptos/src --include="*.rs" | wc -l)"
echo "  - Hook usage: $(grep -r "use_" flow-leptos/src --include="*.rs" | wc -l)"

echo ""
echo "📁 Files with Leptos Usage:"
find flow-leptos/src -name "*.rs" -exec grep -l "leptos::\|create_signal\|RwSignal\|#\[component\]" {} \; | wc -l | xargs echo "  - Files:"

echo ""
echo "🧪 Test Coverage:"
echo "  - Unit tests: $(find flow-leptos/src -name "*.rs" -exec grep -l "#\[test\]" {} \; | wc -l)"
echo "  - Test files: $(find flow-leptos/src -name "*test*.rs" | wc -l)"

echo ""
echo "📈 Migration Complexity Assessment:"
echo "----------------------------------"

# Calculate complexity score
LEPTOS_IMPORTS=$(grep -r "leptos::" flow-leptos/src --include="*.rs" | wc -l)
COMPONENTS=$(grep -r "#\[component\]" flow-leptos/src --include="*.rs" | wc -l)
SIGNALS=$(grep -r "create_signal\|RwSignal\|ReadSignal\|WriteSignal" flow-leptos/src --include="*.rs" | wc -l)
EFFECTS=$(grep -r "create_effect" flow-leptos/src --include="*.rs" | wc -l)

COMPLEXITY_SCORE=$((LEPTOS_IMPORTS + COMPONENTS * 2 + SIGNALS + EFFECTS))

echo "  - Complexity Score: $COMPLEXITY_SCORE"

if [ $COMPLEXITY_SCORE -lt 50 ]; then
    echo "  - Assessment: LOW COMPLEXITY"
    echo "  - Estimated Time: 3-5 days"
elif [ $COMPLEXITY_SCORE -lt 100 ]; then
    echo "  - Assessment: MEDIUM COMPLEXITY"
    echo "  - Estimated Time: 5-8 days"
else
    echo "  - Assessment: HIGH COMPLEXITY"
    echo "  - Estimated Time: 8-14 days"
fi

echo ""
echo "🎯 Migration Readiness:"
echo "----------------------"

# Check if project compiles
echo "  - Current compilation status:"
if cargo check -p flow-rs-leptos --quiet 2>/dev/null; then
    echo "    ✅ Compiles successfully"
else
    echo "    ❌ Compilation errors present"
fi

# Check test status
echo "  - Test status:"
if cargo test -p flow-rs-leptos --quiet 2>/dev/null; then
    echo "    ✅ Tests pass"
else
    echo "    ❌ Test failures present"
fi

echo ""
echo "📋 Migration Checklist:"
echo "----------------------"
echo "  □ Update Cargo.toml dependencies"
echo "  □ Fix import statements"
echo "  □ Update component macros"
echo "  □ Update signal usage"
echo "  □ Update effect system"
echo "  □ Update custom hooks"
echo "  □ Update tests"
echo "  □ Run full test suite"
echo "  □ Update documentation"
echo "  □ Performance validation"

echo ""
echo "🚀 Next Steps:"
echo "-------------"
echo "  1. Review migration design document"
echo "  2. Create migration branch"
echo "  3. Start with dependency update"
echo "  4. Follow phased migration approach"
echo "  5. Test thoroughly at each phase"

echo ""
echo "📚 Resources:"
echo "------------"
echo "  - Migration Design: docs/design/leptos-0.8-migration.md"
echo "  - Leptos 0.8.9 Release Notes: https://github.com/leptos-rs/leptos/releases"
echo "  - Leptos Documentation: https://leptos.dev/"

echo ""
echo "✅ Assessment Complete!"
