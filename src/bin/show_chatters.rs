fn main() {
    // use self::schema::chatters::dsl::*;

    let connection = &mut established_connection();

    let results = chatters.filter(published.eq(true))
        .limit(5)
        .select(Chatter::as_select())
        .load(connection)
        .expect("Error loading Chatter");

    println!("Displaying {} chatters", results.len());
    for chatter in results {
        println!("{}", chatter.username);
        println!("-----------\n");
        println!("{} - {}/{}", chatter.points, chatter.wins, chatter.losses);
    }
}