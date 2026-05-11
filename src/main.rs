#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::langid;

pub mod components;
pub mod server_fns;

use components::agent_list::AgentList;
use components::chat::Chat;
use components::deploy_agent::DeployAgent;
use components::manage_agent::ManageAgent;
use components::sidebar_layout::SidebarLayout;
use components::skills::Skills;
use serde::{Deserialize, Serialize};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
    #[route("/skills")]
    Skills {},
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub n8n_webhook_send: Option<String>,
    pub n8n_webhook_receive: Option<String>,
}

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
            crate::server_fns::get_agents(l)
                .await
                .unwrap_or_else(|_| vec![])
        }
    });
    use_context_provider(|| agents_resource);

    // true = dark, false = light; starts dark
    let is_dark = use_signal(|| true);
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
