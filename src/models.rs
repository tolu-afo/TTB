use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chatters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatter{
    pub id: i32,
    pub username: String,
    pub points: i32,
    pub wins: i32,
    pub losses: i32
}