use dioxus::prelude::*;

use crate::models::Brand;

#[server(GetServerData)]
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
