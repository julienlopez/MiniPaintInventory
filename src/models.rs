#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg(feature = "server")]
use crate::schema::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
pub struct Brand {
    pub id: i32,
    pub name: String,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name=brands))]
pub struct NewBrand {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
pub struct Paint {
    pub id: i32,
    pub name: String,
    pub brand: i32,
    pub color: String,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name=paints))]
pub struct NewPaint {
    pub name: String,
    pub brand: i32,
    pub color: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name=storage_boxes))]
pub struct StorageBox {
    pub id: i32,
    pub name: String,
    pub flags: String,
    pub capacity: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name=storage_boxes))]
pub struct NewStorageBox {
    pub name: String,
    pub flags: String,
    pub capacity: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct StorageBoxWithContent {
    pub storage_box: StorageBox,
    pub content: Vec<PaintWithBrand>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct PaintWithBrand {
    pub paint: Paint,
    pub brand: Brand,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Selectable, Queryable, Associations)
)]
#[cfg_attr(feature = "server", diesel(belongs_to(Paint, foreign_key = paint)))]
#[cfg_attr(feature = "server", diesel(belongs_to(StorageBox, foreign_key = storage_box)))]
#[cfg_attr(feature = "server", diesel(table_name = paints_2_storage_boxes))]
pub struct PaintsToStorageBoxes {
    pub id: i32,
    pub paint: i32,
    pub storage_box: i32,
}
