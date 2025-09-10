//! Event system for flow editor interactions

use serde::{Deserialize, Serialize};

use leptos_flow_core::{Node, Edge, NodeId, EdgeId, Position};

/// Flow-level events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowEvent {
    /// Canvas was clicked
    CanvasClick {
        position: Position,
        modifiers: KeyboardModifiers,
    },
    /// Canvas pan/zoom changed
    ViewportChanged {
        offset: Position,
        zoom: f64,
    },
    /// Selection changed
    SelectionChanged {
        nodes: Vec<NodeId>,
        edges: Vec<EdgeId>,
    },
    /// Drag operation started
    DragStart {
        position: Position,
        target: DragTarget,
    },
    /// Drag operation updated
    DragUpdate {
        position: Position,
        delta: Position,
    },
    /// Drag operation ended
    DragEnd {
        position: Position,
        target: DragTarget,
    },
}

/// Node-specific events
#[derive(Debug, Clone)]
pub enum NodeEvent<N>
where
    N: Clone,
{
    /// Node was clicked
    Click {
        node: Node<N>,
        position: Position,
        modifiers: KeyboardModifiers,
    },
    /// Node was double-clicked
    DoubleClick {
        node: Node<N>,
        position: Position,
    },
    /// Node was selected
    Select {
        node: Node<N>,
    },
    /// Node was deselected
    Deselect {
        node: Node<N>,
    },
    /// Node drag started
    DragStart {
        node: Node<N>,
        position: Position,
    },
    /// Node being dragged
    Drag {
        node: Node<N>,
        position: Position,
        delta: Position,
    },
    /// Node drag ended
    DragEnd {
        node: Node<N>,
        position: Position,
        final_position: Position,
    },
    /// Node hover started
    HoverStart {
        node: Node<N>,
        position: Position,
    },
    /// Node hover ended
    HoverEnd {
        node: Node<N>,
    },
}

/// Edge-specific events
#[derive(Debug, Clone)]
pub enum EdgeEvent<E>
where
    E: Clone,
{
    /// Edge was clicked
    Click {
        edge: Edge<E>,
        position: Position,
        modifiers: KeyboardModifiers,
    },
    /// Edge was selected
    Select {
        edge: Edge<E>,
    },
    /// Edge was deselected
    Deselect {
        edge: Edge<E>,
    },
    /// Edge hover started
    HoverStart {
        edge: Edge<E>,
        position: Position,
    },
    /// Edge hover ended
    HoverEnd {
        edge: Edge<E>,
    },
}

/// Keyboard modifier keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Default for KeyboardModifiers {
    fn default() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }
}

impl KeyboardModifiers {
    /// Create new keyboard modifiers
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any modifier is pressed
    pub fn any(&self) -> bool {
        self.ctrl || self.shift || self.alt || self.meta
    }

    /// Check if only ctrl is pressed
    pub fn ctrl_only(&self) -> bool {
        self.ctrl && !self.shift && !self.alt && !self.meta
    }

    /// Check if only shift is pressed
    pub fn shift_only(&self) -> bool {
        !self.ctrl && self.shift && !self.alt && !self.meta
    }

    /// Create from web event
    #[cfg(feature = "web-sys")]
    pub fn from_mouse_event(event: &web_sys::MouseEvent) -> Self {
        Self {
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        }
    }

    /// Create from web keyboard event
    #[cfg(feature = "web-sys")]
    pub fn from_keyboard_event(event: &web_sys::KeyboardEvent) -> Self {
        Self {
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        }
    }
}

/// Drag target specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragTarget {
    /// Dragging a node
    Node(NodeId),
    /// Dragging multiple nodes
    Nodes(Vec<NodeId>),
    /// Dragging the canvas (pan)
    Canvas,
    /// Dragging a selection rectangle
    Selection,
}

/// Mouse button specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
    Back = 3,
    Forward = 4,
}

impl MouseButton {
    /// Create from web mouse event button
    #[cfg(feature = "web-sys")]
    pub fn from_mouse_event(event: &web_sys::MouseEvent) -> Option<Self> {
        match event.button() {
            0 => Some(Self::Left),
            1 => Some(Self::Middle),
            2 => Some(Self::Right),
            3 => Some(Self::Back),
            4 => Some(Self::Forward),
            _ => None,
        }
    }
}

/// Event handler trait for custom event processing
pub trait EventHandler<T> {
    fn handle(&self, event: T);
}

/// Convenience implementations
impl<F, T> EventHandler<T> for F
where
    F: Fn(T),
{
    fn handle(&self, event: T) {
        self(event)
    }
}

/// Event dispatcher for managing multiple event handlers
pub struct EventDispatcher<T> {
    handlers: Vec<Box<dyn EventHandler<T>>>,
}

impl<T> Default for EventDispatcher<T> {
    fn default() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
}

impl<T> EventDispatcher<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an event handler
    pub fn add_handler<H>(&mut self, handler: H)
    where
        H: EventHandler<T> + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Dispatch an event to all handlers
    pub fn dispatch(&self, event: T) {
        for handler in &self.handlers {
            handler.handle(event.clone());
        }
    }

    /// Remove all handlers
    pub fn clear(&mut self) {
        self.handlers.clear();
    }
}

/// Utility functions for event handling
pub mod utils {
    use super::*;
    use leptos_flow_core::Rect;

    /// Check if a position is inside a rectangle
    pub fn position_in_rect(pos: Position, rect: &Rect) -> bool {
        pos.x >= rect.x
            && pos.x <= rect.x + rect.width
            && pos.y >= rect.y
            && pos.y <= rect.y + rect.height
    }

    /// Calculate distance from position to rectangle edge
    pub fn distance_to_rect(pos: Position, rect: &Rect) -> f64 {
        let dx = if pos.x < rect.x {
            rect.x - pos.x
        } else if pos.x > rect.x + rect.width {
            pos.x - (rect.x + rect.width)
        } else {
            0.0
        };

        let dy = if pos.y < rect.y {
            rect.y - pos.y
        } else if pos.y > rect.y + rect.height {
            pos.y - (rect.y + rect.height)
        } else {
            0.0
        };

        (dx * dx + dy * dy).sqrt()
    }

    /// Convert canvas coordinates to world coordinates
    pub fn canvas_to_world(
        canvas_pos: Position,
        viewport_offset: Position,
        zoom: f64,
    ) -> Position {
        Position::new(
            (canvas_pos.x / zoom) + viewport_offset.x,
            (canvas_pos.y / zoom) + viewport_offset.y,
        )
    }

    /// Convert world coordinates to canvas coordinates
    pub fn world_to_canvas(
        world_pos: Position,
        viewport_offset: Position,
        zoom: f64,
    ) -> Position {
        Position::new(
            (world_pos.x - viewport_offset.x) * zoom,
            (world_pos.y - viewport_offset.y) * zoom,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_flow_core::{NodeBuilder, EdgeBuilder, Size};

    #[test]
    fn test_keyboard_modifiers() {
        let modifiers = KeyboardModifiers {
            ctrl: true,
            shift: false,
            alt: false,
            meta: false,
        };

        assert!(modifiers.any());
        assert!(modifiers.ctrl_only());
        assert!(!modifiers.shift_only());
    }

    #[test]
    fn test_position_in_rect() {
        let rect = leptos_flow_core::Rect::new(10.0, 10.0, 100.0, 50.0);
        
        assert!(utils::position_in_rect(Position::new(50.0, 25.0), &rect));
        assert!(!utils::position_in_rect(Position::new(5.0, 25.0), &rect));
        assert!(!utils::position_in_rect(Position::new(120.0, 25.0), &rect));
    }

    #[test]
    fn test_coordinate_conversion() {
        let canvas_pos = Position::new(100.0, 200.0);
        let viewport_offset = Position::new(50.0, 75.0);
        let zoom = 2.0;

        let world_pos = utils::canvas_to_world(canvas_pos, viewport_offset, zoom);
        assert_eq!(world_pos.x, 100.0);
        assert_eq!(world_pos.y, 175.0);

        let back_to_canvas = utils::world_to_canvas(world_pos, viewport_offset, zoom);
        assert!((back_to_canvas.x - canvas_pos.x).abs() < f64::EPSILON);
        assert!((back_to_canvas.y - canvas_pos.y).abs() < f64::EPSILON);
    }

    #[test]
    fn test_event_dispatcher() {
        let mut dispatcher = EventDispatcher::<i32>::new();
        let mut received = Vec::new();

        dispatcher.add_handler(|x: i32| {
            // Handler would normally do something with the event
            assert!(x > 0);
        });

        dispatcher.dispatch(42);
        // Event was handled successfully if no panic occurred
    }
}