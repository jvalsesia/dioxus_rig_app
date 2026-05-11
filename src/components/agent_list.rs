use crate::{Agent, Route};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AgentList() -> Element {
    let agents_resource = use_context::<Resource<Vec<Agent>>>();
    let agents_opt = agents_resource.read();
    let count = agents_opt.as_ref().map(|a| a.len()).unwrap_or(0);

    rsx! {
        div { class: "flex-1 overflow-y-auto",
            div { class: "max-w-7xl mx-auto px-10 py-12",

                // ── Header ────────────────────────────────────────────────────
                header { class: "mb-12 reveal flex items-end justify-between gap-6 flex-wrap",
                    div {
                        h1 { class: "font-display text-6xl font-semibold text-zinc-900 dark:text-zinc-100 leading-[0.95] tracking-tight",
                            {t!("dashboard-title")}
                        }
                        p { class: "mt-3 text-lg text-zinc-500 dark:text-zinc-400 max-w-2xl",
                            {t!("dashboard-subtitle")}
                        }
                    }
                    div { class: "flex items-center gap-2 px-4 py-2 rounded-full bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800",
                        span { class: "w-2 h-2 rounded-full bg-emerald-500" }
                        span { class: "font-mono text-xs text-zinc-600 dark:text-zinc-300", "{count} active" }
                    }
                }

                // ── Grid ──────────────────────────────────────────────────────
                if let Some(agents) = agents_opt.as_ref() {
                    if agents.is_empty() {
                        EmptyAgents {}
                    } else {
                        div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5",
                            for agent in agents.iter() {
                                AgentCard { agent: agent.clone() }
                            }
                        }
                    }
                } else {
                    LoadingAgents {}
                }
            }
        }
    }
}

#[component]
fn AgentCard(agent: Agent) -> Element {
    let initial = agent.name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "·".into());
    rsx! {
        article { class: "group relative bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 flex flex-col gap-4 hover:border-zinc-300 dark:hover:border-zinc-700 hover:-translate-y-1 transition-all reveal",

            // status pip
            div { class: "flex items-center justify-end",
                span { class: "font-mono text-[10px] tracking-wider uppercase text-emerald-600 dark:text-emerald-400 flex items-center gap-1.5",
                    span { class: "inline-block w-1.5 h-1.5 rounded-full bg-emerald-500 pulse-violet" }
                    "online"
                }
            }

            // avatar + name
            div { class: "flex items-start gap-4",
                div { class: "monogram-ring w-14 h-14 rounded-2xl border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display font-semibold text-2xl text-zinc-900 dark:text-zinc-100 shrink-0",
                    "{initial}"
                }
                div { class: "flex-1 min-w-0",
                    h3 { class: "font-display font-semibold text-2xl text-zinc-900 dark:text-zinc-100 leading-tight truncate tracking-tight",
                        "{agent.name}"
                    }
                    p { class: "font-mono text-[10px] tracking-wider uppercase text-zinc-500 mt-1",
                        "agent · OCEAN-tuned"
                    }
                }
            }

            // specialty
            p { class: "text-sm text-zinc-600 dark:text-zinc-400 leading-relaxed line-clamp-3",
                "{agent.specialty}"
            }

            div { class: "rule-hairline" }

            // actions
            div { class: "flex gap-1.5",
                Link {
                    to: Route::Chat { id: agent.id.clone() },
                    class: "flex-1 py-2 text-xs font-medium rounded-lg bg-violet-600 text-white hover:bg-violet-500 text-center transition-colors no-underline",
                    {t!("chat-btn")}
                }
                Link {
                    to: Route::ManageAgent { id: agent.id.clone() },
                    class: "flex-1 py-2 text-xs font-medium rounded-lg bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 hover:border-zinc-400 dark:hover:border-zinc-600 text-center transition-colors no-underline",
                    {t!("manage-btn")}
                }
                Link {
                    to: Route::DeployAgent { id: agent.id.clone() },
                    class: "flex-1 py-2 text-xs font-medium rounded-lg bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 hover:border-zinc-400 dark:hover:border-zinc-600 text-center transition-colors no-underline",
                    {t!("deploy-btn")}
                }
            }
        }
    }
}

#[component]
fn LoadingAgents() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5",
            for i in 0..3 {
                div { key: "{i}",
                    class: "h-56 rounded-2xl border border-dashed border-zinc-200 dark:border-zinc-800 bg-white/40 dark:bg-zinc-900/30 flex items-center justify-center",
                    p { class: "text-sm text-zinc-500", {t!("loading-agents")} }
                }
            }
        }
    }
}

#[component]
fn EmptyAgents() -> Element {
    rsx! {
        div { class: "rounded-2xl border border-dashed border-zinc-300 dark:border-zinc-800 bg-white/60 dark:bg-zinc-900/40 px-10 py-16 flex flex-col items-center text-center max-w-xl mx-auto reveal",
            div { class: "monogram-ring w-16 h-16 rounded-2xl border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display text-3xl text-zinc-900 dark:text-zinc-100 mb-5",
                "+"
            }
            h3 { class: "font-display font-semibold text-3xl text-zinc-900 dark:text-zinc-100 mb-3 tracking-tight",
                "No agents yet"
            }
            p { class: "text-sm text-zinc-500 mb-6 max-w-sm",
                "Author your first agent — name it, give it a voice and a personality, then send it into the world."
            }
            Link {
                to: Route::CreateAgent {},
                class: "px-6 py-2.5 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 text-sm font-medium hover:opacity-90 transition-opacity no-underline",
                "Create your first agent"
            }
        }
    }
}
