use crate::{Agent, Route};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn DeployAgent(id: String) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();

    let agent_opt = agents_resource
        .read()
        .as_ref()
        .and_then(|agents| agents.iter().find(|a| a.id == id).cloned());

    if agent_opt.is_none() {
        return rsx! {
            div { class: "flex-1 overflow-y-auto px-10 py-12",
                p { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-500", {t!("loading-agents")} }
            }
        };
    }

    let agent = agent_opt.unwrap();
    let initial = agent.name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "·".into());

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
        let personality = agent.personality;
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
                if crate::server_fns::update_agent(id, name, specialty, send_opt, recv_opt, personality).await.is_ok() {
                    agents_resource.restart();
                }
                *is_saving.write() = false;
            });
        }
    };

    // ── Shared field styles
    let input_cls = "w-full bg-zinc-50/60 dark:bg-zinc-900/40 border border-zinc-200 dark:border-zinc-800 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-600 focus:outline-none focus:border-zinc-400 dark:focus:border-zinc-600 transition-colors";

    let label_cls = "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500";

    let action_btn_cls = "bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-700 dark:text-zinc-300 hover:border-zinc-400 dark:hover:border-zinc-600 px-4 py-2.5 rounded-lg font-mono text-[11px] tracking-[0.22em] uppercase transition-colors disabled:opacity-50 disabled:cursor-not-allowed shrink-0";

    let primary_btn_cls = "px-6 py-2.5 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 font-mono text-[11px] tracking-[0.22em] uppercase hover:opacity-90 transition-opacity disabled:opacity-50";

    let status_ok_cls  = "font-mono text-[10px] tracking-[0.22em] uppercase px-3 py-1.5 rounded-lg bg-emerald-50 dark:bg-emerald-400/10 text-emerald-700 dark:text-emerald-300 border border-emerald-200 dark:border-emerald-400/30";
    let status_err_cls = "font-mono text-[10px] tracking-[0.22em] uppercase px-3 py-1.5 rounded-lg bg-red-50 dark:bg-red-500/10 text-red-600 dark:text-red-400 border border-red-200 dark:border-red-500/30";

    rsx! {
        div { class: "flex-1 overflow-y-auto",
            div { class: "max-w-3xl mx-auto px-10 py-12",

                // ── Breadcrumb ────────────────────────────────────────────────
                div { class: "mb-8 flex items-baseline gap-3",
                    Link {
                        to: Route::AgentList {},
                        class: "font-mono text-[11px] tracking-[0.22em] uppercase text-zinc-500 hover:text-zinc-900 dark:hover:text-zinc-100 no-underline",
                        {t!("deploy-back")}
                    }
                    span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                    span { class: "font-mono text-[11px] tracking-[0.28em] uppercase text-amber-600 dark:text-amber-400",
                        "Deploy"
                    }
                }

                // ── Hero ─────────────────────────────────────────────────────
                div { class: "flex items-center gap-5 mb-10 reveal",
                    div { class: "monogram-ring w-20 h-20 rounded-2xl border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display italic text-4xl text-zinc-900 dark:text-zinc-100 shrink-0",
                        "{initial}"
                    }
                    div {
                        span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500", "deploying" }
                        h1 { class: "font-display text-4xl text-zinc-900 dark:text-zinc-100 leading-none mt-1", "{agent.name}" }
                        p { class: "font-display italic text-sm text-zinc-500 mt-2", {t!("deploy-title")} }
                    }
                }

                // ── n8n webhooks ─────────────────────────────────────────────
                section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 flex flex-col gap-5 mb-6 reveal-1",
                    div { class: "flex items-baseline gap-3",
                        span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none", "01" }
                        span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600", "n8n webhooks" }
                        span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                    }

                    div { class: "flex flex-col gap-2",
                        label { class: "{label_cls}", {t!("deploy-webhook-send")} }
                        input {
                            class: "{input_cls}",
                            value: "{edit_n8n_send}",
                            placeholder: "https://your-n8n.com/webhook/send",
                            oninput: move |evt| *edit_n8n_send.write() = evt.value(),
                        }
                    }
                    div { class: "flex flex-col gap-2",
                        label { class: "{label_cls}", {t!("deploy-webhook-receive")} }
                        input {
                            class: "{input_cls}",
                            value: "{edit_n8n_receive}",
                            placeholder: "https://your-n8n.com/webhook/receive",
                            oninput: move |evt| *edit_n8n_receive.write() = evt.value(),
                        }
                    }

                    div { class: "flex items-center gap-3 flex-wrap",
                        button {
                            class: "{action_btn_cls}",
                            disabled: *is_testing.read() || edit_n8n_send.read().trim().is_empty(),
                            onclick: move |_| {
                                let url = edit_n8n_send.read().clone();
                                *is_testing.write() = true;
                                *test_result.write() = "Testing…".to_string();
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
                            p { class: "font-mono text-[10px] tracking-[0.18em] uppercase text-zinc-500", "{test_result}" }
                        }
                    }

                    div { class: "flex gap-3 pt-1",
                        button {
                            class: "{primary_btn_cls}",
                            onclick: save_webhooks,
                            disabled: *is_saving.read(),
                            if *is_saving.read() { {t!("deploy-saving")} } else { {t!("deploy-save")} }
                        }
                        Link {
                            to: Route::AgentList {},
                            class: "flex items-center justify-center px-6 py-2.5 rounded-lg bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 font-mono text-[11px] tracking-[0.22em] uppercase hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors no-underline",
                            {t!("deploy-cancel")}
                        }
                    }
                }

                // ── Evolution API ────────────────────────────────────────────
                section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 flex flex-col gap-5 reveal-2",
                    div { class: "flex items-baseline gap-3",
                        span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none", "02" }
                        span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600", {t!("deploy-evo-title")} }
                        span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                    }

                    div { class: "flex flex-col gap-2",
                        label { class: "{label_cls}", {t!("deploy-evo-url")} }
                        input {
                            r#type: "text",
                            class: "{input_cls}",
                            value: "{evo_url}",
                            placeholder: "http://localhost:8080",
                            oninput: move |evt| *evo_url.write() = evt.value(),
                        }
                    }
                    div { class: "flex flex-col gap-2",
                        label { class: "{label_cls}", {t!("deploy-evo-api-key")} }
                        input {
                            r#type: "password",
                            class: "{input_cls}",
                            value: "{evo_api_key}",
                            placeholder: "••••••••••••••••••••",
                            oninput: move |evt| *evo_api_key.write() = evt.value(),
                        }
                    }

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

                    div { class: "flex flex-col gap-2",
                        label { class: "{label_cls}", {t!("deploy-evo-instance")} }
                        select {
                            class: "{input_cls} cursor-pointer",
                            value: "{evo_instance}",
                            onchange: move |evt| *evo_instance.write() = evt.value(),
                            option { value: "chip-sales", "chip-sales" }
                        }
                    }

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
                            span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500 bg-zinc-50 dark:bg-zinc-800/50 border border-zinc-200 dark:border-zinc-800 px-3 py-1.5 rounded-lg",
                                {t!("deploy-evo-last-verification", time: evo_verify_time.read().clone())}
                            }
                        }
                    }

                    div { class: "flex items-center gap-2 cursor-pointer",
                        input {
                            r#type: "checkbox",
                            id: "ignore-groups",
                            class: "w-4 h-4 accent-zinc-900 dark:accent-zinc-100 cursor-pointer",
                            checked: *ignore_groups.read(),
                            onchange: move |evt| *ignore_groups.write() = evt.checked(),
                        }
                        label {
                            r#for: "ignore-groups",
                            class: "text-sm text-zinc-600 dark:text-zinc-400 cursor-pointer select-none",
                            {t!("deploy-evo-ignore-groups")}
                        }
                    }

                    div { class: "flex flex-col gap-3 pt-1",
                        button {
                            class: "{primary_btn_cls} max-w-xs",
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
