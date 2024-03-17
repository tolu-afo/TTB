use self::models::Chatter;
use diesel::prelude::*;
use duel_bot::*;
use std::env::args;
use std::ops::Add;
use duel_bot::schema::chatters::points;

fn main() {
    use self::schema::chatters::dsl::{chatters, points};

    let id = args()
        .nth(1)
        .expect("add_points_to_chatter requires a chatter id")
        .parse::<i32>()
        .expect("Invalid ID");

    let point_value = args()
        .nth(2)
        .expect("add_points_to_chatter requires points")
        .parse::<i32>()
        .expect("Invalid Point Value");

    let connection = &mut establish_connection();
    let chatter = chatters
        .find(id)
        .select(Chatter::as_select())
        .first(connection)
        .optional();

    let chatter = diesel::update(chatters.find(id))
        .set(points.eq(point_value))
        .returning(Chatter::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Updated Chatter {}; points update to: {}", chatter.username, chatter.points);
}