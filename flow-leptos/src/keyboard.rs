//! Keyboard event handling for Leptos Flow
//!
//! Provides keyboard shortcuts and navigation for flow editors

use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventTarget, KeyboardEvent};

use crate::signals::{FlowState, ViewportState};
use flow_core::{Graph, KeyboardShortcut, NavigationDirection};

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

        let shortcut = match key.as_str() {
            // Select All (Ctrl+A)
            "a" | "A" if modifiers.ctrl => {
                event.prevent_default();
                Some(KeyboardShortcut::SelectAll)
            }

            // Delete selected nodes
            "Delete" | "Backspace" => {
                event.prevent_default();
                Some(KeyboardShortcut::Delete)
            }

            // Clear selection (Escape)
            "Escape" => Some(KeyboardShortcut::Escape),

            // Navigation (Arrow keys)
            "ArrowRight" => {
                event.prevent_default();
                Some(KeyboardShortcut::ArrowRight)
            }

            "ArrowLeft" => {
                event.prevent_default();
                Some(KeyboardShortcut::ArrowLeft)
            }

            "ArrowDown" => {
                event.prevent_default();
                Some(KeyboardShortcut::ArrowDown)
            }

            "ArrowUp" => {
                event.prevent_default();
                Some(KeyboardShortcut::ArrowUp)
            }

            // Tab navigation (legacy support)
            "Tab" => {
                event.prevent_default();
                if modifiers.shift {
                    Some(KeyboardShortcut::ArrowLeft)
                } else {
                    Some(KeyboardShortcut::ArrowRight)
                }
            }

            _ => None,
        };

        if let Some(shortcut) = shortcut {
            match shortcut {
                KeyboardShortcut::Delete => {
                    // Handle destructive operation that modifies the graph
                    graph.update(|graph_val| {
                        flow_state.update(|state| {
                            state.handle_destructive_keyboard_shortcut(graph_val, shortcut);
                        });
                    });
                }
                _ => {
                    // Handle non-destructive operations
                    flow_state.update(|state| {
                        let graph_val = graph.get_untracked();
                        state.handle_keyboard_shortcut(&graph_val, shortcut);
                    });
                }
            }
        }
    }
}

/// Handle mouse click events with keyboard modifiers
pub fn handle_node_click<N, E>(
    graph: &Graph<N, E>,
    flow_state: &mut FlowState,
    node_id: flow_core::NodeId,
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
    use flow_core::{Node, Position};

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
        graph
            .add_node(Node::simple("node1", Position::new(100.0, 100.0)))
            .unwrap();

        let mut flow_state = FlowState::new();
        let node_id = flow_core::NodeId::new("node1");

        // Test regular click
        let modifiers = KeyboardModifiers::default();
        handle_node_click(&graph, &mut flow_state, node_id.clone(), modifiers);
        assert!(flow_state.is_node_selected(&node_id));

        // Test Ctrl+Click (toggle)
        let modifiers = KeyboardModifiers {
            ctrl: true,
            shift: false,
            alt: false,
        };
        handle_node_click(&graph, &mut flow_state, node_id.clone(), modifiers);
        assert!(!flow_state.is_node_selected(&node_id)); // Should be deselected
    }
}
