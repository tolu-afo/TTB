// @generated automatically by Diesel CLI.

diesel::table! {
    accepted_duels (id) {
        id -> Int4,
        duel_id -> Int4,
        #[max_length = 255]
        challenger_id -> Varchar,
        #[max_length = 255]
        challenged_id -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

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

diesel::table! {
    duels (id) {
        id -> Int4,
        accepted -> Bool,
        points -> Int4,
        challenger -> Varchar,
        challenged -> Varchar,
        winner -> Nullable<Varchar>,
        #[max_length = 255]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 255]
        question -> Nullable<Varchar>,
        #[max_length = 255]
        answer -> Nullable<Varchar>,
        #[max_length = 255]
        challenger_id -> Nullable<Varchar>,
        #[max_length = 255]
        challenged_id -> Nullable<Varchar>,
        challenger_guesses -> Int4,
        challenged_guesses -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accepted_duels,
    chatters,
    duels,
);
