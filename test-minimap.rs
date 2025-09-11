// Standalone test file for MiniMap component logic
use leptos_flow_core::{Graph, NodeBuilder, EdgeBuilder, Position};

/// MiniMap configuration for testing
#[derive(Debug, Clone)]
pub struct MiniMapConfig {
    pub width: f64,
    pub height: f64,
    pub background_color: String,
    pub border_color: String,
    pub viewport_color: String,
    pub node_color: String,
    pub edge_color: String,
    pub scale_factor: f64,
}

impl Default for MiniMapConfig {
    fn default() -> Self {
        Self {
            width: 200.0,
            height: 150.0,
            background_color: "#f8f9fa".to_string(),
            border_color: "#dee2e6".to_string(),
            viewport_color: "#007bff".to_string(),
            node_color: "#6c757d".to_string(),
            edge_color: "#adb5bd".to_string(),
            scale_factor: 0.1,
        }
    }
}

/// Bounding box for graph bounds calculation
#[derive(Debug, Clone, Copy)]
struct Bounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Calculate the bounding box of all nodes in the graph
fn calculate_graph_bounds<N, E>(graph: &Graph<N, E>) -> Option<Bounds> {
    if graph.nodes().count() == 0 {
        return None;
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for node in graph.nodes() {
        let left = node.position.x - node.size.width / 2.0;
        let right = node.position.x + node.size.width / 2.0;
        let top = node.position.y - node.size.height / 2.0;
        let bottom = node.position.y + node.size.height / 2.0;

        min_x = min_x.min(left);
        min_y = min_y.min(top);
        max_x = max_x.max(right);
        max_y = max_y.max(bottom);
    }

    Some(Bounds {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
    })
}

/// Convert minimap coordinates to world coordinates
fn minimap_to_world_coords<N, E>(
    minimap_pos: Position,
    config: &MiniMapConfig,
    graph: &Graph<N, E>,
) -> Position {
    let bounds = calculate_graph_bounds(graph);
    if bounds.is_none() {
        return Position::zero();
    }
    let bounds = bounds.unwrap();

    let scale_x = config.width / bounds.width;
    let scale_y = config.height / bounds.height;
    let scale = scale_x.min(scale_y) * config.scale_factor;

    let offset_x = (config.width - bounds.width * scale) / 2.0 - bounds.x * scale;
    let offset_y = (config.height - bounds.height * scale) / 2.0 - bounds.y * scale;

    let world_x = (minimap_pos.x - offset_x) / scale;
    let world_y = (minimap_pos.y - offset_y) / scale;

    Position::new(world_x, world_y)
}

fn create_test_graph() -> Graph<(), ()> {
    let mut graph = Graph::new();

    graph.add_node(NodeBuilder::<()>::new("node1")
        .position(0.0, 0.0)
        .size(100.0, 50.0)
        .build()).unwrap();

    graph.add_node(NodeBuilder::<()>::new("node2")
        .position(200.0, 100.0)
        .size(100.0, 50.0)
        .build()).unwrap();

    graph.add_edge(EdgeBuilder::new()
        .connect("node1", "node2")
        .build()
        .unwrap()).unwrap();

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimap_config_default() {
        let config = MiniMapConfig::default();
        assert_eq!(config.width, 200.0);
        assert_eq!(config.height, 150.0);
        assert_eq!(config.scale_factor, 0.1);
        assert_eq!(config.background_color, "#f8f9fa");
        assert_eq!(config.viewport_color, "#007bff");
    }

    #[test]
    fn test_calculate_graph_bounds_empty() {
        let graph: Graph<(), ()> = Graph::new();
        let bounds = calculate_graph_bounds(&graph);
        assert!(bounds.is_none());
    }

    #[test]
    fn test_calculate_graph_bounds() {
        let graph = create_test_graph();
        let bounds = calculate_graph_bounds(&graph).unwrap();

        // Node1: center (0,0), size 100x50 -> bounds (-50,-25) to (50,25)
        // Node2: center (200,100), size 100x50 -> bounds (150,75) to (250,125)
        // Total bounds: (-50,-25) to (250,125) -> width=300, height=150

        assert_eq!(bounds.x, -50.0);
        assert_eq!(bounds.y, -25.0);
        assert_eq!(bounds.width, 300.0);
        assert_eq!(bounds.height, 150.0);
    }

    #[test]
    fn test_minimap_to_world_coords() {
        let graph = create_test_graph();
        let config = MiniMapConfig::default();

        // Test center of minimap maps to center of graph bounds
        let center_minimap = Position::new(config.width / 2.0, config.height / 2.0);
        let world_pos = minimap_to_world_coords(center_minimap, &config, &graph);

        // Should map to approximately the center of graph bounds
        let bounds = calculate_graph_bounds(&graph).unwrap();
        let expected_center_x = bounds.x + bounds.width / 2.0;
        let expected_center_y = bounds.y + bounds.height / 2.0;

        assert!((world_pos.x - expected_center_x).abs() < 1.0);
        assert!((world_pos.y - expected_center_y).abs() < 1.0);
    }

    #[test]
    fn test_minimap_config_builder_pattern() {
        let config = MiniMapConfig {
            width: 300.0,
            height: 200.0,
            scale_factor: 0.15,
            background_color: "#ffffff".to_string(),
            viewport_color: "#ff0000".to_string(),
            ..Default::default()
        };

        assert_eq!(config.width, 300.0);
        assert_eq!(config.height, 200.0);
        assert_eq!(config.scale_factor, 0.15);
        assert_eq!(config.background_color, "#ffffff");
        assert_eq!(config.viewport_color, "#ff0000");
        // Other fields should use defaults
        assert_eq!(config.border_color, "#dee2e6");
        assert_eq!(config.node_color, "#6c757d");
    }

    #[test]
    fn test_bounds_calculation_single_node() {
        let mut graph = Graph::new();
        graph.add_node(NodeBuilder::<()>::new("single")
            .position(100.0, 50.0)
            .size(80.0, 40.0)
            .build()).unwrap();

        let bounds = calculate_graph_bounds(&graph).unwrap();

        // Node at (100,50) with size 80x40
        // Bounds: (60,30) to (140,70) -> width=80, height=40
        assert_eq!(bounds.x, 60.0);
        assert_eq!(bounds.y, 30.0);
        assert_eq!(bounds.width, 80.0);
        assert_eq!(bounds.height, 40.0);
    }
}

fn main() {
    println!("Testing MiniMap logic...");

    // Run tests manually
    tests::test_minimap_config_default();
    println!("✅ test_minimap_config_default passed");

    tests::test_calculate_graph_bounds_empty();
    println!("✅ test_calculate_graph_bounds_empty passed");

    tests::test_calculate_graph_bounds();
    println!("✅ test_calculate_graph_bounds passed");

    tests::test_minimap_to_world_coords();
    println!("✅ test_minimap_to_world_coords passed");

    tests::test_minimap_config_builder_pattern();
    println!("✅ test_minimap_config_builder_pattern passed");

    tests::test_bounds_calculation_single_node();
    println!("✅ test_bounds_calculation_single_node passed");

    println!("🎉 All MiniMap tests passed!");
}
