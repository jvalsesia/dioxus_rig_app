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

    let agent_opt = agents_resource.read().as_ref().and_then(|agents| {
        agents.iter().find(|a| a.id == id).cloned()
    });

    let (agent_name, agent_specialty) = match agent_opt {
        Some(a) => (a.name, a.specialty),
        None => (t!("unknown-agent"), t!("unknown-specialty")),
    };

    let specialty_for_keydown = agent_specialty.clone();
    let specialty_for_click = agent_specialty.clone();
    let name_clone = agent_name.clone();

    let messages = use_signal(|| {
        vec![Message {
            role: "assistant".to_string(),
            content: t!("hello-agent", name: name_clone),
        }]
    });

    let mut current_input = use_signal(String::new);
    let is_loading = use_signal(|| false);

    let do_submit = move |mut messages: Signal<Vec<Message>>,
                          mut current_input: Signal<String>,
                          mut is_loading: Signal<bool>,
                          specialty: String| {
        let text = current_input.read().clone();
        if text.trim().is_empty() {
            return;
        }
        messages.write().push(Message { role: "user".to_string(), content: text.clone() });
        *current_input.write() = String::new();
        *is_loading.write() = true;

        spawn(async move {
            match chat_with_agent(text, specialty).await {
                Ok(response) => messages.write().push(Message {
                    role: "assistant".to_string(),
                    content: response,
                }),
                Err(e) => messages.write().push(Message {
                    role: "assistant".to_string(),
                    content: t!("error-prefix", error: e.to_string()),
                }),
            }
            *is_loading.write() = false;
        });
    };

    use_effect(move || {
        let _ = messages.read().len();
        let _ = is_loading.read();
        let _ = document::eval(r#"
            setTimeout(() => {
                let el = document.getElementById('chat-messages');
                if (el) el.scrollTop = el.scrollHeight;
            }, 50);
        "#);
    });

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0",

            // Header
            div { class: "px-6 py-4 border-b border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shrink-0",
                div { class: "mb-1.5",
                    Link {
                        to: Route::AgentList {},
                        class: "text-sm text-violet-600 dark:text-violet-400 font-medium hover:text-violet-500 dark:hover:text-violet-300 no-underline",
                        {t!("chat-back")}
                    }
                }
                h2 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100",
                    {t!("chat-title", name: agent_name.clone())}
                }
                p { class: "text-xs text-zinc-500 mt-0.5",
                    {t!("agent-specialty", specialty: agent_specialty.clone())}
                }
            }

            // Message list
            div {
                id: "chat-messages",
                class: "flex-1 overflow-y-auto p-6 flex flex-col gap-4 chat-scroll",
                for msg in messages.read().iter() {
                    div {
                        class: if msg.role == "user" {
                            "flex justify-end msg-fade-in"
                        } else {
                            "flex justify-start msg-fade-in"
                        },
                        div {
                            class: if msg.role == "user" {
                                "max-w-[80%] px-4 py-3 rounded-2xl rounded-br-none bg-violet-600 text-white text-sm leading-relaxed"
                            } else {
                                "max-w-[80%] px-4 py-3 rounded-2xl rounded-bl-none bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 text-sm leading-relaxed border border-zinc-200 dark:border-zinc-700"
                            },
                            "{msg.content}"
                        }
                    }
                }
                if *is_loading.read() {
                    div { class: "flex justify-start msg-fade-in",
                        div { class: "px-4 py-3 rounded-2xl rounded-bl-none bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 flex gap-1.5 items-center",
                            span { class: "dot" }
                            span { class: "dot" }
                            span { class: "dot" }
                        }
                    }
                }
            }

            // Input bar
            div { class: "px-6 py-4 border-t border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 flex gap-3 shrink-0",
                input {
                    class: "flex-1 bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-500 focus:outline-none focus:border-violet-400 dark:focus:border-violet-500/50 focus:ring-2 focus:ring-violet-200 dark:focus:ring-violet-500/20 transition-all",
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
                    class: "bg-violet-600 text-white px-6 py-3 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed shrink-0",
                    onclick: move |_| {
                        let spec = specialty_for_click.clone();
                        do_submit(messages, current_input, is_loading, spec);
                    },
                    disabled: *is_loading.read(),
                    {t!("chat-send")}
                }
            }
        }
    }
}
