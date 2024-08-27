// use diesel::prelude::*;
// use diesel::sqlite::Sqlite;

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::brands)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]

// #[derive(Queryable, PartialEq, Debug)]
pub struct Brand {
    pub id: i32,
    pub name: String,
}
