use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use super::schema::{ingredients, categories};
#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = ingredients)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = ingredients)]
pub struct NewIngredient {
    pub name: String,
    pub description: String,
    pub category_id: i32,
}

