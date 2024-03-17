use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chatters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatter {
    pub id: i32,
    pub username: String,
    pub points: i32,
    pub wins: i32,
    pub losses: i32
}

use crate::schema::chatters;

#[derive(Insertable)]
#[diesel(table_name = chatters)]
pub struct NewChatter<'a> {
    pub username: &'a str,
}