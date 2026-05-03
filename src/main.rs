#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
pub mod server_fns;

use components::chat::Chat;

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-wrapper",
            Chat {}
        }
    }
}
