// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Bool,
    }
}
