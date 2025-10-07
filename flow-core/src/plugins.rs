//! Plugin System Architecture for Flow-RS
//!
//! Provides a comprehensive plugin system enabling third-party extensions including:
//! - Custom node types and behaviors
//! - Layout algorithms and positioning
//! - Themes and visual styling
//! - Export formats and data transformations
//! - Custom interactions and behaviors
//! - Plugin marketplace and discovery

use crate::graph::Graph;
use crate::types::{NodeId, Position, Size};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Plugin metadata
#[derive(Clone, Debug)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: String,
    pub categories: Vec<PluginCategory>,
    pub dependencies: Vec<String>,
    pub min_flow_rs_version: String,
}

/// Plugin categories for organization and discovery
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PluginCategory {
    NodeType,
    Layout,
    Theme,
    Export,
    Interaction,
    Background,
    Utility,
}

/// Plugin capabilities declaration
#[derive(Clone, Debug)]
pub struct PluginCapabilities {
    pub provides_nodes: Vec<String>,
    pub provides_layouts: Vec<String>,
    pub provides_themes: Vec<String>,
    pub provides_exports: Vec<String>,
    pub provides_interactions: Vec<String>,
    pub provides_backgrounds: Vec<String>,
}

/// Plugin instance trait
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Get plugin capabilities
    fn capabilities(&self) -> &PluginCapabilities;

    /// Initialize the plugin
    fn initialize(&mut self, registry: &mut PluginRegistry) -> Result<(), PluginError>;

    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Handle plugin-specific messages
    fn handle_message(&mut self, message: PluginMessage) -> Result<Option<PluginMessage>, PluginError>;
}

/// Plugin message system for inter-plugin communication
#[derive(Clone, Debug)]
pub struct PluginMessage {
    pub from_plugin: String,
    pub to_plugin: Option<String>, // None = broadcast
    pub message_type: String,
    pub payload: String, // JSON string payload
}

/// Plugin error types
#[derive(Clone, Debug)]
pub enum PluginError {
    InitializationFailed(String),
    DependencyMissing(String),
    CapabilityConflict(String),
    InvalidConfiguration(String),
    RuntimeError(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::InitializationFailed(msg) => write!(f, "Initialization failed: {}", msg),
            PluginError::DependencyMissing(msg) => write!(f, "Dependency missing: {}", msg),
            PluginError::CapabilityConflict(msg) => write!(f, "Capability conflict: {}", msg),
            PluginError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            PluginError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

/// Plugin registry for managing loaded plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    node_factories: HashMap<String, Box<dyn NodeFactory>>,
    layout_engines: HashMap<String, Box<dyn LayoutEngine>>,
    themes: HashMap<String, Box<dyn Theme>>,
    exporters: HashMap<String, Box<dyn Exporter>>,
    interaction_handlers: HashMap<String, Box<dyn InteractionHandler>>,
    background_renderers: HashMap<String, Box<dyn BackgroundRenderer>>,
    capabilities_index: HashMap<String, Vec<String>>, // capability -> plugin_ids
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            node_factories: HashMap::new(),
            layout_engines: HashMap::new(),
            themes: HashMap::new(),
            exporters: HashMap::new(),
            interaction_handlers: HashMap::new(),
            background_renderers: HashMap::new(),
            capabilities_index: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, mut plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let plugin_id = plugin.metadata().id.clone();

        // Check for duplicate plugin ID
        if self.plugins.contains_key(&plugin_id) {
            return Err(PluginError::CapabilityConflict(format!("Plugin '{}' already registered", plugin_id)));
        }

        // Initialize the plugin
        plugin.initialize(self)?;

        // Index capabilities
        let capabilities = plugin.capabilities().clone();
        self.index_capabilities(&plugin_id, &capabilities);

        // Store the plugin
        self.plugins.insert(plugin_id, plugin);

        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(mut plugin) = self.plugins.remove(plugin_id) {
            plugin.shutdown()?;

            // Remove from capability index
            self.capabilities_index.retain(|_, plugins| {
                plugins.retain(|p| p != plugin_id);
                !plugins.is_empty()
            });

            // Note: In a full implementation, we would need to track which
            // capabilities belong to which plugins for proper cleanup
        }

        Ok(())
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(plugin_id)
    }

    /// Get all registered plugins
    pub fn get_all_plugins(&self) -> &HashMap<String, Box<dyn Plugin>> {
        &self.plugins
    }

    /// Register a node factory
    pub fn register_node_factory(&mut self, name: String, factory: Box<dyn NodeFactory>) {
        self.node_factories.insert(name, factory);
    }

    /// Get node factory by name
    pub fn get_node_factory(&self, name: &str) -> Option<&Box<dyn NodeFactory>> {
        self.node_factories.get(name)
    }

    /// Register a layout engine
    pub fn register_layout_engine(&mut self, name: String, engine: Box<dyn LayoutEngine>) {
        self.layout_engines.insert(name, engine);
    }

    /// Get layout engine by name
    pub fn get_layout_engine(&self, name: &str) -> Option<&Box<dyn LayoutEngine>> {
        self.layout_engines.get(name)
    }

    /// Register a theme
    pub fn register_theme(&mut self, name: String, theme: Box<dyn Theme>) {
        self.themes.insert(name, theme);
    }

    /// Get theme by name
    pub fn get_theme(&self, name: &str) -> Option<&Box<dyn Theme>> {
        self.themes.get(name)
    }

    /// Register an exporter
    pub fn register_exporter(&mut self, name: String, exporter: Box<dyn Exporter>) {
        self.exporters.insert(name, exporter);
    }

    /// Get exporter by name
    pub fn get_exporter(&self, name: &str) -> Option<&Box<dyn Exporter>> {
        self.exporters.get(name)
    }

    /// Register an interaction handler
    pub fn register_interaction_handler(&mut self, name: String, handler: Box<dyn InteractionHandler>) {
        self.interaction_handlers.insert(name, handler);
    }

    /// Get interaction handler by name
    pub fn get_interaction_handler(&self, name: &str) -> Option<&Box<dyn InteractionHandler>> {
        self.interaction_handlers.get(name)
    }

    /// Register a background renderer
    pub fn register_background_renderer(&mut self, name: String, renderer: Box<dyn BackgroundRenderer>) {
        self.background_renderers.insert(name, renderer);
    }

    /// Get background renderer by name
    pub fn get_background_renderer(&self, name: &str) -> Option<&Box<dyn BackgroundRenderer>> {
        self.background_renderers.get(name)
    }

    /// Send message to plugin
    pub fn send_message(&mut self, message: PluginMessage) -> Result<(), PluginError> {
        if let Some(target_plugin) = &message.to_plugin {
            if let Some(plugin) = self.plugins.get_mut(target_plugin) {
                if let Some(response) = plugin.handle_message(message)? {
                    // Handle response if needed
                    self.broadcast_message(response)?;
                }
            }
        } else {
            self.broadcast_message(message)?;
        }
        Ok(())
    }

    /// Broadcast message to all plugins
    pub fn broadcast_message(&mut self, message: PluginMessage) -> Result<(), PluginError> {
        for plugin in self.plugins.values_mut() {
            if plugin.metadata().id != message.from_plugin {
                if let Some(response) = plugin.handle_message(message.clone())? {
                    // Could handle responses in a more sophisticated way
                }
            }
        }
        Ok(())
    }

    /// Get plugins by capability
    pub fn get_plugins_by_capability(&self, capability: &str) -> Vec<&str> {
        self.capabilities_index
            .get(capability)
            .map(|plugins| plugins.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Index plugin capabilities
    fn index_capabilities(&mut self, plugin_id: &str, capabilities: &PluginCapabilities) {
        let all_capabilities = [
            (&capabilities.provides_nodes, "node"),
            (&capabilities.provides_layouts, "layout"),
            (&capabilities.provides_themes, "theme"),
            (&capabilities.provides_exports, "export"),
            (&capabilities.provides_interactions, "interaction"),
            (&capabilities.provides_backgrounds, "background"),
        ];

        for (capability_list, _capability_type) in &all_capabilities {
            for capability in *capability_list {
                self.capabilities_index
                    .entry(capability.clone())
                    .or_insert_with(Vec::new)
                    .push(plugin_id.to_string());
            }
        }
    }
}

/// Node factory trait for creating custom node types
pub trait NodeFactory: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn create_node(&self, id: NodeId, position: Position, config: String) -> Result<Box<dyn CustomNode>, PluginError>;
}

/// Layout engine trait for custom layout algorithms
/// Note: Simplified for dyn compatibility - works with string node/edge data
pub trait LayoutEngine: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn layout(&self, graph: &mut Graph<String, String>) -> Result<(), PluginError>;
}

/// Theme trait for custom visual themes
pub trait Theme: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn get_styles(&self) -> HashMap<String, String>;
    fn get_colors(&self) -> HashMap<String, String>;
}

/// Exporter trait for custom export formats
/// Note: Simplified for dyn compatibility - works with string node/edge data
pub trait Exporter: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn export(&self, graph: &Graph<String, String>, config: String) -> Result<String, PluginError>;
}

/// Interaction handler trait for custom interactions
pub trait InteractionHandler: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn handle_interaction(&self, event: InteractionEvent) -> Result<Option<InteractionResult>, PluginError>;
}

/// Background renderer trait for custom backgrounds
pub trait BackgroundRenderer: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn render(&self, context: &mut dyn std::any::Any, viewport: &crate::types::Viewport) -> Result<(), PluginError>;
}

/// Custom node trait for plugin-defined node types
pub trait CustomNode: Send + Sync {
    fn plugin_id(&self) -> &str;
    fn node_type(&self) -> &str;
    fn render(&self, context: &mut dyn std::any::Any) -> Result<(), PluginError>;
    fn handle_event(&mut self, event: NodeEvent) -> Result<Option<NodeEventResult>, PluginError>;
    fn get_bounds(&self) -> (Position, Size);
    fn get_ports(&self) -> Vec<NodePort>;
}

/// Node event types
#[derive(Clone, Debug)]
pub enum NodeEvent {
    Click { position: Position },
    DoubleClick { position: Position },
    DragStart { position: Position },
    DragMove { position: Position, delta: Position },
    DragEnd { position: Position },
    ContextMenu { position: Position },
}

/// Node event results
#[derive(Clone, Debug)]
pub enum NodeEventResult {
    UpdatePosition(Position),
    UpdateSize(Size),
    ShowContextMenu(Vec<String>),
    Custom(String),
}

/// Node port definition
#[derive(Clone, Debug)]
pub struct NodePort {
    pub id: String,
    pub label: String,
    pub position: Position,
    pub port_type: PortType,
}

/// Port types
#[derive(Clone, Debug, PartialEq)]
pub enum PortType {
    Input,
    Output,
    Bidirectional,
}

/// Interaction event types
#[derive(Clone, Debug)]
pub enum InteractionEvent {
    MouseDown { position: Position, button: u32 },
    MouseMove { position: Position },
    MouseUp { position: Position, button: u32 },
    KeyDown { key: String, ctrl: bool, shift: bool, alt: bool },
    KeyUp { key: String },
}

/// Interaction result types
#[derive(Clone, Debug)]
pub enum InteractionResult {
    Handled,
    Continue,
    Custom(String),
}

/// Plugin manager for high-level plugin operations
pub struct PluginManager {
    registry: Arc<RwLock<PluginRegistry>>,
    loaded_plugins: HashMap<String, PluginMetadata>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(PluginRegistry::new())),
            loaded_plugins: HashMap::new(),
        }
    }

    /// Load a plugin from metadata and implementation
    pub fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let plugin_id = metadata.id.clone();

        {
            let mut registry = self.registry.write().unwrap();
            registry.register_plugin(plugin)?;
        }

        self.loaded_plugins.insert(plugin_id, metadata);
        Ok(())
    }

    /// Unload a plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        {
            let mut registry = self.registry.write().unwrap();
            registry.unregister_plugin(plugin_id)?;
        }

        self.loaded_plugins.remove(plugin_id);
        Ok(())
    }

    /// Get registry reference
    pub fn registry(&self) -> Arc<RwLock<PluginRegistry>> {
        Arc::clone(&self.registry)
    }

    /// Get loaded plugin metadata
    pub fn get_loaded_plugins(&self) -> &HashMap<String, PluginMetadata> {
        &self.loaded_plugins
    }

    /// Check if plugin is loaded
    pub fn is_plugin_loaded(&self, plugin_id: &str) -> bool {
        self.loaded_plugins.contains_key(plugin_id)
    }

    /// Get plugin statistics
    pub fn get_stats(&self) -> PluginStats {
        let registry = self.registry.read().unwrap();

        PluginStats {
            loaded_plugins: self.loaded_plugins.len(),
            registered_nodes: registry.node_factories.len(),
            registered_layouts: registry.layout_engines.len(),
            registered_themes: registry.themes.len(),
            registered_exporters: registry.exporters.len(),
            registered_interactions: registry.interaction_handlers.len(),
            registered_backgrounds: registry.background_renderers.len(),
        }
    }
}

/// Plugin statistics
#[derive(Clone, Debug)]
pub struct PluginStats {
    pub loaded_plugins: usize,
    pub registered_nodes: usize,
    pub registered_layouts: usize,
    pub registered_themes: usize,
    pub registered_exporters: usize,
    pub registered_interactions: usize,
    pub registered_backgrounds: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock plugin for testing
    struct MockPlugin {
        metadata: PluginMetadata,
        capabilities: PluginCapabilities,
    }

    impl MockPlugin {
        fn new(id: &str, name: &str) -> Self {
            Self {
                metadata: PluginMetadata {
                    id: id.to_string(),
                    name: name.to_string(),
                    version: "1.0.0".to_string(),
                    author: "Test Author".to_string(),
                    description: "Test plugin".to_string(),
                    homepage: None,
                    repository: None,
                    license: "MIT".to_string(),
                    categories: vec![PluginCategory::NodeType],
                    dependencies: vec![],
                    min_flow_rs_version: "0.1.0".to_string(),
                },
                capabilities: PluginCapabilities {
                    provides_nodes: vec!["test-node".to_string()],
                    provides_layouts: vec![],
                    provides_themes: vec![],
                    provides_exports: vec![],
                    provides_interactions: vec![],
                    provides_backgrounds: vec![],
                },
            }
        }
    }

    impl Plugin for MockPlugin {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        fn capabilities(&self) -> &PluginCapabilities {
            &self.capabilities
        }

        fn initialize(&mut self, _registry: &mut PluginRegistry) -> Result<(), PluginError> {
            Ok(())
        }

        fn shutdown(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        fn handle_message(&mut self, _message: PluginMessage) -> Result<Option<PluginMessage>, PluginError> {
            Ok(None)
        }
    }

    #[test]
    fn test_plugin_registration() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", "Test Plugin"));

        assert!(manager.load_plugin(plugin).is_ok());
        assert!(manager.is_plugin_loaded("test-plugin"));

        let stats = manager.get_stats();
        assert_eq!(stats.loaded_plugins, 1);
    }

    #[test]
    fn test_plugin_unregistration() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", "Test Plugin"));

        manager.load_plugin(plugin).unwrap();
        assert!(manager.unload_plugin("test-plugin").is_ok());
        assert!(!manager.is_plugin_loaded("test-plugin"));
    }

    #[test]
    fn test_plugin_capabilities() {
        let manager = PluginManager::new();
        let registry = manager.registry();
        let registry_read = registry.read().unwrap();

        let plugins = registry_read.get_plugins_by_capability("test-node");
        assert!(plugins.is_empty()); // No plugins loaded yet
    }

    #[test]
    fn test_plugin_message_system() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", "Test Plugin"));
        manager.load_plugin(plugin).unwrap();

        let message = PluginMessage {
            from_plugin: "test-plugin".to_string(),
            to_plugin: None,
            message_type: "test".to_string(),
            payload: serde_json::json!({"test": "data"}),
        };

        assert!(manager.registry().write().unwrap().send_message(message).is_ok());
    }
}
