use dioxus::prelude::*;

use crate::models::{Brand, NewStorageBox, Paint, StorageBox, StorageBoxWithContent};

#[cfg(feature = "server")]
use diesel::r2d2;

#[cfg(feature = "server")]
thread_local! {
    pub static DB_POOL: r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>> = {
        dotenv::dotenv().expect ("Failed to read .env file");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        println!("Connecting to database at: {}", database_url);
        let manager = r2d2::ConnectionManager::<diesel::PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    };
}

#[server(GetBrands)]
pub async fn get_brands() -> Result<Vec<Brand>, ServerFnError> {
    DB_POOL.with(|pool| {
        let conn = &mut pool.get().map_err(|e| ServerFnError::new(e.to_string()))?;
        crate::queries::list_brands(conn).map_err(|e| {
            println!("Error fetching brands: {}", e);
            ServerFnError::new(e.to_string())
        })
    })
}

#[server(GetPaints)]
pub async fn get_paints(brand_filter: Option<i32>) -> Result<Vec<Paint>, ServerFnError> {
    DB_POOL.with(|pool| {
        let conn = &mut pool.get().map_err(|e| ServerFnError::new(e.to_string()))?;
        crate::queries::list_paints(conn, brand_filter).map_err(|e| {
            println!("Error fetching paints: {}", e);
            ServerFnError::new(e.to_string())
        })
    })
}

#[server(GetBoxes)]
pub async fn get_boxes() -> Result<Vec<StorageBox>, ServerFnError> {
    DB_POOL.with(|pool| {
        let conn = &mut pool.get().map_err(|e| ServerFnError::new(e.to_string()))?;
        crate::queries::list_boxes(conn).map_err(|e| {
            println!("Error fetching storage boxes: {}", e);
            ServerFnError::new(e.to_string())
        })
    })
}

#[server(PostBox)]
pub async fn create_box(new_box: NewStorageBox) -> Result<StorageBox, ServerFnError> {
    todo!("TODO create_box")
}

#[server(GetBoxWithContent)]
pub async fn get_box_with_content(box_id: i32) -> Result<StorageBoxWithContent, ServerFnError> {
    DB_POOL.with(|pool| {
        let conn = &mut pool.get().map_err(|e| ServerFnError::new(e.to_string()))?;
        crate::queries::get_box_with_content(conn, box_id).map_err(|e| {
            println!("Error fetching storage box content #{box_id}: {}", e);
            ServerFnError::new(e.to_string())
        })
    })
}
