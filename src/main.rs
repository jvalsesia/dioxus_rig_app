#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_i18n::{t, prelude::*};
use unic_langid::langid;

pub mod components;
pub mod server_fns;

use components::chat::Chat;
use components::agent_list::AgentList;
use components::manage_agent::ManageAgent;
use components::deploy_agent::DeployAgent;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SidebarLayout)]
    #[route("/")]
    AgentList {},
    #[route("/chat/:id")]
    Chat { id: String },
    #[route("/manage/:id")]
    ManageAgent { id: String },
    #[route("/deploy/:id")]
    DeployAgent { id: String },
}

#[component]
fn SidebarLayout() -> Element {
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
                components::create_agent_modal::CreateAgentModal {
                    onclose: move |_| *show_create_modal.write() = false
                }
            }
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub n8n_webhook_send: Option<String>,
    pub n8n_webhook_receive: Option<String>,
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/agents_wizard_logo.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("pt-BR"))
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("../locales/en-US.ftl"),
            ))
            .with_locale(Locale::new_static(
                langid!("pt-BR"),
                include_str!("../locales/pt-BR.ftl"),
            ))
    });

    let lang_str = i18n.language().to_string();

    let agents_resource = use_resource(move || {
        let l = lang_str.clone();
        async move {
            crate::server_fns::get_agents(l).await.unwrap_or_else(|_| vec![])
        }
    });
    use_context_provider(|| agents_resource);

    // true = dark, false = light; starts dark
    let mut is_dark = use_signal(|| true);
    use_context_provider(|| is_dark);

    // Keep the `dark` class on <html> in sync with the signal
    use_effect(move || {
        let script = if *is_dark.read() {
            "document.documentElement.classList.add('dark');"
        } else {
            "document.documentElement.classList.remove('dark');"
        };
        let _ = document::eval(script);
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
