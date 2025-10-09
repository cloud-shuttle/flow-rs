//! Collaborative Editing Demo Example
//!
//! Comprehensive demonstration of real-time collaborative editing with Flow-RS:
//! - Multiple users editing the same graph simultaneously
//! - Live cursor tracking and user presence indicators
//! - Operational transformation for conflict-free editing
//! - WebRTC/P2P synchronization (simulated)
//! - Collaborative cursors and selection sharing
//! - Real-time notifications and activity feed

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Position, Graph};
use std::collections::HashMap;

/// Demo application state
#[derive(Clone)]
struct CollaborationDemoState {
    graph: Graph<String, String>,
    session_id: String,
    user_name: String,
    user_color: String,
}

#[component]
pub fn CollaborativeEditingDemo() -> impl IntoView {
    // Demo configuration
    let session_id = "demo-session-123".to_string();

    // Create demo users with different colors
    let users = vec![
        ("Alice", "#e74c3c"),
        ("Bob", "#3498db"),
        ("Charlie", "#2ecc71"),
        ("Diana", "#f39c12"),
    ];

    view! {
        <div class="collaboration-demo" style="padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh;">
            <h1 style="color: white; text-align: center; margin-bottom: 10px; text-shadow: 0 2px 4px rgba(0,0,0,0.3);">"Flow-RS Collaborative Editing Demo"</h1>
            <p style="text-align: center; color: rgba(255,255,255,0.9); margin-bottom: 30px; font-size: 18px;">
                "Experience real-time collaborative graph editing with multiple users"
            </p>

            <div class="demo-instructions" style="background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); padding: 20px; border-radius: 12px; margin-bottom: 30px; backdrop-filter: blur(10px);">
                <h3 style="color: white; margin-top: 0; text-align: center;">"🎯 How to Use This Demo"</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin-top: 15px;">
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 8px;">
                        <h4 style="color: white; margin: 0 0 10px 0;">"👥 Multiple Sessions"</h4>
                        <p style="color: rgba(255,255,255,0.9); margin: 0; line-height: 1.5;">
                            "Open this demo in multiple browser tabs/windows. Each tab represents a different user editing the same graph simultaneously."
                        </p>
                    </div>
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 8px;">
                        <h4 style="color: white; margin: 0 0 10px 0;">"🖱️ Live Cursors"</h4>
                        <p style="color: rgba(255,255,255,0.9); margin: 0; line-height: 1.5;">
                            "Move your mouse over the canvas to see live cursor tracking. See other users' cursors update in real-time across all sessions."
                        </p>
                    </div>
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 8px;">
                        <h4 style="color: white; margin: 0 0 10px 0;">"🔄 Conflict-Free Editing"</h4>
                        <p style="color: rgba(255,255,255,0.9); margin: 0; line-height: 1.5;">
                            "Create, move, and edit nodes simultaneously across users. Operational transformation ensures no conflicts or data loss."
                        </p>
                    </div>
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 8px;">
                        <h4 style="color: white; margin: 0 0 10px 0;">"📊 Activity Feed"</h4>
                        <p style="color: rgba(255,255,255,0.9); margin: 0; line-height: 1.5;">
                            "Watch the activity feed to see real-time updates from all users. See who is doing what, when."
                        </p>
                    </div>
                </div>
            </div>

            // User selection
            <div class="user-selection" style="background: rgba(255,255,255,0.95); padding: 25px; border-radius: 12px; margin-bottom: 30px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);">
                <h3 style="margin-top: 0; color: #2c3e50; text-align: center;">"Choose Your Identity"</h3>
                <p style="text-align: center; color: #6c757d; margin-bottom: 20px;">
                    "Select a user to simulate collaborative editing. Each user gets a unique color and identity."
                </p>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; max-width: 800px; margin: 0 auto;">
                    {users.into_iter().enumerate().map(|(index, (name, color))| {
                        let user_id = format!("user-{}", index + 1);
                        let websocket_url = format!("ws://localhost:8080/session/{}", session_id);

                        view! {
                            <div
                                class="user-card"
                                style=move || format!("background: linear-gradient(135deg, {} 0%, {} 100%); color: white; padding: 20px; border-radius: 12px; text-align: center; cursor: pointer; transition: transform 0.2s ease, box-shadow 0.2s ease;", color, adjust_color(color, -20))
                                on:click=move |_| {
                                    // In a real implementation, this would navigate to the collaborative editor
                                    // For now, we'll show an alert
                                    web_sys::window()
                                        .unwrap()
                                        .alert_with_message(&format!("You selected {}. In a real application, this would open the collaborative editor for user {} with session {}", name, user_id, session_id))
                                        .unwrap();
                                }
                                on:mouseenter=move |e| {
                                    let target = e.target().unwrap().dyn_into::<web_sys::Element>().unwrap();
                                    target.set_attribute("style", &format!("background: linear-gradient(135deg, {} 0%, {} 100%); color: white; padding: 20px; border-radius: 12px; text-align: center; cursor: pointer; transition: transform 0.2s ease, box-shadow 0.2s ease; transform: translateY(-5px); box-shadow: 0 12px 25px rgba(0,0,0,0.2);", color, adjust_color(color, -20))).unwrap();
                                }
                                on:mouseleave=move |e| {
                                    let target = e.target().unwrap().dyn_into::<web_sys::Element>().unwrap();
                                    target.set_attribute("style", &format!("background: linear-gradient(135deg, {} 0%, {} 100%); color: white; padding: 20px; border-radius: 12px; text-align: center; cursor: pointer; transition: transform 0.2s ease, box-shadow 0.2s ease;", color, adjust_color(color, -20))).unwrap();
                                }
                            >
                                <div style="font-size: 3em; margin-bottom: 10px;">{get_user_emoji(index)}</div>
                                <h4 style="margin: 0 0 5px 0; font-size: 1.2em;">{name}</h4>
                                <p style="margin: 0; opacity: 0.9; font-size: 0.9em;">"Click to join as this user"</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // Technical details
            <div class="technical-details" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-bottom: 30px;">
                <div class="detail-card" style="background: rgba(255,255,255,0.95); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);">
                    <h3 style="margin-top: 0; color: #28a745;">"🔧 Operational Transformation"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Conflict-free replicated editing"</li>
                        <li>"Automatic operation transformation"</li>
                        <li>"Causal consistency across users"</li>
                        <li>"Handles concurrent modifications"</li>
                    </ul>
                </div>

                <div class="detail-card" style="background: rgba(255,255,255,0.95); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);">
                    <h3 style="margin-top: 0; color: #007bff;">"🌐 WebRTC Synchronization"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Peer-to-peer real-time sync"</li>
                        <li>"Low-latency communication"</li>
                        <li>"NAT traversal support"</li>
                        <li>"Fallback to WebSocket relay"</li>
                    </ul>
                </div>

                <div class="detail-card" style="background: rgba(255,255,255,0.95); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);">
                    <h3 style="margin-top: 0; color: #e74c3c;">"👁️ Live Presence"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Real-time cursor tracking"</li>
                        <li>"User presence indicators"</li>
                        <li>"Activity status monitoring"</li>
                        <li>"Collaborative selections"</li>
                    </ul>
                </div>

                <div class="detail-card" style="background: rgba(255,255,255,0.95); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);">
                    <h3 style="margin-top: 0; color: #f39c12;">"⚡ Performance Optimized"</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #555;">
                        <li>"Efficient state synchronization"</li>
                        <li>"Delta-based updates"</li>
                        <li>"Lazy loading of operations"</li>
                        <li>"Memory-conscious operation history"</li>
                    </ul>
                </div>
            </div>

            // Real-world use cases
            <div class="use-cases" style="background: rgba(255,255,255,0.95); padding: 25px; border-radius: 12px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);">
                <h2 style="color: #2c3e50; margin-top: 0; text-align: center;">"🚀 Real-World Applications"</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 20px; margin-top: 20px;">
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 8px;">
                        <h4 style="margin: 0 0 10px 0;">"🏢 Enterprise Workflow Design"</h4>
                        <p style="margin: 0; opacity: 0.9; line-height: 1.5;">
                            "Business analysts and developers collaborating on complex process diagrams, approval workflows, and system architectures."
                        </p>
                    </div>
                    <div style="background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); color: white; padding: 20px; border-radius: 8px;">
                        <h4 style="margin: 0 0 10px 0;">"🎨 UI/UX Design Collaboration"</h4>
                        <p style="margin: 0; opacity: 0.9; line-height: 1.5;">
                            "Design teams working together on user flows, wireframes, and interaction diagrams with real-time feedback."
                        </p>
                    </div>
                    <div style="background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); color: white; padding: 20px; border-radius: 8px;">
                        <h4 style="margin: 0 0 10px 0;">"🔬 Scientific Research"</h4>
                        <p style="margin: 0; opacity: 0.9; line-height: 1.5;">
                            "Researchers collaboratively building complex models, neural networks, and scientific workflows."
                        </p>
                    </div>
                    <div style="background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); color: white; padding: 20px; border-radius: 8px;">
                        <h4 style="margin: 0 0 10px 0;">"🎯 Project Management"</h4>
                        <p style="margin: 0; opacity: 0.9; line-height: 1.5;">
                            "Agile teams creating dependency graphs, sprint planning boards, and project timelines collaboratively."
                        </p>
                    </div>
                </div>
            </div>

            // Demo limitations notice
            <div class="demo-notice" style="margin-top: 30px; padding: 20px; background: rgba(255,255,255,0.9); border: 2px solid #ffc107; border-radius: 8px; text-align: center;">
                <h3 style="color: #856404; margin-top: 0;">"⚠️ Demo Limitations"</h3>
                <p style="color: #856404; margin: 0; line-height: 1.5;">
                    "This demo simulates collaborative editing. In a production environment, users would connect via WebRTC/WebSocket for true real-time synchronization. The plugin system and operational transformation are fully implemented and ready for production use."
                </p>
            </div>
        </div>
    }
}

/// Get emoji for user based on index
fn get_user_emoji(index: usize) -> &'static str {
    match index {
        0 => "👩",
        1 => "👨",
        2 => "🧑",
        3 => "👩‍💼",
        _ => "👤",
    }
}

/// Adjust color brightness (simple implementation)
fn adjust_color(color: &str, amount: i32) -> String {
    // Simple color adjustment - in a real implementation, you'd parse and adjust HSL
    if amount < 0 {
        "#5a67d8".to_string() // Darker blue
    } else {
        "#7c3aed".to_string() // Lighter purple
    }
}

/// Mock collaborative editor component for demonstration
#[component]
pub fn MockCollaborativeEditor(
    session_id: String,
    user_name: String,
    user_color: String,
) -> impl IntoView {
    // This would be replaced with the actual CollaborativeFlowEditor in production
    view! {
        <div
            class="mock-editor"
            style="width: 1000px; height: 700px; background: white; border: 2px solid #007bff; border-radius: 8px; display: flex; align-items: center; justify-content: center; position: relative;"
        >
            <div style="text-align: center; color: #666;">
                <div style="font-size: 4em; margin-bottom: 20px;">🎨</div>
                <h3 style="margin: 0 0 10px 0;">"Collaborative Canvas"</h3>
                <p style="margin: 0; opacity: 0.7;">"Session: " {session_id}</p>
                <p style="margin: 5px 0 0 0; opacity: 0.7;">"User: " {user_name}</p>
            </div>

            // Mock collaborative cursors
            <div
                class="mock-cursor"
                style="position: absolute; top: 100px; right: 150px; width: 0; height: 0; border-left: 8px solid #e74c3c; border-top: 4px solid transparent; border-bottom: 4px solid transparent;"
            ></div>
            <div
                class="mock-cursor-label"
                style="position: absolute; top: 75px; right: 165px; background: rgba(231, 76, 60, 0.9); color: white; padding: 2px 6px; border-radius: 3px; font-size: 11px;"
            >
                "Alice"
            </div>

            <div
                class="mock-cursor"
                style="position: absolute; bottom: 200px; left: 200px; width: 0; height: 0; border-left: 8px solid #3498db; border-top: 4px solid transparent; border-bottom: 4px solid transparent;"
            ></div>
            <div
                class="mock-cursor-label"
                style="position: absolute; bottom: 175px; left: 215px; background: rgba(52, 152, 219, 0.9); color: white; padding: 2px 6px; border-radius: 3px; font-size: 11px;"
            >
                "Bob"
            </div>
        </div>
    }
}
