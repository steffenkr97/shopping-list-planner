use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use super::schema::categories;
#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
#[derive(Insertable, Deserialize)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
}
