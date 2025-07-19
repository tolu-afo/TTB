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
        points -> Int8,
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
        points -> Int8,
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
    losers_pool (id) {
        id -> Int4,
        amount -> Int8,
        winner -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    orders (id) {
        id -> Int4,
        stock_id -> Int4,
        owner_id -> Int4,
        num_shares -> Int4,
        strike_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::table! {
    stocks (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        symbol -> Varchar,
        ticket_price -> Numeric,
        future_value -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        roi_percentage -> Numeric,
    }
}

diesel::joinable!(orders -> chatters (owner_id));
diesel::joinable!(orders -> stocks (stock_id));
diesel::joinable!(questions -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    accepted_duels,
    categories,
    chatters,
    duels,
    losers_pool,
    lurkers,
    orders,
    questions,
    stocks,
);
