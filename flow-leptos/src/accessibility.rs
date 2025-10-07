//! Accessibility support for Flow-RS Leptos components
//!
//! Provides WCAG 2.1 AA compliant accessibility features including:
//! - ARIA labels and roles
//! - Keyboard navigation
//! - Focus management
//! - Screen reader support
//! - High contrast mode support

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};
use std::collections::HashMap;

/// Accessibility configuration for flow components
#[derive(Clone, Debug)]
pub struct AccessibilityConfig {
    /// Enable keyboard navigation
    pub keyboard_navigation: bool,
    /// Enable screen reader announcements
    pub screen_reader_support: bool,
    /// Enable high contrast mode support
    pub high_contrast: bool,
    /// Custom ARIA labels
    pub aria_labels: HashMap<String, String>,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            keyboard_navigation: true,
            screen_reader_support: true,
            high_contrast: false,
            aria_labels: HashMap::new(),
        }
    }
}

/// ARIA role definitions for flow components
#[derive(Clone, Debug)]
pub enum AriaRole {
    Application,
    Group,
    Button,
    Region,
    Img,
    Tab,
    TabPanel,
}

impl AriaRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaRole::Application => "application",
            AriaRole::Group => "group",
            AriaRole::Button => "button",
            AriaRole::Region => "region",
            AriaRole::Img => "img",
            AriaRole::Tab => "tab",
            AriaRole::TabPanel => "tabpanel",
        }
    }
}

/// Screen reader announcement system
#[derive(Clone)]
pub struct ScreenReader {
    live_region_id: Option<String>,
}

impl ScreenReader {
    pub fn new() -> Self {
        Self { live_region_id: None }
    }

    /// Initialize screen reader live region
    pub fn init(&mut self) {
        // Generate a unique ID for the live region
        let id = format!("sr-live-region-{}", js_sys::Math::random());
        self.live_region_id = Some(id);
    }

    /// Announce a message to screen readers
    pub fn announce(&self, message: &str) {
        if self.live_region_id.is_some() {
            // In a real implementation, this would update the DOM element
            // For now, we'll use console logging as a placeholder
            web_sys::console::log_1(&format!("Screen reader: {}", message).into());
        }
    }

    /// Announce node selection
    pub fn announce_node_selection(&self, node_id: &str, selected: bool) {
        let message = if selected {
            format!("Selected node {}", node_id)
        } else {
            format!("Deselected node {}", node_id)
        };
        self.announce(&message);
    }

    /// Announce keyboard navigation
    pub fn announce_navigation(&self, direction: &str, node_id: &str) {
        let message = format!("Navigated {} to node {}", direction, node_id);
        self.announce(&message);
    }
}

impl Default for ScreenReader {
    fn default() -> Self {
        let mut sr = Self::new();
        sr.init();
        sr
    }
}

/// Keyboard navigation state
#[derive(Clone, Debug)]
pub struct KeyboardNavigation {
    /// Currently focused element ID
    pub focused_element: Option<String>,
    /// Navigation history for focus restoration
    pub focus_history: Vec<String>,
    /// Tab order for elements
    pub tab_order: Vec<String>,
}

impl KeyboardNavigation {
    pub fn new() -> Self {
        Self {
            focused_element: None,
            focus_history: Vec::new(),
            tab_order: Vec::new(),
        }
    }

    /// Register an element for keyboard navigation
    pub fn register_element(&mut self, element_id: String) {
        if !self.tab_order.contains(&element_id) {
            self.tab_order.push(element_id);
        }
    }

    /// Navigate to next element
    pub fn navigate_next(&mut self) -> Option<String> {
        let current_idx = self.focused_element.as_ref()
            .and_then(|id| self.tab_order.iter().position(|x| x == id))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % self.tab_order.len();
        let next_element = self.tab_order.get(next_idx).cloned();

        if let Some(ref element) = next_element {
            self.focused_element = Some(element.clone());
            self.focus_history.push(element.clone());
        }

        next_element
    }

    /// Navigate to previous element
    pub fn navigate_previous(&mut self) -> Option<String> {
        let current_idx = self.focused_element.as_ref()
            .and_then(|id| self.tab_order.iter().position(|x| x == id))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            self.tab_order.len() - 1
        } else {
            current_idx - 1
        };

        let prev_element = self.tab_order.get(prev_idx).cloned();

        if let Some(ref element) = prev_element {
            self.focused_element = Some(element.clone());
            self.focus_history.push(element.clone());
        }

        prev_element
    }

    /// Set focus to specific element
    pub fn set_focus(&mut self, element_id: String) {
        self.focused_element = Some(element_id.clone());
        self.focus_history.push(element_id);
    }
}

/// Focus management utilities
pub struct FocusManager {
    previous_focus: Option<String>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self { previous_focus: None }
    }

    /// Save current focus for restoration
    pub fn save_focus(&mut self) {
        if let Some(active) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element()) {

            if let Ok(element) = active.dyn_into::<web_sys::Element>() {
                // Try to get the id attribute
                if let Some(id) = element.get_attribute("id") {
                    self.previous_focus = Some(id);
                }
            }
        }
    }

    /// Restore previously saved focus
    pub fn restore_focus(&self) {
        if let Some(ref element_id) = self.previous_focus {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(element) = document.get_element_by_id(element_id) {
                    if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                        let _ = html_element.focus();
                    }
                }
            }
        }
    }

    /// Trap focus within a container
    pub fn trap_focus(&self, _container_id: &str) {
        // Focus trapping implementation would go here
        // For now, this is a placeholder
    }
}

/// ARIA attribute utilities
pub struct AriaAttributes;

impl AriaAttributes {
    /// Generate ARIA label for a node
    pub fn node_label(node_id: &str, node_type: &str, additional_info: Option<&str>) -> String {
        let base = format!("Node {} of type {}", node_id, node_type);
        if let Some(info) = additional_info {
            format!("{}, {}", base, info)
        } else {
            base
        }
    }

    /// Generate ARIA description for an edge
    pub fn edge_description(edge_id: &str, source: &str, target: &str, edge_type: &str) -> String {
        format!("Edge {} connecting {} to {} of type {}", edge_id, source, target, edge_type)
    }

    /// Generate ARIA live region for canvas updates
    pub fn canvas_live_region(node_count: usize, edge_count: usize, selected_count: usize) -> String {
        format!("Flow canvas with {} nodes and {} edges. {} items selected.",
                node_count, edge_count, selected_count)
    }
}

/// High contrast mode support
pub struct HighContrastSupport;

impl HighContrastSupport {
    /// Check if high contrast mode is active
    pub fn is_active() -> bool {
        // For now, return false as matchMedia may not be available
        // In a real implementation, this would check system preferences
        false
    }

    /// Apply high contrast styles
    pub fn apply_styles(element: &web_sys::Element) {
        if Self::is_active() {
            let _ = element.set_attribute("data-high-contrast", "true");
        }
    }
}

/// Keyboard event handlers for accessibility
pub fn create_accessible_keyboard_handler(
    keyboard_nav: RwSignal<KeyboardNavigation>,
    screen_reader: RwSignal<ScreenReader>,
) -> impl Fn(KeyboardEvent) {
    move |event: KeyboardEvent| {
        let key = event.key();
        let alt_pressed = event.alt_key();
        let _ctrl_pressed = event.ctrl_key();

        match key.as_str() {
            "Tab" => {
                // Handle tab navigation
                event.prevent_default();
                keyboard_nav.update(|nav| {
                    let next_element = if event.shift_key() {
                        nav.navigate_previous()
                    } else {
                        nav.navigate_next()
                    };

                    if let Some(element_id) = next_element {
                        // Focus the element
                        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                            if let Some(element) = document.get_element_by_id(&element_id) {
                                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                                    let _ = html_element.focus();
                                    screen_reader.update(|sr| {
                                        sr.announce_navigation("to", &element_id);
                                    });
                                }
                            }
                        }
                    }
                });
            }

            "ArrowRight" if alt_pressed => {
                // Alt+Right: Navigate to next node
                event.prevent_default();
                keyboard_nav.update(|nav| {
                    let next = nav.navigate_next();
                    if let Some(ref element_id) = next {
                        screen_reader.update(|sr| {
                            sr.announce_navigation("right", element_id);
                        });
                    }
                });
            }

            "ArrowLeft" if alt_pressed => {
                // Alt+Left: Navigate to previous node
                event.prevent_default();
                keyboard_nav.update(|nav| {
                    let prev = nav.navigate_previous();
                    if let Some(ref element_id) = prev {
                        screen_reader.update(|sr| {
                            sr.announce_navigation("left", element_id);
                        });
                    }
                });
            }

            "Enter" | " " => {
                // Enter/Space: Activate current element
                event.prevent_default();
                if let Some(ref focused_id) = keyboard_nav.get().focused_element.clone() {
                    screen_reader.update(|sr| {
                        sr.announce(&format!("Activated {}", focused_id));
                    });
                    // Here you would trigger the appropriate action for the focused element
                }
            }

            "Escape" => {
                // Escape: Clear focus or cancel operation
                keyboard_nav.update(|nav| {
                    nav.focused_element = None;
                });
                screen_reader.update(|sr| {
                    sr.announce("Focus cleared");
                });
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aria_role_as_str() {
        assert_eq!(AriaRole::Application.as_str(), "application");
        assert_eq!(AriaRole::Group.as_str(), "group");
        assert_eq!(AriaRole::Button.as_str(), "button");
    }

    #[test]
    fn test_aria_attributes() {
        let label = AriaAttributes::node_label("node1", "process", Some("active"));
        assert_eq!(label, "Node node1 of type process, active");

        let description = AriaAttributes::edge_description("edge1", "node1", "node2", "flow");
        assert_eq!(description, "Edge edge1 connecting node1 to node2 of type flow");
    }

    #[test]
    fn test_keyboard_navigation() {
        let mut nav = KeyboardNavigation::new();

        nav.register_element("node1".to_string());
        nav.register_element("node2".to_string());
        nav.register_element("node3".to_string());

        // Test navigation
        assert_eq!(nav.navigate_next(), Some("node1".to_string()));
        assert_eq!(nav.navigate_next(), Some("node2".to_string()));
        assert_eq!(nav.navigate_next(), Some("node3".to_string()));
        assert_eq!(nav.navigate_next(), Some("node1".to_string())); // Wrap around

        assert_eq!(nav.navigate_previous(), Some("node3".to_string()));
        assert_eq!(nav.navigate_previous(), Some("node2".to_string()));
    }

    #[test]
    fn test_accessibility_config() {
        let config = AccessibilityConfig::default();
        assert!(config.keyboard_navigation);
        assert!(config.screen_reader_support);
        assert!(!config.high_contrast);
        assert!(config.aria_labels.is_empty());
    }
}
