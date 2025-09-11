//! Keyboard event handling for Leptos Flow
//!
//! Provides keyboard shortcuts and navigation for flow editors

use leptos::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{KeyboardEvent, EventTarget};

use leptos_flow_core::{Graph, NavigationDirection};
use crate::signals::{FlowState, ViewportState};

/// Keyboard modifiers state
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyboardModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

impl KeyboardModifiers {
    fn from_event(event: &KeyboardEvent) -> Self {
        Self {
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            alt: event.alt_key(),
        }
    }
}

/// Create keyboard shortcuts for flow editor
pub fn create_keyboard_shortcuts<N, E>(
    graph: RwSignal<Graph<N, E>>,
    flow_state: RwSignal<FlowState>,
    _viewport_state: RwSignal<ViewportState>,
) -> impl Fn(KeyboardEvent) -> ()
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    move |event: KeyboardEvent| {
        let modifiers = KeyboardModifiers::from_event(&event);
        let key = event.key();

        match key.as_str() {
            // Select All (Ctrl+A)
            "a" | "A" if modifiers.ctrl => {
                event.prevent_default();
                flow_state.update(|state| {
                    let graph_val = graph.get_untracked();
                    state.select_all(&graph_val);
                });
            }

            // Clear selection (Escape)
            "Escape" => {
                flow_state.update(|state| {
                    state.clear_selection();
                });
            }

            // Navigation (Arrow keys)
            "ArrowRight" => {
                event.prevent_default();
                flow_state.update(|state| {
                    let graph_val = graph.get_untracked();
                    state.navigate_selection(&graph_val, NavigationDirection::Next);
                });
            }

            "ArrowLeft" => {
                event.prevent_default();
                flow_state.update(|state| {
                    let graph_val = graph.get_untracked();
                    state.navigate_selection(&graph_val, NavigationDirection::Previous);
                });
            }

            "ArrowDown" => {
                event.prevent_default();
                flow_state.update(|state| {
                    let graph_val = graph.get_untracked();
                    state.navigate_selection(&graph_val, NavigationDirection::Next);
                });
            }

            "ArrowUp" => {
                event.prevent_default();
                flow_state.update(|state| {
                    let graph_val = graph.get_untracked();
                    state.navigate_selection(&graph_val, NavigationDirection::Previous);
                });
            }

            // Tab navigation
            "Tab" => {
                event.prevent_default();
                flow_state.update(|state| {
                    let graph_val = graph.get_untracked();
                    let direction = if modifiers.shift {
                        NavigationDirection::Previous
                    } else {
                        NavigationDirection::Next
                    };
                    state.navigate_selection(&graph_val, direction);
                });
            }

            _ => {
                // Other keys pass through
            }
        }
    }
}

/// Handle mouse click events with keyboard modifiers
pub fn handle_node_click<N, E>(
    graph: &Graph<N, E>,
    flow_state: &mut FlowState,
    node_id: leptos_flow_core::NodeId,
    modifiers: KeyboardModifiers,
) where
    N: Clone,
    E: Clone,
{
    if modifiers.ctrl {
        // Ctrl+Click: Toggle selection
        flow_state.toggle_node_selection(node_id);
    } else if modifiers.shift {
        // Shift+Click: Multi-select (extend selection)
        flow_state.add_node_to_selection(node_id);
    } else {
        // Regular click: Single select
        flow_state.select_node(node_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_flow_core::{Node, Position};

    #[test]
    fn test_keyboard_modifiers() {
        // Test modifiers detection (Note: requires actual KeyboardEvent for full test)
        let modifiers = KeyboardModifiers::default();
        assert!(!modifiers.ctrl);
        assert!(!modifiers.shift);
        assert!(!modifiers.alt);
    }

    #[test]
    fn test_handle_node_click() {
        let mut graph = Graph::<(), ()>::new();
        graph.add_node(Node::simple("node1", Position::new(100.0, 100.0))).unwrap();

        let mut flow_state = FlowState::new();
        let node_id = leptos_flow_core::NodeId::new("node1");

        // Test regular click
        let modifiers = KeyboardModifiers::default();
        handle_node_click(&graph, &mut flow_state, node_id.clone(), modifiers);
        assert!(flow_state.is_node_selected(&node_id));

        // Test Ctrl+Click (toggle)
        let modifiers = KeyboardModifiers { ctrl: true, shift: false, alt: false };
        handle_node_click(&graph, &mut flow_state, node_id.clone(), modifiers);
        assert!(!flow_state.is_node_selected(&node_id)); // Should be deselected
    }
}
