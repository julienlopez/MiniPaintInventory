#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Admin() -> Element {
    rsx!(
        link { rel: "stylesheet", href: "admin.css" }
        div { "Administration page" }
        div {
            id: "admin_content",
            style: "",
            div {
                id: "brand_panel",
                class: "list",
                "Brands"
                div {
                    id: "brand_list"
                }
            }
            div {
                id: "paint_panel",
                class: "list",
                "Paints"
                div {
                    id: "paint_list"
                }
            }
        }
    )
}
