use super::schema::*;
use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
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
    pub description: Option<String>,
    pub category_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = ingredients)]
pub struct NewIngredient {
    pub name: String,
    pub description: Option<String>,
    pub category_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = recipe_ingredients)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = recipe_ingredients)]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = recipes)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = weekly_plan_recipes)]
pub struct WeeklyPlanRecipe {
    pub id: i32,
    pub weekly_plan_id: i32,
    pub recipe_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = weekly_plan_recipes)]
pub struct NewWeeklyPlanRecipe {
    pub weekly_plan_id: i32,
    pub recipe_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = weekly_plans)]
pub struct WeeklyPlan {
    pub id: i32,
    pub week_start_date: chrono::NaiveDate,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = weekly_plans)]
pub struct NewWeeklyPlan {
    pub week_start_date: chrono::NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}