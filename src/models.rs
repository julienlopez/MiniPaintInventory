use diesel::prelude::*;

use crate::schema::*;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Brand {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name=brands)]
pub struct NewBrand {
    pub name: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Paint {
    pub id: i32,
    pub name: String,
    pub brand: i32,
    pub color: String,
}

#[derive(Insertable)]
#[diesel(table_name=paints)]
pub struct NewPaint {
    pub name: String,
    pub brand: i32,
    pub color: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct StorageBox {
    pub id: i32,
    pub name: String,
    pub flags: String,
}

#[derive(Insertable)]
#[diesel(table_name=storage_boxes)]
pub struct NewStorageBox {
    pub name: String,
    pub flags: String,
}
