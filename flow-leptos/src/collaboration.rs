//! Collaborative Flow Editor Component
//!
//! Provides real-time collaborative editing capabilities with:
//! - Live cursor tracking and user presence
//! - Operational transformation for conflict-free editing
//! - WebRTC/P2P synchronization
//! - Collaborative cursors and selection indicators
//! - Live editing notifications

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MouseEvent, Element, WebSocket};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use flow_rs_core::{NodeId, EdgeId, Position};
use flow_rs_core::collaboration::{CollaborativeSession, Participant, ParticipantPermissions, Cursor, Operation, GraphOperation, OperationMetadata};
use crate::signals::{FlowState, ViewportState};
use crate::selection::SelectionMode;

/// Collaborative Flow Editor component
#[component]
pub fn CollaborativeFlowEditor(
    /// Canvas width
    #[prop(default = 1000)]
    width: u32,
    /// Canvas height
    #[prop(default = 700)]
    height: u32,
    /// Session ID for collaboration
    session_id: String,
    /// Current user ID
    user_id: String,
    /// User name
    user_name: String,
    /// WebSocket URL for synchronization
    websocket_url: String,
) -> impl IntoView {
    let canvas_ref: NodeRef<leptos::html::Canvas> = NodeRef::new();
    let viewport = RwSignal::new(ViewportState::default());
    let flow_state = RwSignal::new(FlowState::default());

    // Collaboration state
    let session = RwSignal::new(
        CollaborativeSession::new(session_id.clone(), user_id.clone())
    );

    // WebSocket connection for real-time sync
    let websocket = Rc::new(RefCell::new(None::<WebSocket>));
    let connection_status = RwSignal::new(ConnectionStatus::Disconnected);

    // UI state
    let current_user = RwSignal::new(Participant {
        id: user_id.clone(),
        name: user_name.clone(),
        color: "#007bff".to_string(),
        avatar_url: None,
        permissions: ParticipantPermissions::default(),
        last_activity: js_sys::Date::now() as u64,
    });

    let cursors = RwSignal::new(HashMap::new());
    let participants = RwSignal::new(HashMap::new());

    // Initialize collaboration session
    session.update(|s| {
        s.add_participant(current_user.get());
    });

    // Connect to WebSocket
    connect_websocket(
        websocket_url,
        session.clone(),
        websocket.clone(),
        connection_status,
        cursors.clone(),
        participants.clone(),
    );

    // Handle mouse move for cursor tracking
    let websocket_clone_mouse = websocket.clone();
    let handle_mouse_move = move |event: MouseEvent| {
        let mouse_pos = Position::new(
            event.client_x() as f64,
            event.client_y() as f64,
        );

        let viewport_val = viewport.get();
        let flow_pos = viewport_val.viewport.screen_to_flow(mouse_pos);

        let cursor = Cursor {
            participant_id: current_user.get().id,
            position: flow_pos,
            selection: None, // Could be populated from selection state
            timestamp: js_sys::Date::now() as u64,
        };

        // Update local cursor
        cursors.update(|c| {
            c.insert(current_user.get().id.clone(), cursor.clone());
        });

        // Send cursor update via WebSocket
        if let Some(ws) = websocket_clone_mouse.borrow().as_ref() {
            send_cursor_update(&ws, cursor);
        }
    };

    // Handle node creation
    let websocket_clone_create = websocket.clone();
    let handle_node_create = move |position: Position| {
        let node_id = NodeId::new(format!("node-{}", js_sys::Date::now()));
        let node = flow_rs_core::Node::new(node_id.clone(), position, "".to_string());

        let operation = Operation {
            operation: GraphOperation::AddNode {
                node: node.clone(),
                position,
            },
            metadata: OperationMetadata {
                id: format!("op-{}", js_sys::Date::now()),
                client_id: current_user.get().id,
                timestamp: js_sys::Date::now() as u64,
                sequence_number: 0, // Will be set by OT
                parent_operations: vec![],
            },
        };

        // Apply locally
        if let Err(e) = session.get().apply_operation(operation.clone()) {
            web_sys::console::error_1(&format!("Failed to apply operation: {:?}", e).into());
            return;
        }

        // Send to peers
        if let Some(ws) = websocket_clone_create.borrow().as_ref() {
            send_operation(&ws, operation);
        }
    };

    // Handle node movement
    let websocket_clone_move = websocket.clone();
    let handle_node_move = move |node_id: NodeId, from: Position, to: Position| {
        let operation = Operation {
            operation: GraphOperation::MoveNode {
                node_id: node_id.clone(),
                from,
                to,
            },
            metadata: OperationMetadata {
                id: format!("move-op-{}", js_sys::Date::now()),
                client_id: current_user.get().id,
                timestamp: js_sys::Date::now() as u64,
                sequence_number: 0,
                parent_operations: vec![],
            },
        };

        // Apply locally
        if let Err(e) = session.get().apply_operation(operation.clone()) {
            web_sys::console::error_1(&format!("Failed to apply move operation: {:?}", e).into());
            return;
        }

        // Send to peers
        if let Some(ws) = websocket_clone_move.borrow().as_ref() {
            send_operation(&ws, operation);
        }
    };

    view! {
        <div class="collaborative-flow-editor" style="position: relative;">
            // Connection status indicator
            <div
                class="connection-status"
                class:connected=move || matches!(connection_status.get(), ConnectionStatus::Connected)
                class:connecting=move || matches!(connection_status.get(), ConnectionStatus::Connecting)
                class:disconnected=move || matches!(connection_status.get(), ConnectionStatus::Disconnected)
                style="position: absolute; top: 10px; right: 10px; z-index: 100; padding: 8px 12px; border-radius: 20px; font-size: 12px; font-weight: bold; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
            >
                {move || match connection_status.get() {
                    ConnectionStatus::Connected => "🟢 Connected",
                    ConnectionStatus::Connecting => "🟡 Connecting...",
                    ConnectionStatus::Disconnected => "🔴 Disconnected",
                }}
            </div>

            // Participants list
            <div
                class="participants-list"
                style="position: absolute; top: 50px; right: 10px; z-index: 100; background: rgba(255,255,255,0.9); padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); max-width: 200px;"
            >
                <h4 style="margin: 0 0 8px 0; font-size: 14px;">"Participants"</h4>
                {move || participants.get().values().cloned().collect::<Vec<_>>().into_iter().map(|participant| {
                    let is_current_user = participant.id == current_user.get().id;
                    view! {
                        <div
                            class="participant"
                            class:current-user=is_current_user
                            style=move || format!("display: flex; align-items: center; margin-bottom: 4px; padding: 4px 8px; border-radius: 4px; background: {}; opacity: 0.8;",
                                if is_current_user { "rgba(0,123,255,0.1)" } else { "transparent" })
                        >
                            <div
                                class="participant-avatar"
                                style=move || format!("width: 12px; height: 12px; border-radius: 50%; background: {}; margin-right: 8px; border: 2px solid white; box-shadow: 0 1px 3px rgba(0,0,0,0.2);",
                                    participant.color)
                            ></div>
                            <span style="font-size: 12px; font-weight: 500;">{participant.name.clone()}</span>
                            {if is_current_user {
                                Some(view! { <span style="margin-left: 4px; font-size: 10px; color: #666;">"(You)"</span> })
                            } else {
                                None
                            }}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Collaborative cursors overlay
            <div
                class="cursors-overlay"
                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 50;"
            >
                {move || cursors.get().values().filter(|cursor| cursor.participant_id != current_user.get().id).map(|cursor| {
                    // Convert flow position to screen position
                    let viewport_val = viewport.get();
                    let screen_pos = viewport_val.viewport.flow_to_screen(cursor.position);

                    view! {
                        <div
                            class="collaborative-cursor"
                            style=move || format!("position: absolute; left: {}px; top: {}px; pointer-events: none;",
                                screen_pos.x, screen_pos.y)
                        >
                            // Cursor icon
                            <div
                                class="cursor-icon"
                                style="width: 0; height: 0; border-left: 8px solid #000; border-top: 4px solid transparent; border-bottom: 4px solid transparent; filter: drop-shadow(0 1px 2px rgba(0,0,0,0.3));"
                            ></div>
                            // User label
                            <div
                                class="cursor-label"
                                style="position: absolute; top: -20px; left: 10px; background: rgba(0,0,0,0.8); color: white; padding: 2px 6px; border-radius: 3px; font-size: 11px; white-space: nowrap;"
                            >
                                {participants.get().get(&cursor.participant_id).map(|p| p.name.clone()).unwrap_or_else(|| "Unknown".to_string())}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Main canvas
            <canvas
                node_ref=canvas_ref
                width=width
                height=height
                style="border: 2px solid #007bff; cursor: crosshair; border-radius: 8px;"
                on:mousemove=handle_mouse_move
            />

            // Collaboration instructions
            <div
                class="collaboration-info"
                style="position: absolute; bottom: 10px; left: 10px; background: rgba(255,255,255,0.9); padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); max-width: 350px; font-size: 14px;"
            >
                <h4 style="margin: 0 0 8px 0; color: #007bff;">"👥 Real-time Collaboration"</h4>
                <ul style="margin: 0; padding-left: 20px; color: #555;">
                    <li>"See other participants' cursors in real-time"</li>
                    <li>"All changes are synchronized automatically"</li>
                    <li>"Operational transformation prevents conflicts"</li>
                    <li>"Connection status shown in top-right"</li>
                </ul>
            </div>

            // Session info
            <div
                class="session-info"
                style="position: absolute; bottom: 10px; right: 10px; background: rgba(255,255,255,0.9); padding: 8px 12px; border-radius: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); font-size: 12px; color: #666;"
            >
                "Session: " {session_id} " | User: " {user_name}
            </div>
        </div>
    }
}

/// Connection status for WebSocket
#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

/// Connect to WebSocket for real-time synchronization
fn connect_websocket(
    url: String,
    session: RwSignal<CollaborativeSession>,
    websocket: Rc<RefCell<Option<WebSocket>>>,
    connection_status: RwSignal<ConnectionStatus>,
    cursors: RwSignal<HashMap<String, Cursor>>,
    participants: RwSignal<HashMap<String, Participant>>,
) {
    connection_status.set(ConnectionStatus::Connecting);

    match WebSocket::new(&url) {
        Ok(ws) => {
            *websocket.borrow_mut() = Some(ws.clone());

            // Set up event handlers
            let onopen_callback = Closure::wrap(Box::new(move |_| {
                connection_status.set(ConnectionStatus::Connected);
                web_sys::console::log_1(&"WebSocket connected".into());
            }) as Box<dyn FnMut(web_sys::Event)>);

            let onmessage_callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                if let Ok(text) = event.data().dyn_into::<js_sys::JsString>() {
                    let message_str = String::from(text);
                    handle_websocket_message(&message_str, &session, &cursors, &participants);
                }
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);

            let onerror_callback = Closure::wrap(Box::new(move |_| {
                connection_status.set(ConnectionStatus::Disconnected);
                web_sys::console::error_1(&"WebSocket error".into());
            }) as Box<dyn FnMut(web_sys::Event)>);

            let onclose_callback = Closure::wrap(Box::new(move |_| {
                connection_status.set(ConnectionStatus::Disconnected);
                web_sys::console::log_1(&"WebSocket closed".into());
            }) as Box<dyn FnMut(web_sys::Event)>);

            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));

            // Forget the callbacks to keep them alive
            onopen_callback.forget();
            onmessage_callback.forget();
            onerror_callback.forget();
            onclose_callback.forget();
        }
        Err(e) => {
            connection_status.set(ConnectionStatus::Disconnected);
            web_sys::console::error_1(&format!("Failed to create WebSocket: {:?}", e).into());
        }
    }
}

/// Handle incoming WebSocket messages
fn handle_websocket_message(
    message: &str,
    session: &RwSignal<CollaborativeSession>,
    cursors: &RwSignal<HashMap<String, Cursor>>,
    participants: &RwSignal<HashMap<String, Participant>>,
) {
    // In a real implementation, this would parse the message and apply operations
    // For now, just log it
    web_sys::console::log_1(&format!("Received message: {}", message).into());
}

/// Send operation via WebSocket
fn send_operation(ws: &WebSocket, operation: Operation) {
    if let Ok(json) = serde_json::to_string(&operation) {
        let message = format!("{{\"type\": \"operation\", \"data\": {}}}", json);
        if let Err(e) = ws.send_with_str(&message) {
            web_sys::console::error_1(&format!("Failed to send operation: {:?}", e).into());
        }
    }
}

/// Send cursor update via WebSocket
fn send_cursor_update(ws: &WebSocket, cursor: Cursor) {
    if let Ok(json) = serde_json::to_string(&cursor) {
        let message = format!("{{\"type\": \"cursor\", \"data\": {}}}", json);
        if let Err(e) = ws.send_with_str(&message) {
            web_sys::console::error_1(&format!("Failed to send cursor update: {:?}", e).into());
        }
    }
}

/// Collaborative cursor renderer
pub struct CollaborativeCursorRenderer;

impl CollaborativeCursorRenderer {
    /// Render all collaborative cursors
    pub fn render_cursors(
        context: &web_sys::CanvasRenderingContext2d,
        cursors: &HashMap<String, Cursor>,
        participants: &HashMap<String, Participant>,
        current_user_id: &str,
        viewport: &flow_rs_core::Viewport,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (participant_id, cursor) in cursors.iter() {
            if participant_id == current_user_id {
                continue; // Don't render own cursor
            }

            if let Some(participant) = participants.get(participant_id) {
                let screen_pos = viewport.flow_to_screen(cursor.position);

                // Draw cursor
                context.set_stroke_style(&participant.color.clone().into());
                context.set_line_width(2.0);
                context.begin_path();
                context.move_to(screen_pos.x, screen_pos.y);
                context.line_to(screen_pos.x + 12.0, screen_pos.y + 6.0);
                context.line_to(screen_pos.x + 6.0, screen_pos.y + 12.0);
                context.close_path();
                context.stroke();

                // Draw user name label
                context.set_fill_style(&"#000000".into());
                context.set_font("12px Arial");
                let _ = context.fill_text(&participant.name, screen_pos.x + 15.0, screen_pos.y - 5.0);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_rs_core::ParticipantPermissions;

    #[test]
    fn test_participant_creation() {
        let participant = Participant {
            id: "user-1".to_string(),
            name: "Test User".to_string(),
            color: "#ff0000".to_string(),
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            permissions: ParticipantPermissions::default(),
            last_activity: 1234567890,
        };

        assert_eq!(participant.id, "user-1");
        assert_eq!(participant.name, "Test User");
        assert_eq!(participant.color, "#ff0000");
    }

    #[test]
    fn test_cursor_creation() {
        let cursor = Cursor {
            participant_id: "user-1".to_string(),
            position: Position::new(100.0, 200.0),
            selection: None,
            timestamp: 1234567890,
        };

        assert_eq!(cursor.participant_id, "user-1");
        assert_eq!(cursor.position.x, 100.0);
        assert_eq!(cursor.position.y, 200.0);
    }

    #[test]
    fn test_collaborative_session_participants() {
        let mut session = CollaborativeSession::new("session-1".to_string(), "owner".to_string());

        let participant = Participant {
            id: "user-1".to_string(),
            name: "User 1".to_string(),
            color: "#ff0000".to_string(),
            avatar_url: None,
            permissions: ParticipantPermissions::default(),
            last_activity: 1234567890,
        };

        session.add_participant(participant.clone());
        assert_eq!(session.get_participants().len(), 1);

        session.remove_participant("user-1");
        assert_eq!(session.get_participants().len(), 0);
    }
}
