use crate::{Agent, Route};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn DeployAgent(id: String) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();

    let agent_opt = agents_resource.read().as_ref().and_then(|agents| {
        agents.iter().find(|a| a.id == id).cloned()
    });

    if agent_opt.is_none() {
        return rsx! {
            div { class: "flex-1 overflow-y-auto p-8",
                p { class: "text-sm text-zinc-500", {t!("loading-agents")} }
            }
        };
    }

    let agent = agent_opt.unwrap();

    let mut edit_n8n_send    = use_signal(|| agent.n8n_webhook_send.clone().unwrap_or_default());
    let mut edit_n8n_receive = use_signal(|| agent.n8n_webhook_receive.clone().unwrap_or_default());
    let mut test_result      = use_signal(|| "".to_string());
    let mut is_testing       = use_signal(|| false);
    let mut is_saving        = use_signal(|| false);

    // Evolution API
    let mut evo_url          = use_signal(|| "http://localhost:8080".to_string());
    let mut evo_api_key      = use_signal(|| "".to_string());
    let mut evo_conn_status  = use_signal(|| "".to_string());
    let mut is_testing_evo   = use_signal(|| false);
    let mut evo_instance     = use_signal(|| "chip-sales".to_string());
    let mut evo_verify_time  = use_signal(|| "".to_string());
    let mut is_verifying_evo = use_signal(|| false);
    let mut ignore_groups    = use_signal(|| false);
    let mut is_associating   = use_signal(|| false);
    let mut assoc_status     = use_signal(|| "".to_string());

    let save_webhooks = {
        let id        = id.clone();
        let name      = agent.name.clone();
        let specialty = agent.specialty.clone();
        move |_| {
            let id        = id.clone();
            let name      = name.clone();
            let specialty = specialty.clone();
            let send_val  = edit_n8n_send.read().clone();
            let recv_val  = edit_n8n_receive.read().clone();
            let send_opt  = if send_val.trim().is_empty() { None } else { Some(send_val) };
            let recv_opt  = if recv_val.trim().is_empty() { None } else { Some(recv_val) };

            *is_saving.write() = true;
            spawn(async move {
                if crate::server_fns::update_agent(id, name, specialty, send_opt, recv_opt).await.is_ok() {
                    agents_resource.restart();
                }
                *is_saving.write() = false;
            });
        }
    };

    // Shared class strings
    let input_cls = "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-500 focus:outline-none focus:border-violet-400 dark:focus:border-violet-500/50 focus:ring-2 focus:ring-violet-200 dark:focus:ring-violet-500/20 transition-all";

    let action_btn_cls = "bg-sky-50 dark:bg-sky-400/10 text-sky-600 dark:text-sky-400 border border-sky-200 dark:border-sky-400/30 hover:bg-sky-100 dark:hover:bg-sky-400/20 px-4 py-2.5 rounded-xl text-sm font-semibold transition-all disabled:opacity-50 disabled:cursor-not-allowed shrink-0";

    let status_ok_cls  = "text-xs font-medium px-3 py-1.5 rounded-lg bg-emerald-50 dark:bg-emerald-400/10 text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-400/30";
    let status_err_cls = "text-xs font-medium px-3 py-1.5 rounded-lg bg-red-50 dark:bg-red-500/10 text-red-600 dark:text-red-400 border border-red-200 dark:border-red-500/30";

    rsx! {
        div { class: "flex-1 overflow-y-auto p-8",
            div { class: "max-w-2xl mx-auto",

                div { class: "mb-6",
                    Link {
                        to: Route::AgentList {},
                        class: "text-sm text-violet-600 dark:text-violet-400 hover:text-violet-500 dark:hover:text-violet-300 font-medium no-underline",
                        {t!("deploy-back")}
                    }
                }

                div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-8 flex flex-col gap-6 shadow-sm",

                    // ── Agent title ───────────────────────────────────────────
                    div {
                        h2 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", "{agent.name}" }
                        h3 { class: "text-sm font-semibold text-zinc-500 mt-1", {t!("deploy-title")} }
                    }

                    // ── n8n webhooks ──────────────────────────────────────────
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("deploy-webhook-send")} }
                        input {
                            class: "{input_cls}",
                            value: "{edit_n8n_send}",
                            placeholder: "https://your-n8n.com/webhook/send",
                            oninput: move |evt| *edit_n8n_send.write() = evt.value()
                        }
                    }
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("deploy-webhook-receive")} }
                        input {
                            class: "{input_cls}",
                            value: "{edit_n8n_receive}",
                            placeholder: "https://your-n8n.com/webhook/receive",
                            oninput: move |evt| *edit_n8n_receive.write() = evt.value()
                        }
                    }

                    // Test send webhook
                    div { class: "flex items-center gap-3 flex-wrap",
                        button {
                            class: "{action_btn_cls}",
                            disabled: *is_testing.read() || edit_n8n_send.read().trim().is_empty(),
                            onclick: move |_| {
                                let url = edit_n8n_send.read().clone();
                                *is_testing.write() = true;
                                *test_result.write() = "Testing connection...".to_string();
                                spawn(async move {
                                    match crate::server_fns::test_n8n_webhook(url).await {
                                        Ok(msg) => *test_result.write() = msg,
                                        Err(e)  => *test_result.write() = format!("Error: {}", e),
                                    }
                                    *is_testing.write() = false;
                                });
                            },
                            if *is_testing.read() { {t!("deploy-testing")} } else { {t!("deploy-test-send")} }
                        }
                        if !test_result.read().is_empty() {
                            p { class: "text-xs text-zinc-500", "{test_result}" }
                        }
                    }

                    // Save / Cancel
                    div { class: "flex gap-3",
                        button {
                            class: "bg-violet-600 text-white py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed",
                            onclick: save_webhooks,
                            disabled: *is_saving.read(),
                            if *is_saving.read() { {t!("deploy-saving")} } else { {t!("deploy-save")} }
                        }
                        Link {
                            to: Route::AgentList {},
                            class: "flex items-center justify-center bg-transparent text-zinc-500 border border-zinc-200 dark:border-zinc-700 py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-700 dark:hover:text-zinc-200 transition-all no-underline",
                            {t!("deploy-cancel")}
                        }
                    }

                    // ── Divider ───────────────────────────────────────────────
                    div { class: "border-t border-zinc-200 dark:border-zinc-800" }

                    // ── Evolution API ─────────────────────────────────────────
                    h3 { class: "text-base font-semibold text-zinc-700 dark:text-zinc-300", {t!("deploy-evo-title")} }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("deploy-evo-url")} }
                        input {
                            r#type: "text",
                            class: "{input_cls}",
                            value: "{evo_url}",
                            placeholder: "http://localhost:8080",
                            oninput: move |evt| *evo_url.write() = evt.value()
                        }
                    }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("deploy-evo-api-key")} }
                        input {
                            r#type: "password",
                            class: "{input_cls}",
                            value: "{evo_api_key}",
                            placeholder: "••••••••••••••••••••",
                            oninput: move |evt| *evo_api_key.write() = evt.value()
                        }
                    }

                    // Test Evolution connection
                    div { class: "flex items-center gap-3 flex-wrap",
                        button {
                            class: "{action_btn_cls}",
                            disabled: *is_testing_evo.read(),
                            onclick: move |_| {
                                let url = evo_url.read().clone();
                                let key = evo_api_key.read().clone();
                                *is_testing_evo.write() = true;
                                *evo_conn_status.write() = "".to_string();
                                spawn(async move {
                                    match crate::server_fns::test_evolution_connection(url, key).await {
                                        Ok(msg) => *evo_conn_status.write() = msg,
                                        Err(e)  => *evo_conn_status.write() = format!("✗ {}", e),
                                    }
                                    *is_testing_evo.write() = false;
                                });
                            },
                            if *is_testing_evo.read() { {t!("deploy-testing")} } else { {t!("deploy-evo-test")} }
                        }
                        if !evo_conn_status.read().is_empty() {
                            span {
                                class: if evo_conn_status.read().starts_with("✗") { "{status_err_cls}" } else { "{status_ok_cls}" },
                                "{evo_conn_status}"
                            }
                        }
                    }

                    // Instance selector
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("deploy-evo-instance")} }
                        select {
                            class: "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm focus:outline-none focus:border-violet-400 cursor-pointer appearance-auto",
                            value: "{evo_instance}",
                            onchange: move |evt| *evo_instance.write() = evt.value(),
                            option { value: "chip-sales", "Chip Sales" }
                        }
                    }

                    // Verify instance
                    div { class: "flex items-center gap-3 flex-wrap",
                        button {
                            class: "{action_btn_cls}",
                            disabled: *is_verifying_evo.read(),
                            onclick: move |_| {
                                let url  = evo_url.read().clone();
                                let key  = evo_api_key.read().clone();
                                let inst = evo_instance.read().clone();
                                *is_verifying_evo.write() = true;
                                spawn(async move {
                                    match crate::server_fns::verify_evolution_instance(url, key, inst).await {
                                        Ok(time) => *evo_verify_time.write() = time,
                                        Err(e)   => *evo_verify_time.write() = format!("✗ {}", e),
                                    }
                                    *is_verifying_evo.write() = false;
                                });
                            },
                            if *is_verifying_evo.read() { {t!("deploy-evo-verifying")} } else { {t!("deploy-evo-verify")} }
                        }
                        if !evo_verify_time.read().is_empty() {
                            span { class: "text-xs text-zinc-500 bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-3 py-1.5 rounded-lg",
                                {t!("deploy-evo-last-verification", time: evo_verify_time.read().clone())}
                            }
                        }
                    }

                    // Ignore groups checkbox
                    div { class: "flex items-center gap-2 cursor-pointer",
                        input {
                            r#type: "checkbox",
                            id: "ignore-groups",
                            class: "w-4 h-4 accent-violet-600 cursor-pointer",
                            checked: *ignore_groups.read(),
                            onchange: move |evt| *ignore_groups.write() = evt.checked()
                        }
                        label {
                            r#for: "ignore-groups",
                            class: "text-sm text-zinc-500 cursor-pointer select-none",
                            {t!("deploy-evo-ignore-groups")}
                        }
                    }

                    // Associate webhook
                    div { class: "flex flex-col gap-3",
                        button {
                            class: "bg-violet-600 text-white py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed w-full max-w-xs",
                            disabled: *is_associating.read(),
                            onclick: move |_| {
                                let url  = evo_url.read().clone();
                                let key  = evo_api_key.read().clone();
                                let inst = evo_instance.read().clone();
                                let recv = edit_n8n_receive.read().clone();
                                let ig   = *ignore_groups.read();
                                *is_associating.write() = true;
                                spawn(async move {
                                    match crate::server_fns::associate_evolution_webhook(url, key, inst, recv, ig).await {
                                        Ok(msg) => *assoc_status.write() = msg,
                                        Err(e)  => *assoc_status.write() = format!("✗ {}", e),
                                    }
                                    *is_associating.write() = false;
                                });
                            },
                            if *is_associating.read() { {t!("deploy-evo-associating")} } else { {t!("deploy-evo-associate")} }
                        }
                        if !assoc_status.read().is_empty() {
                            p {
                                class: if assoc_status.read().starts_with("✗") { "{status_err_cls}" } else { "{status_ok_cls}" },
                                "{assoc_status}"
                            }
                        }
                    }
                }
            }
        }
    }
}
