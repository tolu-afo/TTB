pub mod models;
pub mod schema;
pub mod chatter;

use dotenv::dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::env::args;

use log::debug;
use log::error;
use log::info;
use log::warn;

use crate::schema::chatters::dsl::chatters;

use self::models::{NewChatter, Chatter};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_chatter(conn: &mut PgConnection, twitch_id: &str, username: &str) -> Chatter{
    use crate::schema::chatters;

    let new_chatter = NewChatter { username, twitch_id };

    diesel::insert_into(chatters::table)
        .values(&new_chatter)
        .returning(Chatter::as_returning())
        .get_result(conn)
        .expect("Error saving new chatter")
}

pub fn get_chatter(conn: &mut PgConnection, chatter_id: &str) -> Option<Chatter> {
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

fn update_last_seen(conn: &mut PgConnection, chatter_id: i32){
    use crate::schema::chatters::dsl::{chatters, last_seen, id};
    use diesel::dsl;

    diesel::update(chatters).filter(id.eq(chatter_id))
        .set(last_seen.eq(dsl::now))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

pub fn record_user_presence(twitch_id: &str, username: &str) -> Chatter {
    use crate::schema::chatters;
    use chrono::NaiveDateTime;

    let conn = &mut establish_connection();

    match get_chatter(conn, twitch_id) {
        Some(chatter) => {
            info!("Chatter found for {}", chatter.username);
            update_last_seen(conn, chatter.id);
            chatter
        },
        None => {
            let chatter = create_chatter(conn, twitch_id, username);
            info!("Chatter created for twitch user {}", chatter.username);
            chatter
        }
    }
}

