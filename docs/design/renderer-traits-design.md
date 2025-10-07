# Renderer Traits Design Document

## Overview
**Component**: Renderer Traits (`flow-renderer/src/traits.rs` - 825 lines)
**Status**: Needs refactoring into smaller, focused modules
**Complexity**: High - Multiple rendering backends and extensive trait definitions

## Current Architecture Problems

### Issues
1. **Massive monolithic file** (825 lines) - violates 300-line limit
2. **Mixed abstraction levels** - Traits, implementations, and utilities combined
3. **Multiple rendering backends** mixed together
4. **Poor separation** between 2D and 3D rendering concerns

### Current Structure Analysis
```rust
// Traits (200+ lines)
pub trait Renderer { ... }
pub trait BackgroundConfig { ... }

// Canvas2D Implementation (300+ lines)
impl Renderer for Canvas2DRenderer { ... }

// WebGL Implementation (175+ lines)
impl Renderer for WebGLRenderer { ... }

// Utilities and helpers (150+ lines)
```

## Proposed Refactored Architecture

### Directory Structure
```
flow-renderer/src/
├── traits/
│   ├── mod.rs              (40 lines) - Main trait exports
│   ├── renderer.rs         (120 lines) - Core Renderer trait
│   ├── background.rs       (80 lines) - Background configuration
│   ├── config.rs           (60 lines) - Renderer configuration
│   └── types.rs            (40 lines) - Common types and enums
├── canvas2d/
│   ├── mod.rs              (30 lines) - Canvas2D exports
│   ├── renderer.rs         (180 lines) - Canvas2DRenderer impl
│   ├── drawing.rs          (120 lines) - Drawing operations
│   ├── text.rs             (80 lines) - Text rendering
│   └── shapes.rs           (100 lines) - Shape primitives
├── webgl/
│   ├── mod.rs              (30 lines) - WebGL exports
│   ├── renderer.rs         (180 lines) - WebGLRenderer impl
│   ├── shaders.rs          (120 lines) - Shader management
│   ├── buffers.rs          (100 lines) - Buffer management
│   └── context.rs          (80 lines) - WebGL context setup
└── shared/
    ├── mod.rs              (30 lines) - Shared utilities
    ├── utils.rs            (80 lines) - Common utilities
    ├── error.rs            (60 lines) - Error types
    └── performance.rs      (100 lines) - Performance monitoring
```

## Core Trait Design

### Renderer Trait Hierarchy
```rust
pub trait Renderer {
    type Config: RendererConfig;
    type Error: std::error::Error;

    fn new(config: Self::Config) -> Result<Self, Self::Error>
        where Self: Sized;

    fn render_graph(&mut self, graph: &Graph, viewport: &Viewport) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn resize(&mut self, width: f64, height: f64) -> Result<(), Self::Error>;
}

pub trait Renderer2D: Renderer {
    fn draw_line(&mut self, start: Position, end: Position, style: &LineStyle) -> Result<(), Self::Error>;
    fn draw_rect(&mut self, rect: Rect, style: &ShapeStyle) -> Result<(), Self::Error>;
    fn draw_circle(&mut self, center: Position, radius: f64, style: &ShapeStyle) -> Result<(), Self::Error>;
    fn draw_text(&mut self, text: &str, position: Position, style: &TextStyle) -> Result<(), Self::Error>;
}

pub trait Renderer3D: Renderer {
    fn set_projection(&mut self, projection: &ProjectionMatrix) -> Result<(), Self::Error>;
    fn set_view(&mut self, view: &ViewMatrix) -> Result<(), Self::Error>;
    fn draw_mesh(&mut self, mesh: &Mesh, transform: &Transform) -> Result<(), Self::Error>;
}
```

### Background Configuration
```rust
pub trait BackgroundConfig {
    fn variant(&self) -> BackgroundVariant;
    fn color(&self) -> Color;
    fn pattern(&self) -> Option<&BackgroundPattern>;
}

#[derive(Debug, Clone)]
pub enum BackgroundVariant {
    Solid,
    Grid { size: f64, color: Color },
    Dots { radius: f64, spacing: f64, color: Color },
    Custom(String),
}
```

## Implementation Architecture

### Canvas2D Renderer
**File**: `canvas2d/renderer.rs` (180 lines)

```rust
pub struct Canvas2DRenderer {
    context: CanvasRenderingContext2d,
    config: Canvas2DConfig,
    background: Box<dyn BackgroundConfig>,
    performance_monitor: PerformanceMonitor,
}

impl Renderer2D for Canvas2DRenderer {
    fn draw_line(&mut self, start: Position, end: Position, style: &LineStyle) -> Result<(), Canvas2DError> {
        self.context.set_stroke_style(&style.color.to_js_value());
        self.context.set_line_width(style.width);
        self.context.begin_path();
        self.context.move_to(start.x, start.y);
        self.context.line_to(end.x, end.y);
        self.context.stroke();
        Ok(())
    }

    // ... other drawing methods
}
```

### WebGL Renderer
**File**: `webgl/renderer.rs` (180 lines)

```rust
pub struct WebGLRenderer {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    buffers: BufferManager,
    config: WebGLConfig,
    performance_monitor: PerformanceMonitor,
}

impl Renderer3D for WebGLRenderer {
    fn set_projection(&mut self, projection: &ProjectionMatrix) -> Result<(), WebGLError> {
        let location = self.context.get_uniform_location(&self.program, "u_projection");
        self.context.uniform_matrix4fv_with_f32_array(location.as_ref(), false, projection.as_slice());
        Ok(())
    }

    // ... other WebGL methods
}
```

## Configuration Design

### Renderer Configuration Traits
```rust
pub trait RendererConfig: Clone + Send + Sync {
    fn canvas_id(&self) -> &str;
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn pixel_ratio(&self) -> f64;
    fn background(&self) -> &dyn BackgroundConfig;
}

#[derive(Debug, Clone)]
pub struct Canvas2DConfig {
    pub canvas_id: String,
    pub width: f64,
    pub height: f64,
    pub pixel_ratio: f64,
    pub background: Box<dyn BackgroundConfig>,
    pub antialias: bool,
    pub alpha: bool,
}

#[derive(Debug, Clone)]
pub struct WebGLConfig {
    pub canvas_id: String,
    pub width: f64,
    pub height: f64,
    pub pixel_ratio: f64,
    pub background: Box<dyn BackgroundConfig>,
    pub antialias: bool,
    pub alpha: bool,
    pub depth: bool,
    pub stencil: bool,
}
```

## Error Handling

### Unified Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("Canvas2D error: {0}")]
    Canvas2D(#[from] Canvas2DError),

    #[error("WebGL error: {0}")]
    WebGL(#[from] WebGLError),

    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Performance monitoring error: {0}")]
    Performance(#[from] PerformanceError),
}

#[derive(Debug, thiserror::Error)]
pub enum Canvas2DError {
    #[error("Canvas context not available")]
    ContextNotAvailable,

    #[error("Invalid canvas dimensions: {width}x{height}")]
    InvalidDimensions { width: f64, height: f64 },

    #[error("Drawing operation failed: {operation}")]
    DrawingFailed { operation: String },
}
```

## Performance Monitoring

### Performance Traits
```rust
pub trait PerformanceMonitor {
    fn start_frame(&mut self);
    fn end_frame(&mut self) -> FrameStats;
    fn record_operation(&mut self, operation: &str, duration: Duration);
    fn get_stats(&self) -> PerformanceStats;
}

#[derive(Debug, Clone)]
pub struct FrameStats {
    pub frame_time: Duration,
    pub draw_calls: u32,
    pub triangles: u32,
    pub pixels: u32,
}

#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub average_frame_time: Duration,
    pub fps: f64,
    pub total_frames: u64,
    pub memory_usage: Option<u64>,
}
```

## Testing Strategy

### Unit Tests (per module)
- **traits/renderer.rs**: Trait contract verification
- **canvas2d/renderer.rs**: Canvas2D-specific functionality
- **webgl/renderer.rs**: WebGL-specific functionality
- **shared/utils.rs**: Utility function testing

### Integration Tests
- Cross-renderer compatibility
- Performance regression detection
- Memory leak detection

### Mock Testing
```rust
// Mock renderer for testing
pub struct MockRenderer {
    draw_calls: Vec<DrawCall>,
    config: MockConfig,
}

impl Renderer for MockRenderer {
    // Mock implementations for testing
}
```

## Migration Plan

### Phase 1: Extract Core Traits
1. Create `traits/` directory structure
2. Move trait definitions to separate files
3. Update imports and exports

### Phase 2: Split Canvas2D Implementation
1. Create `canvas2d/` directory
2. Extract Canvas2DRenderer implementation
3. Split drawing operations into separate files

### Phase 3: Split WebGL Implementation
1. Create `webgl/` directory
2. Extract WebGLRenderer implementation
3. Separate shader and buffer management

### Phase 4: Extract Shared Utilities
1. Create `shared/` directory
2. Move common utilities and error types
3. Update all references

### Phase 5: Update Public API
1. Update main `lib.rs` exports
2. Ensure backward compatibility
3. Add deprecation warnings for old API

## Success Metrics

- **File sizes**: All modules < 300 lines
- **Test coverage**: > 90% for each renderer
- **Performance**: No regression in rendering performance
- **API compatibility**: 100% backward compatibility
- **Maintainability**: Clear separation between 2D/3D rendering

## Timeline: 3-4 weeks
## Risk Level: Medium
## Dependencies: Performance monitoring module
