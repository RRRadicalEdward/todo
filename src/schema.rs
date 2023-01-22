// @generated automatically by Diesel CLI.

diesel::table! {
    entry (uuid) {
        uuid -> Binary,
        title -> Text,
        status -> Integer,
    }
}
