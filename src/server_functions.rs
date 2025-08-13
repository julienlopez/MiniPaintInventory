use dioxus::prelude::*;

use crate::models::{Brand, Paint};

#[server(GetBrands)]
pub async fn get_brands() -> Result<Vec<Brand>, ServerFnError> {
    Ok(vec![
        Brand {
            id: 0,
            name: "Citadel".to_string(),
        },
        Brand {
            id: 1,
            name: "Army Painter".to_string(),
        },
    ])
}

#[server(GetPaints)]
pub async fn get_paints(brand_filter: Option<i32>) -> Result<Vec<Paint>, ServerFnError> {
    Ok(vec![
        Paint {
            id: 0,
            name: "Abaddon Black".to_string(),
            brand: 0,
            color: "#000000".to_string(),
        },
        Paint {
            id: 1,
            name: "Corax White".to_string(),
            brand: 0,
            color: "#FFFFFF".to_string(),
        },
        Paint {
            id: 2,
            name: "Wraithbone".to_string(),
            brand: 1,
            color: "#E0D8C0".to_string(),
        },
        Paint {
            id: 3,
            name: "Skeleton Bone".to_string(),
            brand: 1,
            color: "#D8C0A0".to_string(),
        },
    ])
}
