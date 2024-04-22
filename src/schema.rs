// @generated automatically by Diesel CLI.

diesel::table! {
    chatters (id) {
        id -> Int4,
        username -> Varchar,
        points -> Int4,
        wins -> Int4,
        losses -> Int4,
        last_seen -> Timestamp,
        twitch_id -> Int4,
    }
}

diesel::table! {
    duels (id) {
        id -> Int4,
        accepted -> Bool,
        points -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        challenger -> Int4,
        challenged -> Int4,
        winner -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chatters,
    duels,
);
