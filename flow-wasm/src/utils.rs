//! Utility functions for WASM integration

use wasm_bindgen::prelude::*;
use web_sys::{console};

/// Log levels for WASM console output
#[wasm_bindgen]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

/// Logging utility for WASM
#[wasm_bindgen]
pub struct Logger;

#[wasm_bindgen]
impl Logger {
    pub fn log(level: LogLevel, message: &str) {
        match level {
            LogLevel::Error => console::error_1(&JsValue::from_str(message)),
            LogLevel::Warn => console::warn_1(&JsValue::from_str(message)),
            LogLevel::Info => console::log_1(&JsValue::from_str(message)),
            LogLevel::Debug => console::log_1(&JsValue::from_str(&format!("[DEBUG] {}", message))),
        }
    }

    pub fn error(message: &str) {
        Self::log(LogLevel::Error, message);
    }

    pub fn warn(message: &str) {
        Self::log(LogLevel::Warn, message);
    }

    pub fn info(message: &str) {
        Self::log(LogLevel::Info, message);
    }

    pub fn debug(message: &str) {
        Self::log(LogLevel::Debug, message);
    }
}

/// Performance timing utilities
#[wasm_bindgen]
pub struct PerformanceTimer {
    start_time: f64,
    name: String,
}

#[wasm_bindgen]
impl PerformanceTimer {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        let start_time = get_performance_now().unwrap_or(0.0);
        Self {
            start_time,
            name: name.to_string(),
        }
    }

    pub fn elapsed(&self) -> f64 {
        let current_time = get_performance_now().unwrap_or(0.0);
        current_time - self.start_time
    }

    pub fn log_elapsed(&self) {
        let elapsed = self.elapsed();
        Logger::info(&format!("{}: {:.2}ms", self.name, elapsed));
    }

    pub fn reset(&mut self) {
        self.start_time = get_performance_now().unwrap_or(0.0);
    }
}

/// Get current performance timestamp
#[wasm_bindgen]
pub fn get_performance_now() -> Result<f64, JsValue> {
    let window = web_sys::window().ok_or("No window object")?;
    let performance = window.performance().ok_or("No performance object")?;
    Ok(performance.now())
}

/// Frame rate calculator
#[wasm_bindgen]
pub struct FrameRateCalculator {
    frame_times: Vec<f64>,
    last_frame_time: f64,
    max_samples: usize,
}

#[wasm_bindgen]
impl FrameRateCalculator {
    #[wasm_bindgen(constructor)]
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            last_frame_time: get_performance_now().unwrap_or(0.0),
            max_samples,
        }
    }

    pub fn update(&mut self) -> f64 {
        let current_time = get_performance_now().unwrap_or(0.0);
        let frame_time = current_time - self.last_frame_time;
        self.last_frame_time = current_time;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        self.average_fps()
    }

    pub fn average_fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let average_frame_time: f64 = self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64;
        if average_frame_time > 0.0 {
            1000.0 / average_frame_time
        } else {
            0.0
        }
    }

    pub fn current_fps(&self) -> f64 {
        if let Some(&last_frame_time) = self.frame_times.last() {
            if last_frame_time > 0.0 {
                1000.0 / last_frame_time
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.last_frame_time = get_performance_now().unwrap_or(0.0);
    }
}

/// Memory usage monitoring
#[wasm_bindgen]
pub struct MemoryMonitor;

#[wasm_bindgen]
impl MemoryMonitor {
    /// Get memory usage information (if available)
    pub fn get_memory_info() -> JsValue {
        if let Some(window) = web_sys::window() {
            if let Ok(performance) = js_sys::Reflect::get(&window, &"performance".into()) {
                if let Ok(memory) = js_sys::Reflect::get(&performance, &"memory".into()) {
                    return memory;
                }
            }
        }
        JsValue::NULL
    }

    /// Log current memory usage
    pub fn log_memory_usage() {
        let memory = Self::get_memory_info();
        if !memory.is_null() {
            Logger::info(&format!("Memory info: {:?}", memory));
        } else {
            Logger::warn("Memory API not available");
        }
    }
}

/// Canvas utilities
#[wasm_bindgen]
pub struct CanvasUtils;

#[wasm_bindgen]
impl CanvasUtils {
    /// Get canvas element by ID
    pub fn get_canvas(id: &str) -> Result<web_sys::HtmlCanvasElement, JsValue> {
        let window = web_sys::window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        let canvas = document
            .get_element_by_id(id)
            .ok_or("Canvas element not found")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;
        Ok(canvas)
    }

    /// Get device pixel ratio
    pub fn get_device_pixel_ratio() -> f64 {
        web_sys::window()
            .and_then(|w| w.device_pixel_ratio().into())
            .unwrap_or(1.0)
    }

    /// Set up high DPI canvas
    pub fn setup_high_dpi_canvas(canvas: &web_sys::HtmlCanvasElement, width: u32, height: u32) -> Result<(), JsValue> {
        let dpr = Self::get_device_pixel_ratio();
        let scaled_width = (width as f64 * dpr) as u32;
        let scaled_height = (height as f64 * dpr) as u32;

        canvas.set_width(scaled_width);
        canvas.set_height(scaled_height);

        // Set canvas style using JavaScript
        let style_width = format!("{}px", width);
        let style_height = format!("{}px", height);
        canvas.set_attribute("style", &format!("width: {}; height: {};", style_width, style_height))?;

        // Scale context for high DPI
        if let Some(context) = canvas.get_context("2d").ok().flatten() {
            if let Ok(context) = context.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                context.scale(dpr, dpr)?;
            }
        }

        Ok(())
    }
}

/// Animation frame utilities
#[wasm_bindgen]
pub struct AnimationFrame {
    callback: Option<js_sys::Function>,
    request_id: Option<i32>,
}

#[wasm_bindgen]
impl AnimationFrame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            callback: None,
            request_id: None,
        }
    }

    /// Start animation loop with callback
    pub fn start(&mut self, callback: js_sys::Function) -> Result<(), JsValue> {
        self.callback = Some(callback);
        self.request_next_frame()
    }

    /// Stop animation loop
    pub fn stop(&mut self) {
        if let Some(id) = self.request_id.take() {
            if let Some(window) = web_sys::window() {
                let _ = window.cancel_animation_frame(id);
            }
        }
        self.callback = None;
    }

    fn request_next_frame(&mut self) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            if let Some(callback) = &self.callback {
                let id = window.request_animation_frame(callback)?;
                self.request_id = Some(id);
            }
        }
        Ok(())
    }
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Event utilities
#[wasm_bindgen]
pub struct EventUtils;

#[wasm_bindgen]
impl EventUtils {
    /// Get mouse position relative to canvas
    pub fn get_mouse_position(_canvas: &web_sys::HtmlCanvasElement, event: &web_sys::MouseEvent) -> js_sys::Array {
        // For now, use a simple approach without get_bounding_client_rect
        // TODO: Implement proper mouse position calculation
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;

        let array = js_sys::Array::new();
        array.push(&JsValue::from(x));
        array.push(&JsValue::from(y));
        array
    }

    /// Check if event has modifier keys
    pub fn has_modifiers(event: &web_sys::MouseEvent) -> JsValue {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"ctrl".into(), &JsValue::from(event.ctrl_key())).unwrap();
        js_sys::Reflect::set(&obj, &"shift".into(), &JsValue::from(event.shift_key())).unwrap();
        js_sys::Reflect::set(&obj, &"alt".into(), &JsValue::from(event.alt_key())).unwrap();
        js_sys::Reflect::set(&obj, &"meta".into(), &JsValue::from(event.meta_key())).unwrap();
        obj.into()
    }
}

/// Version information
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Build information
#[wasm_bindgen]
pub fn get_build_info() -> JsValue {
    let info = js_sys::Object::new();

    js_sys::Reflect::set(&info, &"version".into(), &JsValue::from_str(env!("CARGO_PKG_VERSION"))).unwrap();
    js_sys::Reflect::set(&info, &"name".into(), &JsValue::from_str(env!("CARGO_PKG_NAME"))).unwrap();
    js_sys::Reflect::set(&info, &"authors".into(), &JsValue::from_str(env!("CARGO_PKG_AUTHORS"))).unwrap();
    js_sys::Reflect::set(&info, &"rustc_version".into(), &JsValue::from_str(option_env!("RUSTC_VERSION").unwrap_or("unknown"))).unwrap_or_default();

    info.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_performance_timer() {
        let mut timer = PerformanceTimer::new("test");

        // Timer should start with near-zero elapsed time
        let elapsed = timer.elapsed();
        assert!(elapsed >= 0.0);
        assert!(elapsed < 10.0); // Should be less than 10ms for this simple test

        timer.reset();
        let new_elapsed = timer.elapsed();
        assert!(new_elapsed >= 0.0);
        assert!(new_elapsed < 10.0);
    }

    #[wasm_bindgen_test]
    fn test_frame_rate_calculator() {
        let mut fps_calc = FrameRateCalculator::new(10);

        // Initial state
        assert_eq!(fps_calc.average_fps(), 0.0);
        assert_eq!(fps_calc.current_fps(), 0.0);

        // After update, should have some measurement
        fps_calc.update();
        // Can't assert specific values since timing is unpredictable in tests
    }

    #[wasm_bindgen_test]
    fn test_device_pixel_ratio() {
        let dpr = CanvasUtils::get_device_pixel_ratio();
        assert!(dpr > 0.0);
        assert!(dpr <= 4.0); // Reasonable upper bound for current devices
    }

    #[wasm_bindgen_test]
    fn test_version_info() {
        let version = get_version();
        assert!(!version.is_empty());

        let build_info = get_build_info();
        assert!(!build_info.is_null());
    }
}
