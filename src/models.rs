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

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable))]
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

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Queryable))]
pub struct StorageBox {
    pub id: i32,
    pub name: String,
    pub flags: String,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name=storage_boxes))]
pub struct NewStorageBox {
    pub name: String,
    pub flags: String,
}
