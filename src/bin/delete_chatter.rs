use diesel::prelude::*;
use duel_bot::*;
use std::env::args;

fn main() {
    use self::schema::chatters::dsl::*;

    let chatter_id = args()
        .nth(1)
        .expect("Expected a target to match against")
        .parse::<i32>()
        .expect("Invalid ID");

    // let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(chatters.find(chatter_id))
        .execute(connection)
        .expect("Error deleting Chatters");

    println!("Deleted {} chatters", num_deleted);
}