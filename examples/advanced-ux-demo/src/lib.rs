//! Advanced UX Demo Example
//!
//! Comprehensive demonstration of Flow-RS professional editing features:
//! - Context menus with dynamic actions
//! - Keyboard shortcuts for all operations
//! - Undo/Redo with full operation history
//! - Copy/Paste with clipboard management
//! - Bulk operations on selected nodes
//! - Professional editing workflow

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Node, Position, Graph};

#[component]
pub fn AdvancedUxDemo() -> impl IntoView {
    // Create a comprehensive demo graph
    let graph = Graph::new();

    // Create nodes in a structured layout
    let node_configs = vec![
        ("start", "Start Process", "#4CAF50", Position::new(200.0, 100.0)),
        ("validate", "Validate Input", "#2196F3", Position::new(400.0, 100.0)),
        ("process", "Process Data", "#FF9800", Position::new(600.0, 100.0)),
        ("check", "Check Result", "#9C27B0", Position::new(400.0, 250.0)),
        ("success", "Success", "#4CAF50", Position::new(200.0, 400.0)),
        ("error", "Error Handler", "#F44336", Position::new(600.0, 400.0)),
        ("end", "End Process", "#607D8B", Position::new(400.0, 550.0)),
    ];

    for (id, label, color, pos) in node_configs {
        let node = Node::new(id.to_string(), pos, format!("{}\n{}", label, color));
        graph.add_node(node).unwrap();
    }

    // Add edges
    let edges = vec![
        ("start", "validate"),
        ("validate", "process"),
        ("process", "check"),
        ("check", "success"),
        ("check", "error"),
        ("success", "end"),
        ("error", "end"),
    ];

    for (source, target) in edges {
        let edge = flow_rs_core::Edge::new(
            format!("edge-{}-{}", source, target),
            source.to_string(),
            target.to_string(),
            "flow".to_string()
        );
        graph.add_edge(edge).unwrap();
    }

    view! {
        <div class="advanced-ux-demo" style="padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            <h1 style="color: #2c3e50; text-align: center; margin-bottom: 10px;">"Flow-RS Advanced UX Demo"</h1>
            <p style="text-align: center; color: #6c757d; margin-bottom: 30px;">
                "Professional editing features: context menus, keyboard shortcuts, undo/redo, copy/paste"
            </p>

            <div class="demo-container" style="border: 3px solid #007bff; border-radius: 12px; overflow: hidden; margin: 20px 0; box-shadow: 0 8px 32px rgba(0, 123, 255, 0.2);">
                <FlowCanvas
                    width=800
                    height=600
                    enable_selection=true
                    enable_context_menu=true
                    enable_keyboard_shortcuts=true
                    enable_history=true
                />
            </div>

            <div class="features-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-top: 30px;">
                <div class="feature-card" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"🎯 Context Menus"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Right-click nodes for actions"</li>
                        <li>"Right-click canvas for global actions"</li>
                        <li>"Dynamic menus based on selection"</li>
                        <li>"Keyboard shortcut hints"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"⌨️ Keyboard Shortcuts"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li><strong>"Ctrl+Z"</strong>" - Undo"</li>
                        <li><strong>"Ctrl+Y"</strong>" - Redo"</li>
                        <li><strong>"Ctrl+C"</strong>" - Copy"</li>
                        <li><strong>"Ctrl+V"</strong>" - Paste"</li>
                        <li><strong>"Delete"</strong>" - Remove selected"</li>
                        <li><strong>"Ctrl+A"</strong>" - Select all"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"⏪ Undo/Redo"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Full operation history"</li>
                        <li>"Batch complex operations"</li>
                        <li>"Memory-efficient storage"</li>
                        <li>"Non-destructive editing"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"📋 Copy/Paste"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Copy nodes and connections"</li>
                        <li>"Smart positioning on paste"</li>
                        <li>"Preserve relative layout"</li>
                        <li>"Cross-session clipboard"</li>
                    </ul>
                </div>
            </div>

            <div class="workflow-demo" style="margin-top: 40px; padding: 25px; background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%); border-radius: 12px;">
                <h2 style="color: #2c3e50; margin-top: 0; text-align: center;">"🚀 Professional Workflow Demo"</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin-top: 20px;">
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 10px;">🎯</div>
                        <h4 style="color: #34495e; margin: 0;">"Select Multiple Nodes"</h4>
                        <p style="color: #7f8c8d; margin: 5px 0 0 0;">"Click and drag to marquee select, or hold Shift/Ctrl"</p>
                    </div>
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 10px;">📋</div>
                        <h4 style="color: #34495e; margin: 0;">"Copy Selection"</h4>
                        <p style="color: #7f8c8d; margin: 5px 0 0 0;">"Ctrl+C to copy selected nodes and their connections"</p>
                    </div>
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 10px;">📍</div>
                        <h4 style="color: #34495e; margin: 0;">"Paste Anywhere"</h4>
                        <p style="color: #7f8c8d; margin: 5px 0 0 0;">"Right-click and paste, or use Ctrl+V"</p>
                    </div>
                    <div style="text-align: center;">
                        <div style="font-size: 2em; margin-bottom: 10px;">↶</div>
                        <h4 style="color: #34495e; margin: 0;">"Undo Mistakes"</h4>
                        <p style="color: #7f8c8d; margin: 5px 0 0 0;">"Ctrl+Z to undo, Ctrl+Y to redo any operation"</p>
                    </div>
                </div>
            </div>

            <div class="tips-section" style="margin-top: 30px; padding: 20px; background: #fff3cd; border: 1px solid #ffeaa7; border-radius: 8px;">
                <h3 style="color: #856404; margin-top: 0;">"💡 Pro Tips"</h3>
                <ul style="color: #856404; margin: 0;">
                    <li>"Right-click on any node to see context-specific actions"</li>
                    <li>"Use Shift+Click to add nodes to your selection without replacing it"</li>
                    <li>"Ctrl+A selects all nodes, Escape clears the selection"</li>
                    <li>"All operations are undoable - experiment freely!"</li>
                    <li>"Context menus show keyboard shortcuts for quick learning"</li>
                </ul>
            </div>
        </div>
    }
}
