use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::chatter::TwitchId;
use crate::schema::chatters;
use crate::schema::duels;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chatters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatter {
    pub id: i32,
    pub twitch_id: TwitchId,
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
    pub challenger: TwitchId,
    pub challenged: TwitchId,
    pub winner: Option<TwitchId>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = chatters)]
pub struct NewChatter<'a> {
    pub twitch_id: TwitchId,
    pub username: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = duels)]
pub struct NewDuel {
    pub challenger: TwitchId,
    pub challenged: TwitchId,
    pub points: i32,
}
