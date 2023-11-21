// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Int4,
        text_string -> Text,
        user_role -> Text,
    }
}

diesel::table! {
    counter (id) {
        id -> Int4,
        count -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    config,
    counter,
);
