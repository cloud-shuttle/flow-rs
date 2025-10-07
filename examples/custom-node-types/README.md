# Custom Node Types Example

Interactive demonstration of custom node components with specialized behaviors, rendering, and user interactions. Shows how to extend Flow-RS with plugin-like functionality for custom node types.

## What it demonstrates

- **Custom node components**: Specialized node types with unique behaviors
- **Interactive behaviors**: Click, hover, drag interactions
- **Custom rendering**: Node-specific visual styles and animations
- **Plugin architecture**: Extensible system for adding new node types
- **State management**: Node-specific data and real-time updates

## Node Types Featured

### **Button Nodes** 🔘
- **Interactive buttons** with click handlers
- **Action triggers** that affect other nodes
- **Visual feedback** on hover and click states
- **Custom styling** with rounded corners and colors

### **Slider Nodes** 🎚️
- **Draggable controls** for value adjustment
- **Real-time feedback** showing current percentage
- **Smooth interactions** with mouse drag handling
- **Visual indicators** for handle position and fill

### **Progress Nodes** 📊
- **Animated progress bars** with continuous updates
- **Smooth animations** using sine wave patterns
- **Visual progress indication** with fill and border
- **Real-time value changes** demonstrating live updates

### **Toggle Nodes** 🔄
- **On/off switches** with smooth transitions
- **Visual state indication** with color changes
- **Smooth animations** when toggling states
- **Interactive feedback** on hover and click

### **Data Display Nodes** 📈
- **Information displays** showing dynamic values
- **Periodic updates** simulating real-time data
- **Custom layouts** with labels and values
- **Visual styling** for data presentation

## Interactive Features

### **Mouse Interactions**
- **Hover effects**: Color changes and visual feedback on mouse over
- **Click handlers**: Specialized behavior for different node types
- **Drag interactions**: Smooth slider control with mouse dragging
- **Selection states**: Visual indication of active/selected nodes

### **Dynamic Behaviors**
- **Button actions**: "Click Me!" randomizes other node values, "Reset" restores defaults
- **Slider control**: Real-time value adjustment with visual feedback
- **Toggle switching**: Smooth on/off transitions with animations
- **Progress animation**: Continuous updates with smooth transitions

### **Node Management**
- **Add new nodes**: UI controls to create nodes of different types
- **Type selection**: Dropdown to choose node type for new additions
- **Dynamic positioning**: Automatic placement of newly added nodes
- **Interactive creation**: Real-time addition with immediate interaction

## Technical Implementation

### **Custom Node Architecture**
```rust
#[derive(Clone, Debug, PartialEq)]
enum CustomNodeType {
    Button, Slider, Progress, Toggle, DataDisplay
}

#[derive(Clone, Debug)]
struct CustomNodeData {
    pub node_type: CustomNodeType,
    pub label: String,
    pub value: f64,
    pub is_active: bool,
    pub color: String,
    pub hover_color: String,
}
```

### **Specialized Rendering**
```rust
match node.data.node_type {
    CustomNodeType::Button => render_button_node(renderer, node, color)?,
    CustomNodeType::Slider => render_slider_node(renderer, node, color, is_selected)?,
    CustomNodeType::Progress => render_progress_node(renderer, node, color)?,
    CustomNodeType::Toggle => render_toggle_node(renderer, node, color, is_selected)?,
    CustomNodeType::DataDisplay => render_data_display_node(renderer, node, color)?,
}
```

### **Interactive Event Handling**
```rust
// Mouse event processing
handle_mouse_move(event);    // Hover effects
handle_mouse_down(event);    // Click and drag start
handle_mouse_up(event);      // Drag end

// Node-specific interactions
match node_type {
    Button => handle_button_click(node),
    Toggle => node.data.value = 1.0 - node.data.value,
    Slider => handle_slider_drag(node, mouse_pos),
}
```

### **Animation and Updates**
```rust
// Continuous animation loop
fn update_custom_nodes(viewport: &Viewport) {
    // Update animated progress bars
    for node in graph.nodes_mut() {
        if node.data.node_type == Progress {
            node.data.value = (current_time * 0.001).sin() * 0.5 + 0.5;
        }
    }
    render_custom_nodes(viewport);
}
```

## Extensibility Features

### **Plugin-like Architecture**
- **Node type registry**: Easy addition of new node types
- **Rendering pipeline**: Modular rendering for different components
- **Event handling**: Extensible interaction system
- **Data management**: Flexible state storage per node type

### **Custom Rendering API**
- **Shape primitives**: Rectangles, circles, rounded rectangles
- **Text rendering**: Labels and value displays
- **Color management**: Dynamic color schemes and hover states
- **Animation support**: Smooth transitions and state changes

### **Interaction Framework**
- **Event routing**: Mouse events to appropriate node handlers
- **State management**: Node-specific interaction states
- **Feedback system**: Visual and behavioral responses
- **Coordinate handling**: World-to-screen transformations

## Performance Characteristics

### **Rendering Performance**
- **60 FPS animations**: Smooth visual updates for all node types
- **Efficient rendering**: Only re-render changed elements
- **Memory optimized**: Minimal overhead per custom node
- **Scalable interactions**: Handles multiple simultaneous interactions

### **Interaction Responsiveness**
- **Sub-millisecond feedback**: Instant response to user actions
- **Smooth animations**: Fluid transitions and state changes
- **Real-time updates**: Live value changes and visual feedback
- **Concurrent handling**: Multiple node interactions simultaneously

## Use Cases Demonstrated

### **UI Component Libraries**
- **Reusable components**: Custom nodes as building blocks
- **Interactive widgets**: Sliders, buttons, toggles, progress bars
- **Data visualization**: Information displays and charts
- **Control panels**: Real-time monitoring and control interfaces

### **Domain-Specific Applications**
- **Process monitoring**: Industrial control system interfaces
- **Data dashboards**: Real-time analytics and reporting
- **IoT interfaces**: Sensor control and monitoring panels
- **Configuration tools**: Parameter adjustment interfaces

### **Educational Tools**
- **Interactive diagrams**: Educational content with user interaction
- **Simulation interfaces**: Real-time model controls
- **Training systems**: Interactive learning environments
- **Demonstration tools**: Feature showcase and tutorials

## Browser Compatibility

Compatible with all modern browsers supporting:
- WebAssembly
- Canvas 2D API
- Mouse events
- requestAnimationFrame
- ES6 modules

## Customization Examples

### **Adding New Node Types**
```rust
// 1. Add to enum
enum CustomNodeType {
    // ... existing types
    CustomGauge,
}

// 2. Implement rendering
fn render_gauge_node(renderer: &mut Canvas2DRenderer, node: &Node<CustomNodeData>)
    -> Result<(), Box<dyn std::error::Error>> {
    // Custom gauge rendering logic
    Ok(())
}

// 3. Add interaction handling
match node.data.node_type {
    CustomNodeType::CustomGauge => handle_gauge_interaction(node),
    // ... other handlers
}
```

### **Extending Existing Types**
```rust
// Add new properties to CustomNodeData
struct CustomNodeData {
    // ... existing fields
    pub custom_property: Option<String>,
    pub animation_speed: f64,
}

// Customize rendering behavior
match node.data.custom_property.as_deref() {
    Some("special") => render_special_variant(renderer, node),
    _ => render_standard_variant(renderer, node),
}
```

## Educational Value

This example teaches:
- Component-based architecture patterns
- Custom rendering techniques in WebAssembly
- Interactive UI design principles
- State management for complex components
- Plugin system design patterns
- Real-time animation techniques
- Event-driven programming
- User experience design

## Business Impact

### **Developer Productivity**
- **Reusable components**: Faster development with custom node library
- **Consistent interfaces**: Standardized interaction patterns
- **Extensible architecture**: Easy addition of new node types
- **Visual consistency**: Professional appearance across applications

### **User Experience**
- **Intuitive interactions**: Familiar UI patterns and behaviors
- **Real-time feedback**: Immediate response to user actions
- **Visual polish**: Smooth animations and professional styling
- **Accessibility**: Clear visual states and interaction feedback

### **Application Capabilities**
- **Rich interfaces**: Complex control panels and dashboards
- **Real-time monitoring**: Live data visualization and control
- **Interactive diagrams**: Dynamic flowcharts and process diagrams
- **Configuration tools**: Parameter adjustment and system setup

---

## Key Takeaway

**Flow-RS enables the creation of rich, interactive node-based interfaces with custom components that rival traditional UI frameworks, all while maintaining WebAssembly performance advantages.**

The custom node types example demonstrates how Flow-RS can be extended to create sophisticated, interactive applications that would typically require heavy JavaScript frameworks, but with the performance and efficiency of WebAssembly.
