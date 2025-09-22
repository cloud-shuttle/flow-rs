//! Auto layout configuration and strategy types

// Default configuration values
const DEFAULT_SMALL_GRAPH_THRESHOLD: usize = 10;
const DEFAULT_TRANSITION_DURATION: f64 = 0.5;
const DEFAULT_FORCE_DIRECTED_ITERATIONS: usize = 50;

/// Strategies for automatic layout selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoLayoutStrategy {
    /// Automatically detect best layout based on graph structure
    Smart,
    /// Prefer hierarchical layouts when possible, fallback to force-directed
    HierarchyFirst,
    /// Always use force-directed as primary with fallbacks
    ForceDirectedFirst,
    /// Use simple layouts (grid/circular) for small graphs
    SimpleFirst,
}

/// Configuration for automatic layout management
///
/// This configuration controls how the AutoLayoutManager selects and applies
/// layout algorithms based on graph characteristics.
#[derive(Debug, Clone)]
pub struct AutoLayoutConfig {
    /// Strategy for automatic algorithm selection
    pub strategy: AutoLayoutStrategy,
    /// Maximum number of nodes to consider a graph "small" for simple layouts
    pub small_graph_threshold: usize,
    /// Whether to enable smooth transitions between layout changes
    pub enable_transitions: bool,
    /// Duration of layout transitions in seconds
    pub transition_duration: f64,
    /// Whether to force relayout when graph structure changes
    pub force_relayout_on_change: bool,
}

impl Default for AutoLayoutConfig {
    fn default() -> Self {
        Self {
            strategy: AutoLayoutStrategy::Smart,
            small_graph_threshold: DEFAULT_SMALL_GRAPH_THRESHOLD,
            enable_transitions: true,
            transition_duration: DEFAULT_TRANSITION_DURATION,
            force_relayout_on_change: false,
        }
    }
}

impl AutoLayoutConfig {
    /// Create a new configuration builder
    pub fn builder() -> AutoLayoutConfigBuilder {
        AutoLayoutConfigBuilder::new()
    }
}

/// Builder for AutoLayoutConfig
#[derive(Debug)]
pub struct AutoLayoutConfigBuilder {
    config: AutoLayoutConfig,
}

impl AutoLayoutConfigBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            config: AutoLayoutConfig::default(),
        }
    }

    /// Set the layout selection strategy
    pub fn strategy(mut self, strategy: AutoLayoutStrategy) -> Self {
        self.config.strategy = strategy;
        self
    }

    /// Set the small graph threshold
    pub fn small_graph_threshold(mut self, threshold: usize) -> Self {
        self.config.small_graph_threshold = threshold;
        self
    }

    /// Enable or disable transitions
    pub fn enable_transitions(mut self, enable: bool) -> Self {
        self.config.enable_transitions = enable;
        self
    }

    /// Set the transition duration
    pub fn transition_duration(mut self, duration: f64) -> Self {
        self.config.transition_duration = duration.max(0.0);
        self
    }

    /// Set whether to force relayout on change
    pub fn force_relayout_on_change(mut self, force: bool) -> Self {
        self.config.force_relayout_on_change = force;
        self
    }

    /// Build the configuration
    pub fn build(self) -> AutoLayoutConfig {
        self.config
    }
}
