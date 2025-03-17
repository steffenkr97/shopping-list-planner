// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        category_id -> Integer,
    }
}

diesel::table! {
    recipe_ingredients (id) {
        id -> Integer,
        recipe_id -> Integer,
        ingredient_id -> Integer,
    }
}

diesel::table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        category_id -> Nullable<Integer>,
    }
}

diesel::table! {
    weekly_plan_recipes (id) {
        id -> Integer,
        weekly_plan_id -> Integer,
        recipe_id -> Integer,
    }
}

diesel::table! {
    weekly_plans (id) {
        id -> Integer,
        week_start_date -> Date,
    }
}

diesel::joinable!(ingredients -> categories (category_id));
diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(recipe_ingredients -> recipes (recipe_id));
diesel::joinable!(recipes -> categories (category_id));
diesel::joinable!(weekly_plan_recipes -> recipes (recipe_id));
diesel::joinable!(weekly_plan_recipes -> weekly_plans (weekly_plan_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    ingredients,
    recipe_ingredients,
    recipes,
    weekly_plan_recipes,
    weekly_plans,
);
