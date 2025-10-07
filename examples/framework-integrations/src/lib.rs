//! Framework Integrations Example
//!
//! Demonstrates integration patterns for Flow-RS with popular Rust web frameworks.
//! This example shows how to use Flow-RS components within different framework ecosystems,
//! providing reusable patterns and best practices for framework integration.
//!
//! Frameworks Covered:
//! - Leptos: Reactive signals and component integration
//! - Yew: Component-based architecture with function components
//! - Dioxus: Virtual DOM with hooks-based state management
//! - Sycamore: Fine-grained reactivity with context system
//! - Generic: Framework-agnostic integration patterns

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Integration patterns and examples
#[derive(Clone, Debug, Serialize, Deserialize)]
struct IntegrationExample {
    pub framework: String,
    pub pattern: String,
    pub description: String,
    pub code_example: String,
    pub complexity: String,
}

// Framework-specific integration guides
#[derive(Clone, Debug)]
struct FrameworkGuide {
    pub name: String,
    pub setup_steps: Vec<String>,
    pub integration_pattern: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub best_use_case: String,
}

// Global state for demo
static mut CURRENT_FRAMEWORK: Option<String> = None;
static mut INTEGRATION_EXAMPLES: Option<Vec<IntegrationExample>> = None;
static mut FRAMEWORK_GUIDES: Option<Vec<FrameworkGuide>> = None;

// This is the main entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"🔧 Flow-RS Framework Integrations Demo Started!".into());

    // Initialize integration examples and guides
    initialize_integration_data();

    // Demonstrate framework integrations
    demonstrate_integrations();

    web_sys::console::log_1(&"📚 Framework Integration Examples Available!".into());
    web_sys::console::log_1(&"🎯 Try these commands:".into());
    web_sys::console::log_1(&"   • show_framework_guide('leptos') - Show Leptos integration guide".into());
    web_sys::console::log_1(&"   • show_integration_pattern('reactive') - Show reactive pattern example".into());
    web_sys::console::log_1(&"   • compare_frameworks() - Compare all framework integrations".into());
    web_sys::console::log_1(&"   • generate_integration_code('yew') - Generate Yew integration code".into());
}

/// Initialize integration examples and framework guides
fn initialize_integration_data() {
    unsafe {
        // Integration examples
        INTEGRATION_EXAMPLES = Some(vec![
            IntegrationExample {
                framework: "Leptos".to_string(),
                pattern: "Reactive Signals".to_string(),
                description: "Using Leptos signals for reactive Flow-RS state management".to_string(),
                complexity: "Medium".to_string(),
                code_example: r#"
// Leptos Integration Example
#[component]
pub fn FlowEditor() -> impl IntoView {
    let (graph, set_graph) = create_signal(Graph::new());

    // Reactive graph updates
    let add_node = move |_| {
        set_graph.update(|g| {
            let node = Node::new(
                format!("node-{}", g.nodes().count()),
                Position::new(100.0, 100.0),
                "New Node".to_string()
            );
            g.add_node(node).unwrap();
        });
    };

    view! {
        <div class="flow-editor">
            <button on:click=add_node>"Add Node"</button>
            <FlowCanvas graph=graph.get()/>
        </div>
    }
}
"#.to_string(),
            },
            IntegrationExample {
                framework: "Yew".to_string(),
                pattern: "Component Props".to_string(),
                description: "Passing Flow-RS graphs as Yew component properties".to_string(),
                complexity: "Low".to_string(),
                code_example: r#"
// Yew Integration Example
#[derive(Clone, Properties, PartialEq)]
pub struct FlowProps {
    pub graph: Graph<String, String>,
    pub on_node_click: Callback<String>,
}

#[function_component(FlowEditor)]
pub fn flow_editor(props: &FlowProps) -> Html {
    let graph = &props.graph;

    html! {
        <div class="flow-editor">
            <canvas ref={canvas_ref} />
            {for graph.nodes().map(|node| {
                html! {
                    <FlowNode
                        node={node.clone()}
                        on_click={props.on_node_click.clone()}
                    />
                }
            })}
        </div>
    }
}
"#.to_string(),
            },
            IntegrationExample {
                framework: "Dioxus".to_string(),
                pattern: "Hooks-based State".to_string(),
                description: "Using Dioxus hooks for Flow-RS state management".to_string(),
                complexity: "Medium".to_string(),
                code_example: r#"
// Dioxus Integration Example
#[allow(non_snake_case)]
pub fn FlowEditor(cx: Scope) -> Element {
    let graph = use_state(cx, || Graph::<String, String>::new());

    let add_node = move |_| {
        graph.modify(|g| {
            let node = Node::new(
                format!("node-{}", g.nodes().count()),
                Position::new(100.0, 100.0),
                "New Node".to_string()
            );
            g.add_node(node).unwrap();
        });
    };

    cx.render(rsx! {
        div { class: "flow-editor",
            button { onclick: add_node, "Add Node" }
            FlowCanvas { graph: graph.get() }
        }
    })
}
"#.to_string(),
            },
            IntegrationExample {
                framework: "Sycamore".to_string(),
                pattern: "Context System".to_string(),
                description: "Using Sycamore's context system for shared Flow-RS state".to_string(),
                complexity: "High".to_string(),
                code_example: r#"
// Sycamore Integration Example
#[component]
pub fn FlowEditor<G: Html>(cx: Scope) -> View<G> {
    let graph = create_signal(cx, Graph::<String, String>::new());

    // Provide graph context to child components
    provide_context(cx, graph);

    let add_node = move |_| {
        graph.update(|g| {
            let node = Node::new(
                format!("node-{}", g.nodes().count()),
                Position::new(100.0, 100.0),
                "New Node".to_string()
            );
            g.add_node(node).unwrap();
        });
    };

    view! { cx,
        div(class="flow-editor") {
            button(on:click=add_node) { "Add Node" }
            FlowCanvas {}
        }
    }
}
"#.to_string(),
            },
            IntegrationExample {
                framework: "Generic".to_string(),
                pattern: "Vanilla Integration".to_string(),
                description: "Framework-agnostic integration using direct DOM manipulation".to_string(),
                complexity: "Low".to_string(),
                code_example: r#"
// Generic Integration Example
pub struct FlowComponent {
    canvas: HtmlCanvasElement,
    graph: Graph<String, String>,
}

impl FlowComponent {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let canvas = document()
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        Ok(Self {
            canvas,
            graph: Graph::new(),
        })
    }

    pub fn add_node(&mut self, label: &str, x: f64, y: f64) {
        let node = Node::new(
            format!("node-{}", self.graph.nodes().count()),
            Position::new(x, y),
            label.to_string()
        );
        self.graph.add_node(node).unwrap();
        self.render();
    }

    pub fn render(&self) {
        // Direct canvas rendering without framework
        let context = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // Clear and render graph
        context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);

        for node in self.graph.nodes() {
            // Render node as circle
            context.begin_path();
            context.arc(node.position.x, node.position.y, 20.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.set_fill_style(&JsValue::from_str(\"blue\"));
            context.fill();
        }
    }
}
"#.to_string(),
            },
        ]);

        // Framework guides
        FRAMEWORK_GUIDES = Some(vec![
            FrameworkGuide {
                name: "Leptos".to_string(),
                setup_steps: vec![
                    "Add leptos dependency to Cargo.toml".to_string(),
                    "Enable 'csr' feature for client-side rendering".to_string(),
                    "Import Flow-RS components in your Leptos components".to_string(),
                    "Use Leptos signals for reactive graph state".to_string(),
                ],
                integration_pattern: "Reactive Components".to_string(),
                pros: vec![
                    "Excellent reactivity with signals".to_string(),
                    "Type-safe component composition".to_string(),
                    "Fine-grained updates".to_string(),
                    "Great developer experience".to_string(),
                ],
                cons: vec![
                    "Learning curve for signals".to_string(),
                    "More complex than simple components".to_string(),
                ],
                best_use_case: "Complex reactive applications requiring fine-grained updates".to_string(),
            },
            FrameworkGuide {
                name: "Yew".to_string(),
                setup_steps: vec![
                    "Add yew dependency with 'csr' feature".to_string(),
                    "Create Yew components that wrap Flow-RS functionality".to_string(),
                    "Use Yew's Props system for component communication".to_string(),
                    "Handle events through Yew's event system".to_string(),
                ],
                integration_pattern: "Component Composition".to_string(),
                pros: vec![
                    "Mature and stable framework".to_string(),
                    "Excellent component reusability".to_string(),
                    "Strong TypeScript interop".to_string(),
                    "Comprehensive ecosystem".to_string(),
                ],
                cons: vec![
                    "More verbose than newer frameworks".to_string(),
                    "Macro-based component system".to_string(),
                ],
                best_use_case: "Enterprise applications requiring stability and ecosystem".to_string(),
            },
            FrameworkGuide {
                name: "Dioxus".to_string(),
                setup_steps: vec![
                    "Add dioxus dependency with 'web' feature".to_string(),
                    "Use Dioxus hooks for state management".to_string(),
                    "Create components with rsx! macro".to_string(),
                    "Handle events through Dioxus event handlers".to_string(),
                ],
                integration_pattern: "Hooks-based Architecture".to_string(),
                pros: vec![
                    "Modern React-like API".to_string(),
                    "Excellent performance".to_string(),
                    "Cross-platform support".to_string(),
                    "Intuitive for React developers".to_string(),
                ],
                cons: vec![
                    "Newer framework, smaller ecosystem".to_string(),
                    "Less mature than Leptos/Yew".to_string(),
                ],
                best_use_case: "Modern web applications with React-like development experience".to_string(),
            },
            FrameworkGuide {
                name: "Sycamore".to_string(),
                setup_steps: vec![
                    "Add sycamore dependency with 'web' feature".to_string(),
                    "Use Sycamore's reactive primitives".to_string(),
                    "Create components with view! macro".to_string(),
                    "Use context for shared state".to_string(),
                ],
                integration_pattern: "Context-based State".to_string(),
                pros: vec![
                    "Fine-grained reactivity".to_string(),
                    "Excellent performance".to_string(),
                    "Simple and intuitive API".to_string(),
                    "Great for complex state management".to_string(),
                ],
                cons: vec![
                    "Smaller ecosystem".to_string(),
                    "Less mature documentation".to_string(),
                ],
                best_use_case: "Applications requiring complex state relationships and high performance".to_string(),
            },
            FrameworkGuide {
                name: "Generic/Vanilla".to_string(),
                setup_steps: vec![
                    "No framework dependencies required".to_string(),
                    "Direct DOM manipulation with web-sys".to_string(),
                    "Manual event handling".to_string(),
                    "Custom state management".to_string(),
                ],
                integration_pattern: "Direct DOM Integration".to_string(),
                pros: vec![
                    "No framework overhead".to_string(),
                    "Maximum flexibility".to_string(),
                    "Framework agnostic".to_string(),
                    "Smallest bundle size".to_string(),
                ],
                cons: vec![
                    "Manual state management".to_string(),
                    "More boilerplate code".to_string(),
                    "Less maintainable for complex UIs".to_string(),
                ],
                best_use_case: "Simple integrations or when framework overhead is unacceptable".to_string(),
            },
        ]);
    }
}

/// Demonstrate framework integrations
fn demonstrate_integrations() {
    unsafe {
        if let Some(examples) = &INTEGRATION_EXAMPLES {
            web_sys::console::log_1(&format!("📚 Loaded {} integration examples", examples.len()).into());

            // Show available frameworks
            let frameworks: Vec<String> = examples.iter()
                .map(|ex| ex.framework.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            web_sys::console::log_1(&format!("🎯 Available frameworks: {}", frameworks.join(", ")).into());
        }
    }
}

// Public API functions
#[wasm_bindgen]
pub fn show_framework_guide(framework: &str) {
    unsafe {
        if let Some(guides) = &FRAMEWORK_GUIDES {
            if let Some(guide) = guides.iter().find(|g| g.name.to_lowercase() == framework.to_lowercase()) {
                web_sys::console::log_1(&format!("📖 {} Integration Guide", guide.name).into());
                web_sys::console::log_1(&format!("🎯 Best for: {}", guide.best_use_case).into());
                web_sys::console::log_1(&"📋 Setup Steps:".into());

                for (i, step) in guide.setup_steps.iter().enumerate() {
                    web_sys::console::log_1(&format!("  {}. {}", i + 1, step).into());
                }

                web_sys::console::log_1(&"✅ Pros:".into());
                for pro in &guide.pros {
                    web_sys::console::log_1(&format!("  • {}", pro).into());
                }

                web_sys::console::log_1(&"⚠️ Cons:".into());
                for con in &guide.cons {
                    web_sys::console::log_1(&format!("  • {}", con).into());
                }
            } else {
                web_sys::console::log_1(&format!("❌ Framework '{}' not found", framework).into());
            }
        }
    }
}

#[wasm_bindgen]
pub fn show_integration_pattern(pattern: &str) {
    unsafe {
        if let Some(examples) = &INTEGRATION_EXAMPLES {
            let matching: Vec<&IntegrationExample> = examples.iter()
                .filter(|ex| ex.pattern.to_lowercase().contains(&pattern.to_lowercase()))
                .collect();

            if !matching.is_empty() {
                web_sys::console::log_1(&format!("🔍 Found {} examples matching '{}':", matching.len(), pattern).into());

                for example in matching {
                    web_sys::console::log_1(&format!("🎯 {} - {} ({})", example.framework, example.pattern, example.complexity).into());
                    web_sys::console::log_1(&format!("📝 {}", example.description).into());
                    web_sys::console::log_1(&"💻 Code Example:".into());
                    web_sys::console::log_1(&example.code_example.clone().into());
                    web_sys::console::log_1(&"".into());
                }
            } else {
                web_sys::console::log_1(&format!("❌ No examples found for pattern '{}'", pattern).into());
            }
        }
    }
}

#[wasm_bindgen]
pub fn compare_frameworks() {
    unsafe {
        if let Some(guides) = &FRAMEWORK_GUIDES {
            web_sys::console::log_1(&"🔄 Framework Comparison:".into());

            for guide in guides {
                web_sys::console::log_1(&format!("🎯 {}: {}", guide.name, guide.integration_pattern).into());
                web_sys::console::log_1(&format!("  📈 Best for: {}", guide.best_use_case).into());
                web_sys::console::log_1(&format!("  ✅ Pros: {}", guide.pros.len()).into());
                web_sys::console::log_1(&format!("  ⚠️ Cons: {}", guide.cons.len()).into());
                web_sys::console::log_1(&"".into());
            }

            web_sys::console::log_1(&"💡 Recommendation: Choose based on your project needs:".into());
            web_sys::console::log_1(&"  • Leptos: Complex reactive applications".into());
            web_sys::console::log_1(&"  • Yew: Enterprise applications".into());
            web_sys::console::log_1(&"  • Dioxus: Modern React-like development".into());
            web_sys::console::log_1(&"  • Sycamore: High-performance complex state".into());
            web_sys::console::log_1(&"  • Generic: Minimal overhead integrations".into());
        }
    }
}

#[wasm_bindgen]
pub fn generate_integration_code(framework: &str) -> String {
    unsafe {
        if let Some(examples) = &INTEGRATION_EXAMPLES {
            if let Some(example) = examples.iter().find(|ex| ex.framework.to_lowercase() == framework.to_lowercase()) {
                format!(
                    "// {} Integration - {}\n// {}\n\n{}",
                    example.framework,
                    example.pattern,
                    example.description,
                    example.code_example
                )
            } else {
                format!("// Framework '{}' not found. Available: Leptos, Yew, Dioxus, Sycamore, Generic", framework)
            }
        } else {
            "// Integration examples not loaded".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn get_framework_features(framework: &str) -> String {
    let features = match framework.to_lowercase().as_str() {
        "leptos" => vec![
            "Reactive signals for automatic UI updates",
            "Component-based architecture",
            "Server-side rendering support",
            "Fine-grained reactivity",
            "Type-safe component composition",
        ],
        "yew" => vec![
            "Mature component framework",
            "Strong TypeScript interop",
            "Agent system for background tasks",
            "Comprehensive ecosystem",
            "Stable and battle-tested",
        ],
        "dioxus" => vec![
            "React-like hooks and components",
            "Cross-platform support (Web, Desktop, Mobile)",
            "Modern async/await support",
            "Virtual DOM optimization",
            "Excellent performance",
        ],
        "sycamore" => vec![
            "Fine-grained reactive system",
            "Context-based state management",
            "Excellent performance benchmarks",
            "Simple and intuitive API",
            "Great for complex applications",
        ],
        "generic" => vec![
            "No framework dependencies",
            "Maximum flexibility",
            "Smallest bundle size",
            "Direct DOM manipulation",
            "Framework agnostic",
        ],
        _ => vec!["Framework not recognized"],
    };

    format!("🎯 {} Features:\n{}", framework, features.iter()
        .enumerate()
        .map(|(i, f)| format!("{}. {}", i + 1, f))
        .collect::<Vec<_>>()
        .join("\n"))
}

#[wasm_bindgen]
pub fn create_sample_integration(framework: &str) -> String {
    match framework.to_lowercase().as_str() {
        "leptos" => r#"
// Sample Leptos + Flow-RS Integration
use leptos::*;
use flow_rs_core::{Graph, Node, Position};

#[component]
pub fn FlowDiagram() -> impl IntoView {
    let (graph, set_graph) = create_signal(Graph::new());

    // Add initial nodes
    create_effect(move |_| {
        set_graph.update(|g| {
            let node1 = Node::new("start".to_string(), Position::new(100.0, 100.0), "Start".to_string());
            let node2 = Node::new("process".to_string(), Position::new(300.0, 100.0), "Process".to_string());
            let node3 = Node::new("end".to_string(), Position::new(500.0, 100.0), "End".to_string());

            g.add_node(node1).unwrap();
            g.add_node(node2).unwrap();
            g.add_node(node3).unwrap();
        });
    });

    let node_count = move || graph.get().nodes().count();

    view! {
        <div class="flow-diagram">
            <h3>"Flow-RS + Leptos Integration"</h3>
            <p>"Nodes in graph: " {node_count}</p>
            <div class="graph-container">
                // Here you would integrate Flow-RS canvas rendering
                <canvas id="flow-canvas" width="600" height="400"></canvas>
            </div>
        </div>
    }
}
"#.to_string(),

        "yew" => r#"
// Sample Yew + Flow-RS Integration
use yew::prelude::*;
use flow_rs_core::{Graph, Node, Position};

#[derive(Clone, Properties, PartialEq)]
pub struct FlowProps {
    pub width: u32,
    pub height: u32,
}

#[function_component(FlowDiagram)]
pub fn flow_diagram(props: &FlowProps) -> Html {
    let graph = use_state(|| {
        let mut g = Graph::new();
        let node = Node::new("sample".to_string(), Position::new(200.0, 150.0), "Sample Node".to_string());
        g.add_node(node).unwrap();
        g
    });

    let node_count = graph.nodes().count();

    html! {
        <div class="flow-diagram">
            <h3>{"Flow-RS + Yew Integration"}</h3>
            <p>{format!("Nodes in graph: {}", node_count)}</p>
            <div class="graph-container">
                <canvas
                    id="flow-canvas"
                    width={props.width.to_string()}
                    height={props.height.to_string()}
                ></canvas>
            </div>
        </div>
    }
}
"#.to_string(),

        _ => format!("// Sample integration for {} not available. Try: leptos, yew, dioxus, sycamore", framework),
    }
}
