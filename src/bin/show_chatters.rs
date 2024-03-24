use diesel::prelude::*;
use duel_bot::models::*;
use duel_bot::*;

fn main() {
    use duel_bot::establish_connection;
    use duel_bot::schema::chatters::dsl::chatters;

    let connection = &mut establish_connection();

    let results = chatters
        .limit(5)
        .select(Chatter::as_select())
        .load(connection)
        .expect("Error loading Chatter");

    println!("Displaying {} chatters", results.len());
    for chatter in results {
        println!("{}", chatter.username);
        println!("-----------\n");
        println!(
            "{} Points - Record: {}/{}",
            chatter.points, chatter.wins, chatter.losses
        );
    }
}
