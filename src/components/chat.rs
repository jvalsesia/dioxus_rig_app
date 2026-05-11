use crate::server_fns::chat_with_agent;
use crate::{compose_preamble, Agent, Personality, Route};
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

    let agent_opt = agents_resource
        .read()
        .as_ref()
        .and_then(|agents| agents.iter().find(|a| a.id == id).cloned());

    let (agent_name, agent_specialty, agent_personality) = match agent_opt {
        Some(a) => (a.name, a.specialty, a.personality),
        None => (
            t!("unknown-agent"),
            t!("unknown-specialty"),
            Personality::default(),
        ),
    };

    let initial = agent_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "·".into());

    let preamble = compose_preamble(&agent_specialty, &agent_personality);
    let preamble_for_keydown = preamble.clone();
    let preamble_for_click = preamble.clone();
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
                          preamble: String| {
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
            match chat_with_agent(text, preamble).await {
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
        let _ = document::eval(
            r#"
            setTimeout(() => {
                let el = document.getElementById('chat-messages');
                if (el) el.scrollTop = el.scrollHeight;
            }, 50);
        "#,
        );
    });

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0",

            // ── Header ────────────────────────────────────────────────────────
            header { class: "px-10 pt-8 pb-5 border-b border-zinc-200 dark:border-zinc-800 bg-white/60 dark:bg-zinc-900/40 backdrop-blur shrink-0",
                div { class: "flex items-baseline gap-3 mb-3",
                    Link {
                        to: Route::AgentList {},
                        class: "font-mono text-[11px] tracking-[0.22em] uppercase text-zinc-500 hover:text-zinc-900 dark:hover:text-zinc-100 no-underline",
                        {t!("chat-back")}
                    }
                    span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                    span { class: "font-mono text-[11px] tracking-[0.28em] uppercase text-amber-600 dark:text-amber-400",
                        "Live session"
                    }
                }
                div { class: "flex items-center gap-4",
                    div { class: "monogram-ring w-14 h-14 rounded-2xl border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display italic text-2xl text-zinc-900 dark:text-zinc-100 shrink-0",
                        "{initial}"
                    }
                    div { class: "flex-1 min-w-0",
                        span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500", "in conversation with" }
                        h2 { class: "font-display text-3xl text-zinc-900 dark:text-zinc-100 leading-tight truncate",
                            "{agent_name}"
                        }
                    }
                }
                p { class: "mt-2 font-display italic text-sm text-zinc-500 dark:text-zinc-400 line-clamp-2",
                    "{agent_specialty}"
                }
            }

            // ── Message list ──────────────────────────────────────────────────
            div {
                id: "chat-messages",
                class: "flex-1 overflow-y-auto px-10 py-8 flex flex-col gap-4 chat-scroll bg-paper",
                for (i, msg) in messages.read().iter().enumerate() {
                    {
                        let is_user = msg.role == "user";
                        rsx! {
                            div {
                                key: "{i}",
                                class: if is_user {
                                    "flex justify-end gap-3 msg-fade-in"
                                } else {
                                    "flex justify-start gap-3 msg-fade-in"
                                },
                                if !is_user {
                                    div { class: "monogram-ring w-8 h-8 rounded-lg border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display italic text-sm text-zinc-700 dark:text-zinc-200 shrink-0 mt-1",
                                        "{initial}"
                                    }
                                }
                                div { class: "max-w-[78%] flex flex-col gap-1",
                                    span { class: if is_user {
                                        "self-end font-mono text-[9px] tracking-[0.22em] uppercase text-zinc-400 dark:text-zinc-600"
                                    } else {
                                        "self-start font-mono text-[9px] tracking-[0.22em] uppercase text-zinc-400 dark:text-zinc-600"
                                    },
                                        if is_user { "you" } else { "{agent_name}" }
                                    }
                                    div {
                                        class: if is_user {
                                            "px-5 py-3 rounded-2xl rounded-br-md bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 text-sm leading-relaxed"
                                        } else {
                                            "px-5 py-3 rounded-2xl rounded-bl-md bg-white dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 text-sm leading-relaxed border border-zinc-200 dark:border-zinc-800"
                                        },
                                        "{msg.content}"
                                    }
                                }
                                if is_user {
                                    div { class: "w-8 h-8 rounded-lg border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-mono text-[10px] text-zinc-500 shrink-0 mt-1",
                                        "YOU"
                                    }
                                }
                            }
                        }
                    }
                }
                if *is_loading.read() {
                    div { class: "flex justify-start gap-3 msg-fade-in",
                        div { class: "monogram-ring w-8 h-8 rounded-lg border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display italic text-sm text-zinc-700 dark:text-zinc-200 shrink-0 mt-1",
                            "{initial}"
                        }
                        div { class: "px-5 py-3 rounded-2xl rounded-bl-md bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 flex gap-1.5 items-center",
                            span { class: "dot" }
                            span { class: "dot" }
                            span { class: "dot" }
                        }
                    }
                }
            }

            // ── Input bar ────────────────────────────────────────────────────
            div { class: "px-10 py-5 border-t border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-900/60 backdrop-blur flex gap-3 items-center shrink-0",
                span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-amber-600 dark:text-amber-400 hidden md:inline",
                    "→"
                }
                input {
                    class: "flex-1 bg-transparent border border-zinc-200 dark:border-zinc-800 px-5 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-600 focus:outline-none focus:border-zinc-400 dark:focus:border-zinc-600 transition-colors",
                    placeholder: t!("chat-placeholder"),
                    value: "{current_input}",
                    oninput: move |evt| *current_input.write() = evt.value(),
                    onkeydown: move |evt| {
                        let pre = preamble_for_keydown.clone();
                        if evt.key() == Key::Enter {
                            do_submit(messages, current_input, is_loading, pre);
                        }
                    }
                }
                button {
                    class: "bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 px-6 py-3 rounded-xl font-mono text-[11px] tracking-[0.22em] uppercase hover:opacity-90 transition-opacity disabled:opacity-50 shrink-0",
                    onclick: move |_| {
                        let pre = preamble_for_click.clone();
                        do_submit(messages, current_input, is_loading, pre);
                    },
                    disabled: *is_loading.read(),
                    {t!("chat-send")}
                }
            }
        }
    }
}
