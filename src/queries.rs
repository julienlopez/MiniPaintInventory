use crate::models::{Brand, Paint, StorageBox, StorageBoxWithContent};
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

pub fn get_box_with_content(
    connection: &mut PgConnection,
    box_id: i32,
) -> Result<StorageBoxWithContent> {
    use crate::models::PaintsToStorageBoxes;
    use crate::schema::paints::dsl::*;
    use crate::schema::storage_boxes::dsl::*;

    let s_box = crate::schema::storage_boxes::table
        .filter(crate::schema::storage_boxes::id.eq(box_id))
        .select(StorageBox::as_select())
        .get_result(connection)?;

    let paint_list = PaintsToStorageBoxes::belonging_to(&s_box)
        .inner_join(crate::schema::paints::table)
        .select(Paint::as_select())
        .load(connection)?;

    let brands: Vec<Brand> = list_brands(connection)?;

    Ok(StorageBoxWithContent {
        storage_box: s_box,
        content: paint_list
            .into_iter()
            .map(|p| {
                let b = brands.iter().find(|b| b.id == p.brand).unwrap();
                crate::models::PaintWithBrand {
                    paint: p,
                    brand: b.clone(),
                }
            })
            .collect(),
    })
}
