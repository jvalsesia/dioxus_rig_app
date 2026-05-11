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

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub emotional_stability: f32,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            emotional_stability: 0.5,
        }
    }
}

impl Personality {
    /// Build a natural-language descriptor block appended to the agent's
    /// specialty so the LLM adopts a consistent voice grounded in the
    /// Five-Factor (OCEAN) model.
    pub fn describe(&self) -> String {
        fn line(label: &str, low: &str, high: &str, v: f32) -> Option<String> {
            if (v - 0.5).abs() < 0.15 {
                return None;
            }
            let intensity = if v >= 0.85 || v <= 0.15 { "very " } else { "" };
            let word = if v > 0.5 { high } else { low };
            Some(format!("- {}: {}{}", label, intensity, word))
        }
        let mut lines = Vec::new();
        if let Some(l) = line("Openness", "conventional and practical", "curious, imaginative, open to new ideas", self.openness) { lines.push(l); }
        if let Some(l) = line("Conscientiousness", "spontaneous and flexible", "organized, diligent, detail-oriented", self.conscientiousness) { lines.push(l); }
        if let Some(l) = line("Extraversion", "reserved and reflective", "outgoing, energetic, talkative", self.extraversion) { lines.push(l); }
        if let Some(l) = line("Agreeableness", "blunt and challenging", "warm, cooperative, empathetic", self.agreeableness) { lines.push(l); }
        if let Some(l) = line("Emotional Stability", "sensitive and intense", "calm, composed, resilient under pressure", self.emotional_stability) { lines.push(l); }
        if lines.is_empty() {
            String::new()
        } else {
            format!("\n\nPersonality traits (Five-Factor Model) you must consistently express in tone and word choice:\n{}", lines.join("\n"))
        }
    }
}

pub fn compose_preamble(specialty: &str, personality: &Personality) -> String {
    format!("{}{}", specialty, personality.describe())
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub n8n_webhook_send: Option<String>,
    pub n8n_webhook_receive: Option<String>,
    #[serde(default)]
    pub personality: Personality,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
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
