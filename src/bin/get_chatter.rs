use self::models::Chatter;
use diesel::prelude::*;
use duel_bot::*;
use std::env::args;

fn main() {
    use self::schema::chatters::dsl::chatters;

    let chatter_id = args()
        .nth(1)
        .expect("get_chatter requires a chatter id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let chatter = chatters
        .find(chatter_id)
        .select(Chatter::as_select())
        .first(connection)
        .optional();

    match chatter {
        Ok(Some(chatter)) => println!(
            "Chatter with id: {} has a username: {}, {} points, {} win(s), and {} loss(es)",
            chatter.id, chatter.username, chatter.points, chatter.wins, chatter.losses
        ),

        Ok(None) => println!("Unable to find chatter {}", chatter_id),
        Err(_) => println!("An error occurred while fetching chatter {}", chatter_id),
    }
}
