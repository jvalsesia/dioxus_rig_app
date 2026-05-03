#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
pub mod server_fns;

use components::chat::Chat;
use components::agent_list::AgentList;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    AgentList {},
    #[route("/chat/:id")]
    Chat { id: String },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub specialty: String,
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize global state for agents
    use_context_provider(|| {
        Signal::new(vec![
            Agent {
                id: "1".to_string(),
                name: "General Assistant".to_string(),
                specialty: "You are a helpful, friendly, and highly capable general AI assistant. You provide concise and accurate answers.".to_string(),
            }
        ])
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-wrapper",
            Router::<Route> {}
        }
    }
}
