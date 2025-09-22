//! Custom generators for property-based testing

use proptest::prelude::*;

use crate::{
    types::{EdgeId, NodeId},
    Edge, Graph, Node, Position, Rect, Size, Viewport,
};

/// Generate valid positions within reasonable bounds
pub fn arb_position() -> impl Strategy<Value = Position> {
    (-1000.0..1000.0, -1000.0..1000.0).prop_map(|(x, y)| Position::new(x, y))
}

/// Generate valid sizes (positive dimensions)
pub fn arb_size() -> impl Strategy<Value = Size> {
    (1.0..1000.0, 1.0..1000.0).prop_map(|(width, height)| Size::new(width, height))
}

/// Generate valid rectangles
pub fn arb_rect() -> impl Strategy<Value = Rect> {
    (arb_position(), arb_size()).prop_map(|(pos, size)| Rect::from_pos_size(pos, size))
}

/// Generate valid viewports
pub fn arb_viewport() -> impl Strategy<Value = Viewport> {
    (arb_position(), arb_size(), 0.1..10.0)
        .prop_map(|(pos, size, zoom)| Viewport::new(pos.x, pos.y, size.width, size.height, zoom))
}

/// Generate valid node IDs
pub fn arb_node_id() -> impl Strategy<Value = NodeId> {
    prop::string::string_regex("[a-zA-Z][a-zA-Z0-9_-]{2,8}") // Ensure minimum length to reduce collisions
        .unwrap()
        .prop_map(|s| NodeId::new(s))
}

/// Generate valid edge IDs
pub fn arb_edge_id() -> impl Strategy<Value = EdgeId> {
    prop::string::string_regex("[a-zA-Z][a-zA-Z0-9_-]*")
        .unwrap()
        .prop_map(|s| EdgeId::new(s))
}

/// Generate nodes with valid properties
pub fn arb_node() -> impl Strategy<Value = Node<()>> {
    (arb_node_id(), arb_position(), arb_size()).prop_map(|(id, position, size)| {
        let mut node = Node::new(id, position, ());
        node.size = size;
        node
    })
}

/// Generate edges with valid properties
pub fn arb_edge() -> impl Strategy<Value = Edge<()>> {
    (arb_edge_id(), arb_node_id(), arb_node_id())
        .prop_filter("edges cannot be self-connections", |(_, source, target)| {
            source != target
        })
        .prop_map(|(id, source, target)| Edge::new(id, source, target, ()))
}

/// Generate graphs with nodes and edges
pub fn arb_graph() -> impl Strategy<Value = Graph<(), ()>> {
    prop::collection::vec(arb_node(), 0..10) // Reduce size to avoid too many rejections
        .prop_flat_map(|nodes| {
            // Ensure unique node IDs by using a more robust approach
            let mut unique_nodes = Vec::new();
            let mut used_ids = std::collections::HashSet::new();
            for (i, node) in nodes.iter().enumerate() {
                let mut unique_node = node.clone();
                // Create a guaranteed unique ID
                let base_id = node.id.as_str();
                let mut unique_id = format!("{}_{}", base_id, i);
                let mut counter = 0;
                while used_ids.contains(&unique_id) {
                    counter += 1;
                    unique_id = format!("{}_{}_{}", base_id, i, counter);
                }
                used_ids.insert(unique_id.clone());
                unique_node.id = NodeId::new(unique_id);
                unique_nodes.push(unique_node);
            }

            let node_count = unique_nodes.len();
            let node_ids: Vec<NodeId> = unique_nodes.iter().map(|n| n.id.clone()).collect();

            let edges_strategy = if node_count == 0 {
                // No nodes, so no edges possible
                Just(Vec::new()).boxed()
            } else {
                prop::collection::vec(
                    // Generate edges that are guaranteed to connect existing nodes
                    (0..node_count, 0..node_count)
                        .prop_filter(
                            "edges cannot be self-connections",
                            |(source_idx, target_idx)| source_idx != target_idx,
                        )
                        .prop_map(move |(source_idx, target_idx)| {
                            Edge::new(
                                EdgeId::generate(),
                                node_ids[source_idx].clone(),
                                node_ids[target_idx].clone(),
                                (),
                            )
                        }),
                    0..=node_count.max(1).saturating_sub(1), // Allow fewer edges to reduce rejections
                )
                .boxed()
            };

            edges_strategy.prop_map(move |edges| {
                let mut graph = Graph::new();
                for node in &unique_nodes {
                    let _ = graph.add_node(node.clone());
                }
                for edge in &edges {
                    let _ = graph.add_edge(edge.clone());
                }
                graph
            })
        })
}

/// Generate connected graphs (graphs where all nodes are reachable)
pub fn arb_connected_graph() -> impl Strategy<Value = Graph<(), ()>> {
    prop::collection::vec(arb_node(), 1..10)
        .prop_flat_map(|nodes| {
            // Ensure unique node IDs by using a more robust approach
            let mut unique_nodes = Vec::new();
            let mut used_ids = std::collections::HashSet::new();
            for (i, node) in nodes.iter().enumerate() {
                let mut unique_node = node.clone();
                // Create a guaranteed unique ID
                let base_id = node.id.as_str();
                let mut unique_id = format!("{}_{}", base_id, i);
                let mut counter = 0;
                while used_ids.contains(&unique_id) {
                    counter += 1;
                    unique_id = format!("{}_{}_{}", base_id, i, counter);
                }
                used_ids.insert(unique_id.clone());
                unique_node.id = NodeId::new(unique_id);
                unique_nodes.push(unique_node);
            }

            let node_count = unique_nodes.len();
            let node_ids: Vec<NodeId> = unique_nodes.iter().map(|n| n.id.clone()).collect();

            // Create a connected graph by ensuring each node (except the first) has at least one edge
            // We'll create a simple chain: node[0] -> node[1] -> node[2] -> ...
            let mut edges = Vec::new();
            for i in 0..(node_count - 1) {
                edges.push(Edge::new(
                    EdgeId::generate(),
                    node_ids[i].clone(),
                    node_ids[i + 1].clone(),
                    (),
                ));
            }

            // Add some additional random edges to make it more interesting
            let additional_edges = prop::collection::vec(
                (0..node_count, 0..node_count)
                    .prop_filter("edges cannot be self-connections", |(source_idx, target_idx)| {
                        source_idx != target_idx
                    })
                    .prop_map(move |(source_idx, target_idx)| {
                        Edge::new(
                            EdgeId::generate(),
                            node_ids[source_idx].clone(),
                            node_ids[target_idx].clone(),
                            (),
                        )
                    }),
                0..=node_count.max(1) / 2, // Add fewer additional edges to reduce rejections
            );

            additional_edges.prop_map(move |additional_edges| {
                let mut graph = Graph::new();
                for node in &unique_nodes {
                    let _ = graph.add_node(node.clone());
                }
                for edge in &edges {
                    let _ = graph.add_edge(edge.clone());
                }
                for edge in &additional_edges {
                    let _ = graph.add_edge(edge.clone());
                }
                graph
            })
        })
}
