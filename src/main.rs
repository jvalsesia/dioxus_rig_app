#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
pub mod server_fns;

use components::chat::Chat;
use components::agent_list::AgentList;
use components::manage_agent::ManageAgent;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SidebarLayout)]
    #[route("/")]
    AgentList {},
    #[route("/chat/:id")]
    Chat { id: String },
    #[route("/manage/:id")]
    ManageAgent { id: String },
}

#[component]
fn SidebarLayout() -> Element {
    let mut show_create_modal = use_signal(|| false);

    rsx! {
        div { class: "dashboard-layout",
            // Sidebar
            div { class: "sidebar",
                div { class: "sidebar-header",
                    h2 { "Agent Maker" }
                    p { "Manage & Create AI" }
                }
                div { class: "sidebar-nav",
                    Link {
                        to: Route::AgentList {},
                        class: "nav-btn",
                        span { class: "btn-icon", "🏠" }
                        "Dashboard"
                    }
                    button {
                        class: "new-agent-btn",
                        onclick: move |_| *show_create_modal.write() = true,
                        span { class: "btn-icon", "➕" }
                        "New Agent"
                    }
                }
            }

            // Main Content Area where the route's components will render
            div { class: "main-content",
                Outlet::<Route> {}
            }
            
            if *show_create_modal.read() {
                components::create_agent_modal::CreateAgentModal {
                    onclose: move |_| *show_create_modal.write() = false
                }
            }
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub specialty: String,
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize global state for agents from LanceDB
    let agents_resource = use_resource(|| async move {
        crate::server_fns::get_agents().await.unwrap_or_else(|_| vec![])
    });
    
    use_context_provider(|| agents_resource);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-wrapper",
            Router::<Route> {}
        }
    }
}
