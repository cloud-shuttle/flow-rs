# Flow-RS Leptos Demo

A demonstration of Flow-RS integrated with the Leptos framework, showcasing reactive flow-based node editing in a web application.

## Features

- Interactive node-based flow editor
- Real-time reactive updates using Leptos
- Canvas-based rendering with Flow-RS
- Drag and drop node manipulation
- Edge creation and management

## Building

This example uses Trunk for building and serving the web application.

### Prerequisites

- Rust 1.70+
- Trunk (install with `cargo install trunk`)

### Build and Run

```bash
# Build the application
trunk build

# Serve the application (development)
trunk serve

# Build for production
trunk build --release
```

## Usage

1. Open the application in your browser
2. Click and drag to create nodes
3. Connect nodes by dragging from one to another
4. Use the controls to manipulate the flow

## Architecture

This demo showcases the integration between:
- **Flow-RS Core**: Graph data structures and algorithms
- **Flow-RS Leptos**: Reactive components and state management
- **Flow-RS Renderer**: Canvas-based rendering
- **Leptos**: Reactive UI framework

## Development

The main application logic is in `src/main.rs`, which demonstrates:
- Setting up the Flow-RS graph
- Creating reactive components
- Handling user interactions
- Rendering the flow editor
