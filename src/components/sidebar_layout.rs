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
            aside { class: "w-72 shrink-0 flex flex-col bg-white dark:bg-zinc-900/40 border-r border-zinc-200/80 dark:border-zinc-800/80 px-6 pt-8 pb-6 relative",

                // Faint vertical rule at far left for editorial frame
                span { class: "absolute left-0 top-12 bottom-12 w-px bg-gradient-to-b from-transparent via-amber-400/50 to-transparent" }

                // Brand block
                div { class: "flex flex-col items-start mb-10",
                    div { class: "flex items-center gap-3 mb-4",
                        div { class: "relative",
                            img {
                                src: LOGO,
                                alt: "Agents Wizard Logo",
                                class: "w-12 h-12 rounded-xl ring-1 ring-zinc-200 dark:ring-zinc-800",
                            }
                            span { class: "absolute -top-1 -right-1 font-mono text-[9px] tracking-widest text-amber-600 dark:text-amber-400 bg-white dark:bg-zinc-900 border border-amber-300/60 dark:border-amber-400/30 rounded-full px-1.5 py-0.5",
                                "v0.1"
                            }
                        }
                        div { class: "flex flex-col",
                            span { class: "font-mono text-[10px] tracking-[0.22em] text-zinc-500 uppercase",
                                "Nº 001"
                            }
                            h2 { class: "font-display italic font-medium text-2xl text-zinc-900 dark:text-zinc-100 leading-none",
                                {t!("app-title")}
                            }
                        }
                    }
                    p { class: "font-mono text-[10px] tracking-[0.18em] uppercase text-zinc-500",
                        {t!("app-subtitle")}
                    }
                    div { class: "rule-hairline w-24 mt-4" }
                }

                // Navigation
                nav { class: "flex flex-col gap-1",
                    span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-400 dark:text-zinc-600 mb-2",
                        "01 — Navigation"
                    }

                    Link {
                        to: Route::AgentList {},
                        class: "group flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800/60 transition-colors no-underline",
                        span { class: "font-mono text-[10px] text-zinc-400 dark:text-zinc-600 group-hover:text-amber-500 transition-colors", "01" }
                        span { class: "font-display italic text-base", {t!("nav-dashboard")} }
                    }
                    Link {
                        to: Route::CreateAgent {},
                        class: "group flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm bg-violet-50/70 dark:bg-violet-500/10 border border-dashed border-violet-300/80 dark:border-violet-500/30 text-violet-700 dark:text-violet-300 hover:bg-violet-100 dark:hover:bg-violet-500/15 transition-colors no-underline",
                        span { class: "font-mono text-[10px] text-violet-500/80 dark:text-violet-400/80", "02" }
                        span { class: "font-display italic text-base", {t!("nav-new-agent")} }
                        span { class: "ml-auto font-mono text-[10px] tracking-widest text-violet-500/70", "+ NEW" }
                    }
                    Link {
                        to: Route::Skills {},
                        class: "group flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800/60 transition-colors no-underline",
                        span { class: "font-mono text-[10px] text-zinc-400 dark:text-zinc-600 group-hover:text-amber-500 transition-colors", "03" }
                        span { class: "font-display italic text-base", {t!("nav-skills")} }
                    }
                }

                // Editorial filler note
                div { class: "mt-8 mb-auto px-3",
                    p { class: "font-display italic text-xs leading-relaxed text-zinc-500 dark:text-zinc-500",
                        "“Each agent is a small constellation — a name, a voice, and a handful of skills."
                    }
                    p { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-amber-600/80 dark:text-amber-400/80 mt-2",
                        "— from the manual"
                    }
                }

                // Footer controls
                div { class: "mt-auto pt-6 border-t border-zinc-200 dark:border-zinc-800 flex flex-col gap-2",
                    label { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-400 dark:text-zinc-600",
                        "02 — Locale"
                    }
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
                        span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500",
                            if *is_dark.read() { "Light" } else { "Dark" }
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
