extern crate diesel;
extern crate rocket;

use crate::db::establish_connection;
use crate::models::{Category, NewCategory};
use crate::schema::categories::name;
use diesel::prelude::*;
use rocket::fs::FileServer;
use rocket::response::Debug;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes};

mod db;
mod models;
mod schema;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/category", format = "json", data = "<post_body>")]
fn create_category(post_body: Json<NewCategory>) -> Result<Created<Json<NewCategory>>> {
    use crate::schema::categories::dsl::*;
    let mut conn = establish_connection();
    let new_category = NewCategory {
        name: post_body.name.to_string(),
    };

    diesel::insert_into(categories)
        .values(&new_category)
        .execute(&mut conn)
        .expect("Error saving new category");

    Ok(Created::new("/").body(post_body))
}

#[get("/category/all")]
fn get_all_categories() -> Result<Json<Vec<Category>>> {
    use crate::schema::categories::dsl::categories;
    let conn = &mut establish_connection();
    let cats: Vec<Category> = categories
        .load::<Category>(conn)
        .expect("Error loading categories");

    Ok(Json(cats))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/api", routes![create_category, get_all_categories])
}
