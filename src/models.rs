use chrono::NaiveDateTime;
use diesel::prelude::*;

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
    pub last_seen: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::duels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Duel {
    pub id: i32,
    pub accepted: bool,
    pub points: i32,
    pub challenger: String,
    pub challenged: String,
    pub winner: Option<String>,
}

use crate::schema::chatters;

#[derive(Insertable)]
#[diesel(table_name = chatters)]
pub struct NewChatter<'a> {
    pub twitch_id: &'a str,
    pub username: &'a str,
}

use crate::schema::duels;

#[derive(Insertable)]
#[diesel(table_name = duels)]
pub struct NewDuel<'a> {
    pub challenger: &'a str,
    pub challenged: &'a str,
    pub points: i32,
}
