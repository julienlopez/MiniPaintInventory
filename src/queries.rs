use crate::models::{Brand, Paint, StorageBox};
use crate::server_side_error::Result;
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn list_brands(connection: &mut PgConnection) -> Result<Vec<Brand>> {
    use crate::schema::brands::dsl::*;
    Ok(brands.select(Brand::as_select()).load(connection)?)
}

pub fn list_paints(connection: &mut PgConnection, brand_filter: Option<i32>) -> Result<Vec<Paint>> {
    use crate::schema::paints::dsl::*;
    if let Some(brand_id) = brand_filter {
        Ok(paints
            .filter(brand.eq(brand_id))
            .select(Paint::as_select())
            .load(connection)?)
    } else {
        Ok(paints.select(Paint::as_select()).load(connection)?)
    }
}

pub fn list_boxes(connection: &mut PgConnection) -> Result<Vec<StorageBox>> {
    use crate::schema::storage_boxes::dsl::*;
    Ok(storage_boxes
        .select(StorageBox::as_select())
        .load(connection)?)
}
