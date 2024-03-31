use std::env;

use diesel::dsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use log::info;

use crate::models::{Chatter, Duel, NewChatter, NewDuel};
// TODO: Import to each function what they need.
use crate::schema::chatters::dsl::*;
use crate::schema::duels;
use crate::schema::duels::dsl::*;

// TODO: Refactor to one Connection, or enable/confirm connection pooling

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_chatter(conn: &mut PgConnection, chatter_id: &str, chatter_name: &str) -> Chatter {
    use crate::schema::chatters;

    let new_chatter = NewChatter {
        username: chatter_name,
        twitch_id: chatter_id,
    };

    diesel::insert_into(chatters::table)
        .values(&new_chatter)
        .returning(Chatter::as_returning())
        .get_result(conn)
        .expect("Error saving new chatter")
}

pub fn db_get_chatter(conn: &mut PgConnection, chatter_id: &str) -> Option<Chatter> {
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

pub fn db_get_chatter_by_username(conn: &mut PgConnection, chatter_name: &str) -> Option<Chatter> {
    let chatter = chatters
        .filter(username.eq(chatter_name))
        .select(Chatter::as_select())
        .first(conn)
        .optional();

    chatter.unwrap_or_else(|_| {
        println!("An error occurred while fetching chatter {}", chatter_name);
        None
    })
}

pub fn get_chatter_by_username(chatter_name: &str) -> Option<Chatter> {
    db_get_chatter_by_username(&mut establish_connection(), chatter_name)
}

fn update_last_seen(conn: &mut PgConnection, pk: i32) {
    diesel::update(chatters.find(pk))
        .set(last_seen.eq(dsl::now))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

fn update_username(conn: &mut PgConnection, chatter_id: i32, new_username: &str) {
    diesel::update(chatters.filter(twitch_id.eq(chatter_id)))
        .set(username.eq(new_username))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

pub fn record_user_presence(chatter_id: &str, chatter_name: &str) -> Chatter {
    let conn = &mut establish_connection();

    match db_get_chatter(conn, chatter_id) {
        Some(chatter) => {
            info!("Chatter found for {}", chatter.username);
            update_last_seen(conn, chatter.id);
            if chatter.username != chatter_name {
                update_username(conn, chatter.id, chatter_name);
            }
            chatter
        }
        None => {
            let chatter = create_chatter(conn, chatter_id, chatter_name);
            info!("Chatter created for twitch user {}", chatter.username);
            chatter
        }
    }
}

fn db_update_points(conn: &mut PgConnection, chatter_id: &str, new_points: i32) {
    use crate::schema::duels::dsl::points;
    let chatter = diesel::update(chatters.filter(twitch_id.eq(chatter_id)))
        .set(points.eq(new_points))
        .execute(conn)
        .expect("Points value should be i32");
}

pub fn update_points(chatter_id: &str, new_points: i32) {
    db_update_points(&mut establish_connection(), chatter_id, new_points)
}

fn db_update_wins(conn: &mut PgConnection, chatter_id: &str, new_wins: i32) {
    let chatter = diesel::update(chatters.filter(twitch_id.eq(chatter_id)))
        .set(wins.eq(new_wins))
        .execute(conn)
        .expect("Wins value should be i32");
}

pub fn update_wins(chatter_id: &str, new_wins: i32) {
    db_update_wins(&mut establish_connection(), chatter_id, new_wins);
}

fn db_update_losses(conn: &mut PgConnection, chatter_id: &str, new_losses: i32) {
    let chatter = diesel::update(chatters.filter(twitch_id.eq(chatter_id)))
        .set(losses.eq(new_losses))
        .execute(conn)
        .expect("Losses value should be i32");
}

pub fn update_losses(chatter_id: &str, new_losses: i32) {
    db_update_losses(&mut establish_connection(), chatter_id, new_losses);
}

fn db_create_duel(
    conn: &mut PgConnection,
    challenger_id: &str,
    challenged_id: &str,
    new_points: i32,
) -> Duel {
    let new_duel = NewDuel {
        challenger: challenger_id,
        challenged: challenged_id,
        points: new_points,
    };

    diesel::insert_into(duels::table)
        .values(&new_duel)
        .returning(Duel::as_returning())
        .get_result(conn)
        .expect("Error saving new duel")
}

pub fn create_duel(challenger_id: &str, challenged_id: &str, new_points: i32) -> Duel {
    db_create_duel(
        &mut establish_connection(),
        challenger_id,
        challenged_id,
        new_points,
    )
}

fn db_accept_duel(conn: &mut PgConnection, challenger_id: &str, challenged_id: &str) {
    diesel::update(
        duels
            .filter(challenged.eq(challenged_id))
            .filter(challenger.eq(challenger_id))
            .filter(accepted.eq(false)),
    )
    .set(accepted.eq(true))
    .execute(conn)
    .expect("This duel does not exist");
}

pub fn accept_duel(challenger_id: &str, challenged_id: &str) {
    db_accept_duel(&mut establish_connection(), challenger_id, challenged_id);
}
