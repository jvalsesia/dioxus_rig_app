use crate::{Agent, Route};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AgentList() -> Element {
    let agents_resource = use_context::<Resource<Vec<Agent>>>();

    rsx! {
        div { class: "flex-1 overflow-y-auto p-8",
            // Page header
            div { class: "mb-8",
                h2 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", {t!("dashboard-title")} }
                p { class: "text-sm text-zinc-500 mt-1", {t!("dashboard-subtitle")} }
            }

            // Agent grid
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4",
                if let Some(agents) = agents_resource.read().as_ref() {
                    for agent in agents.iter() {
                        div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-5 flex flex-col items-center text-center gap-3 hover:border-violet-300 dark:hover:border-violet-500/50 hover:-translate-y-0.5 transition-all shadow-sm",

                            div { class: "w-14 h-14 rounded-full bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center text-2xl border border-zinc-200 dark:border-zinc-700",
                                span { "🤖" }
                            }

                            h4 { class: "text-sm font-semibold text-zinc-900 dark:text-zinc-100 w-full truncate",
                                "{agent.name}"
                            }
                            p { class: "text-xs text-zinc-500 line-clamp-2 flex-1",
                                {t!("agent-specialty", specialty: agent.specialty.clone())}
                            }

                            div { class: "flex gap-1.5 w-full pt-3 border-t border-zinc-100 dark:border-zinc-800 mt-auto",
                                Link {
                                    to: Route::Chat { id: agent.id.clone() },
                                    class: "flex-1 py-1.5 text-xs font-semibold rounded-lg bg-violet-50 dark:bg-violet-500/10 text-violet-600 dark:text-violet-400 border border-violet-200 dark:border-violet-500/30 hover:bg-violet-100 dark:hover:bg-violet-500/20 text-center transition-all no-underline",
                                    {t!("chat-btn")}
                                }
                                Link {
                                    to: Route::ManageAgent { id: agent.id.clone() },
                                    class: "flex-1 py-1.5 text-xs font-semibold rounded-lg bg-sky-50 dark:bg-sky-400/10 text-sky-600 dark:text-sky-400 border border-sky-200 dark:border-sky-400/30 hover:bg-sky-100 dark:hover:bg-sky-400/20 text-center transition-all no-underline",
                                    {t!("manage-btn")}
                                }
                                Link {
                                    to: Route::DeployAgent { id: agent.id.clone() },
                                    class: "flex-1 py-1.5 text-xs font-semibold rounded-lg bg-emerald-50 dark:bg-emerald-400/10 text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-400/30 hover:bg-emerald-100 dark:hover:bg-emerald-400/20 text-center transition-all no-underline",
                                    {t!("deploy-btn")}
                                }
                            }
                        }
                    }
                } else {
                    p { class: "text-sm text-zinc-500", {t!("loading-agents")} }
                }
            }
        }
    }
}
