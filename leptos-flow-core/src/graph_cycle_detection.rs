//! Tests for Graph Cycle Detection system
//!
//! This module contains tests that define the expected behavior of cycle detection
//! before implementing the functionality (TDD Red phase).

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::types::Position;

    #[test]
    fn test_has_cycle_empty_graph() {
        // Test: Empty graph should have no cycles
        let graph: Graph<(), ()> = Graph::new();

        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_single_node() {
        // Test: Single node with no edges should have no cycles
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();

        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_single_edge() {
        // Test: Two nodes with single edge should have no cycles
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("node2", Position::new(200.0, 100.0))).unwrap();

        graph.add_edge(Edge::simple("edge1", "node1", "node2")).unwrap();

        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_simple_two_node_cycle() {
        // Test: Two nodes with bidirectional edges should form a cycle
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("node2", Position::new(200.0, 100.0))).unwrap();

        graph.add_edge(Edge::simple("edge1", "node1", "node2")).unwrap();
        graph.add_edge(Edge::simple("edge2", "node2", "node1")).unwrap();

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_three_node_cycle() {
        // Test: Three node cycle (A -> B -> C -> A) should be detected
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("nodeA", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeB", Position::new(200.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeC", Position::new(150.0, 200.0))).unwrap();

        graph.add_edge(Edge::simple("edge1", "nodeA", "nodeB")).unwrap();
        graph.add_edge(Edge::simple("edge2", "nodeB", "nodeC")).unwrap();
        graph.add_edge(Edge::simple("edge3", "nodeC", "nodeA")).unwrap();

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_self_loop() {
        // Test: Self-loop (node pointing to itself) should be detected as cycle
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_edge(Edge::simple("self_loop", "node1", "node1")).unwrap();

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_complex_graph_no_cycle() {
        // Test: Complex graph with multiple paths but no cycles
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a DAG (Directed Acyclic Graph)
        graph.add_node(Node::simple("root", Position::new(100.0, 50.0))).unwrap();
        graph.add_node(Node::simple("left", Position::new(50.0, 150.0))).unwrap();
        graph.add_node(Node::simple("right", Position::new(150.0, 150.0))).unwrap();
        graph.add_node(Node::simple("leaf1", Position::new(25.0, 250.0))).unwrap();
        graph.add_node(Node::simple("leaf2", Position::new(75.0, 250.0))).unwrap();
        graph.add_node(Node::simple("leaf3", Position::new(125.0, 250.0))).unwrap();
        graph.add_node(Node::simple("leaf4", Position::new(175.0, 250.0))).unwrap();

        graph.add_edge(Edge::simple("e1", "root", "left")).unwrap();
        graph.add_edge(Edge::simple("e2", "root", "right")).unwrap();
        graph.add_edge(Edge::simple("e3", "left", "leaf1")).unwrap();
        graph.add_edge(Edge::simple("e4", "left", "leaf2")).unwrap();
        graph.add_edge(Edge::simple("e5", "right", "leaf3")).unwrap();
        graph.add_edge(Edge::simple("e6", "right", "leaf4")).unwrap();

        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_has_cycle_complex_graph_with_cycle() {
        // Test: Complex graph with one cycle buried within
        let mut graph: Graph<(), ()> = Graph::new();

        // Create mostly DAG with one cycle
        graph.add_node(Node::simple("root", Position::new(100.0, 50.0))).unwrap();
        graph.add_node(Node::simple("left", Position::new(50.0, 150.0))).unwrap();
        graph.add_node(Node::simple("right", Position::new(150.0, 150.0))).unwrap();
        graph.add_node(Node::simple("middle", Position::new(100.0, 200.0))).unwrap();
        graph.add_node(Node::simple("leaf", Position::new(100.0, 300.0))).unwrap();

        // DAG structure
        graph.add_edge(Edge::simple("e1", "root", "left")).unwrap();
        graph.add_edge(Edge::simple("e2", "root", "right")).unwrap();
        graph.add_edge(Edge::simple("e3", "left", "middle")).unwrap();
        graph.add_edge(Edge::simple("e4", "right", "middle")).unwrap();
        graph.add_edge(Edge::simple("e5", "middle", "leaf")).unwrap();

        // Add cycle: leaf -> root (creates root -> ... -> leaf -> root cycle)
        graph.add_edge(Edge::simple("cycle_edge", "leaf", "root")).unwrap();

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_creates_cycle_adding_edge() {
        // Test: Check if adding specific edge would create a cycle
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("nodeA", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeB", Position::new(200.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeC", Position::new(150.0, 200.0))).unwrap();

        // Create partial cycle: A -> B -> C
        graph.add_edge(Edge::simple("edge1", "nodeA", "nodeB")).unwrap();
        graph.add_edge(Edge::simple("edge2", "nodeB", "nodeC")).unwrap();

        // Should not have cycle yet
        assert!(!graph.has_cycle());

        // Adding C -> A should create cycle
        assert!(graph.creates_cycle(&"nodeC".into(), &"nodeA".into()));

        // Adding C -> B should create cycle (B -> C -> B)
        assert!(graph.creates_cycle(&"nodeC".into(), &"nodeB".into()));

        // Adding A -> C should NOT create cycle (just another path)
        assert!(!graph.creates_cycle(&"nodeA".into(), &"nodeC".into()));
    }

    #[test]
    fn test_creates_cycle_nonexistent_nodes() {
        // Test: creates_cycle should handle nonexistent nodes gracefully
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();

        // Should return false for nonexistent source
        assert!(!graph.creates_cycle(&"nonexistent".into(), &"node1".into()));

        // Should return false for nonexistent target
        assert!(!graph.creates_cycle(&"node1".into(), &"nonexistent".into()));

        // Should return false for both nonexistent
        assert!(!graph.creates_cycle(&"nonexistent1".into(), &"nonexistent2".into()));
    }

    #[test]
    fn test_find_cycle_no_cycle() {
        // Test: find_cycle should return None for acyclic graph
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("node2", Position::new(200.0, 100.0))).unwrap();

        graph.add_edge(Edge::simple("edge1", "node1", "node2")).unwrap();

        let cycle = graph.find_cycle();
        assert!(cycle.is_none());
    }

    #[test]
    fn test_find_cycle_simple_cycle() {
        // Test: find_cycle should return the cycle path
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("nodeA", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeB", Position::new(200.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeC", Position::new(150.0, 200.0))).unwrap();

        graph.add_edge(Edge::simple("edge1", "nodeA", "nodeB")).unwrap();
        graph.add_edge(Edge::simple("edge2", "nodeB", "nodeC")).unwrap();
        graph.add_edge(Edge::simple("edge3", "nodeC", "nodeA")).unwrap();

        let cycle = graph.find_cycle().expect("Should find a cycle");

        // Cycle should contain all three nodes
        assert_eq!(cycle.len(), 3);
        assert!(cycle.contains(&"nodeA".into()));
        assert!(cycle.contains(&"nodeB".into()));
        assert!(cycle.contains(&"nodeC".into()));
    }

    #[test]
    fn test_find_cycle_self_loop() {
        // Test: find_cycle should detect self-loops
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();
        graph.add_edge(Edge::simple("self_loop", "node1", "node1")).unwrap();

        let cycle = graph.find_cycle().expect("Should find self-loop cycle");

        assert_eq!(cycle.len(), 1);
        assert_eq!(cycle[0], "node1".into());
    }

    #[test]
    fn test_find_cycle_multiple_cycles() {
        // Test: find_cycle should find at least one cycle when multiple exist
        let mut graph: Graph<(), ()> = Graph::new();

        // Create two separate cycles
        // Cycle 1: A -> B -> A
        graph.add_node(Node::simple("nodeA", Position::new(50.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeB", Position::new(100.0, 100.0))).unwrap();
        graph.add_edge(Edge::simple("edge1", "nodeA", "nodeB")).unwrap();
        graph.add_edge(Edge::simple("edge2", "nodeB", "nodeA")).unwrap();

        // Cycle 2: C -> D -> E -> C
        graph.add_node(Node::simple("nodeC", Position::new(200.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeD", Position::new(250.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeE", Position::new(225.0, 150.0))).unwrap();
        graph.add_edge(Edge::simple("edge3", "nodeC", "nodeD")).unwrap();
        graph.add_edge(Edge::simple("edge4", "nodeD", "nodeE")).unwrap();
        graph.add_edge(Edge::simple("edge5", "nodeE", "nodeC")).unwrap();

        let cycle = graph.find_cycle().expect("Should find a cycle");

        // Should find at least one of the cycles
        assert!(cycle.len() >= 2);

        // Verify it's a valid cycle (first cycle or second cycle)
        let is_first_cycle = cycle.len() == 2 &&
            cycle.contains(&"nodeA".into()) &&
            cycle.contains(&"nodeB".into());

        let is_second_cycle = cycle.len() == 3 &&
            cycle.contains(&"nodeC".into()) &&
            cycle.contains(&"nodeD".into()) &&
            cycle.contains(&"nodeE".into());

        assert!(is_first_cycle || is_second_cycle);
    }

    #[test]
    fn test_cycle_detection_performance() {
        // Test: Cycle detection should perform well on large graphs
        let mut graph: Graph<(), ()> = Graph::new();

        // Create a large DAG
        let node_count = 1000;
        for i in 0..node_count {
            graph.add_node(Node::simple(
                format!("node{}", i),
                Position::new(i as f64 * 10.0, i as f64 * 10.0)
            )).unwrap();
        }

        // Add edges to create long chains
        for i in 0..node_count-1 {
            graph.add_edge(Edge::simple(
                format!("edge{}", i),
                format!("node{}", i),
                format!("node{}", i + 1)
            )).unwrap();
        }

        // Measure performance
        let start_time = std::time::Instant::now();
        let has_cycle = graph.has_cycle();
        let elapsed = start_time.elapsed();

        assert!(!has_cycle); // Should be acyclic
        assert!(elapsed.as_millis() < 150, "Cycle detection took too long: {:?}", elapsed);
    }

    #[test]
    fn test_cycle_detection_with_disconnected_components() {
        // Test: Cycle detection should work with disconnected graph components
        let mut graph: Graph<(), ()> = Graph::new();

        // Component 1: Acyclic
        graph.add_node(Node::simple("comp1_a", Position::new(50.0, 50.0))).unwrap();
        graph.add_node(Node::simple("comp1_b", Position::new(100.0, 50.0))).unwrap();
        graph.add_edge(Edge::simple("comp1_edge", "comp1_a", "comp1_b")).unwrap();

        // Component 2: Has cycle
        graph.add_node(Node::simple("comp2_x", Position::new(200.0, 50.0))).unwrap();
        graph.add_node(Node::simple("comp2_y", Position::new(250.0, 50.0))).unwrap();
        graph.add_edge(Edge::simple("comp2_edge1", "comp2_x", "comp2_y")).unwrap();
        graph.add_edge(Edge::simple("comp2_edge2", "comp2_y", "comp2_x")).unwrap();

        // Should detect the cycle in component 2
        assert!(graph.has_cycle());

        let cycle = graph.find_cycle().expect("Should find cycle in disconnected components");
        assert!(cycle.contains(&"comp2_x".into()) || cycle.contains(&"comp2_y".into()));
    }

    #[test]
    fn test_topological_sort_acyclic() {
        // Test: Topological sort should work on DAG
        let mut graph: Graph<(), ()> = Graph::new();

        // Create simple DAG: A -> B -> C, A -> C
        graph.add_node(Node::simple("nodeA", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeB", Position::new(150.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeC", Position::new(200.0, 100.0))).unwrap();

        graph.add_edge(Edge::simple("edge1", "nodeA", "nodeB")).unwrap();
        graph.add_edge(Edge::simple("edge2", "nodeB", "nodeC")).unwrap();
        graph.add_edge(Edge::simple("edge3", "nodeA", "nodeC")).unwrap();

        let topo_sort = graph.topological_sort().expect("Should succeed on DAG");

        // Should contain all nodes
        assert_eq!(topo_sort.len(), 3);

        // A should come before B and C
        let a_pos = topo_sort.iter().position(|n| n == &"nodeA".into()).unwrap();
        let b_pos = topo_sort.iter().position(|n| n == &"nodeB".into()).unwrap();
        let c_pos = topo_sort.iter().position(|n| n == &"nodeC".into()).unwrap();

        assert!(a_pos < b_pos);
        assert!(a_pos < c_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_topological_sort_cyclic() {
        // Test: Topological sort should fail on cyclic graph
        let mut graph: Graph<(), ()> = Graph::new();

        graph.add_node(Node::simple("nodeA", Position::new(100.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeB", Position::new(150.0, 100.0))).unwrap();
        graph.add_node(Node::simple("nodeC", Position::new(200.0, 100.0))).unwrap();

        // Create cycle: A -> B -> C -> A
        graph.add_edge(Edge::simple("edge1", "nodeA", "nodeB")).unwrap();
        graph.add_edge(Edge::simple("edge2", "nodeB", "nodeC")).unwrap();
        graph.add_edge(Edge::simple("edge3", "nodeC", "nodeA")).unwrap();

        let result = graph.topological_sort();
        assert!(result.is_err());
    }
}
