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
        description -> Text,
        category_id -> Integer,
    }
}

diesel::joinable!(ingredients -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    ingredients,
);
