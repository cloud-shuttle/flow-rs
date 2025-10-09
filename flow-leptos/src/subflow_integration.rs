//! Subflow integration for Flow-RS Leptos components
//!
//! Provides Leptos components and hooks for hierarchical graph editing with:
//! - Subflow navigation and breadcrumb UI
//! - Visual subflow indicators
//! - Cross-level edge rendering
//! - Hierarchical graph management

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MouseEvent, Element};
use std::collections::HashSet;
use flow_rs_core::{NodeId, Position};
use flow_rs_core::subflows::{HierarchicalGraph, HierarchicalNodeRef, NavigationState};
use crate::signals::{FlowState, ViewportState};
use crate::selection::SelectionMode;

/// Subflow-enabled Flow Editor component
#[component]
pub fn HierarchicalFlowEditor(
    /// Canvas width
    #[prop(default = 800)]
    width: u32,
    /// Canvas height
    #[prop(default = 600)]
    height: u32,
    /// Initial graph data
    #[prop(optional)]
    initial_graph: Option<HierarchicalGraph<(), ()>>,
) -> impl IntoView {
    let canvas_ref: NodeRef<leptos::html::Canvas> = NodeRef::new();
    let viewport = RwSignal::new(ViewportState::default());
    let flow_state = RwSignal::new(FlowState::default());

    // Hierarchical graph state
    let hierarchy = RwSignal::new(
        initial_graph.unwrap_or_else(|| HierarchicalGraph::new())
    );

    // Navigation state
    let navigation_state = RwSignal::new(NavigationState {
        path: Vec::new(),
        viewport_states: std::collections::HashMap::new(),
        zoom_levels: std::collections::HashMap::new(),
    });

    // UI state
    let show_breadcrumbs = RwSignal::new(true);
    let selected_nodes = RwSignal::new(HashSet::<NodeId>::new());

    // Get current graph for rendering
    let current_graph = move || hierarchy.get().current_graph().clone();

    // Get breadcrumb path
    let breadcrumbs = move || hierarchy.get().get_breadcrumb_path();

    // Handle subflow navigation
    let enter_subflow = move |node_id: NodeId| {
        hierarchy.update(|h| {
            let _ = h.enter_subflow(node_id);
        });
        navigation_state.update(|nav| {
            nav.path = hierarchy.get().navigation_path.clone();
        });
    };

    let exit_subflow = move || {
        hierarchy.update(|h| {
            h.exit_subflow();
        });
        navigation_state.update(|nav| {
            nav.path = hierarchy.get().navigation_path.clone();
        });
    };

    let go_to_root = move || {
        hierarchy.update(|h| {
            h.go_to_root();
        });
        navigation_state.update(|nav| {
            nav.path.clear();
        });
    };

    // Create subflow for node
    let create_subflow = move |node_id: NodeId| {
        hierarchy.update(|h| {
            let _ = h.create_subflow(node_id);
        });
    };

    // Handle double-click to enter subflow
    let handle_double_click = move |event: MouseEvent| {
        event.prevent_default();

        // Get click position
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let viewport_val = viewport.get();
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        // Find node at position
        let graph_val = hierarchy.get().current_graph().clone();
        for node in graph_val.nodes() {
            let dx = world_pos.x - node.position.x;
            let dy = world_pos.y - node.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= 50.0 { // Assuming node radius
                // Check if node has subflow
                if hierarchy.get().subflows.contains_key(&node.id) {
                    enter_subflow(node.id.clone());
                } else {
                    // Create subflow and enter it
                    create_subflow(node.id.clone());
                    enter_subflow(node.id.clone());
                }
                break;
            }
        }
    };

    // Handle right-click for context menu
    let handle_context_menu = move |event: MouseEvent| {
        event.prevent_default();

        // Get click position
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let viewport_val = viewport.get();
        let world_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        // Find node at position
        let graph_val = hierarchy.get().current_graph().clone();
        let mut target_node = None;

        for node in graph_val.nodes() {
            let dx = world_pos.x - node.position.x;
            let dy = world_pos.y - node.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= 50.0 {
                target_node = Some(node.id.clone());
                break;
            }
        }

        // Show context menu
        if let Some(node_id) = target_node {
            show_subflow_context_menu(node_id, world_pos);
        } else {
            show_canvas_context_menu(world_pos);
        }
    };

    view! {
        <div class="hierarchical-flow-editor" style="position: relative;">
            // Breadcrumb navigation
            <div
                class="breadcrumb-nav"
                class:hidden=move || !show_breadcrumbs.get()
                style="position: absolute; top: 10px; left: 10px; z-index: 100; background: rgba(255,255,255,0.9); padding: 8px 12px; border-radius: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
            >
                <button
                    on:click=move |_| go_to_root()
                    style="margin-right: 8px; padding: 4px 8px; border: 1px solid #ddd; background: white; border-radius: 4px; cursor: pointer;"
                >
                    "🏠"
                </button>
                {move || breadcrumbs().into_iter().enumerate().map(|(index, (node_id, label))| {
                    let is_last = index == breadcrumbs().len() - 1;
                    view! {
                        <>
                            {if index > 0 { Some(view! { <span style="margin: 0 8px; color: #666;">"▶"</span> }) } else { None }}
                            <button
                                on:click=move |_| {
                                    if index == 0 {
                                        go_to_root();
                                    } else {
                                        // Navigate to this level
                                        let target_path = breadcrumbs()[1..=index].iter().map(|(id, _)| id.clone()).collect::<Vec<_>>();
                                        hierarchy.update(|h| {
                                            h.navigation_path = target_path;
                                        });
                                        navigation_state.update(|nav| {
                                            nav.path = hierarchy.get().navigation_path.clone();
                                        });
                                    }
                                }
                                style=move || {
                                    if is_last {
                                        "padding: 4px 8px; border: 1px solid #007bff; background: #007bff; color: white; border-radius: 4px; cursor: pointer;"
                                    } else {
                                        "padding: 4px 8px; border: 1px solid #ddd; background: white; border-radius: 4px; cursor: pointer;"
                                    }
                                }
                            >
                                {label}
                            </button>
                        </>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Main canvas
            <canvas
                node_ref=canvas_ref
                width=width
                height=height
                style="border: 1px solid #ddd; cursor: crosshair;"
                on:dblclick=handle_double_click
                on:contextmenu=handle_context_menu
            />

            // Subflow instructions
            <div
                style="position: absolute; bottom: 10px; right: 10px; background: rgba(255,255,255,0.9); padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); max-width: 300px;"
            >
                <h4 style="margin: 0 0 8px 0; color: #333;">"Subflow Controls"</h4>
                <ul style="margin: 0; padding-left: 20px; color: #555;">
                    <li>"Double-click nodes to enter subflows"</li>
                    <li>"Use breadcrumbs to navigate hierarchy"</li>
                    <li>"Right-click for context menus"</li>
                    <li>"Nodes with subflows show 📁 icon"</li>
                </ul>
            </div>

            // Hierarchy stats
            <div
                style="position: absolute; bottom: 10px; left: 10px; background: rgba(255,255,255,0.9); padding: 8px 12px; border-radius: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); font-size: 12px; color: #666;"
            >
                {move || {
                    let stats = hierarchy.get().get_hierarchy_stats();
                    format!("Nodes: {} | Edges: {} | Depth: {} | Subflows: {}",
                           stats.total_nodes, stats.total_edges, stats.max_depth, stats.subflow_count)
                }}
            </div>
        </div>
    }
}

/// Show context menu for subflow operations
fn show_subflow_context_menu(node_id: NodeId, position: Position) {
    // This would integrate with the context menu system
    // For now, just log the action
    web_sys::console::log_1(&format!("Subflow context menu for node: {:?}", node_id).into());
}

/// Show context menu for canvas operations
fn show_canvas_context_menu(position: Position) {
    // This would integrate with the context menu system
    // For now, just log the action
    web_sys::console::log_1(&format!("Canvas context menu at: {:?}", position).into());
}

/// Subflow visualization utilities
pub struct SubflowVisualizer;

impl SubflowVisualizer {
    /// Get CSS styles for subflow UI elements
    pub fn get_styles() -> &'static str {
        r#"
        .hierarchical-flow-editor {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .breadcrumb-nav button:hover {
            background: #f0f8ff !important;
        }

        .subflow-node {
            position: relative;
        }

        .subflow-indicator {
            position: absolute;
            bottom: -8px;
            right: -8px;
            width: 16px;
            height: 16px;
            background: #007bff;
            border-radius: 2px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 10px;
            color: white;
            box-shadow: 0 1px 3px rgba(0,0,0,0.3);
        }

        .subflow-indicator::before {
            content: '📁';
        }

        .cross-level-edge {
            stroke-dasharray: 5,5;
            stroke: #666;
            stroke-width: 2;
            opacity: 0.7;
        }

        .hierarchy-stats {
            font-size: 11px;
            color: #666;
            background: rgba(255,255,255,0.8);
            padding: 4px 8px;
            border-radius: 4px;
        }
        "#
    }
}

/// Hook for managing hierarchical graph state
pub fn use_hierarchical_graph(
    initial_graph: Option<HierarchicalGraph<(), ()>>,
) -> (ReadSignal<HierarchicalGraph<(), ()>>, impl Fn() + 'static) {
    let hierarchy = RwSignal::new(
        initial_graph.unwrap_or_else(|| HierarchicalGraph::new())
    );

    let cleanup = move || {
        // Cleanup logic if needed
    };

    (hierarchy.read_only(), cleanup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_rs_core::Node;

    #[test]
    fn test_subflow_navigation() {
        let mut hierarchy: HierarchicalGraph<(), ()> = HierarchicalGraph::new();

        // Add a node to root
        let node = Node::new("node1".to_string(), Position::new(100.0, 100.0), ());
        hierarchy.root_graph.add_node(node).unwrap();

        // Create subflow
        hierarchy.create_subflow("node1".into()).unwrap();

        // Test navigation
        assert!(hierarchy.enter_subflow("node1".into()).is_ok());
        assert_eq!(hierarchy.navigation_path, vec!["node1".into()]);

        assert!(hierarchy.exit_subflow());
        assert!(hierarchy.navigation_path.is_empty());
    }

    #[test]
    fn test_breadcrumb_generation() {
        let mut hierarchy: HierarchicalGraph<String, String> = HierarchicalGraph::new();

        // Add nodes with data
        let node1 = Node::new("node1".to_string(), Position::new(100.0, 100.0), "Root Node".to_string());
        hierarchy.root_graph.add_node(node1).unwrap();

        hierarchy.create_subflow("node1".into()).unwrap();
        let node2 = Node::new("node2".to_string(), Position::new(50.0, 50.0), "Sub Node".to_string());
        hierarchy.subflows.get_mut(&"node1".into()).unwrap().add_node(node2).unwrap();

        // Navigate
        hierarchy.enter_subflow("node1".into()).unwrap();

        let breadcrumbs = hierarchy.get_breadcrumb_path();
        assert_eq!(breadcrumbs.len(), 2);
        assert_eq!(breadcrumbs[0].1, "Root");
        assert_eq!(breadcrumbs[1].1, "\"Root Node\"");
    }
}
