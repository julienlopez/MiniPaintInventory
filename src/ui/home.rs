#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::server_functions::get_brands;
use crate::ui::components::paints_panel::PaintsPanel;
use crate::{
    models::StorageBox,
    server_functions::{add_paint_to_box, get_box_with_content, get_boxes},
};

use dioxus::logger::tracing::info;

use dioxus_free_icons::icons::md_navigation_icons::MdClose;
use dioxus_free_icons::Icon;

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
    let mut show_add_paint_to_box = use_signal(|| false);
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
                    onclick: move |event| {
                        event.stop_propagation();
                        show_add_paint_to_box.set(true);
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
                BoxContent { box_id: r#box.id }
            }
            if show_add_paint_to_box() {
                AddPaintToBoxPopup { show_add_paint_to_box, r#box: r#box.clone() }
            }
        }
    )
}

#[component]
fn AddPaintToBoxPopup(show_add_paint_to_box: Signal<bool>, r#box: StorageBox) -> Element {
    let brands = use_resource(get_brands);
    let selected_paint = use_signal(|| None as Option<i32>);
    rsx!(
        div { class: "add_paint_to_box_modal",
            div { class: "modal_background" }
            div { class: "add_paint_to_box_modal_content",
                div {
                    class: "add_paint_to_box_modal_close",
                    onclick: move |_| {
                        show_add_paint_to_box.set(false);
                    },
                    Icon { icon: MdClose }
                }
                div { class: "add_paint_to_box_form",
                    div { "Add paint to : {r#box.name}" }
                    match &*brands.read_unchecked() {
                        Some(Ok(brands)) => {
                            rsx! {
                                PaintsPanel {
                                    id: "add_paint_to_box_paints_panel",
                                    brands: brands.clone(),
                                    selected_paint,
                                }
                            }
                        }
                        Some(Err(_)) => {
                            rsx! { "Error" }
                        }
                        None => {
                            rsx! { "..." }
                        }
                    }
                    input {
                        class: "confirm_add_paint_to_box_button",
                        onclick: move |_| {
                            info!("Add paint button clicked");
                            if let Some(paint) = selected_paint() {
                                spawn(async move {
                                    info!("Add paint button async");
                                    match add_paint_to_box(r#box.id, paint).await {
                                        Ok(_) => info!("Paint added successfully"),
                                        Err(e) => info!("Error adding paint to box: {}", e),
                                    }
                                });
                            }
                        },
                        disabled: selected_paint().is_none(),
                        r#type: "submit",
                        value: "Add Paint",
                    }
                }
            }
        }
    )
}
#[component]
fn BoxContent(box_id: i32) -> Element {
    let content = use_resource(move || get_box_with_content(box_id));
    match &*content.read_unchecked() {
        Some(Ok(content)) => {
            rsx!(div {
                class: "box_content"
            })
        }
        Some(Err(_)) => {
            rsx! { "Error" }
        }
        None => {
            rsx! { "..." }
        }
    }
}
