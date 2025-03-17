extern crate diesel;
extern crate rocket;

use crate::db::establish_connection;
use crate::models::{Category, Ingredient, NewCategory, NewIngredient};
use crate::schema::categories::dsl::categories;
use crate::schema::ingredients::dsl::ingredients;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use rocket::fs::FileServer;
use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes};

mod db;
mod models;
mod schema;

mod service;

type Result<T> = std::result::Result<T, BadRequest<&'static str>>;

#[post("/categories", format = "json", data = "<post_body>")]
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

#[get("/categories")]
fn get_all_categories() -> Result<Json<Vec<Category>>> {
    use crate::schema::categories::dsl::categories;
    let conn = &mut establish_connection();
    let cats: Vec<Category> = categories
        .load::<Category>(conn)
        .expect("Error loading categories");

    Ok(Json(cats))
}

#[post("/ingredients", format = "json", data = "<post_body>")]
fn create_ingredient(post_body: Json<NewIngredient>) -> Result<Created<Json<NewIngredient>>> {
    let conn = &mut establish_connection();

    // check if category exists
    let category_exists = select(exists(
        categories.filter(schema::categories::id.eq(&post_body.category_id)),
    ))
    .get_result::<bool>(conn)
    .expect("Error checking existing categories");

    match category_exists {
        true => {}
        false => return Err(BadRequest("Unbekannte Kategorie")),
    }

    let new_ing = NewIngredient {
        name: post_body.name.to_string(),
        description: post_body.description.clone(),
        category_id: post_body.category_id,
    };

    diesel::insert_into(ingredients)
        .values(&new_ing)
        .execute(conn)
        .expect("Error saving new ingedient");
    Ok(Created::new("/").body(post_body))
}

#[get("/ingredients")]
fn get_all_ingredients() -> Result<Json<Vec<Ingredient>>> {
    use crate::schema::ingredients::dsl::ingredients;
    let conn = &mut establish_connection();
    let result: Vec<Ingredient> = ingredients
        .load::<Ingredient>(conn)
        .expect("Error loading ingredients");

    Ok(Json(result))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount(
            "/api",
            routes![create_category, get_all_categories, create_ingredient, get_all_ingredients],
        )
}
