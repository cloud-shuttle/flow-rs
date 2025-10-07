# Background Variants Example

Demonstrates Flow-RS's comprehensive background customization capabilities with different patterns, colors, and interactive controls.

## What it demonstrates

- **Pattern types**: Dots, Lines, and Crosshatch patterns
- **Interactive controls**: Real-time customization
- **Color customization**: Background and pattern colors
- **Size and opacity**: Adjustable pattern properties
- **Performance optimization**: Efficient background rendering

## Features shown

- ✅ **3 pattern types**: Dots, Lines, Crosshatch
- ✅ **Real-time editing**: Interactive controls
- ✅ **Color customization**: Full RGB color control
- ✅ **Size adjustment**: 5-50px pattern spacing
- ✅ **Opacity control**: 10-90% transparency
- ✅ **Responsive rendering**: Adapts to canvas size

## Pattern types demonstrated

### **Dots Pattern** 🔵
- Circular dots arranged in a regular grid
- Clean, minimal appearance
- Perfect for subtle visual guidance
- Low visual noise, high readability

### **Lines Pattern** 📏
- Horizontal and vertical lines forming a ruled grid
- Technical, structured appearance
- Excellent for precision work
- Traditional graph paper style

### **Crosshatch Pattern** 🔀
- Diagonal lines crossing at 45-degree angles
- Professional, technical aesthetic
- Creates depth and visual interest
- Good for complex diagrams

## Interactive features

### **Real-time Controls**
- **Pattern Selector**: Switch between Dots/Lines/Crosshatch
- **Color Pickers**: Choose background and pattern colors
- **Size Slider**: Adjust pattern spacing (5-50px)
- **Opacity Slider**: Control pattern transparency (10-90%)
- **Live Preview**: Instant visual feedback

### **Customization Options**
- **Hex Color Support**: Full RGB color range
- **Precise Control**: Fine-grained adjustments
- **Visual Feedback**: Immediate preview of changes
- **Persistent Settings**: Changes maintained during session

## Running the example

### Quick Start

```bash
# Navigate to the example directory
cd examples/background-variants

# Build and serve
./build.sh

# Or build manually
wasm-pack build --target web --out-dir pkg --dev

# Serve with Python
python3 -m http.server 8000

# Open http://localhost:8000
```

## Code structure

```
src/lib.rs
├── BackgroundVariant enum (pattern types)
├── setup_background_controls() (interactive controls)
├── rerender_with_current_background() (live updates)
├── render_with_background() (pattern rendering)
├── create_sample_graph() (demo data)
└── WASM entry point
```

## Implementation details

```rust
// Background configuration structure
struct BackgroundConfig {
    color: String,        // Background fill color
    pattern_color: String, // Pattern element color
    variant: BackgroundVariant, // Pattern type
    size: f64,           // Pattern spacing/size
    opacity: f64,        // Pattern transparency
}

// Pattern rendering variants
enum BackgroundVariant {
    Dots,      // Circular grid pattern
    Lines,     // Horizontal/vertical lines
    Crosshatch // Diagonal crossing lines
}

// Interactive control setup
fn setup_background_controls() {
    // DOM event handlers for real-time updates
    // Color picker, slider, and select controls
    // Automatic re-rendering on changes
}
```

## Performance considerations

- **Efficient rendering**: Patterns drawn once, cached
- **Minimal CPU usage**: No continuous animation
- **Memory optimized**: Small pattern data structures
- **Scalable**: Works with any canvas size

## Browser compatibility

Compatible with all modern browsers supporting:
- WebAssembly
- Canvas 2D API
- ES6 modules
- CSS Color input types

## Customization API

### **Basic Usage**
```rust
let config = BackgroundConfig {
    color: "#ffffff".to_string(),
    pattern_color: "#e2e8f0".to_string(),
    variant: BackgroundVariant::Dots,
    size: 20.0,
    opacity: 0.5,
};

renderer.render_background(&config, &viewport)?;
```

### **Advanced Customization**
```rust
// Custom colors
config.color = "#f8fafc".to_string();
config.pattern_color = "#4299e1".to_string();

// Different patterns
config.variant = BackgroundVariant::Lines;

// Size adjustments
config.size = 30.0;  // Larger spacing
config.opacity = 0.3; // More subtle
```

## Related examples

- **Hello World**: Basic static flow display
- **Custom Styles**: Node and edge customization
- **Custom Edges**: Edge styling and routing
- **Basic Layouts**: Layout algorithm showcase
