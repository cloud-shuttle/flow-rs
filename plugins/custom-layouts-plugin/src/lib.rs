//! Example Plugin: Custom Layout Algorithms
//!
//! Demonstrates how to create custom layout algorithms:
//! - Circular layouts for radial arrangements
//! - Grid layouts with custom spacing
//! - Hierarchical layouts for tree structures
//! - Force-directed layouts with custom forces

use flow_rs_core::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::f64::consts::PI;

/// Plugin implementation
pub struct CustomLayoutsPlugin {
    metadata: PluginMetadata,
    capabilities: PluginCapabilities,
    layout_engines: HashMap<String, Arc<dyn LayoutEngine>>,
}

impl CustomLayoutsPlugin {
    pub fn new() -> Self {
        let mut layout_engines = HashMap::new();

        // Register built-in layout algorithms
        layout_engines.insert(
            "circular-layout".to_string(),
            Arc::new(CircularLayoutEngine::new()) as Arc<dyn LayoutEngine>,
        );

        layout_engines.insert(
            "grid-layout-custom".to_string(),
            Arc::new(CustomGridLayoutEngine::new()) as Arc<dyn LayoutEngine>,
        );

        layout_engines.insert(
            "radial-tree-layout".to_string(),
            Arc::new(RadialTreeLayoutEngine::new()) as Arc<dyn LayoutEngine>,
        );

        layout_engines.insert(
            "spiral-layout".to_string(),
            Arc::new(SpiralLayoutEngine::new()) as Arc<dyn LayoutEngine>,
        );

        Self {
            metadata: PluginMetadata {
                id: "custom-layouts-plugin".to_string(),
                name: "Custom Layout Algorithms".to_string(),
                version: "1.0.0".to_string(),
                author: "Flow-RS Team".to_string(),
                description: "Advanced layout algorithms for specialized graph arrangements".to_string(),
                homepage: Some("https://flow-rs.dev".to_string()),
                repository: Some("https://github.com/flow-rs/plugins".to_string()),
                license: "MIT".to_string(),
                categories: vec![PluginCategory::Layout],
                dependencies: vec![],
                min_flow_rs_version: "0.1.0".to_string(),
            },
            capabilities: PluginCapabilities {
                provides_nodes: vec![],
                provides_layouts: vec![
                    "circular-layout".to_string(),
                    "grid-layout-custom".to_string(),
                    "radial-tree-layout".to_string(),
                    "spiral-layout".to_string(),
                ],
                provides_themes: vec![],
                provides_exports: vec![],
                provides_interactions: vec![],
                provides_backgrounds: vec![],
            },
            layout_engines,
        }
    }
}

impl Plugin for CustomLayoutsPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> &PluginCapabilities {
        &self.capabilities
    }

    fn initialize(&mut self, registry: &mut PluginRegistry) -> Result<(), PluginError> {
        // Register all layout engines
        for (name, engine) in &self.layout_engines {
            registry.register_layout_engine(name.clone(), Arc::clone(engine));
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn handle_message(&mut self, _message: PluginMessage) -> Result<Option<PluginMessage>, PluginError> {
        Ok(None)
    }
}

/// Circular layout engine
pub struct CircularLayoutEngine {
    radius: f64,
    center_offset: Position,
}

impl CircularLayoutEngine {
    pub fn new() -> Self {
        Self {
            radius: 200.0,
            center_offset: Position::new(400.0, 300.0), // Center of typical viewport
        }
    }
}

impl LayoutEngine for CircularLayoutEngine {
    fn plugin_id(&self) -> &str {
        "custom-layouts-plugin"
    }

    fn layout(&self, graph: &mut Graph<String, String>) -> Result<(), PluginError> {
        let node_count = graph.nodes().count();
        if node_count == 0 {
            return Ok(());
        }

        let angle_step = 2.0 * PI / node_count as f64;

        for (i, node) in graph.nodes_mut().enumerate() {
            let angle = i as f64 * angle_step;
            let x = self.center_offset.x + self.radius * angle.cos();
            let y = self.center_offset.y + self.radius * angle.sin();

            node.position = Position::new(x, y);
        }

        Ok(())
    }
}

/// Custom grid layout with configurable spacing
pub struct CustomGridLayoutEngine {
    columns: usize,
    horizontal_spacing: f64,
    vertical_spacing: f64,
    start_position: Position,
}

impl CustomGridLayoutEngine {
    pub fn new() -> Self {
        Self {
            columns: 4,
            horizontal_spacing: 150.0,
            vertical_spacing: 100.0,
            start_position: Position::new(100.0, 100.0),
        }
    }
}

impl LayoutEngine for CustomGridLayoutEngine {
    fn plugin_id(&self) -> &str {
        "custom-layouts-plugin"
    }

    fn layout(&self, graph: &mut Graph<String, String>) -> Result<(), PluginError> {
        let mut current_x = self.start_position.x;
        let mut current_y = self.start_position.y;
        let mut column_count = 0;

        for node in graph.nodes_mut() {
            node.position = Position::new(current_x, current_y);

            column_count += 1;
            if column_count >= self.columns {
                column_count = 0;
                current_x = self.start_position.x;
                current_y += self.vertical_spacing;
            } else {
                current_x += self.horizontal_spacing;
            }
        }

        Ok(())
    }
}

/// Radial tree layout for hierarchical data
pub struct RadialTreeLayoutEngine {
    radius_step: f64,
    angle_span: f64, // Radians
    center_position: Position,
}

impl RadialTreeLayoutEngine {
    pub fn new() -> Self {
        Self {
            radius_step: 80.0,
            angle_span: PI, // 180 degrees
            center_position: Position::new(400.0, 300.0),
        }
    }
}

impl LayoutEngine for RadialTreeLayoutEngine {
    fn plugin_id(&self) -> &str {
        "custom-layouts-plugin"
    }

    fn layout(&self, graph: &mut Graph<String, String>) -> Result<(), PluginError> {
        // Build adjacency list
        let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for edge in graph.edges() {
            adjacency.entry(edge.source.clone())
                .or_insert_with(Vec::new)
                .push(edge.target.clone());
        }

        // Find root nodes (nodes with no incoming edges)
        let mut has_incoming = HashSet::new();
        for edge in graph.edges() {
            has_incoming.insert(edge.target.clone());
        }

        let root_nodes: Vec<NodeId> = graph.nodes()
            .map(|n| n.id.clone())
            .filter(|id| !has_incoming.contains(id))
            .collect();

        if root_nodes.is_empty() {
            return Ok(()); // No clear root, fall back to circular
        }

        // Layout from root
        let mut visited = HashSet::new();
        let mut level_nodes = vec![root_nodes];

        while !level_nodes.is_empty() && !level_nodes.last().unwrap().is_empty() {
            let level = level_nodes.len() - 1;
            let current_level_nodes = level_nodes.last().unwrap();
            let radius = self.center_position.x.min(self.center_position.y) -
                        (level as f64 * self.radius_step);

            let angle_step = if current_level_nodes.len() == 1 {
                0.0
            } else {
                self.angle_span / (current_level_nodes.len() - 1) as f64
            };

            let start_angle = -self.angle_span / 2.0;

            for (i, node_id) in current_level_nodes.iter().enumerate() {
                if let Some(node) = graph.nodes_mut().find(|n| n.id == *node_id) {
                    let angle = start_angle + (i as f64 * angle_step);

                    if level == 0 {
                        // Root at center
                        node.position = self.center_position;
                    } else {
                        let x = self.center_position.x + radius * angle.cos();
                        let y = self.center_position.y + radius * angle.sin();
                        node.position = Position::new(x, y);
                    }
                }

                // Find children for next level
                if let Some(children) = adjacency.get(node_id) {
                    let mut next_level = Vec::new();
                    for child in children {
                        if visited.insert(child.clone()) {
                            next_level.push(child.clone());
                        }
                    }
                    if !next_level.is_empty() {
                        if level_nodes.len() == level + 1 {
                            level_nodes.push(next_level);
                        }
                    }
                }
            }

            level_nodes.pop();
        }

        Ok(())
    }
}

/// Spiral layout engine
pub struct SpiralLayoutEngine {
    start_radius: f64,
    radius_step: f64,
    angle_step: f64,
    center_position: Position,
}

impl SpiralLayoutEngine {
    pub fn new() -> Self {
        Self {
            start_radius: 50.0,
            radius_step: 20.0,
            angle_step: 0.8, // Radians
            center_position: Position::new(400.0, 300.0),
        }
    }
}

impl LayoutEngine for SpiralLayoutEngine {
    fn plugin_id(&self) -> &str {
        "custom-layouts-plugin"
    }

    fn layout(&self, graph: &mut Graph<String, String>) -> Result<(), PluginError> {
        let node_count = graph.nodes().count();
        if node_count == 0 {
            return Ok(());
        }

        for (i, node) in graph.nodes_mut().enumerate() {
            let angle = i as f64 * self.angle_step;
            let radius = self.start_radius + (i as f64 * self.radius_step);

            let x = self.center_position.x + radius * angle.cos();
            let y = self.center_position.y + radius * angle.sin();

            node.position = Position::new(x, y);
        }

        Ok(())
    }
}

/// Plugin entry point
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(CustomLayoutsPlugin::new()))
}

/// Free plugin (called by host)
#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = CustomLayoutsPlugin::new();
        assert_eq!(plugin.metadata().id, "custom-layouts-plugin");
        assert_eq!(plugin.metadata().name, "Custom Layout Algorithms");
        assert!(plugin.capabilities().provides_layouts.contains(&"circular-layout".to_string()));
    }

    #[test]
    fn test_circular_layout() {
        let engine = CircularLayoutEngine::new();
        let mut graph = Graph::new();

        // Add some nodes
        for i in 0..5 {
            let node = Node::new(format!("node-{}", i), Position::new(0.0, 0.0), ());
            graph.add_node(node).unwrap();
        }

        engine.layout(&mut graph).unwrap();

        // Check that nodes are positioned in a circle
        for node in graph.nodes() {
            let dx = node.position.x - 400.0; // center x
            let dy = node.position.y - 300.0; // center y
            let distance = (dx * dx + dy * dy).sqrt();

            // Should be approximately at radius distance
            assert!((distance - 200.0).abs() < 1.0);
        }
    }

    #[test]
    fn test_grid_layout() {
        let engine = CustomGridLayoutEngine::new();
        let mut graph = Graph::new();

        // Add 6 nodes
        for i in 0..6 {
            let node = Node::new(format!("node-{}", i), Position::new(0.0, 0.0), ());
            graph.add_node(node).unwrap();
        }

        engine.layout(&mut graph).unwrap();

        let nodes: Vec<_> = graph.nodes().collect();

        // First 4 nodes should be in first row
        for i in 0..4 {
            assert_eq!(nodes[i].position.y, 100.0);
            assert_eq!(nodes[i].position.x, 100.0 + (i as f64 * 150.0));
        }

        // Next 2 nodes should be in second row
        assert_eq!(nodes[4].position.y, 200.0);
        assert_eq!(nodes[4].position.x, 100.0);
        assert_eq!(nodes[5].position.y, 200.0);
        assert_eq!(nodes[5].position.x, 250.0);
    }

    #[test]
    fn test_spiral_layout() {
        let engine = SpiralLayoutEngine::new();
        let mut graph = Graph::new();

        // Add some nodes
        for i in 0..3 {
            let node = Node::new(format!("node-{}", i), Position::new(0.0, 0.0), ());
            graph.add_node(node).unwrap();
        }

        engine.layout(&mut graph).unwrap();

        // Check that nodes are positioned in a spiral pattern
        let nodes: Vec<_> = graph.nodes().collect();

        // First node should be at start radius
        let dx1 = nodes[0].position.x - 400.0;
        let dy1 = nodes[0].position.y - 300.0;
        let dist1 = (dx1 * dx1 + dy1 * dy1).sqrt();
        assert!((dist1 - 50.0).abs() < 1.0);

        // Second node should be further out
        let dx2 = nodes[1].position.x - 400.0;
        let dy2 = nodes[1].position.y - 300.0;
        let dist2 = (dx2 * dx2 + dy2 * dy2).sqrt();
        assert!(dist2 > dist1);
    }
}
