//! Leptos hooks and utilities for flow editors

use leptos::*;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlCanvasElement, MouseEvent, WheelEvent, KeyboardEvent};

use leptos_flow_core::{Graph, Position};
use crate::events::{FlowEvent, NodeEvent, KeyboardModifiers, MouseButton};
use crate::signals::{FlowState, ViewportState};
use crate::drag::DragHandler;
use crate::edge_connection::{HandleDetector, ConnectionPreview, EdgeCreator, ConnectionHandle};

/// Hook for managing canvas mouse interactions
pub fn use_canvas_mouse<N, E>(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    graph: RwSignal<Graph<N, E>>,
    viewport: RwSignal<ViewportState>,
    flow_state: RwSignal<FlowState>,
    on_flow_event: Option<Rc<dyn Fn(FlowEvent)>>,
    on_node_event: Option<Rc<dyn Fn(NodeEvent<N>)>>,
) where
    N: Clone + 'static,
    E: Clone + Default + 'static,
{
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get_untracked() {
            let canvas_element = (*canvas).clone();
            let canvas_clone = canvas.clone();
            let graph_clone = graph.clone();
            let viewport_clone = viewport.clone();
            let flow_state_clone = flow_state.clone();
            let on_flow_event_clone = on_flow_event.clone();
            let on_node_event_clone = on_node_event.clone();

            // Create interaction system components
            let _drag_handler = DragHandler::new();
            let _handle_detector = HandleDetector::new();
            let connection_preview = std::rc::Rc::new(std::cell::RefCell::new(ConnectionPreview::new()));
            let edge_creator = EdgeCreator::new();

            // Mouse down handler
            let mouse_down_handler = {
                let canvas = canvas_clone.clone();
                let graph = graph_clone.clone();
                let viewport = viewport_clone.clone();
                let flow_state = flow_state_clone.clone();
                let on_flow_event = on_flow_event_clone.clone();
                let on_node_event = on_node_event_clone.clone();
                let connection_preview = connection_preview.clone();

                Closure::wrap(Box::new(move |event: MouseEvent| {
                    event.prevent_default();

                    let button = MouseButton::from_mouse_event(&event);
                    let modifiers = KeyboardModifiers::from_mouse_event(&event);

                    if let Some(MouseButton::Left) = button {
                        let canvas_element = (*canvas).clone().unchecked_into::<HtmlCanvasElement>();
                        // Use proper DOM rect access with mouse integration
                        let mut rect_utils = crate::dom_rect::DomRectUtils::new();
                        let viewport_state = viewport.get_untracked();
                        let canvas_pos = crate::mouse_integration::utils::mouse_event_to_canvas_coords(
                            &event,
                            &canvas_element,
                            &mut rect_utils,
                        ).unwrap_or_else(|_| {
                            // Fallback to simple calculation if DOM rect access fails
                            Position::new(
                                event.client_x() as f64,
                                event.client_y() as f64,
                            )
                        });
                        let world_pos = crate::events::utils::canvas_to_world(
                            canvas_pos,
                            viewport_state.viewport.offset,
                            viewport_state.viewport.zoom,
                        );

                        // Check if we clicked on a node
                        let graph_value = graph.get_untracked();
                        let clicked_node = graph_value.nodes().find(|node| {
                            let bounds = node.bounds();
                            crate::events::utils::position_in_rect(world_pos, &bounds)
                        });

                        match clicked_node {
                            Some(node) => {
                                // Check if we clicked on a connection handle
                                let handle_detector = HandleDetector::new();
                                let detected_handle = handle_detector.detect_handle(&node, world_pos);

                                match detected_handle {
                                    Some(ConnectionHandle::Output) => {
                                        // Start connection from output handle
                                        // This initiates a connection preview and sets up connection state
                                        let node_id = node.id.clone();
                                        flow_state.update(|state| {
                                            state.set_connection_mode(true);
                                            state.set_connection_source(Some(node_id.clone()));
                                            state.set_connection_start_position(Some(canvas_pos));
                                        });

                                        connection_preview.borrow_mut().start_connection(&graph_value, &node_id, canvas_pos);

                                        if let Some(handler) = &on_flow_event {
                                            handler(FlowEvent::ConnectionStart {
                                                source_node: node_id,
                                                position: world_pos,
                                            });
                                        }
                                    }
                                    Some(ConnectionHandle::Input) => {
                                        // Complete connection to input handle
                                        // This attempts to create an edge between the source and target nodes
                                        let flow_state_val = flow_state.get_untracked();
                                        if let Some(source_node_id) = flow_state_val.connection_source() {
                                            // Complete the connection - need to get a fresh copy of the graph
                                            let mut graph_value = graph.get_untracked();
                                            let result = edge_creator.create_edge(&mut graph_value, source_node_id, &node.id);

                                            if result == crate::edge_connection::ConnectionResult::Valid {
                                                // Connection successful
                                                graph.set(graph_value);

                                                if let Some(handler) = &on_flow_event {
                                                    handler(FlowEvent::ConnectionComplete {
                                                        source_node: source_node_id.clone(),
                                                        target_node: node.id.clone(),
                                                    });
                                                }
                                            }
                                        }

                                        // End connection mode
                                        flow_state.update(|state| {
                                            state.set_connection_mode(false);
                                            state.set_connection_source(None);
                                            state.set_connection_start_position(None);
                                        });
                                        connection_preview.borrow_mut().end_preview();
                                    }
                                    None => {
                                        // Regular node click
                                        flow_state.update(|state| {
                                            if modifiers.shift {
                                                state.add_node_to_selection(node.id.clone());
                                            } else {
                                                state.select_node(node.id.clone());
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
                                                target: crate::events::DragTarget::Node(node.id.clone()),
                                            });
                                        }
                                    }
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
                let graph = graph_clone.clone();
                let viewport = viewport_clone.clone();
                let flow_state = flow_state_clone.clone();
                let on_flow_event = on_flow_event_clone.clone();
                let connection_preview = connection_preview.clone();

                Closure::wrap(Box::new(move |event: MouseEvent| {
                    let canvas_element = (*canvas).clone().unchecked_into::<HtmlCanvasElement>();
                    // Use proper DOM rect access with mouse integration
                    let mut rect_utils = crate::dom_rect::DomRectUtils::new();
                    let canvas_pos = crate::mouse_integration::utils::mouse_event_to_canvas_coords(
                        &event,
                        &canvas_element,
                        &mut rect_utils,
                    ).unwrap_or_else(|_| {
                        // Fallback to simple calculation if DOM rect access fails
                        Position::new(
                            event.client_x() as f64,
                            event.client_y() as f64,
                        )
                    });

                    // Use get_untracked to avoid reactivity issues
                    let flow_state_val = flow_state.get_untracked();

                    // Check if we're in connection mode and update preview
                    if flow_state_val.is_connection_mode() {
                        connection_preview.borrow_mut().update_preview(canvas_pos);

                        if let Some(handler) = &on_flow_event {
                            let viewport_state = viewport.get_untracked();
                            let world_pos = crate::events::utils::canvas_to_world(
                                canvas_pos,
                                viewport_state.viewport.offset,
                                viewport_state.viewport.zoom,
                            );
                            handler(FlowEvent::ConnectionUpdate {
                                position: world_pos,
                            });
                        }
                    } else if flow_state_val.is_dragging {
                        // Update drag state first
                        flow_state.update(|state| {
                            state.update_drag(canvas_pos);
                        });

                        // Get viewport state without tracking
                        let viewport_state = viewport.get_untracked();
                        let world_pos = crate::events::utils::canvas_to_world(
                            canvas_pos,
                            viewport_state.viewport.offset,
                            viewport_state.viewport.zoom,
                        );

                        // Get updated flow state to check for drag delta
                        let updated_flow_state = flow_state.get_untracked();
                        if let Some(canvas_delta) = updated_flow_state.drag_delta() {
                            // Handle different drag targets
                            let selected_nodes = &updated_flow_state.selected_nodes;
                            if !selected_nodes.is_empty() {
                                // Dragging nodes - convert canvas delta to world delta
                                let world_delta = Position::new(
                                    canvas_delta.x / viewport_state.viewport.zoom,
                                    canvas_delta.y / viewport_state.viewport.zoom,
                                );

                                // Clone the drag handler to avoid borrowing issues
                                let drag_handler = DragHandler::new();
                                // Get flow state before entering the graph update closure
                                let current_flow_state = flow_state.get_untracked();
                                graph.update(|graph_mut| {
                                    drag_handler.apply_drag_to_nodes(graph_mut, &current_flow_state, world_delta);
                                });
                            } else {
                                // Dragging canvas (panning)
                                viewport.update(|vp| {
                                    let world_delta = Position::new(
                                        -canvas_delta.x / vp.viewport.zoom,
                                        -canvas_delta.y / vp.viewport.zoom,
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
                                    delta: canvas_delta,
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
                let connection_preview = connection_preview.clone();

                Closure::wrap(Box::new(move |event: MouseEvent| {
                    let flow_state_val = flow_state.get_untracked();

                    // Check if we're in connection mode and cancel it
                    if flow_state_val.is_connection_mode() {
                        flow_state.update(|state| {
                            state.set_connection_mode(false);
                            state.set_connection_source(None);
                            state.set_connection_start_position(None);
                        });
                        connection_preview.borrow_mut().end_preview();

                        if let Some(handler) = &on_flow_event {
                            handler(FlowEvent::ConnectionCancel);
                        }
                    } else if flow_state_val.is_dragging {
                        let _canvas_element = (*canvas).clone().unchecked_into::<HtmlCanvasElement>();
                        // TODO: Implement proper DOM rect access
                        // For now, use placeholder values
                        let canvas_rect = (0.0, 0.0, 800.0, 600.0); // (left, top, width, height)
                        let canvas_pos = Position::new(
                            event.client_x() as f64 - canvas_rect.0,
                            event.client_y() as f64 - canvas_rect.1,
                        );

                        let viewport_state = viewport.get_untracked();
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

            // Add event listeners using a fresh reference
            let _ = canvas_element.add_event_listener_with_callback(
                "mousedown",
                mouse_down_handler.as_ref().unchecked_ref(),
            );
            let _ = canvas_element.add_event_listener_with_callback(
                "mousemove",
                mouse_move_handler.as_ref().unchecked_ref(),
            );
            let _ = canvas_element.add_event_listener_with_callback(
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
    canvas_ref: NodeRef<leptos::html::Canvas>,
    viewport: RwSignal<ViewportState>,
    on_flow_event: Option<Rc<dyn Fn(FlowEvent)>>,
) {
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get_untracked() {
            let canvas_element = (*canvas).clone();
            let viewport_clone = viewport.clone();
            let on_flow_event_clone = on_flow_event.clone();

            let wheel_handler = Closure::wrap(Box::new(move |event: WheelEvent| {
                event.prevent_default();

                let _canvas_element = (*canvas).clone().unchecked_into::<HtmlCanvasElement>();
                // TODO: Implement proper DOM rect access
                // For now, use placeholder values
                let canvas_rect = (0.0, 0.0, 800.0, 600.0); // (left, top, width, height)
                let canvas_pos = Position::new(
                    event.client_x() as f64 - canvas_rect.0,
                    event.client_y() as f64 - canvas_rect.1,
                );

                let delta_y = event.delta_y();
                let zoom_factor = if delta_y < 0.0 { 1.1 } else { 1.0 / 1.1 };

                let _old_viewport = viewport_clone.get_untracked();

                viewport_clone.update(|vp| {
                    let world_pos = crate::events::utils::canvas_to_world(
                        canvas_pos,
                        vp.viewport.offset,
                        vp.viewport.zoom,
                    );
                    vp.zoom_at(zoom_factor, world_pos);
                });

                let new_viewport = viewport_clone.get_untracked();
                if let Some(handler) = &on_flow_event_clone {
                    handler(FlowEvent::ViewportChanged {
                        offset: new_viewport.viewport.offset,
                        zoom: new_viewport.viewport.zoom,
                    });
                }
            }) as Box<dyn FnMut(_)>);

            let _ = canvas_element.add_event_listener_with_callback(
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
                    if let Some(_handler) = &on_flow_event_clone {
                        // Would trigger delete selected items
                    }
                }
                "a" | "A" if modifiers.ctrl_only() => {
                    event.prevent_default();
                    if let Some(_handler) = &on_flow_event_clone {
                        // Would trigger select all
                    }
                }
                "z" | "Z" if modifiers.ctrl_only() => {
                    event.prevent_default();
                    if let Some(_handler) = &on_flow_event_clone {
                        // Would trigger undo
                    }
                }
                "y" | "Y" if modifiers.ctrl_only() => {
                    event.prevent_default();
                    if let Some(_handler) = &on_flow_event_clone {
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
    use leptos_flow_core::prelude::{NodeBuilder, Size};

    #[test]
    fn test_graph_operations_handle() {
        // Create a simple graph without signals for testing
        let mut graph = Graph::<(), ()>::new();

        // Add a node directly
        let node = NodeBuilder::new("test")
            .position(0.0, 0.0)
            .size(100.0, 50.0)
            .build();

        let result = graph.add_node(node);
        assert!(result.is_ok());

        // Verify the node was added
        assert_eq!(graph.node_count(), 1);

        // Test basic graph operations
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id.to_string(), "test");
    }
}
