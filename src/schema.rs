// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Varchar,
        title -> Text,
        done -> Bool,
    }
}
