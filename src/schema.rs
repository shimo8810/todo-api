// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Varchar,
        title -> Text,
        done -> Bool,
    }
}
