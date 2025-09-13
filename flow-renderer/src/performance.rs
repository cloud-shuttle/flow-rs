//! Performance optimization utilities for the Canvas2D renderer

use std::collections::HashMap;
use flow_core::{Position, Rect, Viewport};
use crate::traits::{NodeStyle, EdgeStyle, BackgroundConfig};

/// Performance monitoring and optimization utilities
pub struct PerformanceMonitor {
    frame_times: Vec<f64>,
    max_frame_samples: usize,
    last_frame_time: f64,
    render_stats: RenderStats,
}

/// Detailed rendering statistics
#[derive(Debug, Clone)]
pub struct RenderStats {
    pub nodes_rendered: usize,
    pub edges_rendered: usize,
    pub nodes_culled: usize,
    pub edges_culled: usize,
    pub batch_count: usize,
    pub draw_calls: usize,
    pub frame_time_ms: f64,
    pub memory_usage_bytes: usize,
}

impl Default for RenderStats {
    fn default() -> Self {
        Self {
            nodes_rendered: 0,
            edges_rendered: 0,
            nodes_culled: 0,
            edges_culled: 0,
            batch_count: 0,
            draw_calls: 0,
            frame_time_ms: 0.0,
            memory_usage_bytes: 0,
        }
    }
}

impl PerformanceMonitor {
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_frame_samples: max_samples,
            last_frame_time: 0.0,
            render_stats: RenderStats::default(),
        }
    }

    pub fn start_frame(&mut self) {
        self.last_frame_time = web_sys::js_sys::Date::now();
    }

    pub fn end_frame(&mut self) {
        let current_time = web_sys::js_sys::Date::now();
        let frame_time = current_time - self.last_frame_time;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > self.max_frame_samples {
            self.frame_times.remove(0);
        }

        self.render_stats.frame_time_ms = frame_time;
    }

    pub fn get_average_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.frame_times.iter().sum();
        sum / self.frame_times.len() as f64
    }

    pub fn get_fps(&self) -> f64 {
        let avg_frame_time = self.get_average_frame_time();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    pub fn is_performance_good(&self) -> bool {
        self.get_average_frame_time() < 16.67 // 60 FPS threshold
    }

    pub fn get_stats(&self) -> &RenderStats {
        &self.render_stats
    }

    pub fn update_stats(&mut self, stats: RenderStats) {
        self.render_stats = stats;
    }
}

/// Spatial indexing for efficient culling
pub struct SpatialIndex {
    cell_size: f64,
    cells: HashMap<(i32, i32), Vec<usize>>,
    node_positions: Vec<Position>,
    node_sizes: Vec<(f64, f64)>,
}

impl SpatialIndex {
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            node_positions: Vec::new(),
            node_sizes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, position: Position, size: (f64, f64)) -> usize {
        let index = self.node_positions.len();
        self.node_positions.push(position);
        self.node_sizes.push(size);

        let cell_x = (position.x / self.cell_size).floor() as i32;
        let cell_y = (position.y / self.cell_size).floor() as i32;

        self.cells.entry((cell_x, cell_y)).or_default().push(index);
        index
    }

    pub fn update_node(&mut self, index: usize, position: Position, size: (f64, f64)) {
        if index >= self.node_positions.len() {
            return;
        }

        let old_position = self.node_positions[index];
        let old_cell_x = (old_position.x / self.cell_size).floor() as i32;
        let old_cell_y = (old_position.y / self.cell_size).floor() as i32;

        let new_cell_x = (position.x / self.cell_size).floor() as i32;
        let new_cell_y = (position.y / self.cell_size).floor() as i32;

        // Remove from old cell
        if let Some(cell) = self.cells.get_mut(&(old_cell_x, old_cell_y)) {
            cell.retain(|&i| i != index);
        }

        // Add to new cell
        self.cells.entry((new_cell_x, new_cell_y)).or_default().push(index);

        // Update position and size
        self.node_positions[index] = position;
        self.node_sizes[index] = size;
    }

    pub fn remove_node(&mut self, index: usize) {
        if index >= self.node_positions.len() {
            return;
        }

        let position = self.node_positions[index];
        let cell_x = (position.x / self.cell_size).floor() as i32;
        let cell_y = (position.y / self.cell_size).floor() as i32;

        // Remove from cell
        if let Some(cell) = self.cells.get_mut(&(cell_x, cell_y)) {
            cell.retain(|&i| i != index);
        }

        // Remove from arrays
        self.node_positions.remove(index);
        self.node_sizes.remove(index);

        // Update indices in all cells
        for cell in self.cells.values_mut() {
            for i in cell.iter_mut() {
                if *i > index {
                    *i -= 1;
                }
            }
        }
    }

    pub fn query_rect(&self, rect: Rect) -> Vec<usize> {
        let mut result = Vec::new();

        let min_cell_x = (rect.x / self.cell_size).floor() as i32;
        let min_cell_y = (rect.y / self.cell_size).floor() as i32;
        let max_cell_x = ((rect.x + rect.width) / self.cell_size).ceil() as i32;
        let max_cell_y = ((rect.y + rect.height) / self.cell_size).ceil() as i32;

        for cell_x in min_cell_x..=max_cell_x {
            for cell_y in min_cell_y..=max_cell_y {
                if let Some(cell) = self.cells.get(&(cell_x, cell_y)) {
                    for &index in cell {
                        if index < self.node_positions.len() {
                            let pos = self.node_positions[index];
                            let size = self.node_sizes[index];

                            // Check if node actually intersects with query rect
                            if rect.intersects(&Rect::new(pos.x, pos.y, size.0, size.1)) {
                                result.push(index);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.node_positions.clear();
        self.node_sizes.clear();
    }
}

/// Batching system for efficient rendering
pub struct RenderBatch {
    pub nodes: Vec<BatchedNode>,
    pub edges: Vec<BatchedEdge>,
    pub background_config: Option<BackgroundConfig>,
}

#[derive(Debug, Clone, Default)]
pub struct BatchedNode {
    pub position: Position,
    pub size: (f64, f64),
    pub style: NodeStyle,
    pub z_index: i32,
}

#[derive(Debug, Clone, Default)]
pub struct BatchedEdge {
    pub source_pos: Position,
    pub target_pos: Position,
    pub style: EdgeStyle,
    pub z_index: i32,
}

impl Default for RenderBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderBatch {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            background_config: None,
        }
    }

    pub fn add_node(&mut self, node: BatchedNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: BatchedEdge) {
        self.edges.push(edge);
    }

    pub fn sort_by_z_index(&mut self) {
        self.nodes.sort_by_key(|node| node.z_index);
        self.edges.sort_by_key(|edge| edge.z_index);
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.background_config = None;
    }
}

/// Level of detail system for performance optimization
pub struct LODSystem {
    zoom_levels: Vec<f64>,
    detail_thresholds: Vec<usize>,
}

impl Default for LODSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl LODSystem {
    pub fn new() -> Self {
        Self {
            zoom_levels: vec![0.5, 1.0, 2.0, 4.0],
            detail_thresholds: vec![100, 50, 25, 10],
        }
    }

    pub fn get_detail_level(&self, zoom: f64) -> usize {
        for (i, &level) in self.zoom_levels.iter().enumerate() {
            if zoom <= level {
                return self.detail_thresholds[i];
            }
        }
        self.detail_thresholds.last().copied().unwrap_or(10)
    }

    pub fn should_render_node(&self, zoom: f64, node_size: f64) -> bool {
        let detail_level = self.get_detail_level(zoom);
        node_size >= detail_level as f64
    }

    pub fn should_render_edge(&self, zoom: f64, edge_length: f64) -> bool {
        let detail_level = self.get_detail_level(zoom);
        edge_length >= detail_level as f64
    }
}

/// Memory pool for efficient allocation
pub struct MemoryPool<T> {
    pool: Vec<T>,
    available: Vec<usize>,
    next_id: usize,
}

impl<T: Default> MemoryPool<T> {
    pub fn new(initial_capacity: usize) -> Self {
        let mut pool = Vec::with_capacity(initial_capacity);
        let mut available = Vec::with_capacity(initial_capacity);

        for i in 0..initial_capacity {
            pool.push(T::default());
            available.push(i);
        }

        Self {
            pool,
            available,
            next_id: initial_capacity,
        }
    }

    pub fn allocate(&mut self) -> usize {
        if let Some(id) = self.available.pop() {
            id
        } else {
            let id = self.next_id;
            self.pool.push(T::default());
            self.next_id += 1;
            id
        }
    }

    pub fn deallocate(&mut self, id: usize) {
        if id < self.pool.len() {
            self.available.push(id);
        }
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.pool.get(id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.pool.get_mut(id)
    }

    pub fn clear(&mut self) {
        self.available.clear();
        for i in 0..self.pool.len() {
            self.available.push(i);
        }
    }
}

/// Performance optimization settings
#[derive(Debug, Clone)]
pub struct PerformanceSettings {
    pub enable_culling: bool,
    pub enable_batching: bool,
    pub enable_lod: bool,
    pub max_nodes_per_frame: usize,
    pub max_edges_per_frame: usize,
    pub target_fps: f64,
    pub cull_margin: f64,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            enable_culling: true,
            enable_batching: true,
            enable_lod: true,
            max_nodes_per_frame: 1000,
            max_edges_per_frame: 2000,
            target_fps: 60.0,
            cull_margin: 100.0,
        }
    }
}

impl PerformanceSettings {
    pub fn high_performance() -> Self {
        Self {
            enable_culling: true,
            enable_batching: true,
            enable_lod: true,
            max_nodes_per_frame: 500,
            max_edges_per_frame: 1000,
            target_fps: 60.0,
            cull_margin: 50.0,
        }
    }

    pub fn balanced() -> Self {
        Self::default()
    }

    pub fn high_quality() -> Self {
        Self {
            enable_culling: false,
            enable_batching: true,
            enable_lod: false,
            max_nodes_per_frame: 2000,
            max_edges_per_frame: 4000,
            target_fps: 30.0,
            cull_margin: 0.0,
        }
    }
}

/// Performance optimization manager
pub struct PerformanceManager {
    monitor: PerformanceMonitor,
    #[allow(dead_code)]
    spatial_index: SpatialIndex,
    render_batch: RenderBatch,
    lod_system: LODSystem,
    settings: PerformanceSettings,
    #[allow(dead_code)]
    node_pool: MemoryPool<BatchedNode>,
    #[allow(dead_code)]
    edge_pool: MemoryPool<BatchedEdge>,
}

impl PerformanceManager {
    pub fn new(settings: PerformanceSettings) -> Self {
        Self {
            monitor: PerformanceMonitor::new(100),
            spatial_index: SpatialIndex::new(50.0),
            render_batch: RenderBatch::new(),
            lod_system: LODSystem::new(),
            settings,
            node_pool: MemoryPool::new(1000),
            edge_pool: MemoryPool::new(2000),
        }
    }

    pub fn start_frame(&mut self) {
        self.monitor.start_frame();
        self.render_batch.clear();
    }

    pub fn end_frame(&mut self) {
        self.monitor.end_frame();
    }

    pub fn should_render_node(&self, position: Position, size: (f64, f64), viewport: &Viewport) -> bool {
        if !self.settings.enable_culling {
            return true;
        }

        // Viewport culling
        let node_rect = Rect::new(position.x, position.y, size.0, size.1);
        let viewport_rect = viewport.bounds().expand(self.settings.cull_margin);

        if !viewport_rect.intersects(&node_rect) {
            return false;
        }

        // LOD culling
        if self.settings.enable_lod {
            let zoom = viewport.zoom;
            let node_size = size.0.max(size.1);
            return self.lod_system.should_render_node(zoom, node_size);
        }

        true
    }

    pub fn should_render_edge(&self, source_pos: Position, target_pos: Position, viewport: &Viewport) -> bool {
        if !self.settings.enable_culling {
            return true;
        }

        // Viewport culling
        let edge_rect = Rect::from_points(source_pos, target_pos);
        let viewport_rect = viewport.bounds().expand(self.settings.cull_margin);

        if !viewport_rect.intersects(&edge_rect) {
            return false;
        }

        // LOD culling
        if self.settings.enable_lod {
            let zoom = viewport.zoom;
            let edge_length = source_pos.distance_to(target_pos);
            return self.lod_system.should_render_edge(zoom, edge_length);
        }

        true
    }

    pub fn add_node_to_batch(&mut self, node: BatchedNode) {
        if self.render_batch.nodes.len() < self.settings.max_nodes_per_frame {
            self.render_batch.add_node(node);
        }
    }

    pub fn add_edge_to_batch(&mut self, edge: BatchedEdge) {
        if self.render_batch.edges.len() < self.settings.max_edges_per_frame {
            self.render_batch.add_edge(edge);
        }
    }

    pub fn get_render_batch(&mut self) -> &mut RenderBatch {
        self.render_batch.sort_by_z_index();
        &mut self.render_batch
    }

    pub fn get_performance_stats(&self) -> &RenderStats {
        self.monitor.get_stats()
    }

    pub fn update_settings(&mut self, settings: PerformanceSettings) {
        self.settings = settings;
    }

    pub fn is_performance_good(&self) -> bool {
        self.monitor.is_performance_good()
    }

    pub fn get_fps(&self) -> f64 {
        self.monitor.get_fps()
    }
}

// Helper trait for Rect operations
#[allow(dead_code)]
trait RectExt {
    fn expand(&self, margin: f64) -> Self;
    fn intersects(&self, other: &Self) -> bool;
}

impl RectExt for Rect {
    fn expand(&self, margin: f64) -> Self {
        Rect::new(
            self.x - margin,
            self.y - margin,
            self.width + 2.0 * margin,
            self.height + 2.0 * margin,
        )
    }

    fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

// Helper trait for Position operations
#[allow(dead_code)]
trait PositionExt {
    fn distance_to(&self, other: Self) -> f64;
}

impl PositionExt for Position {
    fn distance_to(&self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
