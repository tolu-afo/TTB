use duel_bot::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut challenger = String::new();
    let mut challenged = String::new();
    let mut points = String::new();

    println!("Who is the challenger?");
    stdin().read_line(&mut challenger).unwrap();
    let challenger = challenger.trim_end(); // Remove the trailing newline

    println!("Who is the challenged?");
    stdin().read_line(&mut challenged).unwrap();
    let challenged = challenged.trim_end(); // Remove the trailing newline

    println!("How many points?");
    stdin().read_line(&mut points).unwrap();
    let points = points.trim_end().parse().expect("Should be a number"); // Remove the trailing newline

    let duel = create_duel(connection, challenger, challenged, points);
    println!("\nSaved duel with id {}", duel.id);
}
