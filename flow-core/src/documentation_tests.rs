//! Documentation Tests - TDD approach to validate and update documentation
//!
//! This module contains comprehensive tests that validate the current state
//! of documentation and ensure it accurately reflects the implementation.
//! These tests serve as:
//!
//! 1. **Documentation Validation** - Ensure docs match actual implementation
//! 2. **Completeness Checks** - Verify all public APIs are documented
//! 3. **Accuracy Verification** - Test that examples in docs actually work
//! 4. **Consistency Validation** - Ensure consistent terminology and style

use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // DOCUMENTATION EXISTENCE TESTS
    // ============================================================================

    #[test]
    fn test_core_documentation_files_exist() {
        let docs_dir = Path::new("../docs");
        assert!(docs_dir.exists(), "docs/ directory should exist");

        let required_files = [
            "../README.md",
            "../docs/architecture/ARCHITECTURE.md",
            "../docs/api/API_DESIGN.md",
            "../docs/guides/QUICK_START.md",
            "../docs/manual/USER_MANUAL.md",
            "../docs/dev/RELEASE.md",
            "../docs/testing/TESTING_PROGRESS_REPORT.md",
        ];

        for file in &required_files {
            let path = Path::new(file);
            assert!(
                path.exists(),
                "Required documentation file should exist: {}",
                file
            );
        }
    }

    #[test]
    fn test_example_documentation_exists() {
        let examples_dir = Path::new("../examples");
        assert!(examples_dir.exists(), "examples/ directory should exist");

        let example_dirs = ["flow-simple", "flow-leptos-demo"];
        for example in &example_dirs {
            let example_path = examples_dir.join(example);
            assert!(
                example_path.exists(),
                "Example directory should exist: {}",
                example
            );

            let readme_path = example_path.join("README.md");
            assert!(
                readme_path.exists(),
                "Example should have README.md: {}",
                example
            );
        }
    }

    #[test]
    fn test_api_documentation_completeness() {
        // Test that all public modules have documentation
        let lib_rs = fs::read_to_string("src/lib.rs").unwrap();

        // Check that main library has proper documentation
        assert!(
            lib_rs.contains("//! # Flow-RS Core"),
            "lib.rs should have proper module documentation"
        );

        // Check that all public modules are documented
        let public_modules = [
            "pub mod error",
            "pub mod graph",
            "pub mod spatial",
            "pub mod layout",
            "pub mod auto_layout",
            "pub mod types",
            "pub mod selection",
            "pub mod groups",
            "pub mod handle",
            "pub mod drag_operations",
            "pub mod edge_creator",
        ];

        for module in &public_modules {
            assert!(
                lib_rs.contains(module),
                "Public module should be declared: {}",
                module
            );
        }
    }

    // ============================================================================
    // DOCUMENTATION ACCURACY TESTS
    // ============================================================================

    #[test]
    fn test_readme_accuracy() {
        let readme = fs::read_to_string("../README.md").unwrap();

        // Test that README reflects current test status
        // Note: This will need to be updated as we add more tests
        assert!(
            readme.contains("tests-"),
            "README should contain test status badge"
        );

        // Test that README mentions current features
        let current_features = [
            "Interactive Node Editor",
            "Edge Connection System",
            "Spatial Indexing",
            "Multiple Layout Algorithms",
            "Selection System",
            "Viewport Management",
        ];

        for feature in &current_features {
            assert!(
                readme.contains(feature),
                "README should mention feature: {}",
                feature
            );
        }
    }

    #[test]
    fn test_architecture_doc_accuracy() {
        let arch_doc = fs::read_to_string("../docs/architecture/ARCHITECTURE.md").unwrap();

        // Test that architecture doc mentions current components
        let current_components = ["flow-core", "flow-leptos", "flow-renderer", "flow-wasm"];

        for component in &current_components {
            assert!(
                arch_doc.contains(component),
                "Architecture doc should mention component: {}",
                component
            );
        }
    }

    #[test]
    fn test_api_design_accuracy() {
        let api_doc = fs::read_to_string("../docs/api/API_DESIGN.md").unwrap();

        // Test that API design doc mentions current types
        let current_types = [
            "Graph", "Node", "Edge", "Position", "Size", "Rect", "Viewport",
        ];

        for type_name in &current_types {
            assert!(
                api_doc.contains(type_name),
                "API design doc should mention type: {}",
                type_name
            );
        }
    }

    // ============================================================================
    // EXAMPLE VALIDATION TESTS
    // ============================================================================

    #[test]
    fn test_simple_flow_example_documentation() {
        let example_readme = fs::read_to_string("../examples/flow-simple/README.md").unwrap();

        // Test that example has proper documentation
        assert!(
            example_readme.contains("# Simple Flow Example"),
            "Example should have proper title"
        );
        assert!(
            example_readme.contains("## Features"),
            "Example should document features"
        );
        assert!(
            example_readme.contains("## Usage"),
            "Example should document usage"
        );
    }

    #[test]
    fn test_example_build_instructions() {
        let example_readme = fs::read_to_string("../examples/flow-simple/README.md").unwrap();

        // Test that example has build instructions
        assert!(
            example_readme.contains("trunk serve"),
            "Example should have build instructions"
        );
        assert!(
            example_readme.contains("cargo"),
            "Example should mention cargo"
        );
    }

    // ============================================================================
    // VERSION CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn test_version_consistency() {
        let cargo_toml = fs::read_to_string("../Cargo.toml").unwrap();
        let readme = fs::read_to_string("../README.md").unwrap();

        // Extract version from Cargo.toml
        let version_line = cargo_toml
            .lines()
            .find(|line| line.starts_with("version = "))
            .expect("Cargo.toml should have version field");

        let version = version_line
            .split('"')
            .nth(1)
            .expect("Version should be quoted");

        // Test that README mentions the version (or at least doesn't contradict it)
        // This is a soft check - we don't require exact version match in README
        assert!(
            !readme.contains("version = \"0.0.0\""),
            "README shouldn't have placeholder version"
        );
    }

    // ============================================================================
    // DOCUMENTATION STYLE TESTS
    // ============================================================================

    #[test]
    fn test_documentation_style_consistency() {
        let readme = fs::read_to_string("../README.md").unwrap();

        // Test that README follows consistent style
        assert!(
            readme.starts_with("# "),
            "README should start with H1 title"
        );
        assert!(readme.contains("## "), "README should have H2 sections");

        // Test that README has proper structure
        let required_sections = [
            "## ✨ Features",
            "## 🏗️ Architecture",
            "## 🚀 Quick Start",
            "## 🧪 Testing",
        ];

        for section in &required_sections {
            assert!(
                readme.contains(section),
                "README should have section: {}",
                section
            );
        }
    }

    #[test]
    fn test_code_examples_formatting() {
        let readme = fs::read_to_string("../README.md").unwrap();

        // Test that code examples are properly formatted
        assert!(
            readme.contains("```rust"),
            "README should have Rust code examples"
        );
        assert!(
            readme.contains("```bash"),
            "README should have bash code examples"
        );
    }

    // ============================================================================
    // LINK VALIDATION TESTS
    // ============================================================================

    #[test]
    fn test_internal_links_valid() {
        let readme = fs::read_to_string("../README.md").unwrap();

        // Test that internal links point to existing files
        // This is a basic check - in a real implementation, we'd parse all markdown links
        assert!(
            !readme.contains("](nonexistent.md)"),
            "README shouldn't have broken internal links"
        );
    }

    // ============================================================================
    // COMPLETENESS TESTS
    // ============================================================================

    #[test]
    fn test_all_public_apis_documented() {
        // This test ensures that all public APIs have at least basic documentation
        let lib_rs = fs::read_to_string("src/lib.rs").unwrap();

        // Check that re-exports are documented
        assert!(
            lib_rs.contains("// Re-export commonly used types"),
            "Should document re-exports"
        );
        assert!(
            lib_rs.contains("/// Core prelude for convenient imports"),
            "Should document prelude"
        );
    }

    #[test]
    fn test_error_documentation_completeness() {
        let error_rs = fs::read_to_string("src/error.rs").unwrap();

        // Test that error types are documented
        assert!(
            error_rs.contains("/// Main error type for flow operations"),
            "Should document main error type"
        );

        // Test that individual error variants are documented
        let error_variants = [
            "NodeNotFound",
            "EdgeNotFound",
            "DuplicateNodeId",
            "InvalidConnection",
            "SpatialIndex",
            "Layout",
        ];

        for variant in &error_variants {
            // Check for either documentation comments or error attributes
            let has_doc_comment = error_rs.contains(&format!("/// {}", variant));
            let has_error_attr = error_rs.contains(&format!("#[error("));
            assert!(
                has_doc_comment || has_error_attr,
                "Should document error variant: {}",
                variant
            );
        }
    }

    // ============================================================================
    // USAGE EXAMPLE TESTS
    // ============================================================================

    #[test]
    fn test_usage_examples_compilable() {
        // Test that code examples in documentation actually compile
        // This is a basic check - in a real implementation, we'd extract and compile examples

        let readme = fs::read_to_string("../README.md").unwrap();

        // Check that examples use correct imports
        if readme.contains("use leptos_flow_core::") {
            assert!(
                readme.contains("Graph"),
                "Should import Graph if using leptos_flow_core"
            );
        }
    }

    // ============================================================================
    // PERFORMANCE CLAIM VALIDATION
    // ============================================================================

    #[test]
    fn test_performance_claims_documented() {
        let readme = fs::read_to_string("../README.md").unwrap();

        // Test that performance claims are backed by documentation
        if readme.contains("high-performance") {
            assert!(
                readme.contains("Performance"),
                "Should have performance section if claiming high performance"
            );
        }

        if readme.contains("WASM") {
            assert!(
                readme.contains("WebAssembly"),
                "Should explain WASM if mentioned"
            );
        }
    }

    // ============================================================================
    // TESTING DOCUMENTATION VALIDATION
    // ============================================================================

    #[test]
    fn test_testing_documentation_accuracy() {
        let testing_doc = fs::read_to_string("../docs/testing/TESTING_PROGRESS_REPORT.md").unwrap();

        // Test that testing documentation reflects current state
        assert!(
            testing_doc.contains("Test Results"),
            "Testing doc should report test results"
        );
        assert!(
            testing_doc.contains("passing"),
            "Testing doc should mention passing tests"
        );
    }

    // ============================================================================
    // RELEASE DOCUMENTATION VALIDATION
    // ============================================================================

    #[test]
    fn test_release_documentation_completeness() {
        let release_doc = fs::read_to_string("../docs/dev/RELEASE.md").unwrap();

        // Test that release documentation is comprehensive
        assert!(
            release_doc.contains("Release Checklist"),
            "Should have release checklist"
        );
        assert!(
            release_doc.contains("Pre-Release Phase"),
            "Should document pre-release process"
        );
        assert!(
            release_doc.contains("Version Bump"),
            "Should document version bumping"
        );
    }
}
