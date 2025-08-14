use diesel::query_dsl::methods::SelectDsl;
use diesel::SelectableHelper;
use diesel::{PgConnection, RunQueryDsl};

use crate::server_side_error::Result;

use crate::models::*;
// use crate::schema::*;

pub fn list_brands(connection: &mut PgConnection) -> Result<Vec<Brand>> {
    use crate::schema::brands::dsl::*;
    Ok(brands.select(Brand::as_select()).load(connection)?)
}
