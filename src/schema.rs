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
        expires_at -> Int8,
        expires_in -> Int8,
        token_type -> Text,
        refresh_token -> Text,
        access_token -> Text,
        id -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(athletes, token,);
