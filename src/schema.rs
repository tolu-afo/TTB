// @generated automatically by Diesel CLI.

diesel::table! {
    chatters (id) {
        id -> Int4,
        twitch_id -> Varchar,
        username -> Varchar,
        points -> Int4,
        wins -> Int4,
        losses -> Int4,
        last_seen -> Timestamp,
    }
}
