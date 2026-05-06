use crate::{Agent, Route};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AgentList() -> Element {
    let agents_resource = use_context::<Resource<Vec<Agent>>>();

    rsx! {
        div { class: "dashboard-header",
            h2 { {t!("dashboard-title")} }
            p { {t!("dashboard-subtitle")} }
        }

        div { class: "agent-cards-grid",
            if let Some(agents) = agents_resource.read().as_ref() {
                for agent in agents.iter() {
                    div { class: "agent-card",
                        div { class: "agent-icon-wrapper",
                            span { class: "agent-emoji", "🤖" }
                        }
                        h4 { "{agent.name}" }
                        p { class: "agent-specialty", {t!("agent-specialty", specialty: agent.specialty.clone())} }
                        
                        div { class: "card-actions",
                            Link {
                                to: Route::Chat { id: agent.id.clone() },
                                class: "card-btn chat-btn",
                                {t!("chat-btn")}
                            }
                            Link {
                                to: Route::ManageAgent { id: agent.id.clone() },
                                class: "card-btn manage-btn",
                                {t!("manage-btn")}
                            }
                        }
                    }
                }
            } else {
                p { {t!("loading-agents")} }
            }
        }
    }
}
