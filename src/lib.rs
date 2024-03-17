pub mod models;
pub mod schema;
pub mod chatter;

use dotenv::dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use crate::schema::chatters::dsl::chatters;

use self::models::{NewChatter, Chatter};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_chatter(conn: &mut PgConnection, username: &str) -> Chatter{
    use crate::schema::chatters;

    let new_chatter = NewChatter { username };

    diesel::insert_into(chatters::table)
        .values(&new_chatter)
        .returning(Chatter::as_returning())
        .get_result(conn)
        .expect("Error saving new chatter")
}

