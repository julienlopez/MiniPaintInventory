#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
// use dioxus_logger::tracing;

use crate::models::Brand;
use crate::server_functions::{get_brands, get_paints};

#[component]
pub fn Admin() -> Element {
    let brands = use_resource(get_brands);
    match &*brands.read_unchecked() {
        Some(Ok(brands)) => {
            rsx!(
                link { rel: "stylesheet", href: asset!("assets/admin.css") }
                div { "Administration page" }
                div { id: "admin_content",
                    BrandsPanel { brands: brands.clone() }
                    PaintsPanel { brands: brands.clone() }
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
fn BrandsPanel(brands: Vec<Brand>) -> Element {
    rsx!(
        div { id: "brand_panel", class: "list",
            "Brands:"
            BrandList { brands: brands.clone() }
        }
    )
}

#[component]
fn BrandList(brands: Vec<Brand>) -> Element {
    rsx! {
        div { id: "brand_list",
            for b in brands {
                div { class: "brand", {b.name} }
            }
        }
    }
}

#[component]
fn PaintsPanel(brands: Vec<Brand>) -> Element {
    let brand_filter: Signal<Option<i32>> = use_signal(|| None);
    rsx!(
        div { id: "paint_panel", class: "list",
            "Paints"
            BrandsFilters { brands: brands.clone(), brand_filter: brand_filter.clone() }
            PaintList { brand_filter: brand_filter.clone() }
        }
    )
}

#[component]
fn BrandsFilters(brands: Vec<Brand>, brand_filter: Signal<Option<i32>>) -> Element {
    rsx!(
        div { id: "brands_filters",
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
fn PaintList(brand_filter: Signal<Option<i32>>) -> Element {
    let paints = use_resource(move || get_paints(brand_filter()));
    match &*paints.read_unchecked() {
        Some(Ok(paints)) => {
            println!("Paints: {:?}", paints.len());
            tracing::info!("Paints: {:?}", paints.len());
            rsx!(
                div { id: "paint_list" }
                for paint in paints {
                    div {
                        class: "paint_box",
                        style: format!("background-color: {};", paint.color),
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
