use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};

const LOGO: Asset = asset!("/assets/agents_wizard_logo.png");

#[component]
pub fn SidebarLayout() -> Element {
    let mut i18n = i18n();
    let mut is_dark = use_context::<Signal<bool>>();

    rsx! {
        div { class: "flex h-screen overflow-hidden bg-zinc-50 dark:bg-zinc-950 font-sans",

            // ── Sidebar ───────────────────────────────────────────────────────
            aside { class: "w-72 shrink-0 flex flex-col bg-white dark:bg-zinc-900/40 border-r border-zinc-200/80 dark:border-zinc-800/80 px-6 pt-8 pb-6",

                // Brand block
                div { class: "flex items-center gap-3 mb-10",
                    img {
                        src: LOGO,
                        alt: "Agents Wizard Logo",
                        class: "w-12 h-12 rounded-xl ring-1 ring-zinc-200 dark:ring-zinc-800",
                    }
                    div { class: "flex flex-col",
                        h2 { class: "font-display font-semibold text-xl text-zinc-900 dark:text-zinc-100 leading-none tracking-tight",
                            {t!("app-title")}
                        }
                        p { class: "font-mono text-[10px] tracking-[0.18em] uppercase text-zinc-500 mt-1.5",
                            {t!("app-subtitle")}
                        }
                    }
                }

                // Navigation
                nav { class: "flex flex-col gap-1",
                    Link {
                        to: Route::AgentList {},
                        class: "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800/60 transition-colors no-underline",
                        span { class: "text-base", "◇" }
                        span { class: "font-medium", {t!("nav-dashboard")} }
                    }
                    Link {
                        to: Route::CreateAgent {},
                        class: "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm bg-violet-50/70 dark:bg-violet-500/10 border border-dashed border-violet-300/80 dark:border-violet-500/30 text-violet-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-500/15 transition-colors no-underline",
                        span { class: "text-base", "+" }
                        span { class: "font-medium", {t!("nav-new-agent")} }
                    }
                    Link {
                        to: Route::Skills {},
                        class: "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800/60 transition-colors no-underline",
                        span { class: "text-base", "◈" }
                        span { class: "font-medium", {t!("nav-skills")} }
                    }
                }

                // Footer controls
                div { class: "mt-auto pt-6 border-t border-zinc-200 dark:border-zinc-800 flex flex-col gap-2",
                    select {
                        class: "w-full bg-zinc-50 dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 text-zinc-700 dark:text-zinc-300 text-sm rounded-lg px-3 py-2 outline-none cursor-pointer font-mono tracking-wide",
                        onchange: move |evt| {
                            if let Ok(parsed) = evt.value().parse::<unic_langid::LanguageIdentifier>() {
                                i18n.set_language(parsed);
                            }
                        },
                        option { value: "pt-BR", selected: i18n.language().language.as_str() == "pt", "PT-BR" }
                        option { value: "en-US", selected: i18n.language().language.as_str() == "en", "EN-US" }
                    }
                    button {
                        class: "w-full flex items-center justify-between gap-2 bg-zinc-50 dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 text-zinc-700 dark:text-zinc-300 text-sm rounded-lg px-3 py-2 hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors cursor-pointer",
                        onclick: move |_| {
                            let current = *is_dark.read();
                            *is_dark.write() = !current;
                        },
                        span { class: "font-medium",
                            if *is_dark.read() { "Light mode" } else { "Dark mode" }
                        }
                        span { class: "text-base",
                            if *is_dark.read() { "☀" } else { "☾" }
                        }
                    }
                }
            }

            // ── Main content ─────────────────────────────────────────────────
            main { class: "flex-1 flex flex-col min-h-0 bg-zinc-50 dark:bg-zinc-950 bg-paper",
                Outlet::<Route> {}
            }
        }
    }
}
