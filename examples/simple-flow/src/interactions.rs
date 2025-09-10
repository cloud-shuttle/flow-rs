//! Interaction handlers for the flow editor
//! 
//! This module implements the interaction logic defined in the tests:
//! - Node selection on click
//! - Node dragging
//! - Canvas panning
//! - Visual feedback for selected nodes

use leptos_flow_core::{Graph, Position, Viewport};
use leptos_flow_core::types::NodeId;
use leptos_flow_renderer::{Canvas2DRenderer, Renderer};
use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use std::collections::HashSet;

/// Interaction state for the flow editor
#[derive(Debug, Clone)]
pub struct InteractionState {
    /// Currently selected node IDs
    pub selected_nodes: HashSet<NodeId>,
    /// Node being dragged (if any)
    pub dragging_node: Option<NodeId>,
    /// Last mouse position for drag calculations
    pub last_mouse_pos: Option<Position>,
    /// Whether the canvas is being panned
    pub is_panning: bool,
    /// Whether Ctrl/Cmd is held for multi-selection
    pub ctrl_held: bool,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self {
            selected_nodes: HashSet::new(),
            dragging_node: None,
            last_mouse_pos: None,
            is_panning: false,
            ctrl_held: false,
        }
    }
}

/// Interaction handler for the flow editor
pub struct InteractionHandler {
    state: InteractionState,
    canvas: HtmlCanvasElement,
    renderer: Canvas2DRenderer,
    graph: Graph<(), ()>,
    viewport: Viewport,
}

impl InteractionHandler {
    /// Create a new interaction handler
    pub fn new(canvas: HtmlCanvasElement, renderer: Canvas2DRenderer, graph: Graph<(), ()>, viewport: Viewport) -> Self {
        Self {
            state: InteractionState::default(),
            canvas,
            renderer,
            graph,
            viewport,
        }
    }

    /// Handle mouse down events
    pub fn handle_mouse_down(&mut self, event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        let pos = self.get_mouse_position(event)?;
        self.state.last_mouse_pos = Some(pos);
        self.state.ctrl_held = event.ctrl_key() || event.meta_key();

        // Check if clicking on a node
        if let Some(node_id) = self.get_node_at_position(pos) {
            if !self.state.ctrl_held {
                // Single selection - clear others
                self.state.selected_nodes.clear();
            }
            self.state.selected_nodes.insert(node_id.clone());
            self.state.dragging_node = Some(node_id.clone());
            
            web_sys::console::log_1(&format!("Selected node: {:?}", node_id).into());
        } else {
            // Clicked on empty space
            if !self.state.ctrl_held {
                self.state.selected_nodes.clear();
            }
            self.state.is_panning = true;
        }

        self.render()?;
        Ok(())
    }

    /// Handle mouse move events
    pub fn handle_mouse_move(&mut self, event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        let pos = self.get_mouse_position(event)?;
        
        if let Some(dragging_node) = self.state.dragging_node.clone() {
            // Drag the selected node
            if let Some(last_pos) = self.state.last_mouse_pos {
                let delta = pos.sub(last_pos);
                self.drag_node(&dragging_node, delta)?;
            }
        } else if self.state.is_panning {
            // Pan the canvas
            if let Some(last_pos) = self.state.last_mouse_pos {
                let delta = pos.sub(last_pos);
                self.pan_canvas(delta)?;
            }
        }

        self.state.last_mouse_pos = Some(pos);
        self.render()?;
        Ok(())
    }

    /// Handle mouse up events
    pub fn handle_mouse_up(&mut self, _event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        self.state.dragging_node = None;
        self.state.is_panning = false;
        self.state.last_mouse_pos = None;
        Ok(())
    }

    /// Get the node at a specific position
    fn get_node_at_position(&self, pos: Position) -> Option<NodeId> {
        // Simple hit testing - check if position is within any node's bounds
        for node in self.graph.nodes() {
            let node_pos = node.position;
            let node_size = node.size;
            
            if pos.x >= node_pos.x && pos.x <= node_pos.x + node_size.width &&
               pos.y >= node_pos.y && pos.y <= node_pos.y + node_size.height {
                return Some(node.id.clone());
            }
        }
        None
    }

    /// Drag a node by the given delta
    fn drag_node(&mut self, node_id: &NodeId, delta: Position) -> Result<(), JsValue> {
        // Find and update the node position
        for node in self.graph.nodes_mut() {
            if node.id == *node_id {
                node.position = node.position.add(delta);
                break;
            }
        }
        Ok(())
    }

    /// Pan the canvas by the given delta
    fn pan_canvas(&mut self, delta: Position) -> Result<(), JsValue> {
        self.viewport = self.viewport.pan(delta);
        Ok(())
    }

    /// Get mouse position relative to canvas
    fn get_mouse_position(&self, event: &web_sys::MouseEvent) -> Result<Position, JsValue> {
        let rect = self.canvas.get_bounding_client_rect();
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();
        Ok(Position::new(x, y))
    }

    /// Render the current state
    pub fn render(&mut self) -> Result<(), JsValue> {
        // Clear canvas
        self.renderer.clear(Some("#ffffff"))
            .map_err(|e| JsValue::from_str(&format!("Clear error: {:?}", e)))?;
        
        // Render background
        let bg_config = leptos_flow_renderer::traits::BackgroundConfig {
            color: "#ffffff".to_string(),
            pattern_color: "#e2e8f0".to_string(),
            variant: leptos_flow_renderer::traits::BackgroundVariant::Dots,
            size: 20.0,
            opacity: 0.5,
        };
        self.renderer.render_background(&bg_config, &self.viewport)
            .map_err(|e| JsValue::from_str(&format!("Background render error: {:?}", e)))?;
        
        // Render graph with selection highlighting
        self.render_graph_with_selection()?;
        
        // Present frame
        self.renderer.present()
            .map_err(|e| JsValue::from_str(&format!("Present error: {:?}", e)))?;
        Ok(())
    }

    /// Render the graph with selection highlighting
    fn render_graph_with_selection(&mut self) -> Result<(), JsValue> {
        // For now, just render the normal graph
        // TODO: Add selection highlighting
        self.renderer.render_graph(&self.graph, &self.viewport)
            .map_err(|e| JsValue::from_str(&format!("Graph render error: {:?}", e)))?;
        Ok(())
    }

    /// Get currently selected nodes
    pub fn get_selected_nodes(&self) -> Vec<NodeId> {
        self.state.selected_nodes.iter().cloned().collect()
    }

    /// Check if a node is selected
    pub fn is_node_selected(&self, node_id: &NodeId) -> bool {
        self.state.selected_nodes.contains(node_id)
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.state.selected_nodes.clear();
    }
}
