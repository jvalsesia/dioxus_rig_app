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

use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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
    // Initialize global state for agents from LanceDB
    let agents_resource = use_resource(|| async move {
        crate::server_fns::get_agents().await.unwrap_or_else(|_| vec![])
    });
    
    use_context_provider(|| agents_resource);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-wrapper",
            Router::<Route> {}
        }
    }
}
