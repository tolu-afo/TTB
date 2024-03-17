use duel_bot::*;
use std::io::{stdin, Read};

fn main(){
    let connection = &mut establish_connection();

    let mut username = String::new();

    println!("What is the username?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    let chatter = create_chatter(connection, username);
    println!("\nSaved chatter {} with id {}", username, chatter.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";