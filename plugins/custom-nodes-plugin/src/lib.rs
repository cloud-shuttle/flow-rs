//! Example Plugin: Custom Node Types
//!
//! Demonstrates how to create custom node types with specialized behaviors:
//! - Button nodes with click handlers
//! - Slider nodes for value input
//! - Progress nodes for status display
//! - Toggle nodes for boolean states

use flow_rs_core::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Plugin implementation
pub struct CustomNodesPlugin {
    metadata: PluginMetadata,
    capabilities: PluginCapabilities,
    node_factories: HashMap<String, Arc<dyn NodeFactory>>,
}

impl CustomNodesPlugin {
    pub fn new() -> Self {
        let mut node_factories = HashMap::new();

        // Register built-in node types
        node_factories.insert(
            "button-node".to_string(),
            Arc::new(ButtonNodeFactory::new()) as Arc<dyn NodeFactory>,
        );

        node_factories.insert(
            "slider-node".to_string(),
            Arc::new(SliderNodeFactory::new()) as Arc<dyn NodeFactory>,
        );

        node_factories.insert(
            "progress-node".to_string(),
            Arc::new(ProgressNodeFactory::new()) as Arc<dyn NodeFactory>,
        );

        node_factories.insert(
            "toggle-node".to_string(),
            Arc::new(ToggleNodeFactory::new()) as Arc<dyn NodeFactory>,
        );

        Self {
            metadata: PluginMetadata {
                id: "custom-nodes-plugin".to_string(),
                name: "Custom Node Types".to_string(),
                version: "1.0.0".to_string(),
                author: "Flow-RS Team".to_string(),
                description: "Provides specialized interactive node types for enhanced UX".to_string(),
                homepage: Some("https://flow-rs.dev".to_string()),
                repository: Some("https://github.com/flow-rs/plugins".to_string()),
                license: "MIT".to_string(),
                categories: vec![PluginCategory::NodeType],
                dependencies: vec![],
                min_flow_rs_version: "0.1.0".to_string(),
            },
            capabilities: PluginCapabilities {
                provides_nodes: vec![
                    "button-node".to_string(),
                    "slider-node".to_string(),
                    "progress-node".to_string(),
                    "toggle-node".to_string(),
                ],
                provides_layouts: vec![],
                provides_themes: vec![],
                provides_exports: vec![],
                provides_interactions: vec![],
                provides_backgrounds: vec![],
            },
            node_factories,
        }
    }
}

impl Plugin for CustomNodesPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> &PluginCapabilities {
        &self.capabilities
    }

    fn initialize(&mut self, registry: &mut PluginRegistry) -> Result<(), PluginError> {
        // Register all node factories
        for (name, factory) in &self.node_factories {
            registry.register_node_factory(name.clone(), Arc::clone(factory));
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn handle_message(&mut self, message: PluginMessage) -> Result<Option<PluginMessage>, PluginError> {
        // Handle plugin-specific messages
        match message.message_type.as_str() {
            "button-clicked" => {
                println!("Button clicked: {}", message.payload);
            }
            "slider-changed" => {
                println!("Slider value changed: {}", message.payload);
            }
            "toggle-changed" => {
                println!("Toggle state changed: {}", message.payload);
            }
            _ => {}
        }

        Ok(None)
    }
}

/// Button node implementation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ButtonNode {
    pub id: NodeId,
    pub position: Position,
    pub size: Size,
    pub label: String,
    pub is_pressed: bool,
    pub style: ButtonStyle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Success,
}

impl ButtonNode {
    pub fn new(id: NodeId, position: Position, config: ButtonConfig) -> Self {
        Self {
            id,
            position,
            size: Size::new(120.0, 40.0),
            label: config.label,
            is_pressed: false,
            style: config.style,
        }
    }

    pub fn get_color(&self) -> &'static str {
        match self.style {
            ButtonStyle::Primary => "#007bff",
            ButtonStyle::Secondary => "#6c757d",
            ButtonStyle::Danger => "#dc3545",
            ButtonStyle::Success => "#28a745",
        }
    }
}

impl CustomNode for ButtonNode {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn node_type(&self) -> &str {
        "button-node"
    }

    fn render(&self, _context: &mut dyn std::any::Any) -> Result<(), PluginError> {
        // Rendering logic would go here for WASM
        // For now, this is a placeholder
        Ok(())
    }

    fn handle_event(&mut self, event: NodeEvent) -> Result<Option<NodeEventResult>, PluginError> {
        match event {
            NodeEvent::Click { .. } => {
                self.is_pressed = !self.is_pressed;

                // Send message to plugin system
                let message = PluginMessage {
                    from_plugin: self.plugin_id().to_string(),
                    to_plugin: None, // Broadcast
                    message_type: "button-clicked".to_string(),
                    payload: serde_json::json!({
                        "node_id": self.id,
                        "label": self.label,
                        "pressed": self.is_pressed
                    }),
                };

                Ok(Some(NodeEventResult::Custom(format!("button_clicked:{}", self.id))))
            }
            _ => Ok(None),
        }
    }

    fn get_bounds(&self) -> (Position, Size) {
        (self.position, self.size)
    }

    fn get_ports(&self) -> Vec<NodePort> {
        vec![
            NodePort {
                id: "click_out".to_string(),
                label: "Click".to_string(),
                position: Position::new(self.size.width, self.size.height / 2.0),
                port_type: PortType::Output,
            },
        ]
    }
}

/// Button node factory
pub struct ButtonNodeFactory;

impl ButtonNodeFactory {
    pub fn new() -> Self {
        Self
    }
}

impl NodeFactory for ButtonNodeFactory {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn create_node(&self, id: NodeId, position: Position, config: serde_json::Value) -> Result<Box<dyn CustomNode>, PluginError> {
        let button_config: ButtonConfig = serde_json::from_value(config)
            .map_err(|e| PluginError::InvalidConfiguration(format!("Invalid button config: {}", e)))?;

        Ok(Box::new(ButtonNode::new(id, position, button_config)))
    }
}

/// Button configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ButtonConfig {
    pub label: String,
    pub style: ButtonStyle,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            label: "Button".to_string(),
            style: ButtonStyle::Primary,
        }
    }
}

/// Slider node implementation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SliderNode {
    pub id: NodeId,
    pub position: Position,
    pub size: Size,
    pub label: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl SliderNode {
    pub fn new(id: NodeId, position: Position, config: SliderConfig) -> Self {
        Self {
            id,
            position,
            size: Size::new(200.0, 60.0),
            label: config.label,
            value: config.initial_value,
            min: config.min,
            max: config.max,
            step: config.step,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(self.min, self.max);
    }
}

impl CustomNode for SliderNode {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn node_type(&self) -> &str {
        "slider-node"
    }

    fn render(&self, _context: &mut dyn std::any::Any) -> Result<(), PluginError> {
        Ok(())
    }

    fn handle_event(&mut self, event: NodeEvent) -> Result<Option<NodeEventResult>, PluginError> {
        match event {
            NodeEvent::Click { position } | NodeEvent::DragMove { position, .. } => {
                // Calculate value based on click position
                let relative_x = position.x - self.position.x;
                let percentage = (relative_x / self.size.width).clamp(0.0, 1.0);
                let new_value = self.min + (self.max - self.min) * percentage;

                // Snap to step
                let stepped_value = ((new_value / self.step).round() * self.step).clamp(self.min, self.max);
                self.set_value(stepped_value);

                let message = PluginMessage {
                    from_plugin: self.plugin_id().to_string(),
                    to_plugin: None,
                    message_type: "slider-changed".to_string(),
                    payload: serde_json::json!({
                        "node_id": self.id,
                        "value": self.value,
                        "percentage": percentage
                    }),
                };

                Ok(Some(NodeEventResult::Custom(format!("slider_changed:{:.2}", self.value))))
            }
            _ => Ok(None),
        }
    }

    fn get_bounds(&self) -> (Position, Size) {
        (self.position, self.size)
    }

    fn get_ports(&self) -> Vec<NodePort> {
        vec![
            NodePort {
                id: "value_out".to_string(),
                label: "Value".to_string(),
                position: Position::new(self.size.width, self.size.height / 2.0),
                port_type: PortType::Output,
            },
        ]
    }
}

/// Slider node factory
pub struct SliderNodeFactory;

impl SliderNodeFactory {
    pub fn new() -> Self {
        Self
    }
}

impl NodeFactory for SliderNodeFactory {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn create_node(&self, id: NodeId, position: Position, config: serde_json::Value) -> Result<Box<dyn CustomNode>, PluginError> {
        let slider_config: SliderConfig = serde_json::from_value(config)
            .map_err(|e| PluginError::InvalidConfiguration(format!("Invalid slider config: {}", e)))?;

        Ok(Box::new(SliderNode::new(id, position, slider_config)))
    }
}

/// Slider configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SliderConfig {
    pub label: String,
    pub initial_value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl Default for SliderConfig {
    fn default() -> Self {
        Self {
            label: "Slider".to_string(),
            initial_value: 50.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
        }
    }
}

/// Progress node implementation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressNode {
    pub id: NodeId,
    pub position: Position,
    pub size: Size,
    pub label: String,
    pub progress: f64, // 0.0 to 1.0
    pub color: String,
}

impl ProgressNode {
    pub fn new(id: NodeId, position: Position, config: ProgressConfig) -> Self {
        Self {
            id,
            position,
            size: Size::new(180.0, 60.0),
            label: config.label,
            progress: config.initial_progress,
            color: config.color,
        }
    }

    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 1.0);
    }
}

impl CustomNode for ProgressNode {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn node_type(&self) -> &str {
        "progress-node"
    }

    fn render(&self, _context: &mut dyn std::any::Any) -> Result<(), PluginError> {
        Ok(())
    }

    fn handle_event(&mut self, _event: NodeEvent) -> Result<Option<NodeEventResult>, PluginError> {
        // Progress nodes are typically read-only
        Ok(None)
    }

    fn get_bounds(&self) -> (Position, Size) {
        (self.position, self.size)
    }

    fn get_ports(&self) -> Vec<NodePort> {
        vec![
            NodePort {
                id: "progress_in".to_string(),
                label: "Progress".to_string(),
                position: Position::new(0.0, self.size.height / 2.0),
                port_type: PortType::Input,
            },
            NodePort {
                id: "complete_out".to_string(),
                label: "Complete".to_string(),
                position: Position::new(self.size.width, self.size.height / 2.0),
                port_type: PortType::Output,
            },
        ]
    }
}

/// Progress node factory
pub struct ProgressNodeFactory;

impl ProgressNodeFactory {
    pub fn new() -> Self {
        Self
    }
}

impl NodeFactory for ProgressNodeFactory {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn create_node(&self, id: NodeId, position: Position, config: serde_json::Value) -> Result<Box<dyn CustomNode>, PluginError> {
        let progress_config: ProgressConfig = serde_json::from_value(config)
            .map_err(|e| PluginError::InvalidConfiguration(format!("Invalid progress config: {}", e)))?;

        Ok(Box::new(ProgressNode::new(id, position, progress_config)))
    }
}

/// Progress configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressConfig {
    pub label: String,
    pub initial_progress: f64,
    pub color: String,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            label: "Progress".to_string(),
            initial_progress: 0.0,
            color: "#28a745".to_string(),
        }
    }
}

/// Toggle node implementation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToggleNode {
    pub id: NodeId,
    pub position: Position,
    pub size: Size,
    pub label: String,
    pub is_on: bool,
    pub on_color: String,
    pub off_color: String,
}

impl ToggleNode {
    pub fn new(id: NodeId, position: Position, config: ToggleConfig) -> Self {
        Self {
            id,
            position,
            size: Size::new(100.0, 50.0),
            label: config.label,
            is_on: config.initial_state,
            on_color: config.on_color,
            off_color: config.off_color,
        }
    }

    pub fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

impl CustomNode for ToggleNode {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn node_type(&self) -> &str {
        "toggle-node"
    }

    fn render(&self, _context: &mut dyn std::any::Any) -> Result<(), PluginError> {
        Ok(())
    }

    fn handle_event(&mut self, event: NodeEvent) -> Result<Option<NodeEventResult>, PluginError> {
        match event {
            NodeEvent::Click { .. } => {
                self.toggle();

                let message = PluginMessage {
                    from_plugin: self.plugin_id().to_string(),
                    to_plugin: None,
                    message_type: "toggle-changed".to_string(),
                    payload: serde_json::json!({
                        "node_id": self.id,
                        "is_on": self.is_on
                    }),
                };

                Ok(Some(NodeEventResult::Custom(format!("toggle_changed:{}", self.is_on))))
            }
            _ => Ok(None),
        }
    }

    fn get_bounds(&self) -> (Position, Size) {
        (self.position, self.size)
    }

    fn get_ports(&self) -> Vec<NodePort> {
        vec![
            NodePort {
                id: "state_out".to_string(),
                label: "State".to_string(),
                position: Position::new(self.size.width, self.size.height / 2.0),
                port_type: PortType::Output,
            },
        ]
    }
}

/// Toggle node factory
pub struct ToggleNodeFactory;

impl ToggleNodeFactory {
    pub fn new() -> Self {
        Self
    }
}

impl NodeFactory for ToggleNodeFactory {
    fn plugin_id(&self) -> &str {
        "custom-nodes-plugin"
    }

    fn create_node(&self, id: NodeId, position: Position, config: serde_json::Value) -> Result<Box<dyn CustomNode>, PluginError> {
        let toggle_config: ToggleConfig = serde_json::from_value(config)
            .map_err(|e| PluginError::InvalidConfiguration(format!("Invalid toggle config: {}", e)))?;

        Ok(Box::new(ToggleNode::new(id, position, toggle_config)))
    }
}

/// Toggle configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToggleConfig {
    pub label: String,
    pub initial_state: bool,
    pub on_color: String,
    pub off_color: String,
}

impl Default for ToggleConfig {
    fn default() -> Self {
        Self {
            label: "Toggle".to_string(),
            initial_state: false,
            on_color: "#28a745".to_string(),
            off_color: "#dc3545".to_string(),
        }
    }
}

/// Plugin entry point
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(CustomNodesPlugin::new()))
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
        let plugin = CustomNodesPlugin::new();
        assert_eq!(plugin.metadata().id, "custom-nodes-plugin");
        assert_eq!(plugin.metadata().name, "Custom Node Types");
        assert!(plugin.capabilities().provides_nodes.contains(&"button-node".to_string()));
    }

    #[test]
    fn test_button_node() {
        let node = ButtonNode::new(
            NodeId::new("test-button"),
            Position::new(100.0, 100.0),
            ButtonConfig::default(),
        );

        assert_eq!(node.node_type(), "button-node");
        assert_eq!(node.get_color(), "#007bff"); // Primary style

        let bounds = node.get_bounds();
        assert_eq!(bounds.0, Position::new(100.0, 100.0));
        assert_eq!(bounds.1, Size::new(120.0, 40.0));
    }

    #[test]
    fn test_slider_node() {
        let node = SliderNode::new(
            NodeId::new("test-slider"),
            Position::new(50.0, 50.0),
            SliderConfig::default(),
        );

        assert_eq!(node.node_type(), "slider-node");
        assert_eq!(node.value, 50.0);

        node.set_value(75.0);
        assert_eq!(node.value, 75.0);

        // Test bounds clamping
        node.set_value(150.0); // Above max
        assert_eq!(node.value, 100.0);
    }

    #[test]
    fn test_toggle_node() {
        let mut node = ToggleNode::new(
            NodeId::new("test-toggle"),
            Position::new(0.0, 0.0),
            ToggleConfig::default(),
        );

        assert_eq!(node.node_type(), "toggle-node");
        assert!(!node.is_on);

        node.toggle();
        assert!(node.is_on);

        node.toggle();
        assert!(!node.is_on);
    }

    #[test]
    fn test_progress_node() {
        let mut node = ProgressNode::new(
            NodeId::new("test-progress"),
            Position::new(0.0, 0.0),
            ProgressConfig::default(),
        );

        assert_eq!(node.node_type(), "progress-node");
        assert_eq!(node.progress, 0.0);

        node.set_progress(0.75);
        assert_eq!(node.progress, 0.75);

        // Test bounds clamping
        node.set_progress(1.5); // Above 1.0
        assert_eq!(node.progress, 1.0);
    }
}
