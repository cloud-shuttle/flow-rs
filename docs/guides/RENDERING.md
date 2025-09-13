# Rendering Architecture Guide

## Overview

Flow-RS features a pluggable rendering architecture that supports multiple backends with automatic fallback. This guide covers the rendering system, how to choose renderers, and how to implement custom rendering solutions.

## Renderer Architecture

### Renderer Selection Hierarchy

The renderer selection follows a performance-first approach with automatic fallback:

```
WebGPU → WebGL2 → Canvas2D → SVG (export only)
```

### Renderer Capabilities

| Renderer | Performance | Features | Browser Support |
|----------|-------------|----------|-----------------|
| **WebGPU** | Highest | GPU compute, instancing, shaders | Chrome 113+, Firefox 121+ |
| **WebGL2** | High | GPU rendering, shaders, textures | Chrome 56+, Firefox 51+ |
| **Canvas2D** | Moderate | CPU rendering, full compatibility | Universal |
| **SVG** | Export only | Vector graphics, infinite zoom | Export/print only |

## Renderer Selection

### Automatic Selection

By default, Flow-RS automatically detects and selects the best available renderer:

```rust
use flow_leptos::*;

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        // Automatic renderer selection
        // WebGPU → WebGL2 → Canvas2D
    />
}
```

### Manual Renderer Selection

You can force a specific renderer for testing or compatibility:

```rust
use flow_renderer::*;

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        renderer=RendererType::Canvas2D  // Force Canvas2D
        // renderer=RendererType::WebGL2    // Force WebGL2
        // renderer=RendererType::WebGPU    // Force WebGPU
    />
}
```

### Conditional Renderer Selection

Select renderer based on device capabilities:

```rust
let renderer_type = move || {
    let capabilities = detect_renderer_capabilities();

    if capabilities.webgpu && nodes.get().len() > 1000 {
        RendererType::WebGPU  // Use WebGPU for large graphs
    } else if capabilities.webgl2 {
        RendererType::WebGL2  // WebGL2 for medium graphs
    } else {
        RendererType::Canvas2D // Canvas2D fallback
    }
};

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        renderer=renderer_type
    />
}
```

## WebGPU Renderer

### Features

- **GPU Compute Shaders**: Parallel layout calculations
- **Instanced Rendering**: Efficient rendering of similar nodes
- **Advanced Shaders**: Custom visual effects and animations
- **Memory Efficiency**: GPU-resident data structures

### Configuration

```rust
use flow_renderer::webgpu::*;

let webgpu_config = WebGPUConfig {
    // Performance settings
    power_preference: PowerPreference::HighPerformance,
    max_instances_per_draw: 10000,
    use_compute_shaders: true,

    // Quality settings
    msaa_samples: 4,
    anisotropy: 16,

    // Memory settings
    buffer_usage: BufferUsage::DYNAMIC,
    max_buffer_size: 256 * 1024 * 1024, // 256MB
};

view! {
    <FlowEditor
        nodes=nodes
        edges=edges
        renderer=RendererType::WebGPU
        renderer_config=webgpu_config
    />
}
```

### Custom WebGPU Shaders

Define custom node rendering shaders:

```rust
// Custom vertex shader for nodes
const CUSTOM_NODE_VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct InstanceInput {
    @location(2) instance_position: vec2<f32>,
    @location(3) instance_size: vec2<f32>,
    @location(4) instance_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let world_pos = vertex.position * instance.instance_size + instance.instance_position;

    var out: VertexOutput;
    out.clip_position = view_proj * vec4<f32>(world_pos, 0.0, 1.0);
    out.uv = vertex.uv;
    out.color = instance.instance_color;
    return out;
}
"#;

// Custom fragment shader for nodes
const CUSTOM_NODE_FRAGMENT_SHADER: &str = r#"
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Rounded rectangle with border
    let center = in.uv - 0.5;
    let dist = length(max(abs(center) - vec2<f32>(0.4), vec2<f32>(0.0)));
    let border = smoothstep(0.08, 0.1, dist);
    let fill = 1.0 - smoothstep(0.0, 0.02, dist - 0.08);

    return mix(in.color, vec4<f32>(0.0, 0.0, 0.0, 1.0), border) * fill;
}
"#;

// Register custom shader
let custom_renderer = WebGPURenderer::builder()
    .node_vertex_shader(CUSTOM_NODE_VERTEX_SHADER)
    .node_fragment_shader(CUSTOM_NODE_FRAGMENT_SHADER)
    .build();
```

## WebGL2 Renderer

### Features

- **GPU Rendering**: Hardware-accelerated drawing
- **Texture Atlasing**: Efficient sprite rendering
- **Batch Rendering**: Reduced draw calls
- **Post-Processing**: Screen-space effects

### Configuration

```rust
use flow_renderer::webgl2::*;

let webgl2_config = WebGL2Config {
    // Context settings
    alpha: false,
    antialias: true,
    depth: false,
    stencil: false,
    preserve_drawing_buffer: false,

    // Performance settings
    max_textures: 16,
    max_draw_calls: 1000,
    batch_size: 2000,

    // Quality settings
    texture_filtering: TextureFiltering::Linear,
    max_texture_size: 4096,
};

view! {
    <FlowEditor
        renderer=RendererType::WebGL2
        renderer_config=webgl2_config
    />
}
```

### Custom WebGL2 Shaders

```glsl
// Custom vertex shader
attribute vec2 a_position;
attribute vec2 a_uv;
attribute vec2 a_instance_pos;
attribute vec2 a_instance_size;
attribute vec4 a_instance_color;

uniform mat3 u_transform;

varying vec2 v_uv;
varying vec4 v_color;

void main() {
    vec2 world_pos = a_position * a_instance_size + a_instance_pos;
    gl_Position = vec4(u_transform * vec3(world_pos, 1.0), 1.0);
    v_uv = a_uv;
    v_color = a_instance_color;
}
```

```glsl
// Custom fragment shader
precision mediump float;

varying vec2 v_uv;
varying vec4 v_color;

void main() {
    // Rounded rectangle
    vec2 center = v_uv - 0.5;
    float dist = length(max(abs(center) - vec2(0.4), 0.0));
    float alpha = 1.0 - smoothstep(0.0, 0.02, dist - 0.08);

    gl_FragColor = v_color * alpha;
}
```

## Canvas2D Renderer

### Features

- **Universal Compatibility**: Works on all browsers
- **Vector Graphics**: Crisp rendering at any zoom level
- **Text Rendering**: Native font support
- **Simple API**: Easy to customize and debug

### Configuration

```rust
use flow_renderer::canvas2d::*;

let canvas2d_config = Canvas2DConfig {
    // Quality settings
    image_smoothing: true,
    text_rendering: TextRenderingQuality::Optimized,

    // Performance settings
    use_offscreen_canvas: true,
    worker_rendering: false,
    dirty_rect_optimization: true,

    // Appearance settings
    pixel_ratio: window().device_pixel_ratio(),
    font_family: "system-ui, sans-serif".to_string(),
};

view! {
    <FlowEditor
        renderer=RendererType::Canvas2D
        renderer_config=canvas2d_config
    />
}
```

### Custom Canvas2D Rendering

```rust
use flow_renderer::canvas2d::*;

pub struct CustomCanvas2DRenderer {
    context: CanvasRenderingContext2d,
    config: Canvas2DConfig,
}

impl Canvas2DRenderer for CustomCanvas2DRenderer {
    fn render_node(&mut self, node: &Node, viewport: &Viewport) {
        let ctx = &self.context;

        // Transform to node coordinates
        ctx.save();
        ctx.translate(node.position.x, node.position.y);
        ctx.scale(viewport.zoom, viewport.zoom);

        // Custom node rendering
        self.render_node_background(node);
        self.render_node_content(node);
        self.render_node_border(node);

        ctx.restore();
    }

    fn render_node_background(&self, node: &Node) {
        let ctx = &self.context;

        // Gradient background
        let gradient = ctx.create_linear_gradient(0.0, 0.0, node.size.width, 0.0);
        gradient.add_color_stop(0.0, "#ffffff");
        gradient.add_color_stop(1.0, "#f0f0f0");

        ctx.set_fill_style(&gradient);
        ctx.fill_rect(0.0, 0.0, node.size.width, node.size.height);
    }

    fn render_node_content(&self, node: &Node) {
        let ctx = &self.context;

        // Render text content
        ctx.set_font("14px system-ui");
        ctx.set_fill_style(&JsValue::from_str("#333"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");

        let label = node.data.get("label").unwrap_or(&"Node".to_string());
        ctx.fill_text(
            label,
            node.size.width / 2.0,
            node.size.height / 2.0,
        );
    }
}
```

## Custom Renderer Implementation

### Renderer Trait

Implement the `Renderer` trait to create custom rendering backends:

```rust
use flow_renderer::*;

pub trait Renderer {
    type Config: Default;
    type Error: std::error::Error;

    // Lifecycle methods
    fn new(config: Self::Config) -> Result<Self, Self::Error> where Self: Sized;
    fn resize(&mut self, width: u32, height: u32) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn present(&mut self) -> Result<(), Self::Error>;

    // Rendering methods
    fn render_nodes(&mut self, nodes: &[Node], viewport: &Viewport) -> Result<(), Self::Error>;
    fn render_edges(&mut self, edges: &[Edge], viewport: &Viewport) -> Result<(), Self::Error>;
    fn render_selection(&mut self, selection: &Selection, viewport: &Viewport) -> Result<(), Self::Error>;

    // Capability queries
    fn capabilities(&self) -> RendererCapabilities;
    fn max_texture_size(&self) -> u32;
    fn max_instances(&self) -> u32;
}
```

### Example: ASCII Renderer

A simple text-based renderer for debugging:

```rust
pub struct ASCIIRenderer {
    width: usize,
    height: usize,
    buffer: Vec<char>,
}

impl Renderer for ASCIIRenderer {
    type Config = ASCIIConfig;
    type Error = ASCIIError;

    fn new(config: Self::Config) -> Result<Self, Self::Error> {
        Ok(Self {
            width: config.width,
            height: config.height,
            buffer: vec![' '; config.width * config.height],
        })
    }

    fn render_nodes(&mut self, nodes: &[Node], viewport: &Viewport) -> Result<(), Self::Error> {
        for node in nodes {
            let screen_pos = viewport.world_to_screen(node.position);
            let x = (screen_pos.x as usize).min(self.width - 1);
            let y = (screen_pos.y as usize).min(self.height - 1);

            if x < self.width && y < self.height {
                self.buffer[y * self.width + x] = '█';
            }
        }
        Ok(())
    }

    fn present(&mut self) -> Result<(), Self::Error> {
        // Print buffer to console
        for y in 0..self.height {
            let line: String = self.buffer[y * self.width..(y + 1) * self.width]
                .iter()
                .collect();
            println!("{}", line);
        }
        Ok(())
    }
}
```

## Performance Optimization

### Viewport Culling

All renderers support automatic viewport culling:

```rust
let culling_config = CullingConfig {
    // Frustum culling
    viewport_margin: 50.0,  // Render 50px outside viewport

    // LOD (Level of Detail)
    lod_enabled: true,
    lod_distance_threshold: 500.0,

    // Occlusion culling
    occlusion_culling: true,
    max_occlusion_queries: 1000,
};

view! {
    <FlowEditor
        culling_config=culling_config
        only_render_visible_elements=true
    />
}
```

### Batching and Instancing

Configure rendering batching for better performance:

```rust
let batching_config = BatchingConfig {
    // Group similar nodes together
    batch_by_type: true,
    batch_by_material: true,

    // Instance rendering thresholds
    instancing_threshold: 10,  // Use instancing for 10+ similar objects
    max_instances_per_batch: 1000,

    // Draw call optimization
    max_draw_calls_per_frame: 100,
    prefer_large_batches: true,
};

view! {
    <FlowEditor
        batching_config=batching_config
    />
}
```

### Memory Management

Configure memory usage for different scenarios:

```rust
let memory_config = MemoryConfig {
    // Texture memory
    texture_cache_size: 128 * 1024 * 1024,  // 128MB
    max_texture_uploads_per_frame: 4,

    // Vertex buffer memory
    vertex_buffer_size: 64 * 1024 * 1024,   // 64MB
    index_buffer_size: 32 * 1024 * 1024,    // 32MB

    // Object pooling
    node_pool_size: 10000,
    edge_pool_size: 15000,

    // Garbage collection
    gc_interval: Duration::from_secs(30),
    gc_threshold: 0.8,  // GC when 80% of memory used
};
```

## Debugging and Profiling

### Renderer Debug Mode

Enable debug overlays and performance metrics:

```rust
let debug_config = DebugConfig {
    // Visual debugging
    show_bounding_boxes: true,
    show_viewport_bounds: true,
    show_spatial_index: true,

    // Performance debugging
    show_frame_time: true,
    show_draw_calls: true,
    show_memory_usage: true,

    // Validation
    validate_state: true,
    check_gl_errors: true,
};

view! {
    <FlowEditor
        debug_mode=true
        debug_config=debug_config
    />
}
```

### Performance Monitoring

```rust
let performance_monitor = use_renderer_performance();

create_effect(move |_| {
    let stats = performance_monitor.get();

    if stats.frame_time > 16.67 {
        logging::warn!("Frame dropped: {}ms", stats.frame_time);
    }

    if stats.memory_usage > 500_000_000 {  // 500MB
        logging::warn!("High memory usage: {}MB", stats.memory_usage / 1_000_000);
    }
});
```

## Renderer Comparison

### Use Case Recommendations

| Scenario | Recommended Renderer | Reasoning |
|----------|---------------------|-----------|
| **< 100 nodes** | Canvas2D | Simple, universal compatibility |
| **100-1000 nodes** | WebGL2 | Good performance, broad support |
| **1000+ nodes** | WebGPU | Maximum performance, compute shaders |
| **Mobile devices** | Canvas2D/WebGL2 | Battery efficiency |
| **High-end desktop** | WebGPU | Utilize full GPU capabilities |
| **Legacy browsers** | Canvas2D | Maximum compatibility |
| **Print/export** | SVG | Vector graphics, infinite zoom |

### Performance Characteristics

| Renderer | Initialization | Small Graphs | Large Graphs | Memory Usage |
|----------|---------------|--------------|--------------|---------------|
| **WebGPU** | Slow | Fast | Very Fast | High |
| **WebGL2** | Medium | Fast | Fast | Medium |
| **Canvas2D** | Fast | Fast | Slow | Low |
| **SVG** | Fast | Medium | Very Slow | Very Low |

This rendering guide should help you choose and configure the optimal renderer for your specific use case, as well as implement custom rendering solutions when needed.
