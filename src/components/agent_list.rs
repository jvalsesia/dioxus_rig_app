use crate::{Agent, Route};
use dioxus::prelude::*;

#[component]
pub fn AgentList() -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    
    let mut new_name = use_signal(|| String::new());
    let mut new_specialty = use_signal(|| String::new());

    let create_agent = move |_| {
        let name = new_name.read().clone();
        let specialty = new_specialty.read().clone();
        
        if name.trim().is_empty() || specialty.trim().is_empty() {
            return;
        }

        *new_name.write() = String::new();
        *new_specialty.write() = String::new();

        spawn(async move {
            if let Ok(_) = crate::server_fns::add_agent(name, specialty).await {
                agents_resource.restart();
            }
        });
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
                if let Some(agents) = agents_resource.read().as_ref() {
                    for agent in agents.iter() {
                        Link {
                            to: Route::Chat { id: agent.id.clone() },
                            class: "agent-card",
                            h4 { "{agent.name}" }
                            p { class: "agent-specialty", "{agent.specialty}" }
                        }
                    }
                } else {
                    p { "Loading agents..." }
                }
            }
        }
    }
}
