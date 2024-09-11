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
    categories (id) {
        id -> Int4,
        name -> Text,
        submitter_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        lurk_time -> Int4,
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

diesel::table! {
    lurkers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        twitch_id -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        question -> Text,
        answer -> Text,
        category_id -> Int4,
        submitter_id -> Int4,
        times_asked -> Int4,
        times_not_answered -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(questions -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    accepted_duels,
    categories,
    chatters,
    duels,
    lurkers,
    questions,
);
