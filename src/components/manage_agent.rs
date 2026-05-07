use crate::{Agent, Route};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ManageAgent(id: String) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let navigator = use_navigator();
    
    // Find the agent
    let agent_opt = agents_resource.read().as_ref().and_then(|agents| {
        agents.iter().find(|a| a.id == id).cloned()
    });

    if agent_opt.is_none() {
        return rsx! { div { {t!("loading-agents")} } };
    }

    let agent = agent_opt.unwrap();
    
    let mut edit_mode = use_signal(|| false);
    let mut edit_name = use_signal(|| agent.name.clone());
    let mut edit_specialty = use_signal(|| agent.specialty.clone());
    let mut edit_n8n_send = use_signal(|| agent.n8n_webhook_send.clone().unwrap_or_default());
    let mut edit_n8n_receive = use_signal(|| agent.n8n_webhook_receive.clone().unwrap_or_default());
    let mut test_result = use_signal(|| "".to_string());
    let mut is_testing = use_signal(|| false);
    let mut is_saving = use_signal(|| false);

    let delete_agent = {
        let id = id.clone();
        move |_| {
            let id = id.clone();
            spawn(async move {
                if crate::server_fns::delete_agent(id).await.is_ok() {
                    agents_resource.restart();
                    navigator.push(Route::AgentList {});
                }
            });
        }
    };

    let save_agent = {
        let id = id.clone();
        move |_| {
            let id = id.clone();
            let name = edit_name.read().clone();
            let specialty = edit_specialty.read().clone();
            let n8n_send = edit_n8n_send.read().clone();
            let n8n_recv = edit_n8n_receive.read().clone();
            
            let send_opt = if n8n_send.trim().is_empty() { None } else { Some(n8n_send) };
            let recv_opt = if n8n_recv.trim().is_empty() { None } else { Some(n8n_recv) };
            
            *is_saving.write() = true;
            spawn(async move {
                if crate::server_fns::update_agent(id, name, specialty, send_opt, recv_opt).await.is_ok() {
                    agents_resource.restart();
                    *edit_mode.write() = false;
                }
                *is_saving.write() = false;
            });
        }
    };

    rsx! {
        div { class: "manage-agent-container",
            div { class: "chat-header header-actions",
                Link { to: Route::AgentList {}, class: "back-button", {t!("manage-back")} }
            }
            
            div { class: "manage-card",
                if *edit_mode.read() {
                    h3 { {t!("manage-edit-title")} }
                    div { class: "form-group",
                        label { {t!("manage-edit-name")} }
                        input {
                            value: "{edit_name}",
                            oninput: move |evt| *edit_name.write() = evt.value()
                        }
                    }
                    div { class: "form-group",
                        label { {t!("manage-edit-specialty")} }
                        textarea {
                            value: "{edit_specialty}",
                            oninput: move |evt| *edit_specialty.write() = evt.value(),
                            rows: 4
                        }
                    }
                    h3 { style: "margin-top: 1.5rem; color: var(--text-primary); font-size: 1.25rem;", "n8n Configuration" }
                    div { class: "form-group",
                        label { "Webhook to Send Messages" }
                        input {
                            value: "{edit_n8n_send}",
                            placeholder: "https://your-n8n.com/webhook/send",
                            oninput: move |evt| *edit_n8n_send.write() = evt.value()
                        }
                    }
                    div { class: "form-group",
                        label { "Webhook to Receive Messages" }
                        input {
                            value: "{edit_n8n_receive}",
                            placeholder: "https://your-n8n.com/webhook/receive",
                            oninput: move |evt| *edit_n8n_receive.write() = evt.value()
                        }
                    }
                    div { style: "margin-top: 0.5rem;",
                        button {
                            class: "action-button manage-btn",
                            style: "width: 100%; max-width: 250px; font-size: 0.9rem;",
                            disabled: *is_testing.read() || edit_n8n_send.read().trim().is_empty(),
                            onclick: move |_| {
                                let url = edit_n8n_send.read().clone();
                                *is_testing.write() = true;
                                *test_result.write() = "Testing connection...".to_string();
                                spawn(async move {
                                    match crate::server_fns::test_n8n_webhook(url).await {
                                        Ok(msg) => *test_result.write() = msg,
                                        Err(e) => *test_result.write() = format!("Error: {}", e),
                                    }
                                    *is_testing.write() = false;
                                });
                            },
                            if *is_testing.read() { "Testing..." } else { "Test Send Connection" }
                        }
                        if !test_result.read().is_empty() {
                            p { style: "margin-top: 0.5rem; font-size: 0.85rem; color: var(--text-secondary);", "{test_result}" }
                        }
                    }
                    div { class: "manage-actions", style: "margin-top: 1rem;",
                        button {
                            class: "create-button",
                            onclick: save_agent,
                            disabled: *is_saving.read(),
                            if *is_saving.read() { {t!("manage-edit-saving")} } else { {t!("manage-edit-save")} }
                        }
                        button {
                            class: "cancel-button",
                            onclick: move |_| *edit_mode.write() = false,
                            {t!("manage-edit-cancel")}
                        }
                    }
                } else {
                    h2 { class: "agent-name-title", "{agent.name}" }
                    p { class: "agent-specialty-text", "{agent.specialty}" }
                    
                    div { class: "manage-actions",
                        Link {
                            to: Route::Chat { id: id.clone() },
                            class: "action-button chat-action",
                            {t!("manage-start-chat")}
                        }
                        button {
                            class: "action-button edit-action",
                            onclick: move |_| *edit_mode.write() = true,
                            {t!("manage-edit-btn")}
                        }
                        button {
                            class: "action-button delete-action",
                            onclick: delete_agent,
                            {t!("manage-delete-btn")}
                        }
                    }
                }
            }
        }
    }
}
