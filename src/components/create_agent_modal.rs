use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::Agent;

#[derive(Props, Clone, PartialEq)]
pub struct CreateAgentModalProps {
    pub onclose: EventHandler<()>,
}

#[component]
pub fn CreateAgentModal(props: CreateAgentModalProps) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let mut new_name = use_signal(String::new);
    let mut new_specialty = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    let input_cls = "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-500 focus:outline-none focus:border-violet-400 dark:focus:border-violet-500/50 focus:ring-2 focus:ring-violet-200 dark:focus:ring-violet-500/20 transition-all";

    let create_agent = move |_| {
        let name = new_name.read().clone();
        let specialty = new_specialty.read().clone();
        if name.trim().is_empty() || specialty.trim().is_empty() {
            return;
        }
        *is_loading.write() = true;
        spawn(async move {
            if crate::server_fns::add_agent(name, specialty).await.is_ok() {
                agents_resource.restart();
                *is_loading.write() = false;
                props.onclose.call(());
            } else {
                *is_loading.write() = false;
            }
        });
    };

    rsx! {
        div { class: "fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50",
            div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 w-full max-w-md mx-4 flex flex-col gap-5 shadow-2xl",

                // Header
                div { class: "flex items-center justify-between",
                    h3 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100", {t!("create-modal-title")} }
                    button {
                        class: "text-zinc-400 hover:text-zinc-700 dark:hover:text-zinc-200 text-xl cursor-pointer transition-colors bg-transparent border-none leading-none",
                        onclick: move |_| props.onclose.call(()),
                        "✕"
                    }
                }

                // Name field
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-name")} }
                    input {
                        class: "{input_cls}",
                        placeholder: t!("create-modal-name-placeholder"),
                        value: "{new_name}",
                        oninput: move |evt| *new_name.write() = evt.value()
                    }
                }

                // Specialty field
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-specialty")} }
                    textarea {
                        class: "{input_cls} resize-none",
                        placeholder: t!("create-modal-specialty-placeholder"),
                        value: "{new_specialty}",
                        oninput: move |evt| *new_specialty.write() = evt.value(),
                        rows: 3
                    }
                }

                // Submit
                button {
                    class: "w-full bg-violet-600 text-white py-3 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: create_agent,
                    disabled: *is_loading.read(),
                    if *is_loading.read() { {t!("create-modal-creating")} } else { {t!("create-modal-submit")} }
                }
            }
        }
    }
}
