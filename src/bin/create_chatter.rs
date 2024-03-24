use duel_bot::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut username = String::new();
    let mut twitch_id = String::new();

    println!("What is the username?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("What is the twitch_id?");
    stdin().read_line(&mut twitch_id).unwrap();
    let twitch_id = twitch_id.trim_end(); // Remove the trailing newline

    let chatter = create_chatter(connection, username, twitch_id);
    println!("\nSaved chatter {} with id {}", username, chatter.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
