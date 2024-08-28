use diesel::query_dsl::methods::SelectDsl;
use diesel::{PgConnection, QueryResult, RunQueryDsl};

use crate::models::*;
use crate::schema::*;

pub fn list_brands(c: &mut PgConnection) -> Vec<Brand> {
    use crate::schema::brands::dsl::*;
    brands.load(c).expect("Unable to fetch brands")
}
