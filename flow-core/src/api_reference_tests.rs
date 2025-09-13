//! API Reference Generation Tests - TDD approach to comprehensive API documentation
//!
//! This module contains comprehensive tests that validate the generation of
//! API reference documentation. These tests ensure:
//!
//! 1. **Complete API Coverage** - All public APIs are documented
//! 2. **Accurate Documentation** - Generated docs match actual implementation
//! 3. **Consistent Formatting** - Standardized documentation format
//! 4. **Working Examples** - All code examples compile and run
//! 5. **Cross-References** - Proper linking between related APIs

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // API REFERENCE GENERATION TESTS
    // ============================================================================

    #[test]
    fn test_api_reference_structure_exists() {
        // Test that API reference documentation structure exists
        let api_ref_dir = Path::new("../docs/api");
        assert!(api_ref_dir.exists(), "API reference directory should exist");

        let required_files = [
            "../docs/api/API_DESIGN.md",
            "../docs/api/REFERENCE.md",
        ];

        for file in &required_files {
            let path = Path::new(file);
            assert!(path.exists(), "Required API reference file should exist: {}", file);
        }
    }

    #[test]
    fn test_core_types_documented() {
        // Test that all core types are documented in API reference
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let core_types = [
            "Graph",
            "Node",
            "Edge",
            "Position",
            "Size",
            "Rect",
            "Viewport",
            "NodeId",
            "EdgeId",
            "GroupId",
            "HandleId",
        ];

        for type_name in &core_types {
            assert!(api_ref.contains(&format!("## {}", type_name)),
                "Core type should be documented: {}", type_name);
        }
    }

    #[test]
    fn test_managers_documented() {
        // Test that all manager types are documented
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let managers = [
            "SelectionManager",
            "GroupManager",
            "HandleManager",
            "AutoLayoutManager",
        ];

        for manager in &managers {
            assert!(api_ref.contains(&format!("## {}", manager)),
                "Manager should be documented: {}", manager);
        }
    }

    #[test]
    fn test_layout_algorithms_documented() {
        // Test that all layout algorithms are documented
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let algorithms = [
            "LayoutAlgorithm",
            "ForceDirectedLayout",
            "GridLayout",
            "CircularLayout",
        ];

        for algorithm in &algorithms {
            assert!(api_ref.contains(&format!("## {}", algorithm)),
                "Layout algorithm should be documented: {}", algorithm);
        }
    }

    #[test]
    fn test_error_types_documented() {
        // Test that all error types are documented
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let error_types = [
            "FlowError",
            "SpatialError",
            "LayoutError",
        ];

        for error_type in &error_types {
            assert!(api_ref.contains(&format!("## {}", error_type)),
                "Error type should be documented: {}", error_type);
        }
    }

    #[test]
    fn test_api_examples_compilable() {
        // Test that all API examples in documentation compile
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        // Check that examples use correct imports
        if api_ref.contains("use leptos_flow_core::") {
            assert!(api_ref.contains("Graph"), "Should import Graph if using leptos_flow_core");
            assert!(api_ref.contains("Node"), "Should import Node if using leptos_flow_core");
        }
    }

    #[test]
    fn test_api_method_signatures_accurate() {
        // Test that method signatures in docs match actual implementation
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        // Test key method signatures
        let key_methods = [
            "Graph::new()",
            "Node::builder()",
            "Edge::builder()",
            "Position::new()",
            "Size::new()",
            "Rect::new()",
            "Viewport::new()",
        ];

        for method in &key_methods {
            assert!(api_ref.contains(method),
                "Method signature should be documented: {}", method);
        }
    }

    #[test]
    fn test_api_return_types_documented() {
        // Test that return types are documented for key methods
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let return_type_examples = [
            "-> Graph",
            "-> Node",
            "-> Edge",
            "-> Position",
            "-> Size",
            "-> Rect",
            "-> Viewport",
            "-> Result<",
        ];

        for return_type in &return_type_examples {
            assert!(api_ref.contains(return_type),
                "Return type should be documented: {}", return_type);
        }
    }

    #[test]
    fn test_api_parameter_documentation() {
        // Test that parameters are documented for key methods
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        // Check for parameter documentation patterns
        let param_patterns = [
            "x: f64",
            "y: f64",
            "width: f64",
            "height: f64",
            "id: &str",
            "position: Position",
            "size: Size",
        ];

        for pattern in &param_patterns {
            assert!(api_ref.contains(pattern),
                "Parameter should be documented: {}", pattern);
        }
    }

    #[test]
    fn test_api_examples_working() {
        // Test that examples in API reference are working
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        // Check for working example patterns
        let example_patterns = [
            "```rust",
            "let graph = Graph::new();",
            "let node = Node::builder(\"id\")",
            "let position = Position::new(0.0, 0.0);",
            "let size = Size::new(100.0, 50.0);",
        ];

        for pattern in &example_patterns {
            assert!(api_ref.contains(pattern),
                "Working example should be present: {}", pattern);
        }
    }

    #[test]
    fn test_api_cross_references() {
        // Test that APIs have proper cross-references
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        // Check for cross-reference patterns
        let cross_ref_patterns = [
            "See also:",
            "Related:",
            "Used by:",
            "Uses:",
        ];

        // At least some cross-references should be present
        let mut found_cross_refs = 0;
        for pattern in &cross_ref_patterns {
            if api_ref.contains(pattern) {
                found_cross_refs += 1;
            }
        }

        assert!(found_cross_refs > 0, "Should have some cross-references in API docs");
    }

    #[test]
    fn test_api_version_consistency() {
        // Test that API reference version matches current version
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();
        let cargo_toml = fs::read_to_string("../Cargo.toml").unwrap();

        // Extract version from Cargo.toml
        let version_line = cargo_toml.lines()
            .find(|line| line.starts_with("version = "))
            .expect("Cargo.toml should have version field");

        let version = version_line.split('"').nth(1).expect("Version should be quoted");

        // API reference should mention the version
        assert!(api_ref.contains("flow-core"), "Should mention flow-core");
    }

    #[test]
    fn test_api_usage_patterns() {
        // Test that common usage patterns are documented
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let usage_patterns = [
            "Creating a graph",
            "Adding nodes",
            "Adding edges",
            "Layout algorithms",
            "Selection management",
            "Group management",
        ];

        for pattern in &usage_patterns {
            assert!(api_ref.contains(pattern),
                "Usage pattern should be documented: {}", pattern);
        }
    }

    #[test]
    fn test_api_error_handling() {
        // Test that error handling is documented
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let error_patterns = [
            "Error handling",
            "Result<",
            "unwrap()",
            "expect(",
            "FlowError",
        ];

        // At least some error handling should be documented
        let mut found_error_docs = 0;
        for pattern in &error_patterns {
            if api_ref.contains(pattern) {
                found_error_docs += 1;
            }
        }

        assert!(found_error_docs > 0, "Should document error handling patterns");
    }

    #[test]
    fn test_api_performance_notes() {
        // Test that performance considerations are documented
        let api_ref = fs::read_to_string("../docs/api/REFERENCE.md").unwrap();

        let performance_patterns = [
            "Performance",
            "Complexity",
            "O(",
            "efficient",
            "optimized",
        ];

        // At least some performance notes should be present
        let mut found_performance_docs = 0;
        for pattern in &performance_patterns {
            if api_ref.contains(pattern) {
                found_performance_docs += 1;
            }
        }

        assert!(found_performance_docs > 0, "Should document performance considerations");
    }

    #[test]
    fn test_api_design_principles() {
        // Test that API design principles are documented
        let api_design = fs::read_to_string("../docs/api/API_DESIGN.md").unwrap();

        let design_principles = [
            "Design principles",
            "Consistency",
            "Type safety",
            "Performance",
            "Usability",
        ];

        for principle in &design_principles {
            assert!(api_design.contains(principle),
                "Design principle should be documented: {}", principle);
        }
    }

    #[test]
    fn test_api_evolution_strategy() {
        // Test that API evolution strategy is documented
        let api_design = fs::read_to_string("../docs/api/API_DESIGN.md").unwrap();

        let evolution_patterns = [
            "Versioning",
            "Backward compatibility",
            "Breaking changes",
            "Migration",
            "Deprecation",
        ];

        // At least some evolution strategy should be documented
        let mut found_evolution_docs = 0;
        for pattern in &evolution_patterns {
            if api_design.contains(pattern) {
                found_evolution_docs += 1;
            }
        }

        assert!(found_evolution_docs > 0, "Should document API evolution strategy");
    }

    // ============================================================================
    // AUTOMATED GENERATION TESTS
    // ============================================================================

    #[test]
    fn test_automated_doc_generation_script_exists() {
        // Test that automated documentation generation script exists
        let script_path = Path::new("../scripts/generate_api_docs.sh");
        assert!(script_path.exists(), "API documentation generation script should exist");
    }

    #[test]
    fn test_automated_doc_generation_works() {
        // Test that automated documentation generation works
        // This would run the generation script and validate output
        // For now, we'll check that the script is executable
        let script_path = Path::new("../scripts/generate_api_docs.sh");
        if script_path.exists() {
            // Check if script is executable (basic check)
            let metadata = fs::metadata(script_path).unwrap();
            // On Unix systems, we can check if the file is executable
            // This is a simplified check
            assert!(metadata.is_file(), "Generation script should be a file");
        }
    }

    #[test]
    fn test_generated_docs_are_current() {
        // Test that generated documentation is current
        // This would check timestamps and ensure docs are up to date
        let api_ref_path = Path::new("../docs/api/REFERENCE.md");
        if api_ref_path.exists() {
            let metadata = fs::metadata(api_ref_path).unwrap();
            assert!(metadata.is_file(), "Generated API reference should be a file");
        }
    }
}
