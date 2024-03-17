use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chatters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatter {
    pub id: i32,
    pub twitch_id: String,
    pub username: String,
    pub points: i32,
    pub wins: i32,
    pub losses: i32,
    pub last_seen: NaiveDateTime
}

use crate::schema::chatters;

#[derive(Insertable)]
#[diesel(table_name = chatters)]
pub struct NewChatter<'a> {
    pub twitch_id: &'a str,
    pub username: &'a str,
}