#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::models::Brand;
use crate::server_functions::get_brands;
use crate::ui::components::paints_panel::PaintsPanel;

#[component]
pub fn Admin() -> Element {
    let brands = use_resource(get_brands);
    match &*brands.read_unchecked() {
        Some(Ok(brands)) => {
            rsx!(
                link { rel: "stylesheet", href: asset!("assets/admin.css") }
                div { class: "page_title", "Administration page" }
                div { id: "admin_content",
                    BrandsPanel { brands: brands.clone() }
                    PaintsPanel { id: "paint_panel", brands: brands.clone() }
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
