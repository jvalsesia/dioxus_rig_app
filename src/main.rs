#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_i18n::{t, prelude::*};
use unic_langid::langid;

pub mod components;
pub mod server_fns;

use components::chat::Chat;
use components::agent_list::AgentList;
use components::manage_agent::ManageAgent;

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
}

#[component]
fn SidebarLayout() -> Element {
    let mut show_create_modal = use_signal(|| false);
    let mut i18n = i18n();
    let mut theme = use_context::<Signal<String>>();

    rsx! {
        div { class: "dashboard-layout",
            // Sidebar
            div { class: "sidebar",
                div { class: "sidebar-header",
                    img { 
                        src: LOGO, 
                        alt: "Agents Wizard Logo", 
                        style: "width: 100%; max-width: 120px; height: auto; display: block; margin-bottom: 1rem;" 
                    }
                    h2 { {t!("app-title")} }
                    p { {t!("app-subtitle")} }
                }
                div { class: "sidebar-nav",
                    Link {
                        to: Route::AgentList {},
                        class: "nav-btn",
                        span { class: "btn-icon", "🏠" }
                        {t!("nav-dashboard")}
                    }
                    button {
                        class: "new-agent-btn",
                        onclick: move |_| *show_create_modal.write() = true,
                        span { class: "btn-icon", "➕" }
                        {t!("nav-new-agent")}
                    }
                }
                
                // Language Switcher & Theme Switcher
                div { style: "margin-top: auto; padding-top: 2rem; display: flex; flex-direction: column; gap: 0.75rem;",
                    select {
                        class: "lang-switcher",
                        onchange: move |evt| {
                            if let Ok(parsed_langid) = evt.value().parse::<unic_langid::LanguageIdentifier>() {
                                i18n.set_language(parsed_langid);
                            }
                        },
                        option { value: "pt-BR", selected: i18n.language().language.as_str() == "pt", "Português" }
                        option { value: "en-US", selected: i18n.language().language.as_str() == "en", "English" }
                    }
                    button {
                        class: "lang-switcher",
                        style: "display: flex; align-items: center; justify-content: center; gap: 0.5rem;",
                        onclick: move |_| {
                            let current = theme.read().clone();
                            if current == "dark-mode" {
                                *theme.write() = "light-mode".to_string();
                            } else {
                                *theme.write() = "dark-mode".to_string();
                            }
                        },
                        if *theme.read() == "dark-mode" {
                            span { class: "btn-icon", "☀️" }
                            "Light Mode"
                        } else {
                            span { class: "btn-icon", "🌙" }
                            "Dark Mode"
                        }
                    }
                }
            }

            // Main Content Area where the route's components will render
            div { class: "main-content",
                Outlet::<Route> {}
            }
            
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

const MAIN_CSS: Asset = asset!("/assets/main.css");
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

    // Initialize global state for agents from LanceDB
    let agents_resource = use_resource(move || {
        let l = lang_str.clone();
        async move {
            crate::server_fns::get_agents(l).await.unwrap_or_else(|_| vec![])
        }
    });
    
    use_context_provider(|| agents_resource);

    let theme = use_signal(|| "dark-mode".to_string());
    use_context_provider(|| theme);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-wrapper {theme}",
            Router::<Route> {}
        }
    }
}
