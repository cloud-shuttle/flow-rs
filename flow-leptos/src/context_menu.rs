//! Context menu system for Flow-RS Leptos components
//!
//! Provides comprehensive right-click context menus with:
//! - Dynamic menu generation based on context
//! - Node-specific actions (copy, delete, properties)
//! - Canvas actions (paste, select all, clear)
//! - Selection-based bulk operations
//! - Keyboard shortcut hints
//! - Professional styling and animations

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MouseEvent, Element, HtmlElement, HtmlDivElement};
use std::collections::HashMap;
use flow_rs_core::{NodeId, Position, Graph};
use crate::signals::{FlowState, ViewportState};
use crate::selection::SelectionMode;

/// Context menu item types
#[derive(Clone, Debug)]
pub enum ContextMenuItem {
    Action {
        label: String,
        action: String,
        shortcut: Option<String>,
        enabled: bool,
    },
    Separator,
    Submenu {
        label: String,
        items: Vec<ContextMenuItem>,
        enabled: bool,
    },
}

/// Context menu state
#[derive(Clone, Debug)]
pub struct ContextMenuState {
    pub visible: bool,
    pub position: Position,
    pub items: Vec<ContextMenuItem>,
    pub target_node: Option<NodeId>,
    pub selected_nodes: Vec<NodeId>,
}

/// Context menu manager
pub struct ContextMenuManager {
    state: ContextMenuState,
}

impl ContextMenuManager {
    pub fn new() -> Self {
        Self {
            state: ContextMenuState {
                visible: false,
                position: Position::new(0.0, 0.0),
                items: Vec::new(),
                target_node: None,
                selected_nodes: Vec::new(),
            },
        }
    }

    /// Show context menu at position
    pub fn show_menu(
        &mut self,
        position: Position,
        target_node: Option<NodeId>,
        selected_nodes: &[NodeId],
        graph: &Graph<(), ()>,
    ) {
        self.state.position = position;
        self.state.target_node = target_node;
        self.state.selected_nodes = selected_nodes.to_vec();
        self.state.items = self.generate_menu_items(graph);
        self.state.visible = true;
    }

    /// Hide context menu
    pub fn hide_menu(&mut self) {
        self.state.visible = false;
        self.state.items.clear();
        self.state.target_node = None;
        self.state.selected_nodes.clear();
    }

    /// Get current menu state
    pub fn state(&self) -> &ContextMenuState {
        &self.state
    }

    /// Generate menu items based on context
    fn generate_menu_items(&self, graph: &Graph<(), ()>) -> Vec<ContextMenuItem> {
        let mut items = Vec::new();
        let has_selection = !self.state.selected_nodes.is_empty();
        let target_is_selected = self.state.target_node
            .as_ref()
            .map(|id| self.state.selected_nodes.contains(id))
            .unwrap_or(false);

        // Node-specific actions
        if let Some(node_id) = &self.state.target_node {
            items.push(ContextMenuItem::Action {
                label: "Copy".to_string(),
                action: "copy".to_string(),
                shortcut: Some("Ctrl+C".to_string()),
                enabled: true,
            });

            if has_selection && !target_is_selected {
                items.push(ContextMenuItem::Action {
                    label: "Add to Selection".to_string(),
                    action: "add_to_selection".to_string(),
                    shortcut: None,
                    enabled: true,
                });
            }

            items.push(ContextMenuItem::Separator);

            items.push(ContextMenuItem::Action {
                label: "Delete".to_string(),
                action: "delete".to_string(),
                shortcut: Some("Delete".to_string()),
                enabled: true,
            });

            items.push(ContextMenuItem::Separator);

            items.push(ContextMenuItem::Action {
                label: "Properties".to_string(),
                action: "properties".to_string(),
                shortcut: None,
                enabled: true,
            });
        }
        // Canvas actions (no specific target)
        else {
            if has_selection {
                items.push(ContextMenuItem::Action {
                    label: "Copy Selected".to_string(),
                    action: "copy_selected".to_string(),
                    shortcut: Some("Ctrl+C".to_string()),
                    enabled: true,
                });

                items.push(ContextMenuItem::Action {
                    label: "Cut Selected".to_string(),
                    action: "cut_selected".to_string(),
                    shortcut: Some("Ctrl+X".to_string()),
                    enabled: true,
                });

                items.push(ContextMenuItem::Separator);

                items.push(ContextMenuItem::Action {
                    label: "Delete Selected".to_string(),
                    action: "delete_selected".to_string(),
                    shortcut: Some("Delete".to_string()),
                    enabled: true,
                });

                items.push(ContextMenuItem::Separator);
            }

            items.push(ContextMenuItem::Action {
                label: "Paste".to_string(),
                action: "paste".to_string(),
                shortcut: Some("Ctrl+V".to_string()),
                enabled: self.has_clipboard_data(),
            });

            items.push(ContextMenuItem::Separator);

            items.push(ContextMenuItem::Action {
                label: "Select All".to_string(),
                action: "select_all".to_string(),
                shortcut: Some("Ctrl+A".to_string()),
                enabled: !graph.nodes().collect::<Vec<_>>().is_empty(),
            });

            items.push(ContextMenuItem::Action {
                label: "Clear Selection".to_string(),
                action: "clear_selection".to_string(),
                shortcut: Some("Escape".to_string()),
                enabled: has_selection,
            });

            items.push(ContextMenuItem::Separator);

            // Layout submenu
            items.push(ContextMenuItem::Submenu {
                label: "Layout".to_string(),
                items: vec![
                    ContextMenuItem::Action {
                        label: "Force Directed".to_string(),
                        action: "layout_force_directed".to_string(),
                        shortcut: None,
                        enabled: true,
                    },
                    ContextMenuItem::Action {
                        label: "Grid".to_string(),
                        action: "layout_grid".to_string(),
                        shortcut: None,
                        enabled: true,
                    },
                    ContextMenuItem::Action {
                        label: "Hierarchical".to_string(),
                        action: "layout_hierarchical".to_string(),
                        shortcut: None,
                        enabled: true,
                    },
                ],
                enabled: true,
            });
        }

        items
    }

    /// Check if clipboard has data
    fn has_clipboard_data(&self) -> bool {
        // This would be implemented with actual clipboard checking
        // For now, return false
        false
    }

    /// Execute menu action
    pub fn execute_action(&self, action: &str) -> ContextMenuAction {
        match action {
            "copy" => ContextMenuAction::Copy {
                node_ids: self.state.target_node.as_ref().map(|id| vec![id.clone()]).unwrap_or_default(),
            },
            "copy_selected" => ContextMenuAction::Copy {
                node_ids: self.state.selected_nodes.clone(),
            },
            "cut_selected" => ContextMenuAction::Cut {
                node_ids: self.state.selected_nodes.clone(),
            },
            "paste" => ContextMenuAction::Paste {
                position: self.state.position,
            },
            "delete" => ContextMenuAction::Delete {
                node_ids: self.state.target_node.as_ref().map(|id| vec![id.clone()]).unwrap_or_default(),
            },
            "delete_selected" => ContextMenuAction::Delete {
                node_ids: self.state.selected_nodes.clone(),
            },
            "add_to_selection" => ContextMenuAction::AddToSelection {
                node_id: self.state.target_node.as_ref().unwrap().clone(),
            },
            "select_all" => ContextMenuAction::SelectAll,
            "clear_selection" => ContextMenuAction::ClearSelection,
            "properties" => ContextMenuAction::ShowProperties {
                node_id: self.state.target_node.as_ref().unwrap().clone(),
            },
            "layout_force_directed" => ContextMenuAction::ApplyLayout {
                algorithm: "force_directed".to_string(),
            },
            "layout_grid" => ContextMenuAction::ApplyLayout {
                algorithm: "grid".to_string(),
            },
            "layout_hierarchical" => ContextMenuAction::ApplyLayout {
                algorithm: "hierarchical".to_string(),
            },
            _ => ContextMenuAction::None,
        }
    }
}

/// Actions that can be executed from context menu
#[derive(Clone, Debug)]
pub enum ContextMenuAction {
    Copy { node_ids: Vec<NodeId> },
    Cut { node_ids: Vec<NodeId> },
    Paste { position: Position },
    Delete { node_ids: Vec<NodeId> },
    AddToSelection { node_id: NodeId },
    SelectAll,
    ClearSelection,
    ShowProperties { node_id: NodeId },
    ApplyLayout { algorithm: String },
    None,
}

/// Context menu renderer
pub struct ContextMenuRenderer;

impl ContextMenuRenderer {
    /// Render context menu
    pub fn render_menu(
        context_menu: &ContextMenuState,
        viewport: &flow_rs_core::Viewport,
    ) -> String {
        if !context_menu.visible {
            return String::new();
        }

        // Convert world position to screen position
        let screen_pos = viewport.flow_to_screen(context_menu.position);

        let mut html = format!(
            r#"<div class="flow-context-menu" style="position: fixed; left: {}px; top: {}px; z-index: 10000;">"#,
            screen_pos.x, screen_pos.y
        );

        for item in &context_menu.items {
            match item {
                ContextMenuItem::Action { label, action, shortcut, enabled } => {
                    let disabled_class = if *enabled { "" } else { " disabled" };
                    let shortcut_html = shortcut.as_ref()
                        .map(|s| format!(r#"<span class="shortcut">{}</span>"#, s))
                        .unwrap_or_default();

                    html.push_str(&format!(
                        r#"<div class="menu-item{}" data-action="{}">{}{}</div>"#,
                        disabled_class, action, label, shortcut_html
                    ));
                }
                ContextMenuItem::Separator => {
                    html.push_str(r#"<div class="menu-separator"></div>"#);
                }
                ContextMenuItem::Submenu { label, items, enabled } => {
                    let disabled_class = if *enabled { "" } else { " disabled" };
                    html.push_str(&format!(
                        r#"<div class="menu-item submenu{}">{}</div>"#,
                        disabled_class, label
                    ));
                    // Submenu items would be rendered when submenu is opened
                }
            }
        }

        html.push_str("</div>");
        html
    }

    /// Get CSS styles for context menu
    pub fn get_styles() -> &'static str {
        r#"
        .flow-context-menu {
            background: white;
            border: 1px solid #ccc;
            border-radius: 4px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
            padding: 4px 0;
            min-width: 160px;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            font-size: 13px;
            user-select: none;
        }

        .flow-context-menu .menu-item {
            padding: 6px 12px;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .flow-context-menu .menu-item:hover:not(.disabled) {
            background: #f0f8ff;
        }

        .flow-context-menu .menu-item.disabled {
            color: #999;
            cursor: default;
        }

        .flow-context-menu .menu-separator {
            height: 1px;
            background: #e0e0e0;
            margin: 4px 0;
        }

        .flow-context-menu .submenu::after {
            content: '▶';
            margin-left: 8px;
        }

        .flow-context-menu .shortcut {
            color: #666;
            font-size: 11px;
            margin-left: 20px;
        }
        "#
    }
}

/// Context menu hook for Leptos components
pub fn use_context_menu(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    graph: RwSignal<Graph<(), ()>>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
    selected_nodes: ReadSignal<std::collections::HashSet<NodeId>>,
) -> (ReadSignal<ContextMenuState>, impl Fn() + 'static) {
    let context_menu = RwSignal::new(ContextMenuManager::new());
    let menu_state = RwSignal::new(ContextMenuState {
        visible: false,
        position: Position::new(0.0, 0.0),
        items: Vec::new(),
        target_node: None,
        selected_nodes: Vec::new(),
    });

    // Sync menu state
    Effect::new(move |_| {
        menu_state.set(context_menu.get().state().clone());
    });

    // Context menu event handler (right-click)
    let context_menu_handler = move |event: MouseEvent| {
        event.prevent_default();

        // Get mouse position
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let graph_val = graph.get();
        let viewport_val = viewport_state.get();

        // Convert to world coordinates
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        // Check if right-clicking on a node
        let mut target_node = None;
        for node in graph_val.nodes() {
            let dx = world_pos.x - node.position.x;
            let dy = world_pos.y - node.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= 50.0 { // Assuming node radius
                target_node = Some(node.id.clone());
                break;
            }
        }

        // Get selected nodes
        let selected = selected_nodes.get();

        // Show context menu
        context_menu.update(|cm| {
            cm.show_menu(
                world_pos,
                target_node,
                &selected.iter().cloned().collect::<Vec<_>>(),
                &graph_val,
            );
        });

        // Create and show HTML context menu
        Self::show_html_context_menu(&context_menu.get().state().clone(), &viewport_val.viewport);
    };

    // Click handler to hide menu
    let click_handler = move |event: MouseEvent| {
        // Only hide if not clicking on menu
        let target = event.target().unwrap();
        if !target.dyn_ref::<HtmlElement>()
            .map(|el| el.class_list().contains("flow-context-menu") || el.closest(".flow-context-menu").is_some())
            .unwrap_or(false)
        {
            context_menu.update(|cm| cm.hide_menu());
            Self::hide_html_context_menu();
        }
    };

    // Set up event listeners when canvas is available
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_element = canvas.deref().clone();

            // Add context menu event listener
            let _ = canvas_element.add_event_listener_with_callback(
                "contextmenu",
                &Closure::wrap(Box::new(context_menu_handler.clone()) as Box<dyn FnMut(MouseEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );

            // Add click event listener to hide menu
            let _ = canvas_element.add_event_listener_with_callback(
                "click",
                &Closure::wrap(Box::new(click_handler.clone()) as Box<dyn FnMut(MouseEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );
        }
    });

    // Return menu state signal and cleanup function
    (
        menu_state.read_only(),
        move || {
            // Cleanup: hide any visible context menu
            Self::hide_html_context_menu();
        },
    )
}

impl ContextMenuRenderer {
    /// Show HTML context menu
    fn show_html_context_menu(menu_state: &ContextMenuState, viewport: &flow_rs_core::Viewport) {
        // Remove existing menu
        Self::hide_html_context_menu();

        // Create new menu
        let html = Self::render_menu(menu_state, viewport);

        if let Some(document) = web_sys::window().unwrap().document() {
            if let Ok(div) = document.create_element("div") {
                div.set_inner_html(&html);
                let _ = document.body().unwrap().append_child(&div);
            }
        }
    }

    /// Hide HTML context menu
    fn hide_html_context_menu() {
        if let Some(document) = web_sys::window().unwrap().document() {
            if let Ok(elements) = document.query_selector_all(".flow-context-menu") {
                for i in 0..elements.length() {
                    if let Ok(element) = elements.get(i) {
                        let _ = element.parent_node().unwrap().remove_child(&element);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_state() {
        let mut manager = ContextMenuManager::new();
        assert!(!manager.state().visible);

        let graph = Graph::new();
        manager.show_menu(
            Position::new(100.0, 200.0),
            Some(NodeId::new("test-node")),
            &[],
            &graph,
        );

        let state = manager.state();
        assert!(state.visible);
        assert_eq!(state.position, Position::new(100.0, 200.0));
        assert!(state.target_node.is_some());
        assert!(!state.items.is_empty());
    }

    #[test]
    fn test_menu_item_generation() {
        let manager = ContextMenuManager::new();
        let graph = Graph::new();

        let items = manager.generate_menu_items(&graph);
        assert!(!items.is_empty());

        // Should have paste, select all, etc.
        let paste_item = items.iter().find(|item| matches!(item, ContextMenuItem::Action { action, .. } if action == "paste"));
        assert!(paste_item.is_some());
    }

    #[test]
    fn test_execute_action() {
        let mut manager = ContextMenuManager::new();
        let graph = Graph::new();

        manager.show_menu(
            Position::new(100.0, 200.0),
            Some(NodeId::new("test-node")),
            &[],
            &graph,
        );

        let action = manager.execute_action("copy");
        match action {
            ContextMenuAction::Copy { node_ids } => {
                assert_eq!(node_ids.len(), 1);
                assert_eq!(node_ids[0], NodeId::new("test-node"));
            }
            _ => panic!("Expected Copy action"),
        }
    }
}
