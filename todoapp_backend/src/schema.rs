// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        done -> Bool,
        user_id -> Integer,
    }
}

diesel::table! {
    user_token (token) {
        token -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Nullable<Text>,
    }
}

diesel::joinable!(tasks -> users (user_id));
diesel::joinable!(user_token -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tasks, user_token, users,);
