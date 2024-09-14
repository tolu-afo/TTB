use dotenv::dotenv;
use std::env;

use duel_bot::db::create_category;

fn main() {
    // Get environment variables
    dotenv().ok();
    let broadcaster_id = env::var("BROADCASTER_ID").expect("BROADCASTER_ID not set");

    let broad = match broadcaster_id.parse::<i32>() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error parsing broadcaster_id: {}", e);
            std::process::exit(1);
        }
    };

    // Rest of the code...
    let starter_categories = vec![
        "Guess the Programming Language",
        "Guess the Movie by the Quote",
        "Word Scramble",
        "General",
    ];

    for category in starter_categories {
        create_category(&category, broad);
    }
}
