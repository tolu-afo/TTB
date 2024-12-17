use anyhow::Result;
use chrono::TimeZone;
use rand::Rng;

use crate::chatter;
use crate::chatter::get_challenge_to_accept;
use crate::db;
use crate::db::get_category_by_name;
use crate::messaging;
use crate::messaging::{list_with_title, ItemSeparator};
use crate::models;
use crate::models::Question;
use crate::state::State;

pub async fn handle_yo_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let responses = [
        "yo",
        "hey",
        "hello",
        "hi",
        "what's up",
        "greetings",
        "salutations",
        "HAI",
        "Ello Gov'na",
        "Top of the morning to ya",
        "E kaaro",
        "Hola",
        "Bonjour",
        "Ciao",
        "Hallo",
        "Hej",
        "Aloha",
        "Namaste",
        "Konnichiwa",
        "Annyeonghaseyo",
        "Ni hao",
        "Salaam",
        "Shalom",
        "Sawubona",
        "Jambo",
        "Moin",
        "Yerrrr",
        "Wagwan",
        "Wassup",
        "Moi",
    ];
    use rand::seq::SliceRandom;

    let random_response = responses.choose(&mut rand::thread_rng()).unwrap();
    messaging::reply_to(client, msg, random_response).await?;
    Ok(())
}

pub async fn handle_lurk_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    db::create_lurker(&msg.sender().name(), msg.sender().id());
    messaging::reply_to(client, msg, "@ToluAfo We got a lurker over here!!!").await?;
    Ok(())
}

pub async fn handle_lurkers_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let lurkers = dbg!(db::get_lurkers())
        .iter()
        .map(|l| format!("@{} ", dbg!(&l.username)))
        .collect::<Vec<String>>();

    messaging::reply_to(
        client,
        &msg,
        &list_with_title("Lurkers:", &lurkers, ItemSeparator::Dash),
    )
    .await
}

pub async fn handle_lurktime_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let chatter = match db::get_chatter_by_username(&msg.sender().name()) {
        Some(chatter) => chatter,
        None => {
            return messaging::reply_to(client, msg, "You need to lurk first!").await;
        }
    };

    let reply = format!(
        "@{} you have lurked for {} seconds!",
        msg.sender().name(),
        chatter.lurk_time
    );

    messaging::reply_to(client, msg, &reply).await
}

pub async fn handle_points_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    use crate::chatter::get_points;

    let points = get_points(msg.sender().id());

    let reply = format!("@{}, you have {} point(s)!", msg.sender().name(), points);
    messaging::reply_to(client, msg, &reply).await
}

pub async fn handle_commands_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let commands = vec![
        // Point Casino
        "!gamble",
        // Random stuff
        "!yo",
        "!lurk",
        "!lurktime",
        "!lurkers",
        // Git related commands
        "!github",
        "!botrepo",
        // Duel related commands
        "!listcategories",
        "!addquestion",
        "!addcategory",
        "!points",
        "!challenge",
        "!duel",
        "!accept",
        "!kda",
        "!ranking",
        "!top3",
    ];
    messaging::reply_to(
        client,
        msg,
        &list_with_title("Available commands:", &commands, ItemSeparator::Comma),
    )
    .await
}

pub async fn handle_accept_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
    bot_state: &mut State,
) -> anyhow::Result<(), anyhow::Error> {
    // check that username of msg matches a challenged in a duel
    // !accept @<user>
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let challenged = msg.sender().name();
    let challenger = match cmd_iter.next() {
        Some(chal) => match chal.chars().nth(0) {
            Some('@') => &chal[1..],
            _ => chal,
        },
        None => {
            // get challenges from db
            // if challenges.len() == 1
            // accept challenge
            // else
            // return error message
            &match get_challenge_to_accept(&msg.sender().id()) {
                Some(challenger) => challenger,
                None => {
                    return messaging::send_duel_err(
                        &challenged,
                        client,
                        msg,
                        "You have more than one challenge! Provide a username in the format !accept @<user> or !accept <user>",
                    )
                    .await;
                }
            }
        }
    };
    let key = format!("{}{}", challenger, challenged);
    let duel = match bot_state.duel_cache.get_mut(&key.to_lowercase()) {
        Some(d) => {
            d.accept_duel();
            d
        }
        None => {
            return messaging::send_duel_err(&challenged, client, msg, "Wrong opponent!").await;
        }
    };

    let _ = messaging::send_msg(
        client,
        &msg,
        &format!(
            "@{} @{} Accepted! Once you read the question; type '!answer <your_answer>', or '!a <your_answer>' to answer!",
            challenger, challenged
        ),
    )
    .await;
    duel.ask_question(client, &msg).await;

    return Ok(());
}

pub async fn handle_duel_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
    bot_state: &mut State,
) -> anyhow::Result<(), anyhow::Error> {
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let challenger = dbg!(msg.sender().name());
    let challenged = match dbg!(cmd_iter.next()) {
        Some(chal) => {
            // filter @ symbol
            match chal.chars().nth(0) {
                Some('@') => &chal[1..],
                _ => chal,
            }
        }
        None => {
            return messaging::send_duel_err(
                &challenger,
                client,
                msg,
                "You need to provide a username in the format @<user> <points> or <user> <points>, or you can say 'random' to duel a random user!",
            )
            .await;
        }
    };

    if challenger == challenged {
        return messaging::send_duel_err(
            &challenger,
            client,
            msg,
            "You can't duel yourself silly!",
        )
        .await;
    }

    // Find challenger and challenged in chatter table
    // handle nones gracefully
    let challenger_chatter = match db::get_chatter_by_username(&challenger) {
        Some(chatter) => chatter,
        None => {
            return messaging::send_duel_err(&challenger, client, &msg, "Chatter not found!").await;
        }
    };
    let challenged_chatter = if challenged.eq("random") {
        db::get_random_chatter(&challenger_chatter)
    } else {
        match db::get_chatter_by_username(&challenged) {
            Some(chatter) => chatter,
            None => {
                return messaging::send_duel_err(&challenger, client, &msg, "Chatter not found!")
                    .await;
            }
        }
    };

    // check if challenger or challenged have an accepted duel
    match db::get_accepted_duel(&challenger) {
        Some(duel) => {
            // check if duel.created_at is older than 10 minutes
            // if so, delete the duel
            // else, return an error message
            let now = chrono::Utc::now();
            let stale = match duel.created_at {
                Some(date) => {
                    let tz_created_at: chrono::DateTime<chrono::Utc> =
                        chrono::Utc.from_utc_datetime(&date);
                    now.signed_duration_since(tz_created_at) > chrono::Duration::minutes(10)
                }
                None => false,
            };
            if stale {
                match db::get_duel(duel.duel_id) {
                    Some(mut d) => d.complete_duel(),
                    None => (),
                }
            } else {
                return messaging::send_duel_err(
                    &challenger,
                    client,
                    &msg,
                    "You already have an accepted duel!",
                )
                .await;
            }
        }
        None => (),
    }

    let points = match cmd_iter.next() {
        Some(chal) => chal,
        None => "100",
    };

    if challenger_chatter.points < 0 {
        return messaging::reply_to(client, &msg, "You are in the Shadow Realm (coming soon), and thus you can not duel! Chat to climb back into the light").await;
    }

    let points: i32 = match points.parse() {
        Result::Ok(p) => match p {
            p if p < 0 => {
                return messaging::send_duel_err(
                    &challenger,
                    client,
                    &msg,
                    "Provide a positive point value.",
                )
                .await;
            }
            p if p > challenger_chatter.points => {
                return messaging::send_duel_err(
                    &challenger,
                    client,
                    &msg,
                    "You don't have enough points to wager that much!!",
                )
                .await;
            }
            _ => p,
        },
        Result::Err(_) => {
            return messaging::send_duel_err(
                &challenger,
                client,
                &msg,
                "Provide a valid point value.",
            )
            .await;
        }
    };

    if cmd_iter.next().is_some() {
        return messaging::send_duel_err(&challenger, client, &msg, "Too many arguments!").await;
    }

    let curr_duel = models::Duel::new(
        &challenger,
        &challenged_chatter.username,
        &challenger_chatter.twitch_id,
        &challenged_chatter.twitch_id,
        points,
    );

    bot_state.save_duel(&curr_duel);

    messaging::reply_to(
        client,
        &msg,
        &format!(
            "@{} Challenge Announced, @{} type the command '!accept @{}' to begin duel!",
            challenger, challenged_chatter.username, challenger
        ),
    )
    .await
}

pub async fn handle_answer_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let responder = dbg!(msg.sender().name());
    let response = cmd_iter.collect::<Vec<&str>>().join(" ");

    let mut duel = match db::get_accepted_duel(&responder) {
        Some(d) => match db::get_duel(d.duel_id) {
            Some(duel) => duel,
            None => {
                return messaging::send_duel_err(&responder, client, msg, "No duel found!").await;
            }
        },
        None => {
            return messaging::send_duel_err(&responder, client, msg, "No duel found!").await;
        }
    };

    if responder == duel.challenger && duel.challenger_guesses - 1 < 0 {
        let reply = format!("@{} you are out of guesses!", duel.challenger);
        messaging::reply_to(client, &msg, reply.as_str()).await?;
        return Ok(());
    }

    if responder == duel.challenged && duel.challenged_guesses - 1 < 0 {
        let reply = format!("@{} you are out of guesses!", duel.challenged);
        messaging::reply_to(client, &msg, reply.as_str()).await?;
        return Ok(());
    }

    if duel.is_winner(&response) {
        // determine which player owns the current messsage
        // compare twitch id of challenger to the twitch id of the message sender
        // TODO: Add not null constraint on challenger_id and challenged_id
        if responder == duel.challenger {
            duel.award_winner(
                &responder,
                duel.challenger_id.clone().unwrap().as_str(),
                duel.challenged_id.clone().unwrap().as_str(),
            );
            let reply_msg = format!(
                "Correct! @{} won {} Points & @{} lost {} Points!",
                responder,
                duel.points,
                duel.challenged,
                duel.points / 2
            );
            messaging::reply_to(client, &msg, &reply_msg).await?;
        } else if responder == duel.challenged {
            duel.award_winner(
                &responder,
                duel.challenged_id.clone().unwrap().as_str(),
                duel.challenger_id.clone().unwrap().as_str(),
            );
            let reply_msg = format!(
                "Correct! @{} won {} Points & @{} lost {} Points!",
                responder,
                duel.points,
                duel.challenger,
                duel.points / 2
            );
            messaging::reply_to(client, &msg, &reply_msg).await?;
        };
    } else {
        // Deduct points for incorrect guess

        // send message to inform user of point deduction and incorrect guess.
        // max 5 guesses before duel is over, and challenger lose wagered points
        if responder == duel.challenger {
            if duel.challenger_guesses > 0 {
                duel.decrement_challenger_guesses()
            };

            let reply = if duel.challenger_guesses - 1 <= 0 {
                format!("Incorrect! @{} you are out of guesses!", duel.challenger)
            } else {
                format!(
              "Incorrect! @{} you have {} guesses remaining! type '!repeat' to repeat the question",
              duel.challenger, duel.challenger_guesses-1)
            };
            messaging::reply_to(client, &msg, reply.as_str()).await?;
        } else if responder == duel.challenged {
            if duel.challenged_guesses > 0 {
                duel.decrement_challenged_guesses()
            };
            let reply = if duel.challenged_guesses - 1 == 0 {
                format!("Incorrect! @{} you are out of guesses!", duel.challenged)
            } else {
                format!(
                  "Incorrect! @{} you have {} guesses remaining! type '!repeat' to repeat the question",
                  duel.challenged, duel.challenged_guesses-1
               )
            };
            messaging::reply_to(client, &msg, reply.as_str()).await?;
        }

        if duel.challenger_guesses - 1 <= 0 && duel.challenged_guesses - 1 <= 0 {
            duel.complete_duel();
            let reply = format!(
                "Both players have exhausted their guesses! The duel is over! Both @{} and @{} lose {} points! The correct answer was {}",
                duel.challenger, duel.challenged, duel.points / 2, duel.answer.as_ref().unwrap()
            );
            messaging::reply_to(client, &msg, reply.as_str()).await?;
        }
    }
    Ok(())
}

pub async fn handle_repeat_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let responder = msg.sender().name();
    let mut duel = match db::get_accepted_duel(&responder) {
        Some(d) => match db::get_duel(d.duel_id) {
            Some(duel) => duel,
            None => {
                return messaging::send_duel_err(&responder, client, msg, "No duel found!").await;
            }
        },
        None => {
            return messaging::send_duel_err(&responder, client, msg, "No duel found!").await;
        }
    };

    duel.repeat_question(client, &msg).await;
    Ok(())
}

// Send chatter wins and losses as message.
pub async fn handle_kda_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let responder = msg.sender().name();
    let chatter = match db::get_chatter_by_username(&responder) {
        Some(chatter) => chatter,
        None => {
            return messaging::send_duel_err(&responder, client, msg, "Chatter not found!").await;
        }
    };

    let reply = format!(
        "@{} has {} wins and {} losses!",
        responder, chatter.wins, chatter.losses
    );
    messaging::reply_to(client, &msg, &reply).await
}

pub async fn handle_top_duelists_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let top_duelists = db::get_top_duelists()
        .iter()
        .enumerate()
        .map(|(i, d)| format!("{}. {} - {} wins", i + 1, d.username, d.wins))
        .collect::<Vec<String>>();

    messaging::reply_to(
        client,
        &msg,
        &list_with_title("Top Duelists:", &top_duelists, ItemSeparator::GoldStar),
    )
    .await
}

pub async fn handle_ranking_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let ranking = db::get_ranking(msg.sender().id());
    let mut reply = String::from("Your ranking is: ");
    reply.push_str(ranking.to_string().as_str());
    messaging::reply_to(client, &msg, &reply).await
}

pub async fn handle_listcategories_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> Result<(), anyhow::Error> {
    let categories = db::get_categories()
        .iter()
        .map(|c| format!("{} - {}", c.id, c.name))
        .collect::<Vec<String>>();

    let reply = format!(
        "{}",
        list_with_title("Categories:", &categories, ItemSeparator::Dash),
    );

    messaging::reply_to(client, &msg, &reply).await
}

pub async fn handle_addquestion_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // !addquestion <question> | <answer>
    // question add costs 5000 points
    // save question and answer along with default category
    // ask follow up question about which category a user would like to add the question to
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();

    let chatter = match db::get_chatter(&msg.sender().id()) {
        Some(chatter) => chatter,
        None => unreachable!("If a chatter types a message, they should be in the database."),
    };

    let response = cmd_iter.collect::<Vec<&str>>().join(" "); // <question> | <answer> | <category_id>
                                                              // check if | exist in message
    if !response.contains('|') {
        return messaging::reply_to(
            client,
            &msg,
            "Invalid format! Use !addquestion <question> | <answer> | <category_id> type !listcategories to see category ids",
        )
        .await;
    }
    let question_answer: Vec<&str> = response.split('|').collect();

    if question_answer.len() < 3 {
        return messaging::reply_to(
        client,
        &msg,
        "Something is missing! Use !addquestion <question> | <answer> | <category_id> type !listcategories to see category ids",
    )
    .await;
    }

    // strip leading and trailing whitespaces
    let question = question_answer[0].trim();
    let answer = question_answer[1].trim();
    let category_id = question_answer[2].trim();

    // if question or answer is an empty string or special characters only send error
    if question.is_empty() || answer.is_empty() || category_id.is_empty() {
        return messaging::reply_to(
            client,
            &msg,
            "Your Question, Answer, or Category Id is empty! Use '!addquestion <question> | <answer> | <category_id>' to know which category id to use type !listcategories; ",
        )
        .await;
    };

    let category = match category_id.parse() {
        Ok(id) => match db::get_category(id) {
            Some(category) => category,
            None => {
                return messaging::reply_to(
                    client,
                    &msg,
                    "Category not found! Use !listcategories to see available categories.",
                )
                .await;
            }
        },
        Err(_) => {
            return messaging::reply_to(
                client,
                &msg,
                "Invalid category id! Use !listcategories to see available categories.",
            )
            .await;
        }
    };

    if chatter.points < 5000 {
        return messaging::reply_to(
          client,
          &msg,
          "You don't have enough points to add a question! It costs 5000 points to add a question.",
      )
      .await;
    }

    Question::new(question, answer, &category, &chatter);
    chatter::subtract_points(&chatter.twitch_id, 5000);
    messaging::reply_to(client, &msg, "Question Added!").await
}

pub async fn handle_github_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let github_url = "You can check out ToluAfo's projects at https://github.com/tolu-afo";

    messaging::reply_to(client, msg, github_url).await
}

pub async fn handle_botrepo_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let bot_repo_url = "You can check out my source code at https://github.com/tolu-afo/TTB";
    messaging::reply_to(client, msg, bot_repo_url).await
}

pub async fn handle_gamble_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // roll two dice

    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let wager = match cmd_iter.next() {
        Some(w) => match w.parse::<i32>() {
            Ok(w) => w,
            Err(_) => {
                return messaging::reply_to(
                    client,
                    msg,
                    "Invalid wager! Format: '!gamble <points>'",
                )
                .await;
            }
        },
        None => {
            return messaging::reply_to(
                client,
                msg,
                "You need to provide a wager! Format: '!gamble <points>'",
            )
            .await;
        }
    };

    let chatter = match db::get_chatter(&msg.sender().id()) {
        Some(chatter) => chatter,
        None => {
            return messaging::reply_to(client, msg, "Chatter not found!").await;
        }
    };

    if wager <= 0 {
        return messaging::reply_to(client, msg, "You can't gamble with negatives silly!").await;
    }

    if chatter.points < 0 {
        return messaging::reply_to(client, &msg, "You are in the Shadow Realm (coming soon), and thus you can not duel! Chat to climb back into the light").await;
    }

    if chatter.points < wager {
        return messaging::reply_to(
            client,
            msg,
            "You don't have enough points to wager that much!",
        )
        .await;
    }

    fn dice_roll() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..7)
    }

    messaging::reply_to(client, msg, "Rolling the dice!").await?;

    let roll1 = dice_roll();
    let roll2 = dice_roll();

    let sum = roll1 + roll2;

    let reply = match sum {
        12 => {
            let points = wager * 4;
            chatter::add_points(&msg.sender().id(), points);
            format!(
                "You rolled a {} and a {}! You win {} points!",
                roll1, roll2, points
            )
        }
        11 | 10 | 9 => {
            let points = wager * 2;
            chatter::add_points(&msg.sender().id(), points);
            format!(
                "You rolled a {} and a {}! You win {} points!",
                roll1, roll2, points
            )
        }
        8 => {
            let points = wager;
            chatter::add_points(&msg.sender().id(), points);
            format!(
                "You rolled a {} and a {}! You win {} points!",
                roll1, roll2, points
            )
        }
        2 => {
            let points = wager * 8;
            chatter::subtract_points(&msg.sender().id(), points);
            format!("Snake Eyes! You lose {} points!", points)
        }
        7 => {
            let points = wager * 4;
            chatter::subtract_points(&msg.sender().id(), points);
            format!(
                "You rolled a {} and a {}! You lose {} points!",
                roll1, roll2, points
            )
        }
        6 | 5 | 4 | 3 => {
            let points = wager;
            chatter::subtract_points(&msg.sender().id(), points);
            format!(
                "You rolled a {} and a {}! You lose {} points!",
                roll1, roll2, points
            )
        }
        _ => format!(
            "You rolled a {} and a {}! No points won or lost!",
            roll1, roll2
        ),
    };
    messaging::reply_to(client, msg, &reply).await
}

pub async fn handle_contribute_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let contribute_url = "You can contribute to my Code by taking on one of the issues listed here: https://github.com/tolu-afo/TTB/issues";
    messaging::reply_to(client, msg, contribute_url).await
}

pub async fn handle_hackathon_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let hackathon_url = "Join Our Hackathon! ft. ToluAfo (me), BlaiseLabs, & aholliday90! We are building a DND DungeonMaster bot for twitch streamer collaboration! Click here to learn more! https://discordapp.com/channels/1056759561035464705/1290390127922778174";
    messaging::reply_to(client, &msg, hackathon_url).await
}

pub async fn handle_addcategory_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // !addcategory <category>
    // add category to the database
    // check if category already exists
    // if category exists, return error message
    // else add category to the database
    // take 50000 points from user to add category
    // !addcategory <category>

    fn is_valid_category(category: &str) -> bool {
        // reject empty strings
        if category.len() == 0 {
            return false;
        }
        let cleaned_category = category.trim().to_lowercase();
        cleaned_category
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ' || c == '?' || c == '!' || c == '.')
    }

    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next(); // pops off the command
    let responder_id = msg.sender().id();
    let new_category = cmd_iter.collect::<Vec<&str>>().join(" ");
    // check if user has 50000 points to spend
    let responder = match db::get_chatter(&responder_id) {
        Some(chatter) => chatter,
        None => {
            return messaging::reply_to(client, msg, "Chatter not found!").await;
        }
    };

    // import broadcaster_id from env
    let broadcaster_id = dbg!(std::env::var("BROADCASTER_ID").unwrap());
    let comparison = dbg!(msg.sender().id() == broadcaster_id);

    // allow broadcaster to add categories
    if comparison == false {
        if responder.points < 50000 {
            return messaging::reply_to(
            client,
            msg,
            "You don't have enough points to add a category! It costs 50000 points to add a category.",
        )
        .await;
        }
    }

    // check if category already exists
    // strip leading and trailing whitespaces, lowercase, and special characters

    if !is_valid_category(&new_category) {
        return messaging::reply_to(
            client,
            msg,
            "Invalid category name! Category names cannot be empty, and can only contain alphanumeric characters, spaces, and the following special characters: !, ?, .",
        )
        .await;
    }

    let cleaned_category = new_category.trim().to_lowercase();

    match get_category_by_name(&cleaned_category) {
        Some(_) => {
            return messaging::reply_to(
                client,
                msg,
                "Category already exists! Provide a new category name.",
            )
            .await;
        }
        None => {
            db::create_category(&cleaned_category, responder_id.parse().unwrap());
            if !comparison {
                chatter::subtract_points(&responder_id, 50000);
            }
            return messaging::reply_to(client, msg, "Category Added!").await;
        }
    }
}

pub async fn handle_setpoints_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // set a chatters points
    let broadcaster_id = std::env::var("BROADCASTER_ID").expect("BROADCASTER_ID must be set");
    if msg.sender().id() == broadcaster_id {
        // format: !setpoints @<chatter_name> <new_point_value>

        let mut cmd_iter = msg.text().split(' ');
        cmd_iter.next();
        let chatter_name = match (cmd_iter.next()) {
            Some(name) => match name.chars().nth(0) {
                Some('@') => &name[1..],
                _ => name,
            },
            None => {
                return messaging::reply_to(
                    client,
                    msg,
                    "Format is incorrect! try !gift @<username> <points>",
                )
                .await;
            }
        };
        let chatter = match (db::get_chatter_by_username(&chatter_name)) {
            Some(user) => user,
            None => {
                return messaging::reply_to(client, msg, "No chatter with that name!").await;
            }
        };

        let points = match cmd_iter.next() {
            Some(chal) => chal,
            None => "100",
        };

        let new_points: i32 = match points.parse() {
            Result::Ok(p) => match p {
                p if p < 0 => {
                    return messaging::reply_to(client, msg, "provide a positive value").await;
                }
                _ => p,
            },
            Result::Err(_) => {
                return messaging::reply_to(client, msg, "Provide a valid number").await;
            }
        };
        db::update_points(&chatter.twitch_id, new_points);

        return messaging::reply_to(client, msg, "Points updated!").await;
    }

    return messaging::reply_to(client, msg, "You aren't allowed to set points").await;
}

pub async fn handle_gift_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // set a chatters points
    // format: !setpoints @<chatter_name> <new_point_value>

    let gifter = match db::get_chatter(msg.sender().id()) {
        Some(chatter) => chatter,
        None => {
            return messaging::reply_to(client, msg, "Something went wrong! Try Again!").await;
        }
    };

    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let chatter_name = match cmd_iter.next() {
        Some(name) => match name.chars().nth(0) {
            Some('@') => &name[1..],
            _ => name,
        },
        None => {
            return messaging::reply_to(
                client,
                msg,
                "Format is incorrect! try !gift @<username> <points>",
            )
            .await;
        }
    };

    let chatter = match (db::get_chatter_by_username(&chatter_name)) {
        Some(user) => user,
        None => {
            return messaging::reply_to(client, msg, "No chatter with that name!").await;
        }
    };

    let points = match cmd_iter.next() {
        Some(chal) => chal,
        None => "100",
    };

    let new_points: i32 = match points.parse() {
        Result::Ok(p) => match p {
            p if p < 0 => {
                return messaging::reply_to(client, msg, "provide a positive value").await;
            }
            _ => p,
        },
        Result::Err(_) => {
            return messaging::reply_to(client, msg, "Provide a valid number").await;
        }
    };

    if gifter.points < new_points {
        return messaging::reply_to(
            client,
            msg,
            "You don't have enough points to gift that many!",
        )
        .await;
    }
    chatter::add_points(&chatter.twitch_id, new_points);
    chatter::subtract_points(&gifter.twitch_id, new_points);

    let reply_msg = format!(
        "@{} gifted {} points to @{}",
        gifter.username, new_points, chatter.username
    );

    return messaging::send_msg(client, msg, &reply_msg).await;
}
