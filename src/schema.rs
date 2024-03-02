// @generated automatically by Diesel CLI.

diesel::table! {
    chatters (id) {
        id -> Int4,
        username -> Varchar,
        points -> Nullable<Int4>,
        wins -> Nullable<Int4>,
        losses -> Nullable<Int4>,
    }
}
