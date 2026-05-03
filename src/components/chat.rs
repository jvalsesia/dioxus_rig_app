use crate::server_fns::chat_with_agent;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Message {
    role: String,
    content: String,
}

#[component]
pub fn Chat() -> Element {
    let messages = use_signal(|| {
        vec![Message {
            role: "assistant".to_string(),
            content: "Hello! I am your AI assistant. How can I help you today?".to_string(),
        }]
    });
    let mut current_input = use_signal(|| String::new());
    let is_loading = use_signal(|| false);

    let do_submit = move |mut messages: Signal<Vec<Message>>, mut current_input: Signal<String>, mut is_loading: Signal<bool>| {
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
            match chat_with_agent(text).await {
                Ok(response) => {
                    messages.write().push(Message {
                        role: "assistant".to_string(),
                        content: response,
                    });
                }
                Err(e) => {
                    messages.write().push(Message {
                        role: "assistant".to_string(),
                        content: format!("Error: {}", e),
                    });
                }
            }
            *is_loading.write() = false;
        });
    };

    rsx! {
        div { class: "chat-container",
            div { class: "chat-header",
                h2 { "Rig AI Assistant" }
                p { "Powered by OpenAI & Rig Core" }
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
                    placeholder: "Type your message...",
                    value: "{current_input}",
                    oninput: move |evt| *current_input.write() = evt.value(),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            do_submit(messages, current_input, is_loading);
                        }
                    }
                }
                button {
                    class: "send-button",
                    onclick: move |_| do_submit(messages, current_input, is_loading),
                    disabled: *is_loading.read(),
                    "Send"
                }
            }
        }
    }
}
