// @generated automatically by Diesel CLI.

diesel::table! {
    athletes (id) {
        id -> Int8,
        username -> Nullable<Text>,
        firstname -> Nullable<Text>,
        lastname -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        another_column -> Nullable<Text>,
    }
}

diesel::table! {
    token (id) {
        id -> Int8,
        expires_at -> Int4,
        expires_in -> Int4,
        token_type -> Text,
        refresh_token -> Text,
        access_token -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    athletes,
    token,
);
