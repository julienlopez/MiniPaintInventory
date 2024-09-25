#![allow(non_snake_case)]

use dioxus::prelude::*;
// use dioxus_logger::tracing;

use crate::models::Brand;
use crate::server_functions::get_brands;

#[component]
pub fn Admin() -> Element {
    rsx!(
        link { rel: "stylesheet", href: "admin.css" }
        div { "Administration page" },
        div {
            id: "admin_content",
            BrandsPanel{},
            PaintsPanel{},
        }
    )
}

#[component]
fn BrandsPanel() -> Element {
    let brands = use_resource(get_brands);
    rsx!(
        div {
            id: "brand_panel",
            class: "list",
            "Brands:",
            match &*brands.read_unchecked()  {
                Some(Ok(brands)) => {
                    rsx!{ BrandList{
                        brands: brands.clone()
                    } }
                },
                Some(Err(_)) => {
                    rsx!{ "Error" }
                },
                None => { rsx!{ "..." } }
            }
        }
    )
}

#[component]
fn BrandList(brands: Vec<Brand>) -> Element {
    rsx! {
        div {
            id: "brand_list",
            for b in brands {
                div { class: "brand", {b.name} }
            }
        }
    }
}

#[component]
fn PaintsPanel() -> Element {
    rsx!(
        div {
            id: "paint_panel",
            class: "list",
            "Paints"
            div {
                id: "paint_list"
            }
        }
    )
}
