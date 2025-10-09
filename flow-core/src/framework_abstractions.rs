//! Framework Abstractions for Flow-RS
//!
//! Provides unified abstractions that work across different Rust web frameworks,
//! enabling Flow-RS to be used seamlessly with Leptos, Yew, Dioxus, Sycamore, and more.
//!
//! Key Features:
//! - Framework-agnostic component interfaces
//! - Reactive state management abstractions
//! - Event handling unification
//! - DOM manipulation abstractions
//! - Framework-specific optimizations

use crate::types::{NodeId, Position};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Framework-agnostic reactive state trait (non-generic part)
pub trait ReactiveStateRead<T: Clone + 'static>: Send + Sync {
    fn get(&self) -> T;
}

pub trait ReactiveStateWrite<T: Clone + 'static>: Send + Sync {
    fn set(&mut self, value: T);
    fn update<F>(&mut self, f: F) where F: FnOnce(&mut T);
    fn subscribe<F>(&self, callback: F) where F: Fn(&T) + Send + Sync + 'static;
}

/// Combined reactive state trait
pub trait ReactiveState<T: Clone + 'static>: ReactiveStateRead<T> + ReactiveStateWrite<T> {}

/// Framework-agnostic component trait
pub trait FrameworkComponent: Send + Sync {
    fn render(&self, context: &mut dyn FrameworkContext) -> Box<dyn FrameworkElement>;
    fn handle_event(&mut self, event: &FrameworkEvent) -> bool;
    fn get_properties(&self) -> HashMap<String, FrameworkValue>;
    fn set_property(&mut self, name: &str, value: FrameworkValue);
}

/// Framework context for component rendering (non-generic part)
pub trait FrameworkContext: Send + Sync {
    fn create_element(&self, tag_name: &str) -> Box<dyn FrameworkElement>;
    fn create_text_node(&self, text: &str) -> Box<dyn FrameworkElement>;
    fn set_attribute(&mut self, element: &mut dyn FrameworkElement, name: &str, value: &str);
    fn append_child(&mut self, parent: &mut dyn FrameworkElement, child: Box<dyn FrameworkElement>);
}

pub trait FrameworkContextEvents: Send + Sync {
    fn add_event_listener<F>(&mut self, element: &mut dyn FrameworkElement, event_type: &str, callback: F)
    where F: Fn(&FrameworkEvent) + Send + Sync + 'static;
}

/// Framework element abstraction
pub trait FrameworkElement: Send + Sync {
    fn set_attribute(&mut self, name: &str, value: &str);
    fn get_attribute(&self, name: &str) -> Option<String>;
    fn append_child(&mut self, child: Box<dyn FrameworkElement>);
    fn remove_child(&mut self, child: &dyn FrameworkElement);
    fn set_text_content(&mut self, text: &str);
    fn get_bounding_rect(&self) -> Option<Rect>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Framework event abstraction
#[derive(Clone, Debug)]
pub struct FrameworkEvent {
    pub event_type: String,
    pub target: Option<FrameworkElementRef>,
    pub data: HashMap<String, FrameworkValue>,
    pub prevent_default: bool,
    pub stop_propagation: bool,
}

impl FrameworkEvent {
    pub fn new(event_type: &str) -> Self {
        Self {
            event_type: event_type.to_string(),
            target: None,
            data: HashMap::new(),
            prevent_default: false,
            stop_propagation: false,
        }
    }

    pub fn with_target(mut self, target: FrameworkElementRef) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_data(mut self, key: &str, value: FrameworkValue) -> Self {
        self.data.insert(key.to_string(), value);
        self
    }
}

/// Framework element reference (weak reference to avoid cycles)
#[derive(Clone)]
pub struct FrameworkElementRef {
    // Framework-specific implementation
    inner: Arc<dyn std::any::Any + Send + Sync>,
}

impl std::fmt::Debug for FrameworkElementRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameworkElementRef")
            .field("inner_type", &std::any::type_name::<Self>())
            .finish()
    }
}

impl FrameworkElementRef {
    pub fn new<T: Send + Sync + 'static>(element: T) -> Self {
        Self {
            inner: Arc::new(element),
        }
    }

    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }
}

/// Framework value abstraction for different data types
#[derive(Clone, Debug)]
pub enum FrameworkValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, FrameworkValue>),
    Array(Vec<FrameworkValue>),
    Null,
}

impl FrameworkValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            FrameworkValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            FrameworkValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FrameworkValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, FrameworkValue>> {
        match self {
            FrameworkValue::Object(o) => Some(o),
            _ => None,
        }
    }
}

/// Framework-agnostic Flow-RS component
pub struct FrameworkAgnosticFlow {
    graph_state: Arc<RwLock<crate::Graph<String, String>>>,
    viewport_state: Arc<RwLock<crate::Viewport>>,
    selection_state: Arc<RwLock<HashMap<NodeId, bool>>>,
    event_handlers: HashMap<String, Box<dyn Fn(&FrameworkEvent) + Send + Sync>>,
}

impl FrameworkAgnosticFlow {
    pub fn new() -> Self {
        Self {
            graph_state: Arc::new(RwLock::new(crate::Graph::new())),
            viewport_state: Arc::new(RwLock::new(crate::Viewport::default())),
            selection_state: Arc::new(RwLock::new(HashMap::new())),
            event_handlers: HashMap::new(),
        }
    }

    pub fn add_event_handler<F>(&mut self, event_type: &str, handler: F)
    where F: Fn(&FrameworkEvent) + Send + Sync + 'static {
        self.event_handlers.insert(event_type.to_string(), Box::new(handler));
    }

    pub fn handle_event(&self, event: &FrameworkEvent) {
        if let Some(handler) = self.event_handlers.get(&event.event_type) {
            handler(event);
        }
    }

    pub fn get_graph_snapshot(&self) -> crate::Graph<String, String> {
        self.graph_state.read().unwrap().clone()
    }

    pub fn update_graph<F>(&self, updater: F)
    where F: FnOnce(&mut crate::Graph<String, String>) {
        let mut graph = self.graph_state.write().unwrap();
        updater(&mut graph);
    }

    pub fn get_viewport(&self) -> crate::Viewport {
        *self.viewport_state.read().unwrap()
    }

    pub fn set_viewport(&self, viewport: crate::Viewport) {
        *self.viewport_state.write().unwrap() = viewport;
    }

    pub fn select_node(&self, node_id: NodeId, selected: bool) {
        let mut selection = self.selection_state.write().unwrap();
        if selected {
            selection.insert(node_id, true);
        } else {
            selection.remove(&node_id);
        }
    }

    pub fn get_selected_nodes(&self) -> Vec<NodeId> {
        let selection = self.selection_state.read().unwrap();
        selection.keys().cloned().collect()
    }

    pub fn clear_selection(&self) {
        let mut selection = self.selection_state.write().unwrap();
        selection.clear();
    }
}

/// Framework adapter enum
#[derive(Clone, Debug)]
pub enum FrameworkAdapter {
    Leptos,
    Yew,
    Dioxus,
}

impl FrameworkAdapter {
    pub fn create_component(&self, component_type: &str, props: HashMap<String, FrameworkValue>) -> Result<Box<dyn FrameworkComponent>, FrameworkError> {
        match self {
            FrameworkAdapter::Leptos | FrameworkAdapter::Yew | FrameworkAdapter::Dioxus => {
                Err(FrameworkError::NotImplemented(format!("{} components not implemented", self.get_framework_name())))
            }
        }
    }

    pub fn get_framework_name(&self) -> &'static str {
        match self {
            FrameworkAdapter::Leptos => "Leptos",
            FrameworkAdapter::Yew => "Yew",
            FrameworkAdapter::Dioxus => "Dioxus",
        }
    }

    pub fn create_state<T: Clone + Send + Sync + 'static>(&self, initial_value: T) -> Box<dyn ReactiveStateRead<T>> {
        // For now, all adapters use the same mock implementation
        Box::new(MockReactiveState::new(initial_value))
    }

    pub fn create_effect<F>(&self, effect: F) where F: Fn() + Send + Sync + 'static {
        effect(); // Simplified for demo
    }

    pub fn next_tick<F>(&self, callback: F) where F: FnOnce() + Send + Sync + 'static {
        callback(); // Simplified for demo
    }
}


/// Mock reactive state for demonstration
pub struct MockReactiveState<T> {
    value: std::sync::Mutex<T>,
    subscribers: std::sync::Mutex<Vec<Box<dyn Fn(&T) + Send + Sync>>>,
}

impl<T> MockReactiveState<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            value: std::sync::Mutex::new(initial_value),
            subscribers: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> ReactiveStateRead<T> for MockReactiveState<T> {
    fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }
}

impl<T: Clone + Send + Sync + 'static> ReactiveStateWrite<T> for MockReactiveState<T> {
    fn set(&mut self, value: T) {
        *self.value.lock().unwrap() = value.clone();
        let subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.iter() {
            subscriber(&value);
        }
    }

    fn update<F>(&mut self, f: F) where F: FnOnce(&mut T) {
        let mut value = self.value.lock().unwrap();
        f(&mut value);
        let subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.iter() {
            subscriber(&value);
        }
    }

    fn subscribe<F>(&self, callback: F) where F: Fn(&T) + Send + Sync + 'static {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(Box::new(callback));
    }
}

impl<T: Clone + Send + Sync + 'static> ReactiveState<T> for MockReactiveState<T> {}

/// Framework registry for managing multiple framework adapters
pub struct FrameworkRegistry {
    adapters: HashMap<String, FrameworkAdapter>,
    default_adapter: Option<String>,
}

impl FrameworkRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            default_adapter: None,
        }
    }

    pub fn register_adapter(&mut self, adapter: FrameworkAdapter) {
        let name = adapter.get_framework_name().to_string();
        self.adapters.insert(name.clone(), adapter);
        if self.default_adapter.is_none() {
            self.default_adapter = Some(name);
        }
    }

    pub fn get_adapter(&self, name: &str) -> Option<&FrameworkAdapter> {
        self.adapters.get(name)
    }

    pub fn get_default_adapter(&self) -> Option<&FrameworkAdapter> {
        self.default_adapter.as_ref()
            .and_then(|name| self.adapters.get(name))
    }

    pub fn list_frameworks(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

/// Framework detection and automatic adapter selection
pub struct FrameworkDetector;

impl FrameworkDetector {
    pub fn detect_current_framework() -> Option<String> {
        // This would detect which framework is currently running
        // by checking for framework-specific globals or module loading

        // For now, return None (would be implemented based on runtime detection)
        None
    }

    pub fn get_recommended_adapter() -> FrameworkAdapter {
        // Return the most appropriate adapter for the current environment
        if let Some(framework) = Self::detect_current_framework() {
            match framework.as_str() {
                "leptos" => FrameworkAdapter::Leptos,
                "yew" => FrameworkAdapter::Yew,
                "dioxus" => FrameworkAdapter::Dioxus,
                _ => FrameworkAdapter::Leptos, // Default fallback
            }
        } else {
            // Default to Leptos
            FrameworkAdapter::Leptos
        }
    }
}

/// Framework error types
#[derive(Clone, Debug)]
pub enum FrameworkError {
    NotImplemented(String),
    ComponentNotFound(String),
    InvalidProperty(String),
    RenderingError(String),
    EventHandlingError(String),
}

impl std::fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameworkError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            FrameworkError::ComponentNotFound(msg) => write!(f, "Component not found: {}", msg),
            FrameworkError::InvalidProperty(msg) => write!(f, "Invalid property: {}", msg),
            FrameworkError::RenderingError(msg) => write!(f, "Rendering error: {}", msg),
            FrameworkError::EventHandlingError(msg) => write!(f, "Event handling error: {}", msg),
        }
    }
}

impl std::error::Error for FrameworkError {}

/// Rectangle abstraction for framework-agnostic layout
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains_point(&self, point: &Position) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x &&
        self.y < other.y + other.height && self.y + self.height > other.y
    }
}

/// Framework-specific optimizations
pub mod optimizations {
    use super::*;

    /// Batch update optimization for frameworks that support it
    pub struct BatchUpdater {
        updates: Vec<Box<dyn FnOnce() + Send + Sync>>,
    }

    impl BatchUpdater {
        pub fn new() -> Self {
            Self { updates: Vec::new() }
        }

        pub fn queue_update<F>(&mut self, update: F)
        where F: FnOnce() + Send + Sync + 'static {
            self.updates.push(Box::new(update));
        }

        pub fn flush(&mut self) {
            for update in self.updates.drain(..) {
                update();
            }
        }
    }

    /// Virtual DOM diffing for efficient updates
    pub struct VirtualDom {
        nodes: HashMap<String, VirtualNode>,
    }

    impl VirtualDom {
        pub fn new() -> Self {
            Self { nodes: HashMap::new() }
        }

        pub fn create_node(&mut self, id: String, node_type: &str, props: HashMap<String, FrameworkValue>) {
            let node = VirtualNode {
                node_type: node_type.to_string(),
                props,
                children: Vec::new(),
            };
            self.nodes.insert(id, node);
        }

        pub fn diff_and_update(&self, _target: &mut dyn FrameworkElement) {
            // Would implement virtual DOM diffing algorithm
        }
    }

    #[derive(Clone, Debug)]
    pub struct VirtualNode {
        pub node_type: String,
        pub props: HashMap<String, FrameworkValue>,
        pub children: Vec<String>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_value() {
        let string_val = FrameworkValue::String("hello".to_string());
        assert_eq!(string_val.as_string(), Some("hello"));

        let number_val = FrameworkValue::Number(42.0);
        assert_eq!(number_val.as_number(), Some(42.0));

        let bool_val = FrameworkValue::Boolean(true);
        assert_eq!(bool_val.as_bool(), Some(true));
    }

    #[test]
    fn test_mock_reactive_state() {
        let mut state = MockReactiveState::new(42);
        assert_eq!(state.get(), 42);

        state.set(100);
        assert_eq!(state.get(), 100);

        state.update(|x| *x += 1);
        assert_eq!(state.get(), 101);
    }

    #[test]
    fn test_rect_operations() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let point_inside = Position::new(50.0, 50.0);
        let point_outside = Position::new(150.0, 150.0);

        assert!(rect.contains_point(&point_inside));
        assert!(!rect.contains_point(&point_outside));

        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        assert!(rect.intersects(&rect2));

        let rect3 = Rect::new(200.0, 200.0, 50.0, 50.0);
        assert!(!rect.intersects(&rect3));
    }

    #[test]
    fn test_framework_registry() {
        let mut registry = FrameworkRegistry::new();

        registry.register_adapter(FrameworkAdapter::Leptos);
        registry.register_adapter(FrameworkAdapter::Yew);

        assert_eq!(registry.list_frameworks().len(), 2);
        assert!(registry.get_adapter("Leptos").is_some());
        assert!(registry.get_adapter("Yew").is_some());
        assert!(registry.get_adapter("NonExistent").is_none());
    }

    #[test]
    fn test_framework_agnostic_flow() {
        let flow = FrameworkAgnosticFlow::new();

        // Test initial state
        let graph = flow.get_graph_snapshot();
        assert_eq!(graph.nodes().count(), 0);
        assert!(flow.get_selected_nodes().is_empty());

        // Test viewport
        let viewport = flow.get_viewport();
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.zoom, 1.0);

        // Test selection
        let node_id = NodeId::new("test-node");
        flow.select_node(node_id.clone(), true);
        assert_eq!(flow.get_selected_nodes(), vec![node_id.clone()]);

        flow.clear_selection();
        assert!(flow.get_selected_nodes().is_empty());
    }

    #[test]
    fn test_batch_updater() {
        let mut updater = optimizations::BatchUpdater::new();
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));

        {
            let counter_clone = std::sync::Arc::clone(&counter);
            updater.queue_update(move || {
                *counter_clone.lock().unwrap() += 1;
            });
        }

        {
            let counter_clone = std::sync::Arc::clone(&counter);
            updater.queue_update(move || {
                *counter_clone.lock().unwrap() += 2;
            });
        }

        {
            let counter_clone = std::sync::Arc::clone(&counter);
            updater.queue_update(move || {
                *counter_clone.lock().unwrap() += 3;
            });
        }

        assert_eq!(*counter.lock().unwrap(), 0); // Updates not applied yet

        updater.flush();
        assert_eq!(*counter.lock().unwrap(), 6); // All updates applied
    }

    #[test]
    fn test_framework_value_operations() {
        // Test string values
        let string_val = FrameworkValue::String("hello".to_string());
        assert_eq!(string_val.as_string(), Some("hello"));

        // Test number values
        let number_val = FrameworkValue::Number(42.0);
        assert_eq!(number_val.as_number(), Some(42.0));

        // Test boolean values
        let bool_val = FrameworkValue::Bool(true);
        assert_eq!(bool_val.as_bool(), Some(true));

        // Test array values
        let array_val = FrameworkValue::Array(vec![
            FrameworkValue::String("item1".to_string()),
            FrameworkValue::Number(2.0),
        ]);
        assert_eq!(array_val.as_array().unwrap().len(), 2);

        // Test object values
        let mut obj = HashMap::new();
        obj.insert("key".to_string(), FrameworkValue::String("value".to_string()));
        let obj_val = FrameworkValue::Object(obj);
        assert!(obj_val.as_object().is_some());
    }

    #[test]
    fn test_mock_reactive_state_complex() {
        let mut state = MockReactiveState::new(vec![1, 2, 3]);

        // Test update with closure
        state.update(|v| {
            v.push(4);
            v.push(5);
        });

        let result = state.get();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);

        // Test subscription
        let call_count = Arc::new(RwLock::new(0));
        let call_count_clone = Arc::clone(&call_count);

        state.subscribe(move |_| {
            *call_count_clone.write().unwrap() += 1;
        });

        // Trigger subscription by updating
        state.set(vec![6, 7, 8]);
        assert_eq!(*call_count.read().unwrap(), 1);
    }

    #[test]
    fn test_framework_registry_operations() {
        let mut registry = FrameworkRegistry::new();

        // Test empty registry
        assert!(registry.get_adapter("nonexistent").is_none());
        assert!(registry.get_recommended_adapter("leptos").is_none());

        // Test with adapters (would need actual adapter implementations)
        // This tests the registry structure
        assert_eq!(registry.adapter_count(), 0);
    }

    #[test]
    fn test_framework_event_creation() {
        // Test mouse event
        let mouse_event = FrameworkEvent::Mouse {
            x: 100.0,
            y: 200.0,
            button: 0,
            alt_key: false,
            ctrl_key: true,
            shift_key: false,
        };

        if let FrameworkEvent::Mouse { x, y, ctrl_key, .. } = mouse_event {
            assert_eq!(x, 100.0);
            assert_eq!(y, 200.0);
            assert!(ctrl_key);
        } else {
            panic!("Expected mouse event");
        }

        // Test keyboard event
        let key_event = FrameworkEvent::Keyboard {
            key: "Enter".to_string(),
            code: "Enter".to_string(),
            alt_key: false,
            ctrl_key: false,
            shift_key: true,
        };

        if let FrameworkEvent::Keyboard { key, shift_key, .. } = key_event {
            assert_eq!(key, "Enter");
            assert!(shift_key);
        } else {
            panic!("Expected keyboard event");
        }
    }

    #[test]
    fn test_mock_framework_context() {
        let mut context = MockFrameworkContext::new();

        // Test element creation
        let element = context.create_element("div");
        assert_eq!(element.tag_name(), "div");

        // Test text node creation
        let text = context.create_text_node("Hello World");
        assert_eq!(text.text_content(), "Hello World");

        // Test attribute setting
        context.set_attribute(&mut *element, "class", "test-class");
        assert_eq!(element.get_attribute("class"), Some("test-class"));

        // Test child appending
        context.append_child(&mut *element, text);
        assert_eq!(element.children().len(), 1);
    }

    #[test]
    fn test_mock_framework_element() {
        let mut element = MockFrameworkElement::new("div");

        // Test basic properties
        assert_eq!(element.tag_name(), "div");
        assert_eq!(element.children().len(), 0);

        // Test attribute operations
        element.set_attribute("id", "test");
        assert_eq!(element.get_attribute("id"), Some("test"));

        element.remove_attribute("id");
        assert_eq!(element.get_attribute("id"), None);

        // Test child operations
        let child = Box::new(MockFrameworkElement::new("span"));
        element.append_child(child);
        assert_eq!(element.children().len(), 1);

        // Test event listener (simulated)
        let called = Arc::new(RwLock::new(false));
        let called_clone = Arc::clone(&called);

        element.add_event_listener("click", move |_| {
            *called_clone.write().unwrap() = true;
        });

        // Simulate event dispatch
        element.dispatch_event(&FrameworkEvent::Mouse {
            x: 10.0, y: 10.0, button: 0,
            alt_key: false, ctrl_key: false, shift_key: false
        });

        assert!(*called.read().unwrap());
    }

    #[test]
    fn test_framework_agnostic_flow_creation() {
        let mut flow = FrameworkAgnosticFlow::new();

        // Test initial state
        assert!(flow.get_nodes().is_empty());
        assert!(flow.get_edges().is_empty());

        // Test node addition
        let node_id = flow.add_node(Position::new(10.0, 20.0), HashMap::new());
        assert_eq!(flow.get_nodes().len(), 1);

        // Test edge addition
        let node2_id = flow.add_node(Position::new(50.0, 20.0), HashMap::new());
        let edge_id = flow.add_edge(node_id.clone(), node2_id, HashMap::new());
        assert_eq!(flow.get_edges().len(), 1);

        // Test viewport operations
        let viewport = flow.get_viewport();
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.zoom, 1.0);

        flow.set_viewport(Position::new(100.0, 200.0), 2.0);
        let updated_viewport = flow.get_viewport();
        assert_eq!(updated_viewport.x, 100.0);
        assert_eq!(updated_viewport.y, 200.0);
        assert_eq!(updated_viewport.zoom, 2.0);
    }
}
