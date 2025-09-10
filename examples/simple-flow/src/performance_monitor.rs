//! Performance monitoring utilities for the simple flow example

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::collections::VecDeque;

/// Performance monitor for tracking rendering performance
#[wasm_bindgen]
pub struct PerformanceMonitor {
    frame_times: VecDeque<f64>,
    max_samples: usize,
    last_frame_time: f64,
    is_monitoring: bool,
}

#[wasm_bindgen]
impl PerformanceMonitor {
    #[wasm_bindgen(constructor)]
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            max_samples,
            last_frame_time: 0.0,
            is_monitoring: false,
        }
    }

    /// Start monitoring performance
    #[wasm_bindgen]
    pub fn start_monitoring(&mut self) {
        self.is_monitoring = true;
        self.last_frame_time = web_sys::js_sys::Date::now();
    }

    /// Stop monitoring performance
    #[wasm_bindgen]
    pub fn stop_monitoring(&mut self) {
        self.is_monitoring = false;
    }

    /// Record a frame time
    #[wasm_bindgen]
    pub fn record_frame(&mut self) {
        if !self.is_monitoring {
            return;
        }

        let current_time = web_sys::js_sys::Date::now();
        let frame_time = current_time - self.last_frame_time;

        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }

        self.last_frame_time = current_time;
    }

    /// Get average frame time in milliseconds
    #[wasm_bindgen]
    pub fn get_average_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.frame_times.iter().sum();
        sum / self.frame_times.len() as f64
    }

    /// Get current FPS
    #[wasm_bindgen]
    pub fn get_fps(&self) -> f64 {
        let avg_frame_time = self.get_average_frame_time();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get performance statistics
    #[wasm_bindgen]
    pub fn get_stats(&self) -> JsValue {
        let stats = PerformanceStats {
            average_frame_time: self.get_average_frame_time(),
            fps: self.get_fps(),
            frame_count: self.frame_times.len(),
            min_frame_time: self.frame_times.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            max_frame_time: self.frame_times.iter().fold(0.0, |a, &b| a.max(b)),
        };

        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }

    /// Log performance statistics to console
    #[wasm_bindgen]
    pub fn log_stats(&self) {
        let stats = self.get_stats();
        console::log_1(&format!("Performance Stats: {:?}", stats).into());
    }

    /// Check if performance is good (above 60 FPS)
    #[wasm_bindgen]
    pub fn is_performance_good(&self) -> bool {
        self.get_fps() >= 60.0
    }

    /// Get performance grade
    #[wasm_bindgen]
    pub fn get_performance_grade(&self) -> String {
        let fps = self.get_fps();
        if fps >= 60.0 {
            "A".to_string()
        } else if fps >= 30.0 {
            "B".to_string()
        } else if fps >= 15.0 {
            "C".to_string()
        } else {
            "D".to_string()
        }
    }

    /// Clear all recorded data
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.frame_times.clear();
    }
}

#[derive(serde::Serialize)]
struct PerformanceStats {
    average_frame_time: f64,
    fps: f64,
    frame_count: usize,
    min_frame_time: f64,
    max_frame_time: f64,
}

/// Memory usage monitor
#[wasm_bindgen]
pub struct MemoryMonitor {
    initial_memory: Option<f64>,
    peak_memory: f64,
}

#[wasm_bindgen]
impl MemoryMonitor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            initial_memory: None,
            peak_memory: 0.0,
        }
    }

    /// Initialize memory monitoring
    #[wasm_bindgen]
    pub fn init(&mut self) {
        if let Some(memory) = self.get_memory_usage() {
            self.initial_memory = Some(memory);
            self.peak_memory = memory;
        }
    }

    /// Get current memory usage in MB
    #[wasm_bindgen]
    pub fn get_memory_usage(&self) -> Option<f64> {
        let window = web_sys::window()?;
        let performance = window.performance()?;

        // Note: performance.memory() is not available in all browsers
        // This is a placeholder for when it becomes available
        None
    }

    /// Update peak memory usage
    #[wasm_bindgen]
    pub fn update_peak_memory(&mut self) {
        if let Some(current_memory) = self.get_memory_usage() {
            self.peak_memory = self.peak_memory.max(current_memory);
        }
    }

    /// Get memory statistics
    #[wasm_bindgen]
    pub fn get_memory_stats(&self) -> JsValue {
        let stats = MemoryStats {
            current_memory: self.get_memory_usage(),
            initial_memory: self.initial_memory,
            peak_memory: self.peak_memory,
            memory_growth: self.initial_memory.map(|init| {
                self.get_memory_usage().map(|current| current - init)
            }).flatten(),
        };

        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }
}

#[derive(serde::Serialize)]
struct MemoryStats {
    current_memory: Option<f64>,
    initial_memory: Option<f64>,
    peak_memory: f64,
    memory_growth: Option<f64>,
}

/// Rendering performance analyzer
#[wasm_bindgen]
pub struct RenderingAnalyzer {
    render_times: VecDeque<f64>,
    node_counts: VecDeque<usize>,
    edge_counts: VecDeque<usize>,
    max_samples: usize,
}

#[wasm_bindgen]
impl RenderingAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn new(max_samples: usize) -> Self {
        Self {
            render_times: VecDeque::with_capacity(max_samples),
            node_counts: VecDeque::with_capacity(max_samples),
            edge_counts: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Record a rendering operation
    #[wasm_bindgen]
    pub fn record_render(&mut self, render_time: f64, node_count: usize, edge_count: usize) {
        self.render_times.push_back(render_time);
        self.node_counts.push_back(node_count);
        self.edge_counts.push_back(edge_count);

        if self.render_times.len() > self.max_samples {
            self.render_times.pop_front();
            self.node_counts.pop_front();
            self.edge_counts.pop_front();
        }
    }

    /// Get rendering performance metrics
    #[wasm_bindgen]
    pub fn get_metrics(&self) -> JsValue {
        let metrics = RenderingMetrics {
            average_render_time: self.get_average_render_time(),
            average_nodes_per_frame: self.get_average_nodes(),
            average_edges_per_frame: self.get_average_edges(),
            nodes_per_ms: self.get_nodes_per_ms(),
            edges_per_ms: self.get_edges_per_ms(),
        };

        serde_wasm_bindgen::to_value(&metrics).unwrap_or(JsValue::NULL)
    }

    fn get_average_render_time(&self) -> f64 {
        if self.render_times.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.render_times.iter().sum();
        sum / self.render_times.len() as f64
    }

    fn get_average_nodes(&self) -> f64 {
        if self.node_counts.is_empty() {
            return 0.0;
        }
        let sum: usize = self.node_counts.iter().sum();
        sum as f64 / self.node_counts.len() as f64
    }

    fn get_average_edges(&self) -> f64 {
        if self.edge_counts.is_empty() {
            return 0.0;
        }
        let sum: usize = self.edge_counts.iter().sum();
        sum as f64 / self.edge_counts.len() as f64
    }

    fn get_nodes_per_ms(&self) -> f64 {
        let avg_render_time = self.get_average_render_time();
        let avg_nodes = self.get_average_nodes();
        if avg_render_time > 0.0 {
            avg_nodes / avg_render_time
        } else {
            0.0
        }
    }

    fn get_edges_per_ms(&self) -> f64 {
        let avg_render_time = self.get_average_render_time();
        let avg_edges = self.get_average_edges();
        if avg_render_time > 0.0 {
            avg_edges / avg_render_time
        } else {
            0.0
        }
    }
}

#[derive(serde::Serialize)]
struct RenderingMetrics {
    average_render_time: f64,
    average_nodes_per_frame: f64,
    average_edges_per_frame: f64,
    nodes_per_ms: f64,
    edges_per_ms: f64,
}

/// Performance benchmark runner
#[wasm_bindgen]
pub struct PerformanceBenchmark {
    monitor: PerformanceMonitor,
    memory_monitor: MemoryMonitor,
    render_analyzer: RenderingAnalyzer,
    benchmark_results: Vec<BenchmarkResult>,
}

#[wasm_bindgen]
impl PerformanceBenchmark {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            monitor: PerformanceMonitor::new(100),
            memory_monitor: MemoryMonitor::new(),
            render_analyzer: RenderingAnalyzer::new(100),
            benchmark_results: Vec::new(),
        }
    }

    /// Run a complete performance benchmark
    #[wasm_bindgen]
    pub fn run_benchmark(&mut self, duration_ms: f64) -> JsValue {
        self.monitor.start_monitoring();
        self.memory_monitor.init();

        // Simulate benchmark duration
        let start_time = web_sys::js_sys::Date::now();
        let end_time = start_time + duration_ms;

        while web_sys::js_sys::Date::now() < end_time {
            self.monitor.record_frame();
            self.memory_monitor.update_peak_memory();

            // Simulate rendering work
            let render_start = web_sys::js_sys::Date::now();
            // ... rendering work would happen here ...
            let render_end = web_sys::js_sys::Date::now();

            self.render_analyzer.record_render(
                render_end - render_start,
                100, // simulated node count
                50,  // simulated edge count
            );
        }

        self.monitor.stop_monitoring();

        let result = BenchmarkResult {
            duration_ms,
            average_fps: self.monitor.get_fps(),
            average_frame_time: self.monitor.get_average_frame_time(),
            performance_grade: self.monitor.get_performance_grade(),
            memory_stats: serde_json::from_str(&format!("{:?}", self.memory_monitor.get_memory_stats())).unwrap_or(serde_json::Value::Null),
            rendering_metrics: serde_json::from_str(&format!("{:?}", self.render_analyzer.get_metrics())).unwrap_or(serde_json::Value::Null),
        };

        self.benchmark_results.push(result.clone());

        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    }

    /// Get all benchmark results
    #[wasm_bindgen]
    pub fn get_results(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.benchmark_results).unwrap_or(JsValue::NULL)
    }

    /// Clear all benchmark results
    #[wasm_bindgen]
    pub fn clear_results(&mut self) {
        self.benchmark_results.clear();
    }
}

#[derive(serde::Serialize, Clone)]
struct BenchmarkResult {
    duration_ms: f64,
    average_fps: f64,
    average_frame_time: f64,
    performance_grade: String,
    memory_stats: serde_json::Value,
    rendering_metrics: serde_json::Value,
}

/// Utility function to create a performance monitor
#[wasm_bindgen]
pub fn create_performance_monitor(max_samples: usize) -> PerformanceMonitor {
    PerformanceMonitor::new(max_samples)
}

/// Utility function to create a memory monitor
#[wasm_bindgen]
pub fn create_memory_monitor() -> MemoryMonitor {
    MemoryMonitor::new()
}

/// Utility function to create a rendering analyzer
#[wasm_bindgen]
pub fn create_rendering_analyzer(max_samples: usize) -> RenderingAnalyzer {
    RenderingAnalyzer::new(max_samples)
}

/// Utility function to create a performance benchmark
#[wasm_bindgen]
pub fn create_performance_benchmark() -> PerformanceBenchmark {
    PerformanceBenchmark::new()
}
