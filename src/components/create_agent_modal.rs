use dioxus::prelude::*;
use crate::Agent;

#[derive(Props, Clone, PartialEq)]
pub struct CreateAgentModalProps {
    pub onclose: EventHandler<()>,
}

#[component]
pub fn CreateAgentModal(props: CreateAgentModalProps) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let mut new_name = use_signal(|| String::new());
    let mut new_specialty = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let create_agent = move |_| {
        let name = new_name.read().clone();
        let specialty = new_specialty.read().clone();
        
        if name.trim().is_empty() || specialty.trim().is_empty() {
            return;
        }

        *is_loading.write() = true;

        spawn(async move {
            if let Ok(_) = crate::server_fns::add_agent(name, specialty).await {
                agents_resource.restart();
                *is_loading.write() = false;
                props.onclose.call(());
            } else {
                *is_loading.write() = false;
            }
        });
    };

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { "Create a New Agent" }
                    button { class: "close-button", onclick: move |_| props.onclose.call(()), "✕" }
                }
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
                    disabled: *is_loading.read(),
                    if *is_loading.read() {
                        "Creating..."
                    } else {
                        "Create Agent"
                    }
                }
            }
        }
    }
}
