#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{models::StorageBox, server_functions::get_boxes};

use dioxus::logger::tracing::info;

#[component]
pub fn Home() -> Element {
    rsx!(
        link { rel: "stylesheet", href: asset!("assets/home.css") }
        div { class: "page_title", "Mini Paint Inventory" }
        div { id: "home_content", BoxPanel {} }
    )
}

#[component]
fn BoxPanel() -> Element {
    rsx!(
        div { class: "box_panel",
            div { class: "box_panel_header",
                span { class: "box_page_title", "Storage Boxes " }
                div { class: "add_box_button", "+" }
            }
            BoxList {}
        }
    )
}

#[component]
fn BoxList() -> Element {
    let boxes = use_resource(get_boxes);
    match &*boxes.read_unchecked() {
        Some(Ok(boxes)) => {
            rsx!(
                div { class: "box_list",
                    for r#box in boxes {
                        BoxSummary { r#box: r#box.clone() }
                    }
                }
            )
        }
        Some(Err(_)) => {
            rsx! { "Error" }
        }
        None => {
            rsx! { "..." }
        }
    }
}

#[component]
fn BoxSummary(r#box: StorageBox) -> Element {
    let mut collapsed = use_signal(|| true);
    rsx!(
        div { class: "box",
            div {
                class: "box_header",
                onclick: move |_| { *collapsed.write() = !collapsed() },
                div { class: "box_name_group",
                    div { class: "box_name", "{r#box.name}" }
                    div { class: "box_flags", "{r#box.flags}" }
                }
                div { class: "box_capacity", "0 / {r#box.capacity}" }
                div {
                    class: "add_paint_to_box_button",
                    onclick: |event| {
                        event.stop_propagation();
                        info!("plop");
                    },
                    "+"
                }
                div { class: "box_collapsed_indicator",
                    if collapsed() {
                        "\\/"
                    } else {
                        "/\\"
                    }
                }
            }
            div {
                class: "box_body",
                style: if collapsed() { "display: none;" } else { "display: block;" },
                "Blah"
            }
        }
    )
}
