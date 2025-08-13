#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        link { rel: "stylesheet", href: asset!("assets/home.css") }
        div { "Mini Paint Inventory" }
    )
}
