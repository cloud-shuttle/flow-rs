//! Controls component for flow editor
//!
//! Provides a control panel with common flow editor operations like zoom controls,
//! layout algorithm selection, and graph manipulation tools.

use crate::signals::ViewportState;
use flow_rs_core::layout::{
    CircularLayout, ForceDirectedLayout, GridLayout, HierarchicalLayout, LayoutAlgorithm,
};
use flow_rs_core::Graph;
// use flow_rs_core::Position; // Unused import
use leptos::prelude::*;

/// Configuration for the Controls component
#[derive(Debug, Clone)]
pub struct ControlsConfig {
    /// Show zoom controls
    pub show_zoom_controls: bool,
    /// Show layout algorithm selector
    pub show_layout_selector: bool,
    /// Show fit-to-screen button
    pub show_fit_to_screen: bool,
    /// Show clear/reset button
    pub show_clear_button: bool,
    /// Position of the controls panel
    pub position: ControlsPosition,
    /// Visual style theme
    pub theme: ControlsTheme,
}

/// Position of the controls panel
#[derive(Debug, Clone, PartialEq)]
pub enum ControlsPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: f64, y: f64 },
}

/// Visual theme for the controls
#[derive(Debug, Clone, PartialEq)]
pub enum ControlsTheme {
    Light,
    Dark,
    Auto,
}

/// Available layout algorithms for the selector
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    ForceDirected,
    Grid,
    Hierarchical,
    Circular,
}

impl Default for ControlsConfig {
    fn default() -> Self {
        Self {
            show_zoom_controls: true,
            show_layout_selector: true,
            show_fit_to_screen: true,
            show_clear_button: false,
            position: ControlsPosition::TopRight,
            theme: ControlsTheme::Light,
        }
    }
}

/// Controls component for flow editor operations
#[component]
pub fn Controls<N, E>(
    /// Flow graph
    graph: RwSignal<Graph<N, E>>,
    /// Viewport state
    viewport: RwSignal<ViewportState>,
    /// Controls configuration
    #[prop(default = ControlsConfig::default())]
    config: ControlsConfig,
    /// Optional zoom change handler
    #[prop(optional)]
    on_zoom_change: Option<WriteSignal<f64>>,
    /// Optional layout change handler
    #[prop(optional)]
    on_layout_change: Option<WriteSignal<LayoutType>>,
    /// Optional fit to screen handler
    #[prop(optional)]
    on_fit_to_screen: Option<WriteSignal<bool>>,
    /// Optional clear graph handler
    #[prop(optional)]
    on_clear_graph: Option<WriteSignal<bool>>,
) -> impl IntoView
where
    N: Clone + Send + Sync + 'static + PartialEq,
    E: Clone + Send + Sync + 'static + PartialEq,
{
    let current_zoom = Memo::new(move |_| viewport.get().viewport.zoom);
    let current_layout = RwSignal::new(LayoutType::ForceDirected);

    // Handle zoom in
    let handle_zoom_in = {
        let viewport = viewport.clone();
        move |_| {
            viewport.update(|v| {
                v.viewport.zoom = (v.viewport.zoom * 1.2).min(3.0);
            });
            if let Some(handler) = on_zoom_change {
                handler.set(viewport.get_untracked().viewport.zoom);
            }
        }
    };

    // Handle zoom out
    let handle_zoom_out = {
        let viewport = viewport.clone();
        move |_| {
            viewport.update(|v| {
                v.viewport.zoom = (v.viewport.zoom / 1.2).max(0.1);
            });
            if let Some(handler) = on_zoom_change {
                handler.set(viewport.get_untracked().viewport.zoom);
            }
        }
    };

    // Handle zoom reset
    let handle_zoom_reset = {
        let viewport = viewport.clone();
        move |_| {
            viewport.update(|v| {
                v.viewport.zoom = 1.0;
            });
            if let Some(handler) = on_zoom_change {
                handler.set(1.0);
            }
        }
    };

    // Handle layout change
    let handle_layout_change = move |layout_type: LayoutType| {
        current_layout.set(layout_type.clone());

        let mut graph_value = graph.get();
        let result = match layout_type {
            LayoutType::ForceDirected => {
                let mut layout = ForceDirectedLayout::new();
                layout.apply(&mut graph_value)
            }
            LayoutType::Grid => {
                let mut layout = GridLayout::new().columns(Some(3)).cell_size(100.0, 100.0);
                layout.apply(&mut graph_value)
            }
            LayoutType::Hierarchical => {
                let mut layout = HierarchicalLayout::builder()
                    .node_separation(150.0)
                    .level_separation(100.0)
                    .build();
                layout.apply(&mut graph_value)
            }
            LayoutType::Circular => {
                let mut layout = CircularLayout::new().radius(200.0);
                layout.apply(&mut graph_value)
            }
        };

        if result.is_ok() {
            graph.set(graph_value);
            if let Some(handler) = on_layout_change {
                handler.set(layout_type);
            }
        }
    };

    // Handle fit to screen
    let handle_fit_to_screen = move |_| {
        let graph_value = graph.get();
        if let Some(bounds) = calculate_graph_bounds(&graph_value) {
            viewport.update(|v| {
                // Center the viewport on the graph bounds
                v.viewport.x = bounds.center_x;
                v.viewport.y = bounds.center_y;

                // Calculate zoom to fit the graph with some padding
                let padding = 50.0;
                let zoom_x = (v.viewport.width - padding * 2.0) / bounds.width;
                let zoom_y = (v.viewport.height - padding * 2.0) / bounds.height;
                v.viewport.zoom = zoom_x.min(zoom_y).max(0.1).min(3.0);
            });
        }

            if let Some(handler) = on_fit_to_screen {
                handler.set(true);
            }
    };

    // Handle clear graph
    let handle_clear_graph = move |_| {
        graph.update(|g| {
            *g = Graph::new();
        });

            if let Some(handler) = on_clear_graph {
                handler.set(true);
            }
    };

    let position_style = match config.position {
        ControlsPosition::TopLeft => "top: 10px; left: 10px;".to_string(),
        ControlsPosition::TopRight => "top: 10px; right: 10px;".to_string(),
        ControlsPosition::BottomLeft => "bottom: 10px; left: 10px;".to_string(),
        ControlsPosition::BottomRight => "bottom: 10px; right: 10px;".to_string(),
        ControlsPosition::Custom { x, y } => format!("top: {}px; left: {}px;", y, x),
    };

    let theme_class = match config.theme {
        ControlsTheme::Light => "controls-light",
        ControlsTheme::Dark => "controls-dark",
        ControlsTheme::Auto => "controls-auto",
    };

    view! {
        <div class=format!("flow-controls {}", theme_class) style=format!(
            "position: absolute; {}; background: rgba(255, 255, 255, 0.9); border: 1px solid #ddd; border-radius: 8px; padding: 12px; display: flex; flex-direction: column; gap: 8px; z-index: 1000;",
            position_style
        )>
            <div class="zoom-controls" style=move || if config.show_zoom_controls { "display: flex; flex-direction: column; gap: 4px;" } else { "display: none;" }>
                <label style="font-size: 12px; font-weight: bold; margin-bottom: 4px;">"Zoom"</label>
                <div style="display: flex; gap: 4px; align-items: center;">
                    <button on:click=handle_zoom_out style="width: 24px; height: 24px; border: 1px solid #ccc; background: white; cursor: pointer; border-radius: 4px;">"-"</button>
                    <span style="min-width: 40px; text-align: center; font-size: 11px;">{move || format!("{:.0}%", current_zoom.get() * 100.0)}</span>
                    <button on:click=handle_zoom_in style="width: 24px; height: 24px; border: 1px solid #ccc; background: white; cursor: pointer; border-radius: 4px;">"+"</button>
                </div>
                <button on:click=handle_zoom_reset style="width: 100%; height: 20px; border: 1px solid #ccc; background: white; cursor: pointer; border-radius: 4px; font-size: 10px;">"Reset"</button>
            </div>

            <div class="layout-controls" style=move || if config.show_layout_selector { "display: flex; flex-direction: column; gap: 4px;" } else { "display: none;" }>
                <label style="font-size: 12px; font-weight: bold; margin-bottom: 4px;">"Layout"</label>
                <select
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let layout_type = match value.as_str() {
                            "grid" => LayoutType::Grid,
                            "hierarchical" => LayoutType::Hierarchical,
                            "circular" => LayoutType::Circular,
                            _ => LayoutType::ForceDirected,
                        };
                        handle_layout_change(layout_type);
                    }
                    style="width: 100%; padding: 4px; border: 1px solid #ccc; border-radius: 4px; font-size: 11px;"
                >
                    <option value="force">"Force Directed"</option>
                    <option value="grid">"Grid"</option>
                    <option value="hierarchical">"Hierarchical"</option>
                    <option value="circular">"Circular"</option>
                </select>
            </div>

            <div style=move || if config.show_fit_to_screen { "display: block;" } else { "display: none;" }>
                <button
                    on:click=handle_fit_to_screen
                    style="width: 100%; height: 24px; border: 1px solid #ccc; background: white; cursor: pointer; border-radius: 4px; font-size: 11px;"
                >
                    "Fit to Screen"
                </button>
            </div>

            <div style=move || if config.show_clear_button { "display: block;" } else { "display: none;" }>
                <button
                    on:click=handle_clear_graph
                    style="width: 100%; height: 24px; border: 1px solid #ff6b6b; background: #fff5f5; color: #c92a2a; cursor: pointer; border-radius: 4px; font-size: 11px;"
                >
                    "Clear Graph"
                </button>
            </div>
        </div>
    }
}

/// Simple bounding box for graph bounds calculation
#[derive(Debug, Clone)]
struct GraphBounds {
    center_x: f64,
    center_y: f64,
    width: f64,
    height: f64,
}

/// Calculate the bounding box of all nodes in the graph
fn calculate_graph_bounds<N, E>(graph: &Graph<N, E>) -> Option<GraphBounds> {
    if graph.nodes().count() == 0 {
        return None;
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for node in graph.nodes() {
        let left = node.position.x - node.size.width / 2.0;
        let right = node.position.x + node.size.width / 2.0;
        let top = node.position.y - node.size.height / 2.0;
        let bottom = node.position.y + node.size.height / 2.0;

        min_x = min_x.min(left);
        min_y = min_y.min(top);
        max_x = max_x.max(right);
        max_y = max_y.max(bottom);
    }

    Some(GraphBounds {
        center_x: (min_x + max_x) / 2.0,
        center_y: (min_y + max_y) / 2.0,
        width: max_x - min_x,
        height: max_y - min_y,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_rs_core::prelude::{EdgeBuilder, NodeBuilder};

    fn create_test_graph() -> Graph<(), ()> {
        let mut graph: Graph<(), ()> = Graph::new();

        graph
            .add_node(
                NodeBuilder::<()>::new("node1")
                    .position(0.0, 0.0)
                    .size(100.0, 50.0)
                    .build(),
            )
            .unwrap();

        graph
            .add_node(
                NodeBuilder::<()>::new("node2")
                    .position(200.0, 100.0)
                    .size(100.0, 50.0)
                    .build(),
            )
            .unwrap();

        graph
            .add_edge(
                EdgeBuilder::new()
                    .connect("node1", "node2")
                    .build()
                    .unwrap(),
            )
            .unwrap();

        graph
    }

    #[test]
    fn test_controls_config_default() {
        let config = ControlsConfig::default();
        assert_eq!(config.show_zoom_controls, true);
        assert_eq!(config.show_layout_selector, true);
        assert_eq!(config.show_fit_to_screen, true);
        assert_eq!(config.show_clear_button, false);
        assert_eq!(config.position, ControlsPosition::TopRight);
        assert_eq!(config.theme, ControlsTheme::Light);
    }

    #[test]
    fn test_controls_position_variants() {
        assert_eq!(ControlsPosition::TopLeft, ControlsPosition::TopLeft);
        assert_eq!(ControlsPosition::TopRight, ControlsPosition::TopRight);
        assert_eq!(ControlsPosition::BottomLeft, ControlsPosition::BottomLeft);
        assert_eq!(ControlsPosition::BottomRight, ControlsPosition::BottomRight);

        let custom = ControlsPosition::Custom { x: 100.0, y: 200.0 };
        assert_eq!(custom, ControlsPosition::Custom { x: 100.0, y: 200.0 });
    }

    #[test]
    fn test_controls_theme_variants() {
        assert_eq!(ControlsTheme::Light, ControlsTheme::Light);
        assert_eq!(ControlsTheme::Dark, ControlsTheme::Dark);
        assert_eq!(ControlsTheme::Auto, ControlsTheme::Auto);
    }

    #[test]
    fn test_layout_type_variants() {
        assert_eq!(LayoutType::ForceDirected, LayoutType::ForceDirected);
        assert_eq!(LayoutType::Grid, LayoutType::Grid);
        assert_eq!(LayoutType::Hierarchical, LayoutType::Hierarchical);
        assert_eq!(LayoutType::Circular, LayoutType::Circular);
    }

    #[test]
    fn test_calculate_graph_bounds_empty() {
        let graph: Graph<(), ()> = Graph::new();
        let bounds = calculate_graph_bounds(&graph);
        assert!(bounds.is_none());
    }

    #[test]
    fn test_calculate_graph_bounds() {
        let graph = create_test_graph();
        let bounds = calculate_graph_bounds(&graph).unwrap();

        // Node1: center (0,0), size 100x50 -> bounds (-50,-25) to (50,25)
        // Node2: center (200,100), size 100x50 -> bounds (150,75) to (250,125)
        // Total bounds: (-50,-25) to (250,125) -> center (100,50), size 300x150

        assert_eq!(bounds.center_x, 100.0);
        assert_eq!(bounds.center_y, 50.0);
        assert_eq!(bounds.width, 300.0);
        assert_eq!(bounds.height, 150.0);
    }

    #[test]
    fn test_controls_config_builder_pattern() {
        let config = ControlsConfig {
            show_zoom_controls: false,
            show_layout_selector: true,
            show_fit_to_screen: false,
            show_clear_button: true,
            position: ControlsPosition::BottomLeft,
            theme: ControlsTheme::Dark,
        };

        assert_eq!(config.show_zoom_controls, false);
        assert_eq!(config.show_layout_selector, true);
        assert_eq!(config.show_fit_to_screen, false);
        assert_eq!(config.show_clear_button, true);
        assert_eq!(config.position, ControlsPosition::BottomLeft);
        assert_eq!(config.theme, ControlsTheme::Dark);
    }

    #[test]
    fn test_bounds_calculation_single_node() {
        let mut graph: Graph<(), ()> = Graph::new();
        graph
            .add_node(
                NodeBuilder::<()>::new("single")
                    .position(100.0, 50.0)
                    .size(80.0, 40.0)
                    .build(),
            )
            .unwrap();

        let bounds = calculate_graph_bounds(&graph).unwrap();

        // Node at (100,50) with size 80x40
        // Bounds: (60,30) to (140,70) -> center (100,50), size 80x40
        assert_eq!(bounds.center_x, 100.0);
        assert_eq!(bounds.center_y, 50.0);
        assert_eq!(bounds.width, 80.0);
        assert_eq!(bounds.height, 40.0);
    }

    #[test]
    fn test_custom_position_coordinates() {
        let pos = ControlsPosition::Custom {
            x: 123.45,
            y: 678.90,
        };
        if let ControlsPosition::Custom { x, y } = pos {
            assert_eq!(x, 123.45);
            assert_eq!(y, 678.90);
        } else {
            panic!("Expected Custom position");
        }
    }
}
