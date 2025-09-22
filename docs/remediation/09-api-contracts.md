# P2: API Contracts and Breaking Change Detection ⚠️ NEEDS ATTENTION

## Issue Summary
The codebase has API contract tests but they are currently failing due to API changes during file decomposition. The tests need to be updated to match the current API implementations.

## Current Problems

### ✅ API Contract Tests Exist
- **Current**: Comprehensive API contract tests in `flow-core/src/api_contracts/`
- **Problem**: Tests are failing due to API changes during decomposition
- **Impact**: 77 compilation errors in API contract tests

### ⚠️ API Changes During Decomposition
- **Current**: Many method signatures changed during file decomposition
- **Problem**: Tests expect old API signatures
- **Impact**: Tests need to be updated to match current implementations

### ⚠️ Missing Method Implementations
- **Current**: Some methods referenced in tests don't exist
- **Problem**: Tests expect methods that were removed or renamed
- **Impact**: Need to either implement missing methods or update tests

## Implementation Plan

### Phase 1: API Contract Testing (2-3 days)
**Goal**: Create comprehensive API contract tests

```rust
// API contract tests for all public APIs
#[cfg(test)]
mod api_contracts {
    use super::*;
    
    #[test]
    fn test_graph_api_contracts() {
        // Test all public Graph methods
        let mut graph = Graph::new();
        
        // Test node operations
        let node = Node::simple("test", Position::new(0.0, 0.0));
        assert!(graph.add_node(node).is_ok());
        assert_eq!(graph.node_count(), 1);
        
        // Test edge operations
        let edge = Edge::simple("edge", "test", "test");
        assert!(graph.add_edge(edge).is_ok());
        assert_eq!(graph.edge_count(), 1);
        
        // Test iteration
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 1);
        
        let edges: Vec<_> = graph.edges().collect();
        assert_eq!(edges.len(), 1);
    }
    
    #[test]
    fn test_layout_api_contracts() {
        // Test all layout algorithm APIs
        let mut graph = Graph::new();
        // ... add test nodes and edges
        
        let mut layout = CircularLayout::new();
        assert!(layout.apply(&mut graph).is_ok());
        
        let mut layout = GridLayout::new();
        assert!(layout.apply(&mut graph).is_ok());
        
        let mut layout = ForceDirectedLayout::new();
        assert!(layout.apply(&mut graph).is_ok());
        
        let mut layout = HierarchicalLayout::new();
        assert!(layout.apply(&mut graph).is_ok());
    }
}
```

### Phase 2: Breaking Change Detection (1-2 days)
**Goal**: Implement automated breaking change detection

```rust
// Breaking change detection tests
#[cfg(test)]
mod breaking_change_detection {
    use super::*;
    
    #[test]
    fn test_api_stability() {
        // Test that all public APIs maintain their signatures
        // This test should fail if any public API changes
        
        // Graph API
        let _: fn() -> Graph<(), ()> = Graph::new;
        let _: fn(&mut Graph<(), ()>, Node<()>) -> Result<()> = Graph::add_node;
        let _: fn(&Graph<(), ()>) -> usize = Graph::node_count;
        
        // Layout API
        let _: fn() -> CircularLayout = CircularLayout::new;
        let _: fn(&mut CircularLayout, &mut Graph<(), ()>) -> Result<()> = CircularLayout::apply;
        
        // Selection API
        let _: fn() -> SelectionManager = SelectionManager::new;
        let _: fn(&mut SelectionManager, NodeId) = SelectionManager::select_node;
    }
}
```

### Phase 3: Documentation Contracts (1-2 days)
**Goal**: Ensure documentation matches actual API

```rust
// Documentation contract tests
#[cfg(test)]
mod documentation_contracts {
    use super::*;
    
    #[test]
    fn test_documentation_examples() {
        // Test that all documentation examples compile and work
        // This ensures documentation stays up-to-date
        
        // Example from Graph documentation
        let mut graph = Graph::new();
        let node = Node::simple("example", Position::new(100.0, 100.0));
        graph.add_node(node).unwrap();
        
        // Example from Layout documentation
        let mut layout = CircularLayout::new();
        layout.apply(&mut graph).unwrap();
        
        // Example from Selection documentation
        let mut selection = SelectionManager::new();
        selection.select_node("example".into());
    }
}
```

## Testing Requirements

### API Contract Tests
```rust
#[test]
fn test_all_public_apis() {
    // Test every public function, method, and struct
    // Ensure they work as documented
    // Verify return types and error conditions
}

#[test]
fn test_api_consistency() {
    // Test that similar APIs behave consistently
    // Verify naming conventions
    // Check error handling patterns
}
```

## Risk Assessment

**Low Risk**: API contract improvements
- Non-breaking changes to existing functionality
- Only adds testing and validation
- Improves long-term maintainability

## Success Criteria

- [ ] All public APIs have contract tests
- [ ] Breaking change detection implemented
- [ ] Documentation examples are tested
- [ ] API consistency validated
- [ ] Semver enforcement in place

## Implementation Timeline

- **Days 1-3**: API contract testing
- **Days 4-5**: Breaking change detection
- **Days 6-7**: Documentation contracts
- **Day 8**: Integration and validation

**Total**: 1 week (1 engineer)
