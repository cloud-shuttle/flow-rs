//! Subflows Demo Example
//!
//! Comprehensive demonstration of hierarchical graph editing with Flow-RS:
//! - Nested graphs (subflows) within nodes
//! - Breadcrumb navigation between hierarchy levels
//! - Visual subflow indicators
//! - Cross-level edge connections
//! - Hierarchical graph management

use flow_rs_leptos::prelude::*;
use leptos::prelude::*;
use flow_rs_core::{Node, Position, HierarchicalGraph};

#[component]
pub fn SubflowsDemo() -> impl IntoView {
    // Create a hierarchical graph with multiple levels
    let hierarchy = create_demo_hierarchy();

    view! {
        <div class="subflows-demo" style="padding: 20px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            <h1 style="color: #2c3e50; text-align: center; margin-bottom: 10px;">"Flow-RS Subflows Demo"</h1>
            <p style="text-align: center; color: #6c757d; margin-bottom: 30px;">
                "Hierarchical graphs with nested subflows - double-click nodes to navigate"
            </p>

            <div class="demo-container" style="border: 3px solid #28a745; border-radius: 12px; overflow: hidden; margin: 20px 0; box-shadow: 0 8px 32px rgba(40, 167, 69, 0.2);">
                <HierarchicalFlowEditor
                    width=900
                    height=650
                    initial_graph=Some(hierarchy)
                />
            </div>

            <div class="features-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-top: 30px;">
                <div class="feature-card" style="background: linear-gradient(135deg, #28a745 0%, #20c997 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"🏗️ Hierarchical Structure"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Nodes can contain entire sub-graphs"</li>
                        <li>"Unlimited nesting depth"</li>
                        <li>"Modular workflow design"</li>
                        <li>"Visual hierarchy indicators"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #007bff 0%, #6610f2 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"🧭 Navigation"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Double-click to enter subflows"</li>
                        <li>"Breadcrumb navigation bar"</li>
                        <li>"Quick navigation to any level"</li>
                        <li>"Visual depth indicators"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #fd7e14 0%, #e83e8c 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"🔗 Cross-Level Connections"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Edges spanning hierarchy levels"</li>
                        <li>"Dashed visual indicators"</li>
                        <li>"Data flow across boundaries"</li>
                        <li>"Automatic edge routing"</li>
                    </ul>
                </div>

                <div class="feature-card" style="background: linear-gradient(135deg, #6f42c1 0%, #e83e8c 100%); color: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);">
                    <h3 style="margin-top: 0; color: #fff;">"📊 Advanced Features"</h3>
                    <ul style="margin: 0; padding-left: 20px;">
                        <li>"Subflow-specific layouts"</li>
                        <li>"Independent viewport states"</li>
                        <li>"Hierarchy statistics"</li>
                        <li>"Bulk operations per level"</li>
                    </ul>
                </div>
            </div>

            <div class="demo-scenarios" style="margin-top: 40px; padding: 25px; background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); border-radius: 12px;">
                <h2 style="color: #2c3e50; margin-top: 0; text-align: center;">"🎯 Demo Scenarios"</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 20px; margin-top: 20px;">
                    <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <h4 style="color: #28a745; margin-top: 0;">"🏢 Enterprise Workflow"</h4>
                        <p style="margin: 8px 0; color: #666;">"Business processes with approval subflows, error handling branches, and escalation paths."</p>
                        <p style="margin: 0; font-size: 14px; color: #888;">"Try: Start → Validate → Check (error path)"</p>
                    </div>
                    <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <h4 style="color: #007bff; margin-top: 0;">"🤖 AI Pipeline"</h4>
                        <p style="margin: 8px 0; color: #666;">"ML workflows with data preprocessing, model training, and evaluation subflows."</p>
                        <p style="margin: 0; font-size: 14px; color: #888;">"Try: Process → Model → Evaluate"</p>
                    </div>
                    <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <h4 style="color: #fd7e14; margin-top: 0;">"🌐 Microservices"</h4>
                        <p style="margin: 8px 0; color: #666;">"Service architectures with API gateways, databases, and monitoring subflows."</p>
                        <p style="margin: 0; font-size: 14px; color: #888;">"Try: Gateway → Auth → Database"</p>
                    </div>
                    <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <h4 style="color: #6f42c1; margin-top: 0;">"🎮 Game Logic"</h4>
                        <p style="margin: 8px 0; color: #666;">"Game state machines with combat, inventory, and dialog subflows."</p>
                        <p style="margin: 0; font-size: 14px; color: #888;">"Try: Game → Combat → Rewards"</p>
                    </div>
                </div>
            </div>

            <div class="tips-section" style="margin-top: 30px; padding: 20px; background: #fff3cd; border: 1px solid #ffeaa7; border-radius: 8px;">
                <h3 style="color: #856404; margin-top: 0;">"💡 Pro Tips"</h3>
                <ul style="color: #856404; margin: 0; padding-left: 20px;">
                    <li>"Double-click any node to enter its subflow (creates one if it doesn't exist)"</li>
                    <li>"Use the breadcrumb bar to quickly jump between hierarchy levels"</li>
                    <li>"Nodes with subflows show a 📁 icon in the bottom-right corner"</li>
                    <li>"Cross-level edges appear as dashed lines connecting different hierarchy levels"</li>
                    <li>"Each level maintains its own layout and can be edited independently"</li>
                    <li>"The hierarchy statistics show total nodes, edges, and depth across all levels"</li>
                </ul>
            </div>
        </div>
    }
}

/// Create a comprehensive demo hierarchy with multiple levels and cross-level connections
fn create_demo_hierarchy() -> HierarchicalGraph<(), ()> {
    let mut hierarchy = HierarchicalGraph::new();

    // Level 1: Main process nodes
    let main_nodes = vec![
        ("start", "Start Process", Position::new(200.0, 100.0)),
        ("validate", "Validate Input", Position::new(400.0, 100.0)),
        ("process", "Process Data", Position::new(600.0, 100.0)),
        ("check", "Decision Point", Position::new(400.0, 300.0)),
        ("success", "Success Path", Position::new(150.0, 500.0)),
        ("error", "Error Handler", Position::new(650.0, 500.0)),
        ("end", "End Process", Position::new(400.0, 650.0)),
    ];

    for (id, label, pos) in main_nodes {
        let node = Node::new(id.to_string(), pos, ());
        hierarchy.root_graph.add_node(node).unwrap();
    }

    // Level 1 edges
    let main_edges = vec![
        ("start", "validate"),
        ("validate", "process"),
        ("process", "check"),
        ("check", "success"),
        ("check", "error"),
        ("success", "end"),
        ("error", "end"),
    ];

    for (source, target) in main_edges {
        let edge = flow_rs_core::Edge::new(
            format!("edge-{}-{}", source, target),
            source.to_string(),
            target.to_string(),
            (),
        );
        hierarchy.root_graph.add_edge(edge).unwrap();
    }

    // Create subflows for key nodes

    // Validate subflow
    hierarchy.create_subflow("validate".into()).unwrap();
    let validate_nodes = vec![
        ("input_check", "Input Format Check", Position::new(100.0, 100.0)),
        ("schema_validate", "Schema Validation", Position::new(300.0, 100.0)),
        ("business_rules", "Business Rules", Position::new(200.0, 250.0)),
    ];

    for (id, label, pos) in validate_nodes {
        let node = Node::new(id.to_string(), pos, ());
        hierarchy.subflows.get_mut(&"validate".into()).unwrap().add_node(node).unwrap();
    }

    // Process subflow
    hierarchy.create_subflow("process".into()).unwrap();
    let process_nodes = vec![
        ("parse", "Parse Data", Position::new(150.0, 100.0)),
        ("transform", "Transform", Position::new(350.0, 100.0)),
        ("enrich", "Enrich Data", Position::new(250.0, 250.0)),
        ("validate_output", "Validate Output", Position::new(250.0, 400.0)),
    ];

    for (id, label, pos) in process_nodes {
        let node = Node::new(id.to_string(), pos, ());
        hierarchy.subflows.get_mut(&"process".into()).unwrap().add_node(node).unwrap();
    }

    // Error handler subflow
    hierarchy.create_subflow("error".into()).unwrap();
    let error_nodes = vec![
        ("log_error", "Log Error", Position::new(150.0, 100.0)),
        ("notify", "Send Notification", Position::new(350.0, 100.0)),
        ("retry", "Retry Logic", Position::new(150.0, 250.0)),
        ("fallback", "Fallback Action", Position::new(350.0, 250.0)),
        ("escalate", "Escalate Issue", Position::new(250.0, 400.0)),
    ];

    for (id, label, pos) in error_nodes {
        let node = Node::new(id.to_string(), pos, ());
        hierarchy.subflows.get_mut(&"error".into()).unwrap().add_node(node).unwrap();
    }

    // Add cross-level edges (these would connect different hierarchy levels)
    // For demo purposes, we'll add some conceptual cross-level connections
    // In a real implementation, these would be created through the UI

    hierarchy
}
