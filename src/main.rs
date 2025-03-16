extern crate diesel;
#[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket::{get, launch, post, routes};
use rocket::response::Debug;
use rocket::serde::json::Json;
use crate::db::establish_connection;
use crate::models::NewCategory;

mod schema;
mod models;
mod db;


type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<new_category>")]
async fn create_cat(new_category: Json<NewCategory>) -> &'static str {
    use crate::schema::categories::dsl::*;

    let mut conn = establish_connection();

    let new_category = NewCategory {
        name: new_category.name.to_string(),
    };

    diesel::insert_into(categories)
        .values(&new_category)
        .execute(&mut conn)
        .expect("Error saving new category");

    "Erstellt"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello,create_cat])
}
