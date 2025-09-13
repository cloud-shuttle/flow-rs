//! WASM bindings for JavaScript interoperability

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

use flow_core::{
    Graph, Node, Edge, NodeId, EdgeId, Position, Size, Viewport
};
use flow_renderer::Renderer;
use flow_renderer::traits::RenderStats;

/// WASM-compatible position structure
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmPosition {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl WasmPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 { self.x }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 { self.y }
}

impl From<Position> for WasmPosition {
    fn from(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<WasmPosition> for Position {
    fn from(pos: WasmPosition) -> Self {
        Position::new(pos.x, pos.y)
    }
}

/// WASM-compatible size structure
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmSize {
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
impl WasmSize {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 { self.width }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 { self.height }
}

impl From<Size> for WasmSize {
    fn from(size: Size) -> Self {
        Self { width: size.width, height: size.height }
    }
}

impl From<WasmSize> for Size {
    fn from(size: WasmSize) -> Self {
        Size::new(size.width, size.height)
    }
}

/// WASM-compatible viewport structure
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmViewport {
    viewport: Viewport,
}

#[wasm_bindgen]
impl WasmViewport {
    #[wasm_bindgen(constructor)]
    pub fn new(offset_x: f64, offset_y: f64, zoom: f64) -> Self {
        Self {
            viewport: Viewport::new(offset_x, offset_y, zoom, 0.0, 0.0),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn offset(&self) -> WasmPosition {
        self.viewport.offset.into()
    }

    #[wasm_bindgen(setter)]
    pub fn set_offset(&mut self, position: WasmPosition) {
        self.viewport.offset = position.into();
    }

    #[wasm_bindgen(getter)]
    pub fn zoom(&self) -> f64 {
        self.viewport.zoom
    }

    #[wasm_bindgen(setter)]
    pub fn set_zoom(&mut self, zoom: f64) {
        self.viewport.zoom = zoom;
    }

    pub fn pan(&mut self, delta_x: f64, delta_y: f64) {
        self.viewport.offset.x += delta_x;
        self.viewport.offset.y += delta_y;
    }

    pub fn zoom_at(&mut self, factor: f64, point_x: f64, point_y: f64) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = old_zoom * factor;

        // Adjust offset to zoom at the given point
        let zoom_ratio = new_zoom / old_zoom - 1.0;
        self.viewport.offset.x -= point_x * zoom_ratio / old_zoom;
        self.viewport.offset.y -= point_y * zoom_ratio / old_zoom;
        self.viewport.zoom = new_zoom;
    }
}

impl From<Viewport> for WasmViewport {
    fn from(viewport: Viewport) -> Self {
        Self { viewport }
    }
}

impl From<WasmViewport> for Viewport {
    fn from(wasm_viewport: WasmViewport) -> Self {
        wasm_viewport.viewport
    }
}

/// WASM-compatible node structure
#[wasm_bindgen]
pub struct WasmNode {
    inner: Node<JsValue>,
}

#[wasm_bindgen]
impl WasmNode {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str, x: f64, y: f64, width: f64, height: f64, data: JsValue) -> Self {
        let mut node = Node::builder(NodeId::new(id))
            .position(x, y)
            .size(width, height)
            .build();

        node.data = data;

        Self { inner: node }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> WasmPosition {
        self.inner.position.into()
    }

    #[wasm_bindgen(setter)]
    pub fn set_position(&mut self, position: WasmPosition) {
        self.inner.set_position(position.into());
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> WasmSize {
        self.inner.size.into()
    }

    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, size: WasmSize) {
        self.inner.set_size(size.into());
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        self.inner.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, data: JsValue) {
        self.inner.data = data;
    }

    pub fn move_by(&mut self, dx: f64, dy: f64) {
        let current = self.inner.position;
        self.inner.position = Position::new(current.x + dx, current.y + dy);
    }
}

/// WASM-compatible edge structure
#[wasm_bindgen]
pub struct WasmEdge {
    inner: Edge<JsValue>,
}

#[wasm_bindgen]
impl WasmEdge {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str, source_id: &str, target_id: &str, data: JsValue) -> Self {
        let edge = Edge::builder()
            .id(EdgeId::new(id))
            .connect(NodeId::new(source_id), NodeId::new(target_id))
            .build_with_data(data)
            .expect("Failed to create edge");
        Self { inner: edge }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self.inner.source.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn target(&self) -> String {
        self.inner.target.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        self.inner.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, data: JsValue) {
        self.inner.data = data;
    }
}

/// WASM-compatible graph structure
#[wasm_bindgen]
pub struct WasmGraph {
    inner: Graph<JsValue, JsValue>,
}

#[wasm_bindgen]
impl WasmGraph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Graph::new(),
        }
    }

    pub fn add_node(&mut self, node: WasmNode) -> Result<(), JsValue> {
        self.inner
            .add_node(node.inner)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn remove_node(&mut self, node_id: &str) {
        let id = NodeId::new(node_id);
        let _ = self.inner.remove_node(&id);
    }

    pub fn add_edge(&mut self, edge: WasmEdge) -> Result<(), JsValue> {
        self.inner
            .add_edge(edge.inner)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn remove_edge(&mut self, edge_id: &str) {
        let id = EdgeId::new(edge_id);
        let _ = self.inner.remove_edge(&id);
    }

    pub fn get_node(&self, node_id: &str) -> Option<WasmNode> {
        let id = NodeId::new(node_id);
        self.inner.get_node(&id).map(|node| WasmNode {
            inner: node.clone(),
        })
    }

    pub fn get_edge(&self, edge_id: &str) -> Option<WasmEdge> {
        let id = EdgeId::new(edge_id);
        self.inner.get_edge(&id).map(|edge| WasmEdge {
            inner: edge.clone(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    #[wasm_bindgen(getter)]
    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Get all node IDs as JavaScript array
    pub fn node_ids(&self) -> js_sys::Array {
        let ids = js_sys::Array::new();
        for node in self.inner.nodes() {
            ids.push(&JsValue::from_str(&node.id.to_string()));
        }
        ids
    }

    /// Get all edge IDs as JavaScript array
    pub fn edge_ids(&self) -> js_sys::Array {
        let ids = js_sys::Array::new();
        for edge in self.inner.edges() {
            ids.push(&JsValue::from_str(&edge.id.to_string()));
        }
        ids
    }
}

impl Default for WasmGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// WASM-compatible render statistics
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmRenderStats {
    inner: RenderStats,
}

#[wasm_bindgen]
impl WasmRenderStats {
    #[wasm_bindgen(getter)]
    pub fn frame_time_ms(&self) -> f64 {
        self.inner.frame_time_ms
    }

    #[wasm_bindgen(getter)]
    pub fn nodes_rendered(&self) -> usize {
        self.inner.nodes_rendered
    }

    #[wasm_bindgen(getter)]
    pub fn edges_rendered(&self) -> usize {
        self.inner.edges_rendered
    }

    #[wasm_bindgen(getter)]
    pub fn nodes_culled(&self) -> usize {
        self.inner.nodes_culled
    }

    #[wasm_bindgen(getter)]
    pub fn edges_culled(&self) -> usize {
        self.inner.edges_culled
    }

    #[wasm_bindgen(getter)]
    pub fn draw_calls(&self) -> usize {
        self.inner.draw_calls
    }

    #[wasm_bindgen(getter)]
    pub fn triangles(&self) -> usize {
        self.inner.triangles
    }

    #[wasm_bindgen(getter)]
    pub fn memory_used(&self) -> f64 {
        self.inner.memory_used as f64
    }
}

impl From<RenderStats> for WasmRenderStats {
    fn from(stats: RenderStats) -> Self {
        Self { inner: stats }
    }
}

/// WASM-compatible flow editor
#[wasm_bindgen]
pub struct WasmFlowEditor {
    graph: WasmGraph,
    viewport: WasmViewport,
    renderer: Option<Box<dyn Renderer>>,
}

#[wasm_bindgen]
impl WasmFlowEditor {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WasmFlowEditor, JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        let _canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        #[cfg(feature = "canvas2d")]
        {
            use flow_renderer::Canvas2DRenderer;
            let renderer = Canvas2DRenderer::new(&canvas)
                .map(|r| Box::new(r) as Box<dyn Renderer>)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            return Ok(WasmFlowEditor {
                graph: WasmGraph::new(),
                viewport: WasmViewport::new(0.0, 0.0, 1.0),
                renderer: Some(renderer),
            });
        }

        #[cfg(not(feature = "canvas2d"))]
        Err(JsValue::from_str("Canvas2D renderer not available"))
    }

    #[wasm_bindgen(getter)]
    pub fn graph(&self) -> WasmGraph {
        // Note: This creates a copy. In a real implementation,
        // we might want to use Rc<RefCell<>> for shared ownership
        WasmGraph {
            inner: self.graph.inner.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn viewport(&self) -> WasmViewport {
        self.viewport.clone()
    }

    pub fn render(&mut self) -> Result<WasmRenderStats, JsValue> {
        if let Some(renderer) = &mut self.renderer {
            renderer.clear(Some("#f8f9fa"))
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            // For now, we'll use a simple approach without the dyn renderer
            // TODO: Implement proper WASM-compatible rendering
            let stats = RenderStats {
                frame_time_ms: 0.0,
                nodes_rendered: self.graph.inner.nodes().count(),
                edges_rendered: self.graph.inner.edges().count(),
                nodes_culled: 0,
                edges_culled: 0,
                draw_calls: 1,
                triangles: 0,
                memory_used: 0,
            };

            renderer.present()
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            Ok(stats.into())
        } else {
            Err(JsValue::from_str("No renderer available"))
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(width, height)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            Err(JsValue::from_str("No renderer available"))
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.viewport.pan(dx, dy);
    }

    pub fn zoom_at(&mut self, factor: f64, x: f64, y: f64) {
        self.viewport.zoom_at(factor, x, y);
    }

    pub fn add_node(&mut self, node: WasmNode) -> Result<(), JsValue> {
        self.graph.add_node(node)
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.graph.remove_node(node_id);
    }

    pub fn add_edge(&mut self, edge: WasmEdge) -> Result<(), JsValue> {
        self.graph.add_edge(edge)
    }

    pub fn remove_edge(&mut self, edge_id: &str) {
        self.graph.remove_edge(edge_id);
    }
}

/// Utility functions for JavaScript interop
#[wasm_bindgen]
pub fn create_node(id: &str, x: f64, y: f64, width: f64, height: f64, data: JsValue) -> WasmNode {
    WasmNode::new(id, x, y, width, height, data)
}

#[wasm_bindgen]
pub fn create_edge(id: &str, source_id: &str, target_id: &str, data: JsValue) -> WasmEdge {
    WasmEdge::new(id, source_id, target_id, data)
}

#[wasm_bindgen]
pub fn create_position(x: f64, y: f64) -> WasmPosition {
    WasmPosition::new(x, y)
}

#[wasm_bindgen]
pub fn create_size(width: f64, height: f64) -> WasmSize {
    WasmSize::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_position() {
        let pos = WasmPosition::new(10.0, 20.0);
        assert_eq!(pos.x(), 10.0);
        assert_eq!(pos.y(), 20.0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_size() {
        let size = WasmSize::new(100.0, 50.0);
        assert_eq!(size.width(), 100.0);
        assert_eq!(size.height(), 50.0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_graph() {
        let mut graph = WasmGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);

        let node = WasmNode::new("test", 0.0, 0.0, 100.0, 50.0, JsValue::NULL);
        assert!(graph.add_node(node).is_ok());
        assert_eq!(graph.node_count(), 1);
    }
}
