use crate::{Agent, Route};
use crate::server_fns::chat_with_agent;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Clone, PartialEq)]
struct Message {
    role: String,
    content: String,
}

#[component]
pub fn Chat(id: String) -> Element {
    let agents_resource = use_context::<Resource<Vec<Agent>>>();
    
    // Find the agent by id
    let agent_opt = agents_resource.read().as_ref().and_then(|agents| agents.iter().find(|a| a.id == id).cloned());
    
    let (agent_name, agent_specialty) = match agent_opt {
        Some(a) => (a.name, a.specialty),
        None => (t!("unknown-agent"), t!("unknown-specialty"))
    };

    let specialty_for_keydown = agent_specialty.clone();
    let specialty_for_click = agent_specialty.clone();
    let name_clone = agent_name.clone();

    // Use use_hook so we don't recreate the initial message on every render if state changes
    let messages = use_signal(|| {
        vec![Message {
            role: "assistant".to_string(),
            content: t!("hello-agent", name: name_clone),
        }]
    });
    
    let mut current_input = use_signal(String::new);
    let is_loading = use_signal(|| false);

    let do_submit = move |mut messages: Signal<Vec<Message>>, mut current_input: Signal<String>, mut is_loading: Signal<bool>, specialty: String| {
        let text = current_input.read().clone();
        if text.trim().is_empty() {
            return;
        }

        messages.write().push(Message {
            role: "user".to_string(),
            content: text.clone(),
        });
        *current_input.write() = String::new();
        *is_loading.write() = true;

        spawn(async move {
            match chat_with_agent(text, specialty).await {
                Ok(response) => {
                    messages.write().push(Message {
                        role: "assistant".to_string(),
                        content: response,
                    });
                }
                Err(e) => {
                    messages.write().push(Message {
                        role: "assistant".to_string(),
                        content: t!("error-prefix", error: e.to_string()),
                    });
                }
            }
            *is_loading.write() = false;
        });
    };

    use_effect(move || {
        let _ = messages.read().len();
        let _ = is_loading.read();
        
        let _ = document::eval(r#"
            setTimeout(() => {
                let container = document.querySelector('.chat-messages');
                if (container) {
                    container.scrollTop = container.scrollHeight;
                }
            }, 50);
        "#);
    });

    rsx! {
        div { class: "chat-container",
            div { class: "chat-header",
                div { class: "header-actions",
                    Link {
                        to: Route::AgentList {},
                        class: "back-button",
                        {t!("chat-back")}
                    }
                }
                h2 { {t!("chat-title", name: agent_name.clone())} }
                p { {t!("agent-specialty", specialty: agent_specialty.clone())} }
            }
            div { class: "chat-messages",
                for msg in messages.read().iter() {
                    div { class: "message-wrapper {msg.role}",
                        div { class: "message {msg.role}",
                            "{msg.content}"
                        }
                    }
                }
                if *is_loading.read() {
                    div { class: "message-wrapper assistant",
                        div { class: "message assistant loading",
                            span { class: "dot" }
                            span { class: "dot" }
                            span { class: "dot" }
                        }
                    }
                }
            }
            div { class: "chat-input-area",
                input {
                    class: "chat-input",
                    placeholder: t!("chat-placeholder"),
                    value: "{current_input}",
                    oninput: move |evt| *current_input.write() = evt.value(),
                    onkeydown: move |evt| {
                        let spec = specialty_for_keydown.clone();
                        if evt.key() == Key::Enter {
                            do_submit(messages, current_input, is_loading, spec);
                        }
                    }
                }
                button {
                    class: "send-button",
                    onclick: move |_| {
                        let spec = specialty_for_click.clone();
                        do_submit(messages, current_input, is_loading, spec)
                    },
                    disabled: *is_loading.read(),
                    {t!("chat-send")}
                }
            }
        }
    }
}
