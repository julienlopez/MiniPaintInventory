#![allow(non_snake_case)]

use dioxus::prelude::*;

use closure::closure;

use crate::models::Brand;
use crate::server_functions::get_paints;

#[component]
pub fn PaintsPanel(
    id: Option<String>,
    brands: Vec<Brand>,
    selected_paint: Option<Signal<Option<i32>>>,
) -> Element {
    let brand_filter: Signal<Option<i32>> = use_signal(|| None);
    rsx!(
        div { id, class: "list",
            "Paints"
            BrandsFilters { brands: brands.clone(), brand_filter: brand_filter.clone() }
            PaintList { brand_filter: brand_filter.clone(), selected_paint }
        }
    )
}

#[component]
fn BrandsFilters(brands: Vec<Brand>, brand_filter: Signal<Option<i32>>) -> Element {
    rsx!(
        div { class: "brands_filters",
            for b in brands {
                div {
                    class: if brand_filter() == Some(b.id) { "brand_filter brand_filter_checked" } else { "brand_filter" },
                    onclick: move |_| {
                        if brand_filter() == Some(b.id) {
                            brand_filter.set(None);
                        } else {
                            brand_filter.set(Some(b.id));
                        }
                    },
                    {b.name}
                }
            }
        }
    )
}

#[component]
fn PaintList(
    brand_filter: Signal<Option<i32>>,
    selected_paint: Option<Signal<Option<i32>>>,
) -> Element {
    let paints = use_resource(move || get_paints(brand_filter()));
    match &*paints.read_unchecked() {
        Some(Ok(paints)) => {
            rsx!(
                div { id: "paint_list" }
                for paint in paints {
                    div {
                        class: "paint_box",
                        style: format!("background-color: {};", paint.color),
                        onclick: closure!(
                            clone paint, | _ | { if let Some(selected_paint) = selected_paint.as_mut() { if
                            selected_paint() == Some(paint.id) { selected_paint.set(None); } else {
                            selected_paint.set(Some(paint.id)); } } }
                        ),
                        div { style: format!("color: {}; filter: invert(100%);", paint.color),
                            "{paint.name}"
                        }
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
