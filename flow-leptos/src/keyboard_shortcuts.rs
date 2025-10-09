//! Keyboard shortcuts for Flow-RS Leptos components
//!
//! Provides comprehensive keyboard shortcut support with:
//! - Standard editing shortcuts (Ctrl+Z, Ctrl+Y, etc.)
//! - Custom shortcut registration
//! - Keyboard event handling
//! - Integration with history and selection systems

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{KeyboardEvent, Element};
use std::collections::HashSet;
use std::ops::Deref;
use flow_rs_core::{NodeId, Position, Graph};
use crate::signals::{FlowState, ViewportState};
use crate::selection::SelectionManager;
use crate::history::{HistoryManager, ClipboardManager};
use crate::context_menu::ContextMenuAction;

/// Keyboard shortcut action types
#[derive(Clone, Debug, PartialEq)]
pub enum KeyboardAction {
    Undo,
    Redo,
    Copy,
    Cut,
    Paste,
    SelectAll,
    ClearSelection,
    DeleteSelected,
    ZoomIn,
    ZoomOut,
    ResetZoom,
    FitToScreen,
}

/// Keyboard shortcuts manager
#[derive(Clone)]
pub struct KeyboardManager {
    shortcuts: std::collections::HashMap<String, KeyboardShortcut>,
    pressed_keys: HashSet<String>,
}

#[derive(Clone, Debug)]
pub struct KeyboardShortcut {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub action: KeyboardAction,
    pub description: String,
    pub enabled: bool,
}

impl KeyboardManager {
    pub fn new() -> Self {
        let mut shortcuts = std::collections::HashMap::new();

        // Standard editing shortcuts
        shortcuts.insert("undo".to_string(), KeyboardShortcut {
            key: "z".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::Undo,
            description: "Undo last action".to_string(),
            enabled: true,
        });

        shortcuts.insert("redo".to_string(), KeyboardShortcut {
            key: "y".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::Redo,
            description: "Redo last undone action".to_string(),
            enabled: true,
        });

        shortcuts.insert("redo_alt".to_string(), KeyboardShortcut {
            key: "z".to_string(),
            ctrl: true,
            shift: true,
            alt: false,
            action: KeyboardAction::Redo,
            description: "Redo last undone action (alternative)".to_string(),
            enabled: true,
        });

        shortcuts.insert("copy".to_string(), KeyboardShortcut {
            key: "c".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::Copy,
            description: "Copy selected nodes".to_string(),
            enabled: true,
        });

        shortcuts.insert("cut".to_string(), KeyboardShortcut {
            key: "x".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::Cut,
            description: "Cut selected nodes".to_string(),
            enabled: true,
        });

        shortcuts.insert("paste".to_string(), KeyboardShortcut {
            key: "v".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::Paste,
            description: "Paste nodes from clipboard".to_string(),
            enabled: true,
        });

        shortcuts.insert("select_all".to_string(), KeyboardShortcut {
            key: "a".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::SelectAll,
            description: "Select all nodes".to_string(),
            enabled: true,
        });

        shortcuts.insert("delete".to_string(), KeyboardShortcut {
            key: "Delete".to_string(),
            ctrl: false,
            shift: false,
            alt: false,
            action: KeyboardAction::DeleteSelected,
            description: "Delete selected nodes".to_string(),
            enabled: true,
        });

        shortcuts.insert("clear_selection".to_string(), KeyboardShortcut {
            key: "Escape".to_string(),
            ctrl: false,
            shift: false,
            alt: false,
            action: KeyboardAction::ClearSelection,
            description: "Clear selection".to_string(),
            enabled: true,
        });

        // View shortcuts
        shortcuts.insert("zoom_in".to_string(), KeyboardShortcut {
            key: "=".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::ZoomIn,
            description: "Zoom in".to_string(),
            enabled: true,
        });

        shortcuts.insert("zoom_in_alt".to_string(), KeyboardShortcut {
            key: "+".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::ZoomIn,
            description: "Zoom in (alternative)".to_string(),
            enabled: true,
        });

        shortcuts.insert("zoom_out".to_string(), KeyboardShortcut {
            key: "-".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::ZoomOut,
            description: "Zoom out".to_string(),
            enabled: true,
        });

        shortcuts.insert("reset_zoom".to_string(), KeyboardShortcut {
            key: "0".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::ResetZoom,
            description: "Reset zoom to 100%".to_string(),
            enabled: true,
        });

        shortcuts.insert("fit_to_screen".to_string(), KeyboardShortcut {
            key: "1".to_string(),
            ctrl: true,
            shift: false,
            alt: false,
            action: KeyboardAction::FitToScreen,
            description: "Fit graph to screen".to_string(),
            enabled: true,
        });

        Self {
            shortcuts,
            pressed_keys: HashSet::new(),
        }
    }

    /// Handle key down event
    pub fn handle_key_down(&mut self, key: &str, ctrl: bool, shift: bool, alt: bool) -> Option<KeyboardAction> {
        self.pressed_keys.insert(key.to_string());

        // Find matching shortcut
        for shortcut in self.shortcuts.values() {
            if shortcut.key == key &&
               shortcut.ctrl == ctrl &&
               shortcut.shift == shift &&
               shortcut.alt == alt &&
               shortcut.enabled {
                return Some(shortcut.action.clone());
            }
        }

        None
    }

    /// Handle key up event
    pub fn handle_key_up(&mut self, key: &str) {
        self.pressed_keys.remove(key);
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.pressed_keys.contains(key)
    }

    /// Get all shortcuts
    pub fn get_all_shortcuts(&self) -> &std::collections::HashMap<String, KeyboardShortcut> {
        &self.shortcuts
    }

    /// Enable/disable shortcut
    pub fn set_shortcut_enabled(&mut self, action_name: &str, enabled: bool) {
        if let Some(shortcut) = self.shortcuts.get_mut(action_name) {
            shortcut.enabled = enabled;
        }
    }
}

/// Keyboard shortcuts hook for Leptos components
pub fn use_keyboard_shortcuts(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    graph: RwSignal<Graph<String, String>>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
    selection_manager: RwSignal<SelectionManager>,
    history_manager: RwSignal<HistoryManager>,
    clipboard_manager: RwSignal<ClipboardManager>,
) -> (ReadSignal<Option<KeyboardAction>>, impl Fn() + 'static) {
    let keyboard_manager = RwSignal::new(KeyboardManager::new());
    let last_action = RwSignal::new(None::<KeyboardAction>);

    // Keyboard event handlers
    let key_down_handler = move |event: KeyboardEvent| {
        let key = event.key();
        let ctrl = event.ctrl_key() || event.meta_key(); // Cmd on Mac, Ctrl on others
        let shift = event.shift_key();
        let alt = event.alt_key();

        // Handle special keys that should not trigger shortcuts
        match key.as_str() {
            "Control" | "Shift" | "Alt" | "Meta" | "CapsLock" | "Tab" => {
                return;
            }
            _ => {}
        }

        // Prevent default for handled shortcuts
        if let Some(action) = keyboard_manager.get().handle_key_down(&key, ctrl, shift, alt) {
            event.prevent_default();

            // Execute the action
            execute_keyboard_action(
                &action,
                &graph,
                &flow_state,
                &viewport_state,
                &selection_manager,
                &history_manager,
                &clipboard_manager,
            );

            last_action.set(Some(action));
        }
    };

    let key_up_handler = move |event: KeyboardEvent| {
        keyboard_manager.update(|km| km.handle_key_up(&event.key()));
    };

    // Set up event listeners when canvas is available
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_element = canvas.deref().clone();

            // Add keyboard event listeners to window (since canvas might not always have focus)
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback(
                    "keydown",
                    &Closure::wrap(Box::new(key_down_handler.clone()) as Box<dyn FnMut(KeyboardEvent)>)
                        .into_js_value()
                        .unchecked_into(),
                );

                let _ = window.add_event_listener_with_callback(
                    "keyup",
                    &Closure::wrap(Box::new(key_up_handler.clone()) as Box<dyn FnMut(KeyboardEvent)>)
                        .into_js_value()
                        .unchecked_into(),
                );
            }
        }
    });

    // Return last action signal and cleanup function
    (
        last_action.read_only(),
        move || {
            // Cleanup would go here if needed
        },
    )
}

/// Execute keyboard action
fn execute_keyboard_action(
    action: &KeyboardAction,
    graph: &RwSignal<Graph<String, String>>,
    flow_state: &RwSignal<FlowState>,
    viewport_state: &RwSignal<ViewportState>,
    selection_manager: &RwSignal<SelectionManager>,
    history_manager: &RwSignal<HistoryManager>,
    clipboard_manager: &RwSignal<ClipboardManager>,
) {
    match action {
        KeyboardAction::Undo => {
            history_manager.update(|hm| {
                if let Some(entry) = hm.undo() {
                    let operation = entry.operation.clone();
                    let inverse_op = hm.get_inverse(&operation);
                    graph.update(|g| {
                        let _ = hm.apply_operation(
                            &inverse_op,
                            g,
                            |data, pos| data.to_string(),
                            |data, source, target| data.to_string(),
                        );
                    });
                }
            });
        }
        KeyboardAction::Redo => {
            history_manager.update(|hm| {
                if let Some(entry) = hm.redo() {
                    let operation = entry.operation.clone();
                    drop(entry); // Release the borrow on entry
                    graph.update(|g| {
                        let _ = hm.apply_operation(
                            &operation,
                            g,
                            |data, pos| data.to_string(),
                            |data, source, target| data.to_string(),
                        );
                    });
                }
            });
        }
        KeyboardAction::Copy => {
            let selected_nodes = selection_manager.get().selected_nodes().iter().cloned().collect::<Vec<_>>();
            if !selected_nodes.is_empty() {
                clipboard_manager.update(|cm| {
                    cm.copy_nodes(&selected_nodes, &graph.get());
                });
            }
        }
        KeyboardAction::Cut => {
            let selected_nodes = selection_manager.get().selected_nodes().iter().cloned().collect::<Vec<_>>();
            if !selected_nodes.is_empty() {
                clipboard_manager.update(|cm| {
                    cm.copy_nodes(&selected_nodes, &graph.get());
                });
                // Delete selected nodes (would need to be implemented)
                // This is a placeholder - actual implementation would record in history
            }
        }
        KeyboardAction::Paste => {
            let viewport = viewport_state.get();
            let center_pos = viewport.viewport.screen_to_flow(Position::new(
                viewport.viewport.width / 2.0,
                viewport.viewport.height / 2.0,
            ));

            clipboard_manager.update(|cm| {
                let _ = cm.paste_nodes(
                    center_pos,
                    &mut graph.get_untracked(),
                    |data, pos| data.to_string(),
                    |data, source, target| data.to_string(),
                    || NodeId::new(format!("node-{}", js_sys::Math::random())),
                );
            });
        }
        KeyboardAction::SelectAll => {
            selection_manager.update(|sm| {
                sm.select_all(&graph.get());
            });
        }
        KeyboardAction::ClearSelection => {
            selection_manager.update(|sm| sm.clear_selection());
        }
        KeyboardAction::DeleteSelected => {
            let selected_nodes = selection_manager.get().selected_nodes().iter().cloned().collect::<Vec<_>>();
            if !selected_nodes.is_empty() {
                graph.update(|g| {
                    for node_id in &selected_nodes {
                        let _ = g.remove_node(node_id);
                    }
                });
                selection_manager.update(|sm| sm.clear_selection());
            }
        }
        KeyboardAction::ZoomIn => {
            viewport_state.update(|v| {
                let new_zoom = (v.viewport.zoom * 1.2).min(5.0); // Max zoom 500%
                v.viewport.zoom = new_zoom;
            });
        }
        KeyboardAction::ZoomOut => {
            viewport_state.update(|v| {
                let new_zoom = (v.viewport.zoom / 1.2).max(0.1); // Min zoom 10%
                v.viewport.zoom = new_zoom;
            });
        }
        KeyboardAction::ResetZoom => {
            viewport_state.update(|v| {
                v.viewport.zoom = 1.0;
            });
        }
        KeyboardAction::FitToScreen => {
            // Calculate bounds of all nodes and fit to screen
            let graph_val = graph.get();
            let nodes: Vec<_> = graph_val.nodes().collect();

            if !nodes.is_empty() {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut max_y = f64::NEG_INFINITY;

                for node in nodes {
                    min_x = min_x.min(node.position.x - 50.0);
                    min_y = min_y.min(node.position.y - 25.0);
                    max_x = max_x.max(node.position.x + 50.0);
                    max_y = max_y.max(node.position.y + 25.0);
                }

                let graph_width = max_x - min_x;
                let graph_height = max_y - min_y;

                viewport_state.update(|v| {
                    let viewport_width = v.viewport.width;
                    let viewport_height = v.viewport.height;

                    // Calculate zoom to fit
                    let zoom_x = viewport_width / graph_width;
                    let zoom_y = viewport_height / graph_height;
                    let zoom = zoom_x.min(zoom_y).min(1.0); // Don't zoom in more than 100%

                    v.viewport.zoom = zoom;
                    v.viewport.x = min_x + graph_width / 2.0 - viewport_width / (2.0 * zoom);
                    v.viewport.y = min_y + graph_height / 2.0 - viewport_height / (2.0 * zoom);
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_manager() {
        let mut manager = KeyboardManager::new();

        // Test undo shortcut
        let action = manager.handle_key_down("z", true, false, false);
        assert!(action.is_some());
        assert_eq!(action.unwrap(), KeyboardAction::Undo);

        // Test non-existent shortcut
        let action = manager.handle_key_down("z", false, false, false);
        assert!(action.is_none());

        // Test key up
        manager.handle_key_up("z");
        assert!(!manager.is_key_pressed("z"));
    }

    #[test]
    fn test_shortcut_registration() {
        let manager = KeyboardManager::new();

        let shortcuts = manager.get_all_shortcuts();
        assert!(shortcuts.contains_key("undo"));
        assert!(shortcuts.contains_key("copy"));
        assert!(shortcuts.contains_key("zoom_in"));
    }

    #[test]
    fn test_shortcut_enabling() {
        let mut manager = KeyboardManager::new();

        // Disable undo shortcut
        manager.set_shortcut_enabled("undo", false);

        // Should not trigger
        let action = manager.handle_key_down("z", true, false, false);
        assert!(action.is_none());

        // Re-enable
        manager.set_shortcut_enabled("undo", true);

        // Should trigger now
        let action = manager.handle_key_down("z", true, false, false);
        assert!(action.is_some());
        assert_eq!(action.unwrap(), KeyboardAction::Undo);
    }
}
