//! Leptos components for flow editors

use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use leptos_flow_core::{Graph, Node, Edge, Position, Viewport};
use leptos_flow_renderer::{Renderer, RendererType};

#[cfg(feature = "canvas2d")]
use leptos_flow_renderer::Canvas2DRenderer;

use crate::signals::{FlowState, ViewportState};
use crate::events::{FlowEvent, NodeEvent, EdgeEvent};

/// Properties for the FlowEditor component
#[derive(Clone)]
pub struct FlowEditorProps<N, E>
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    /// The graph to display and edit
    pub graph: RwSignal<Graph<N, E>>,
    /// Viewport state for pan/zoom
    pub viewport: RwSignal<ViewportState>,
    /// Canvas width
    pub width: u32,
    /// Canvas height
    pub height: u32,
    /// Preferred renderer type
    pub renderer_type: Option<RendererType>,
    /// Event handlers
    pub on_node_select: Option<Box<dyn Fn(NodeEvent<N>) + 'static>>,
    pub on_edge_select: Option<Box<dyn Fn(EdgeEvent<E>) + 'static>>,
    pub on_flow_event: Option<Box<dyn Fn(FlowEvent) + 'static>>,
}

/// Main flow editor component
#[component]
pub fn FlowEditor<N, E>(
    /// The graph to display and edit
    graph: RwSignal<Graph<N, E>>,
    /// Canvas width
    #[prop(default = 800)]
    width: u32,
    /// Canvas height
    #[prop(default = 600)]
    height: u32,
    /// Preferred renderer type
    #[prop(optional)]
    renderer_type: Option<RendererType>,
    /// Event handler for node events
    #[prop(optional)]
    on_node_select: Option<Box<dyn Fn(NodeEvent<N>) + 'static>>,
    /// Event handler for edge events
    #[prop(optional)]
    on_edge_select: Option<Box<dyn Fn(EdgeEvent<E>) + 'static>>,
    /// Event handler for flow events
    #[prop(optional)]
    on_flow_event: Option<Box<dyn Fn(FlowEvent) + 'static>>,
) -> impl IntoView
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    let canvas_ref = create_node_ref::<HtmlCanvasElement>();
    let viewport = create_rw_signal(ViewportState::default());
    let flow_state = create_rw_signal(FlowState::default());

    // Initialize renderer when canvas is mounted
    create_effect(move |_| {
        if let Some(canvas_el) = canvas_ref.get() {
            let renderer_result = match renderer_type {
                #[cfg(feature = "canvas2d")]
                Some(RendererType::Canvas2D) | None => {
                    Canvas2DRenderer::new(&canvas_el).map(|r| Box::new(r) as Box<dyn Renderer>)
                }
                #[cfg(not(feature = "canvas2d"))]
                _ => {
                    tracing::warn!("No renderer available for type: {:?}", renderer_type);
                    return;
                }
            };

            match renderer_result {
                Ok(mut renderer) => {
                    // Set canvas size
                    if let Err(e) = renderer.resize(width, height) {
                        tracing::error!("Failed to resize renderer: {}", e);
                        return;
                    }

                    // Store renderer in flow state
                    flow_state.update(|state| {
                        state.renderer = Some(renderer);
                    });

                    tracing::info!("Renderer initialized successfully");
                }
                Err(e) => {
                    tracing::error!("Failed to create renderer: {}", e);
                }
            }
        }
    });

    // Render loop effect
    create_effect(move |_| {
        let graph_val = graph.get();
        let viewport_val = viewport.get();

        flow_state.update(|state| {
            if let Some(renderer) = &mut state.renderer {
                // Clear canvas
                if let Err(e) = renderer.clear(Some("#f8f9fa")) {
                    tracing::error!("Failed to clear canvas: {}", e);
                    return;
                }

                // Render graph
                match renderer.render_graph(&graph_val, &viewport_val.viewport) {
                    Ok(stats) => {
                        state.render_stats = Some(stats);
                    }
                    Err(e) => {
                        tracing::error!("Failed to render graph: {}", e);
                    }
                }

                // Present frame
                if let Err(e) = renderer.present() {
                    tracing::error!("Failed to present frame: {}", e);
                }
            }
        });
    });

    view! {
        <div class="flow-editor">
            <canvas
                node_ref=canvas_ref
                width=width
                height=height
                class="flow-canvas"
                style=format!("width: {}px; height: {}px;", width, height)
            />
        </div>
    }
}

/// Simple flow canvas component for basic use cases
#[component]
pub fn FlowCanvas<N, E>(
    /// The graph to display
    graph: ReadSignal<Graph<N, E>>,
    /// Canvas width
    #[prop(default = 400)]
    width: u32,
    /// Canvas height
    #[prop(default = 300)]
    height: u32,
) -> impl IntoView
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    let graph_rw = create_rw_signal(graph.get());

    // Keep graph_rw in sync with the read signal
    create_effect(move |_| {
        graph_rw.set(graph.get());
    });

    view! {
        <FlowEditor
            graph=graph_rw
            width=width
            height=height
        />
    }
}

/// Performance statistics display component
#[component]
pub fn FlowStats(
    /// Flow state containing render stats
    flow_state: ReadSignal<FlowState>,
) -> impl IntoView {
    view! {
        <div class="flow-stats">
            {move || {
                if let Some(stats) = &flow_state.get().render_stats {
                    view! {
                        <div class="stats-grid">
                            <div class="stat-item">
                                <span class="stat-label">"Frame Time:"</span>
                                <span class="stat-value">{format!("{:.2}ms", stats.frame_time_ms)}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">"Nodes:"</span>
                                <span class="stat-value">{stats.nodes_rendered}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">"Edges:"</span>
                                <span class="stat-value">{stats.edges_rendered}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">"Draw Calls:"</span>
                                <span class="stat-value">{stats.draw_calls}</span>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="no-stats">"No render statistics available"</div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_flow_core::{NodeId, EdgeId, Size};

    #[test]
    fn test_flow_editor_props() {
        let graph = create_rw_signal(Graph::new());
        let viewport = create_rw_signal(ViewportState::default());

        let _props = FlowEditorProps {
            graph,
            viewport,
            width: 800,
            height: 600,
            renderer_type: Some(RendererType::Canvas2D),
            on_node_select: None,
            on_edge_select: None,
            on_flow_event: None,
        };
    }
}
