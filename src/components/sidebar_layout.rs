use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::{t, prelude::*};

const LOGO: Asset = asset!("/assets/agents_wizard_logo.png");

#[component]
pub fn SidebarLayout() -> Element {
    let mut show_create_modal = use_signal(|| false);
    let mut i18n = i18n();
    let mut is_dark = use_context::<Signal<bool>>();

    rsx! {
        div { class: "flex h-screen overflow-hidden bg-zinc-50 dark:bg-zinc-950",

            // ── Sidebar ──────────────────────────────────────────────────────
            div { class: "w-64 shrink-0 flex flex-col bg-white dark:bg-zinc-900 border-r border-zinc-200 dark:border-zinc-800 p-6",

                // Logo + title
                div { class: "flex flex-col items-center text-center mb-8",
                    img {
                        src: LOGO,
                        alt: "Agents Wizard Logo",
                        class: "w-24 mb-4"
                    }
                    h2 { class: "text-lg font-bold text-zinc-900 dark:text-zinc-100", {t!("app-title")} }
                    p { class: "text-xs text-zinc-500 mt-0.5", {t!("app-subtitle")} }
                }

                // Navigation
                div { class: "flex flex-col gap-2 flex-1",
                    Link {
                        to: Route::AgentList {},
                        class: "flex items-center gap-2.5 px-3 py-2.5 rounded-xl text-sm font-medium text-zinc-500 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-all no-underline",
                        span { class: "text-base", "🏠" }
                        {t!("nav-dashboard")}
                    }
                    button {
                        class: "flex items-center gap-2.5 px-3 py-2.5 rounded-xl text-sm font-medium text-violet-600 dark:text-violet-400 border border-dashed border-violet-300 dark:border-violet-500/40 bg-violet-50 dark:bg-violet-500/5 hover:bg-violet-100 dark:hover:bg-violet-500/15 hover:border-violet-400 dark:hover:border-violet-500 transition-all cursor-pointer w-full",
                        onclick: move |_| *show_create_modal.write() = true,
                        span { class: "text-base", "➕" }
                        {t!("nav-new-agent")}
                    }
                }

                // Language + theme controls
                div { class: "mt-auto flex flex-col gap-2 pt-6 border-t border-zinc-200 dark:border-zinc-800",
                    select {
                        class: "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-zinc-600 dark:text-zinc-400 text-sm rounded-xl px-3 py-2.5 outline-none cursor-pointer",
                        onchange: move |evt| {
                            if let Ok(parsed) = evt.value().parse::<unic_langid::LanguageIdentifier>() {
                                i18n.set_language(parsed);
                            }
                        },
                        option { value: "pt-BR", selected: i18n.language().language.as_str() == "pt", "Português" }
                        option { value: "en-US", selected: i18n.language().language.as_str() == "en", "English" }
                    }
                    button {
                        class: "w-full flex items-center justify-center gap-2 bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-zinc-600 dark:text-zinc-400 text-sm rounded-xl px-3 py-2.5 hover:bg-zinc-200 dark:hover:bg-zinc-700 hover:text-zinc-900 dark:hover:text-zinc-100 transition-all cursor-pointer",
                        onclick: move |_| {
                            let current = *is_dark.read();
                            *is_dark.write() = !current;
                        },
                        if *is_dark.read() {
                            span { "☀️" }
                            "Light Mode"
                        } else {
                            span { "🌙" }
                            "Dark Mode"
                        }
                    }
                }
            }

            // ── Main content ─────────────────────────────────────────────────
            div { class: "flex-1 flex flex-col min-h-0 bg-zinc-50 dark:bg-zinc-950",
                Outlet::<Route> {}
            }

            // ── Create agent modal ────────────────────────────────────────────
            if *show_create_modal.read() {
                crate::components::create_agent_modal::CreateAgentModal {
                    onclose: move |_| *show_create_modal.write() = false
                }
            }
        }
    }
}
