use crate::{Agent, Route};
use dioxus::prelude::*;

#[component]
pub fn AgentList() -> Element {
    let mut agents = use_context::<Signal<Vec<Agent>>>();
    
    let mut new_name = use_signal(|| String::new());
    let mut new_specialty = use_signal(|| String::new());
    let mut next_id = use_signal(|| 2); // Start at 2 since 1 is the default agent

    let create_agent = move |_| {
        let name = new_name.read().clone();
        let specialty = new_specialty.read().clone();
        
        if name.trim().is_empty() || specialty.trim().is_empty() {
            return;
        }

        let id = next_id.read().to_string();
        *next_id.write() += 1;
        
        agents.write().push(Agent {
            id,
            name,
            specialty,
        });

        *new_name.write() = String::new();
        *new_specialty.write() = String::new();
    };

    rsx! {
        div { class: "agent-list-container",
            div { class: "chat-header",
                h2 { "Agent Maker" }
                p { "Create and interact with specialized AI agents." }
                p {
                    style: "font-size: 0.8rem; justify-self: right",
                    "Built 100% in Rust with Dioxus and Rig" 
                }
            }

            div { class: "create-agent-card",
                h3 { "Create a New Agent" }
                div { class: "form-group",
                    label { "Name" }
                    input {
                        placeholder: "e.g., Car Seller, Stock Manager...",
                        value: "{new_name}",
                        oninput: move |evt| *new_name.write() = evt.value()
                    }
                }
                div { class: "form-group",
                    label { "Specialty (Prompt)" }
                    textarea {
                        placeholder: "e.g., You are an aggressive car salesman...",
                        value: "{new_specialty}",
                        oninput: move |evt| *new_specialty.write() = evt.value(),
                        rows: 3
                    }
                }
                button {
                    class: "create-button",
                    onclick: create_agent,
                    "Create Agent"
                }
            }

            h3 { style: "margin-top: 2rem;", "Available Agents" }
            div { class: "agents-grid",
                for agent in agents.read().iter() {
                    Link {
                        to: Route::Chat { id: agent.id.clone() },
                        class: "agent-card",
                        h4 { "{agent.name}" }
                        p { class: "agent-specialty", "{agent.specialty}" }
                    }
                }
            }
        }
    }
}
