// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        name -> Text,
        gender -> Bool,
        age -> Int4,
    }
}
