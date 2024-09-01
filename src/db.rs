use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use log::info;
use tmi::client::conn;

use crate::duel;
use crate::models::{AcceptedDuel, Chatter, Duel, NewAcceptedDuel, NewChatter, NewDuel};
use crate::schema::chatters::dsl::chatters;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_chatter(conn: &mut PgConnection, twitch_id: &str, username: &str) -> Chatter {
    use crate::schema::chatters;

    let new_chatter = NewChatter {
        username,
        twitch_id,
    };

    diesel::insert_into(chatters::table)
        .values(&new_chatter)
        .returning(Chatter::as_returning())
        .get_result(conn)
        .expect("Error saving new chatter")
}

pub fn db_get_chatter(conn: &mut PgConnection, chatter_id: &str) -> Option<Chatter> {
    use crate::schema::chatters::dsl::{chatters, twitch_id};

    let chatter = chatters
        .filter(twitch_id.eq(chatter_id))
        .select(Chatter::as_select())
        .first(conn)
        .optional();

    chatter.unwrap_or_else(|_| {
        println!("An error occurred while fetching chatter {}", chatter_id);
        None
    })
}

pub fn get_chatter(chatter_id: &str) -> Option<Chatter> {
    db_get_chatter(&mut establish_connection(), chatter_id)
}

fn db_get_chatter_by_username(conn: &mut PgConnection, username: &str) -> Option<Chatter> {
    use crate::schema::chatters::dsl::{chatters, username as chatter_name};

    let chatter = chatters
        .filter(chatter_name.eq(username))
        .select(Chatter::as_select())
        .first(conn)
        .optional();

    chatter.unwrap_or_else(|_| {
        println!("An error occurred while fetching chatter {}", username);
        None
    })
}

pub fn get_chatter_by_username(username: &str) -> Option<Chatter> {
    db_get_chatter_by_username(&mut establish_connection(), username)
}

fn update_last_seen(conn: &mut PgConnection, chatter_id: i32) {
    use crate::schema::chatters::dsl::{chatters, id, last_seen};
    use diesel::dsl;

    diesel::update(chatters)
        .filter(id.eq(chatter_id))
        .set(last_seen.eq(dsl::now))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

fn update_username(conn: &mut PgConnection, chatter_id: i32, new_username: &str) {
    use crate::schema::chatters::dsl::{chatters, id, username};

    diesel::update(chatters)
        .filter(id.eq(chatter_id))
        .set(username.eq(new_username))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

pub fn record_user_presence(twitch_id: &str, username: &str) -> Chatter {
    let conn = &mut establish_connection();

    match db_get_chatter(conn, twitch_id) {
        Some(chatter) => {
            info!("Chatter found for {}", chatter.username);
            update_last_seen(conn, chatter.id);
            if chatter.username != username {
                update_username(conn, chatter.id, username);
            }
            chatter
        }
        None => {
            let chatter = create_chatter(conn, twitch_id, username);
            info!("Chatter created for twitch user {}", chatter.username);
            chatter
        }
    }
}

fn db_update_points(conn: &mut PgConnection, id: &str, new_points: i32) {
    use crate::schema::chatters::dsl::{chatters, points, twitch_id};

    let chatter = diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(points.eq(new_points))
        .execute(conn)
        .expect("Points value should be i32");
}

pub fn update_points(id: &str, new_points: i32) {
    db_update_points(&mut establish_connection(), id, new_points)
}

fn db_update_wins(conn: &mut PgConnection, id: &str, new_wins: i32) {
    use crate::schema::chatters::dsl::{chatters, twitch_id, wins};
    let chatter = diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(wins.eq(new_wins))
        .execute(conn)
        .expect("Wins value should be i32");
}

pub fn update_wins(id: &str, wins: i32) {
    db_update_wins(&mut establish_connection(), id, wins);
}

fn db_update_losses(conn: &mut PgConnection, id: &str, new_losses: i32) {
    use crate::schema::chatters::dsl::{chatters, losses, twitch_id};
    let chatter = diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(losses.eq(new_losses))
        .execute(conn)
        .expect("Losses value should be i32");
}

pub fn update_losses(id: &str, losses: i32) {
    db_update_losses(&mut establish_connection(), id, losses);
}

fn db_create_duel(
    conn: &mut PgConnection,
    challenger: &str,
    challenged: &str,
    challenger_id: &str,
    challenged_id: &str,
    points: i32,
) -> Duel {
    use crate::schema::duels;
    let new_duel = NewDuel {
        challenger,
        challenged,
        challenger_id: challenger_id,
        challenged_id: challenged_id,
        points,
    };

    diesel::insert_into(duels::table)
        .values(&new_duel)
        .returning(Duel::as_returning())
        .get_result(conn)
        .expect("Error saving new duel")
}

pub fn create_duel(
    challenger: &str,
    challenged: &str,
    challenger_id: &str,
    challenged_id: &str,
    points: i32,
) -> Duel {
    db_create_duel(
        &mut establish_connection(),
        challenger,
        challenged,
        challenger_id,
        challenged_id,
        points,
    )
}

fn db_get_duel(conn: &mut PgConnection, id: i32) -> Option<Duel> {
    use crate::schema::duels::dsl::duels;

    let duel = duels
        .find(id)
        .select(Duel::as_select())
        .first(conn)
        .optional();

    duel.unwrap_or_else(|_| {
        println!("An error occurred while fetching duel {}", id);
        None
    })
}

pub fn get_duel(duel_id: i32) -> Option<Duel> {
    db_get_duel(&mut establish_connection(), duel_id)
}

fn db_accept_duel(conn: &mut PgConnection, id: i32) {
    use crate::schema::duels::dsl::{duels, status as duel_status};

    diesel::update(duels.find(id))
        .set(duel_status.eq("accepted"))
        .execute(conn)
        .expect("Duel ID should be i32");
}

pub fn accept_duel(id: i32) {
    db_accept_duel(&mut establish_connection(), id);
}

fn db_get_accepted_duel(conn: &mut PgConnection, responder: &str) -> Option<AcceptedDuel> {
    use crate::schema::accepted_duels::dsl::{
        accepted_duels as duels, challenged_id as duel_challenged, challenger_id as duel_challenger,
    };
    let duel = duels
        .filter(
            duel_challenger
                .eq(responder)
                .or(duel_challenged.eq(responder)),
        )
        .select(AcceptedDuel::as_select())
        .first(conn)
        .optional();
    duel.unwrap_or_else(|_| {
        println!("An error occurred while fetching duel for {}", responder);
        None
    })
}

pub fn get_accepted_duel(responder: &str) -> Option<AcceptedDuel> {
    db_get_accepted_duel(&mut establish_connection(), responder)
}

fn db_set_question_duel(
    conn: &mut PgConnection,
    id: i32,
    question: &str,
    answer: &str,
    status: &str,
) {
    use crate::schema::duels::dsl::{
        answer as duel_answer, duels, question as duel_question, status as duel_status,
    };

    diesel::update(duels.find(id))
        .set((
            duel_status.eq(status),
            duel_question.eq(question),
            duel_answer.eq(answer),
        ))
        .execute(conn)
        .expect("Winner should be a valid twitch id");
}

pub fn set_question_duel(id: i32, question: &str, answer: &str, status: &str) {
    db_set_question_duel(&mut establish_connection(), id, question, answer, status);
}

fn db_complete_duel(conn: &mut PgConnection, id: i32, winner: &str, status: &str) {
    use crate::schema::duels::dsl::{duels, status as duel_status, winner as winner_id};

    diesel::update(duels.find(id))
        .set((winner_id.eq(winner), duel_status.eq(status)))
        .execute(conn)
        .expect("Winner should be a valid twitch id");
}

pub fn complete_duel(id: i32, winner: &str, status: &str) {
    db_complete_duel(&mut establish_connection(), id, winner, status);
}

fn db_create_accepted_duel(
    conn: &mut PgConnection,
    duel_id: i32,
    challenger_id: &str,
    challenged_id: &str,
) -> AcceptedDuel {
    use crate::schema::accepted_duels;
    let new_accepted_duel = NewAcceptedDuel {
        duel_id,
        challenger_id,
        challenged_id,
    };

    diesel::insert_into(accepted_duels::table)
        .values(&new_accepted_duel)
        .returning(AcceptedDuel::as_returning())
        .get_result(conn)
        .expect("Error saving new duel")
}

pub fn create_accepted_duel(
    duel_id: i32,
    challenger_id: &str,
    challenged_id: &str,
) -> AcceptedDuel {
    db_create_accepted_duel(
        &mut establish_connection(),
        duel_id,
        challenger_id,
        challenged_id,
    )
}

fn db_destroy_accepted_duel(conn: &mut PgConnection, id: i32) {
    use crate::schema::accepted_duels::dsl::{accepted_duels, duel_id};

    diesel::delete(accepted_duels.filter(duel_id.eq(id)))
        .execute(conn)
        .expect("Duel ID should be i32");
}

pub fn destroy_accepted_duel(id: i32) {
    db_destroy_accepted_duel(&mut establish_connection(), id);
}

// TODO: Decrement Challenger Guesses
// TODO: Decrement Challenged Guesses
