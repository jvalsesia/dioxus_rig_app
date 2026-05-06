use crate::{Agent, Route};
use dioxus::prelude::*;

#[component]
pub fn AgentList() -> Element {
    let agents_resource = use_context::<Resource<Vec<Agent>>>();

    rsx! {
        div { class: "dashboard-header",
            h2 { "Agent Dashboard" }
            p { "Manage your specialized AI agents." }
        }

        div { class: "agent-cards-grid",
            if let Some(agents) = agents_resource.read().as_ref() {
                for agent in agents.iter() {
                    div { class: "agent-card",
                        div { class: "agent-icon-wrapper",
                            span { class: "agent-emoji", "🤖" }
                        }
                        h4 { "{agent.name}" }
                        p { class: "agent-specialty", "{agent.specialty}" }
                        
                        div { class: "card-actions",
                            Link {
                                to: Route::Chat { id: agent.id.clone() },
                                class: "card-btn chat-btn",
                                "💬 Chat"
                            }
                            Link {
                                to: Route::ManageAgent { id: agent.id.clone() },
                                class: "card-btn manage-btn",
                                "⚙️ Manage"
                            }
                        }
                    }
                }
            } else {
                p { "Loading agents..." }
            }
        }
    }
}
