//! Leptos hooks and utilities for flow editors

use leptos::*;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, HtmlCanvasElement, MouseEvent, WheelEvent, KeyboardEvent};

use leptos_flow_core::{Graph, Node, NodeId, Position, Viewport};
use crate::events::{FlowEvent, NodeEvent, KeyboardModifiers, MouseButton};
use crate::signals::{FlowState, ViewportState};

/// Hook for managing canvas mouse interactions
pub fn use_canvas_mouse<N, E>(
    canvas_ref: NodeRef<HtmlCanvasElement>,
    graph: ReadSignal<Graph<N, E>>,
    viewport: RwSignal<ViewportState>,
    flow_state: RwSignal<FlowState>,
    on_flow_event: Option<Rc<dyn Fn(FlowEvent)>>,
    on_node_event: Option<Rc<dyn Fn(NodeEvent<N>)>>,
) where
    N: Clone + 'static,
    E: Clone + 'static,
{
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_clone = canvas.clone();
            let graph_clone = graph.clone();
            let viewport_clone = viewport.clone();
            let flow_state_clone = flow_state.clone();
            let on_flow_event_clone = on_flow_event.clone();
            let on_node_event_clone = on_node_event.clone();

            // Mouse down handler
            let mouse_down_handler = {
                let canvas = canvas_clone.clone();
                let graph = graph_clone.clone();
                let viewport = viewport_clone.clone();
                let flow_state = flow_state_clone.clone();
                let on_flow_event = on_flow_event_clone.clone();
                let on_node_event = on_node_event_clone.clone();

                Closure::wrap(Box::new(move |event: MouseEvent| {
                    event.prevent_default();
                    
                    let button = MouseButton::from_mouse_event(&event);
                    let modifiers = KeyboardModifiers::from_mouse_event(&event);
                    
                    if let Some(MouseButton::Left) = button {
                        let canvas_rect = canvas.get_bounding_client_rect();
                        let canvas_pos = Position::new(
                            event.client_x() as f64 - canvas_rect.left(),
                            event.client_y() as f64 - canvas_rect.top(),
                        );

                        let viewport_state = viewport.get();
                        let world_pos = crate::events::utils::canvas_to_world(
                            canvas_pos,
                            viewport_state.viewport.offset,
                            viewport_state.viewport.zoom,
                        );

                        // Check if we clicked on a node
                        let clicked_node = graph.get().nodes().find(|node| {
                            let bounds = node.bounds();
                            crate::events::utils::position_in_rect(world_pos, &bounds)
                        });

                        match clicked_node {
                            Some(node) => {
                                // Node click
                                flow_state.update(|state| {
                                    if modifiers.shift {
                                        state.add_node_to_selection(node.id().clone());
                                    } else {
                                        state.select_node(node.id().clone());
                                    }
                                    state.start_drag(canvas_pos);
                                });

                                if let Some(handler) = &on_node_event {
                                    handler(NodeEvent::Click {
                                        node: node.clone(),
                                        position: world_pos,
                                        modifiers,
                                    });
                                }

                                if let Some(handler) = &on_flow_event {
                                    handler(FlowEvent::DragStart {
                                        position: world_pos,
                                        target: crate::events::DragTarget::Node(node.id().clone()),
                                    });
                                }
                            }
                            None => {
                                // Canvas click
                                flow_state.update(|state| {
                                    if !modifiers.shift {
                                        state.clear_selection();
                                    }
                                    state.start_drag(canvas_pos);
                                });

                                if let Some(handler) = &on_flow_event {
                                    handler(FlowEvent::CanvasClick {
                                        position: world_pos,
                                        modifiers,
                                    });
                                    
                                    handler(FlowEvent::DragStart {
                                        position: world_pos,
                                        target: crate::events::DragTarget::Canvas,
                                    });
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };

            // Mouse move handler
            let mouse_move_handler = {
                let canvas = canvas_clone.clone();
                let viewport = viewport_clone.clone();
                let flow_state = flow_state_clone.clone();
                let on_flow_event = on_flow_event_clone.clone();

                Closure::wrap(Box::new(move |event: MouseEvent| {
                    let canvas_rect = canvas.get_bounding_client_rect();
                    let canvas_pos = Position::new(
                        event.client_x() as f64 - canvas_rect.left(),
                        event.client_y() as f64 - canvas_rect.top(),
                    );

                    let flow_state_val = flow_state.get();
                    if flow_state_val.is_dragging {
                        flow_state.update(|state| {
                            state.update_drag(canvas_pos);
                        });

                        let viewport_state = viewport.get();
                        let world_pos = crate::events::utils::canvas_to_world(
                            canvas_pos,
                            viewport_state.viewport.offset,
                            viewport_state.viewport.zoom,
                        );

                        if let Some(delta) = flow_state_val.drag_delta() {
                            // Handle different drag targets
                            let selected_nodes = &flow_state_val.selected_nodes;
                            if !selected_nodes.is_empty() {
                                // Dragging nodes - handled in drag end
                            } else {
                                // Dragging canvas (panning)
                                viewport.update(|vp| {
                                    let world_delta = Position::new(
                                        -delta.x / vp.viewport.zoom,
                                        -delta.y / vp.viewport.zoom,
                                    );
                                    vp.pan(world_delta);
                                });

                                // Reset drag start to current position for smooth panning
                                flow_state.update(|state| {
                                    state.drag_start = Some(canvas_pos);
                                });
                            }

                            if let Some(handler) = &on_flow_event {
                                handler(FlowEvent::DragUpdate {
                                    position: world_pos,
                                    delta,
                                });
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };

            // Mouse up handler
            let mouse_up_handler = {
                let viewport = viewport_clone.clone();
                let flow_state = flow_state_clone.clone();
                let on_flow_event = on_flow_event_clone.clone();

                Closure::wrap(Box::new(move |event: MouseEvent| {
                    let flow_state_val = flow_state.get();
                    if flow_state_val.is_dragging {
                        let canvas_rect = canvas.get_bounding_client_rect();
                        let canvas_pos = Position::new(
                            event.client_x() as f64 - canvas_rect.left(),
                            event.client_y() as f64 - canvas_rect.top(),
                        );

                        let viewport_state = viewport.get();
                        let world_pos = crate::events::utils::canvas_to_world(
                            canvas_pos,
                            viewport_state.viewport.offset,
                            viewport_state.viewport.zoom,
                        );

                        // Determine drag target
                        let drag_target = if !flow_state_val.selected_nodes.is_empty() {
                            if flow_state_val.selected_nodes.len() == 1 {
                                crate::events::DragTarget::Node(flow_state_val.selected_nodes[0].clone())
                            } else {
                                crate::events::DragTarget::Nodes(flow_state_val.selected_nodes.clone())
                            }
                        } else {
                            crate::events::DragTarget::Canvas
                        };

                        flow_state.update(|state| {
                            state.end_drag();
                        });

                        if let Some(handler) = &on_flow_event {
                            handler(FlowEvent::DragEnd {
                                position: world_pos,
                                target: drag_target,
                            });
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };

            // Add event listeners
            let _ = canvas.add_event_listener_with_callback(
                "mousedown",
                mouse_down_handler.as_ref().unchecked_ref(),
            );
            let _ = canvas.add_event_listener_with_callback(
                "mousemove",
                mouse_move_handler.as_ref().unchecked_ref(),
            );
            let _ = canvas.add_event_listener_with_callback(
                "mouseup",
                mouse_up_handler.as_ref().unchecked_ref(),
            );

            // Store closures to prevent garbage collection
            mouse_down_handler.forget();
            mouse_move_handler.forget();
            mouse_up_handler.forget();
        }
    });
}

/// Hook for managing canvas wheel events (zoom)
pub fn use_canvas_wheel(
    canvas_ref: NodeRef<HtmlCanvasElement>,
    viewport: RwSignal<ViewportState>,
    on_flow_event: Option<Rc<dyn Fn(FlowEvent)>>,
) {
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let viewport_clone = viewport.clone();
            let on_flow_event_clone = on_flow_event.clone();

            let wheel_handler = Closure::wrap(Box::new(move |event: WheelEvent| {
                event.prevent_default();

                let canvas_rect = canvas.get_bounding_client_rect();
                let canvas_pos = Position::new(
                    event.client_x() as f64 - canvas_rect.left(),
                    event.client_y() as f64 - canvas_rect.top(),
                );

                let delta_y = event.delta_y();
                let zoom_factor = if delta_y < 0.0 { 1.1 } else { 1.0 / 1.1 };

                let old_viewport = viewport_clone.get();
                
                viewport_clone.update(|vp| {
                    let world_pos = crate::events::utils::canvas_to_world(
                        canvas_pos,
                        vp.viewport.offset,
                        vp.viewport.zoom,
                    );
                    vp.zoom_at(zoom_factor, world_pos);
                });

                let new_viewport = viewport_clone.get();
                if let Some(handler) = &on_flow_event_clone {
                    handler(FlowEvent::ViewportChanged {
                        offset: new_viewport.viewport.offset,
                        zoom: new_viewport.viewport.zoom,
                    });
                }
            }) as Box<dyn FnMut(_)>);

            let _ = canvas.add_event_listener_with_callback(
                "wheel",
                wheel_handler.as_ref().unchecked_ref(),
            );

            wheel_handler.forget();
        }
    });
}

/// Hook for managing keyboard shortcuts
pub fn use_keyboard_shortcuts(
    on_flow_event: Option<Rc<dyn Fn(FlowEvent)>>,
) {
    create_effect(move |_| {
        let on_flow_event_clone = on_flow_event.clone();

        let keydown_handler = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let modifiers = KeyboardModifiers::from_keyboard_event(&event);
            let key = event.key();

            match key.as_str() {
                "Delete" | "Backspace" => {
                    if let Some(handler) = &on_flow_event_clone {
                        // Would trigger delete selected items
                    }
                }
                "a" | "A" if modifiers.ctrl_only() => {
                    event.prevent_default();
                    if let Some(handler) = &on_flow_event_clone {
                        // Would trigger select all
                    }
                }
                "z" | "Z" if modifiers.ctrl_only() => {
                    event.prevent_default();
                    if let Some(handler) = &on_flow_event_clone {
                        // Would trigger undo
                    }
                }
                "y" | "Y" if modifiers.ctrl_only() => {
                    event.prevent_default();
                    if let Some(handler) = &on_flow_event_clone {
                        // Would trigger redo
                    }
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "keydown",
                keydown_handler.as_ref().unchecked_ref(),
            );
            keydown_handler.forget();
        }
    });
}

/// Hook for managing graph operations with undo/redo
pub fn use_graph_operations<N, E>(
    graph: RwSignal<Graph<N, E>>,
) -> GraphOperationsHandle<N, E>
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    let undo_stack = create_rw_signal(Vec::<Graph<N, E>>::new());
    let redo_stack = create_rw_signal(Vec::<Graph<N, E>>::new());

    GraphOperationsHandle {
        graph,
        undo_stack,
        redo_stack,
    }
}

/// Handle for graph operations with undo/redo support
pub struct GraphOperationsHandle<N, E>
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    graph: RwSignal<Graph<N, E>>,
    undo_stack: RwSignal<Vec<Graph<N, E>>>,
    redo_stack: RwSignal<Vec<Graph<N, E>>>,
}

impl<N, E> GraphOperationsHandle<N, E>
where
    N: Clone + 'static,
    E: Clone + 'static,
{
    /// Execute an operation with undo support
    pub fn execute<F>(&self, operation: F)
    where
        F: FnOnce(&mut Graph<N, E>),
    {
        // Save current state to undo stack
        self.undo_stack.update(|stack| {
            stack.push(self.graph.get());
            // Limit undo stack size
            if stack.len() > 100 {
                stack.remove(0);
            }
        });

        // Clear redo stack
        self.redo_stack.update(|stack| stack.clear());

        // Execute operation
        self.graph.update(operation);
    }

    /// Undo last operation
    pub fn undo(&self) -> bool {
        let mut success = false;
        
        self.undo_stack.update(|undo_stack| {
            if let Some(previous_state) = undo_stack.pop() {
                // Save current state to redo stack
                self.redo_stack.update(|redo_stack| {
                    redo_stack.push(self.graph.get());
                });
                
                // Restore previous state
                self.graph.set(previous_state);
                success = true;
            }
        });

        success
    }

    /// Redo last undone operation
    pub fn redo(&self) -> bool {
        let mut success = false;

        self.redo_stack.update(|redo_stack| {
            if let Some(next_state) = redo_stack.pop() {
                // Save current state to undo stack
                self.undo_stack.update(|undo_stack| {
                    undo_stack.push(self.graph.get());
                });
                
                // Restore next state
                self.graph.set(next_state);
                success = true;
            }
        });

        success
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.get().is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.get().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_flow_core::{NodeBuilder, Size};

    #[test]
    fn test_graph_operations_handle() {
        let graph = create_rw_signal(Graph::new());
        let handle = use_graph_operations(graph);

        // Add a node with undo support
        let node = NodeBuilder::new("test")
            .position(Position::new(0.0, 0.0))
            .size(Size::new(100.0, 50.0))
            .build();

        handle.execute(|g| {
            let _ = g.add_node(node);
        });

        assert_eq!(graph.get().node_count(), 1);
        assert!(handle.can_undo());
        assert!(!handle.can_redo());

        // Undo the operation
        assert!(handle.undo());
        assert_eq!(graph.get().node_count(), 0);
        assert!(!handle.can_undo());
        assert!(handle.can_redo());

        // Redo the operation
        assert!(handle.redo());
        assert_eq!(graph.get().node_count(), 1);
        assert!(handle.can_undo());
        assert!(!handle.can_redo());
    }
}