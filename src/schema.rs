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
        category_id -> Nullable<Integer>,
    }
}

diesel::joinable!(ingredients -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    ingredients,
);
