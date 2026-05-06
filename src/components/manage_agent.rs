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
            
            *is_saving.write() = true;
            spawn(async move {
                if crate::server_fns::update_agent(id, name, specialty).await.is_ok() {
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
                    div { class: "manage-actions",
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
