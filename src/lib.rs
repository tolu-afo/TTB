use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use log::info;

use crate::models::{Duel, NewDuel};

use self::models::{Chatter, NewChatter};

pub mod chatter;
pub mod models;
pub mod schema;

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
    use self::schema::chatters::dsl::{chatters, twitch_id};

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
    use self::schema::chatters::dsl::{chatters, points, twitch_id};

    let chatter = diesel::update(chatters.filter(twitch_id.eq(twitch_id)))
        .set(points.eq(new_points))
        .execute(conn)
        .expect("Points value should be i32");
}

pub fn update_points(id: &str, new_points: i32) {
    db_update_points(&mut establish_connection(), id, new_points)
}

fn db_update_wins(conn: &mut PgConnection, id: &str, new_wins: i32) {
    use self::schema::chatters::dsl::{chatters, twitch_id, wins};
    let chatter = diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(wins.eq(new_wins))
        .execute(conn)
        .expect("Wins value should be i32");
}

pub fn update_wins(id: &str, wins: i32) {
    db_update_wins(&mut establish_connection(), id, wins);
}

fn db_update_losses(conn: &mut PgConnection, id: &str, new_losses: i32) {
    use self::schema::chatters::dsl::{chatters, losses, twitch_id};
    let chatter = diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(losses.eq(new_losses))
        .execute(conn)
        .expect("Losses value should be i32");
}

pub fn update_losses(id: &str, losses: i32) {
    db_update_losses(&mut establish_connection(), id, losses);
}

pub fn create_duel(
    conn: &mut PgConnection,
    challenger: &str,
    challenged: &str,
    points: i32,
) -> Duel {
    use crate::schema::duels;
    let new_duel = NewDuel {
        challenger,
        challenged,
        points,
    };

    diesel::insert_into(duels::table)
        .values(&new_duel)
        .returning(Duel::as_returning())
        .get_result(conn)
        .expect("Error saving new duel")
}
