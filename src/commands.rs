use anyhow::Result;
use chrono::TimeZone;

use crate::db;
use crate::messaging;
use crate::models;
use crate::state::State;

pub async fn handle_yo_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // TODO: Different message responses for yo command
    messaging::reply_to(client, msg, "yo").await
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
    messaging::reply_to(client, msg, "!yo !points !challenge !duel !accept").await
}

pub async fn handle_accept_command(
    client: &mut tmi::Client,
    msg: tmi::Privmsg<'_>,
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
            return messaging::send_duel_err(
                &challenged,
                client,
                msg,
                "You need to provide a username in the format @<user> or <user>",
            )
            .await;
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

    // duel
    let _ = messaging::send_msg(
        client,
        &msg,
        &format!(
            "@{} @{} the duel has been accepted! Prepare to battle! Once you read the question; type '!answer <your answer>' to answer!",
            challenger, challenged
        ),
    )
    .await;
    duel.ask_question(client, &msg).await;

    return Ok(());
}

pub async fn handle_duel_command(
    client: &mut tmi::Client,
    msg: tmi::Privmsg<'_>,
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
                "You need to provide a username in the format @<user> or <user>",
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
            return messaging::send_duel_err(&challenger, client, msg, "Chatter not found!").await;
        }
    };
    let challenged_chatter = match db::get_chatter_by_username(&challenged) {
        Some(chatter) => chatter,
        None => {
            return messaging::send_duel_err(&challenger, client, msg, "Chatter not found!").await;
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
                    msg,
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

    let points: i32 = match points.parse() {
        Result::Ok(p) => match p {
            p if p < 0 => {
                return messaging::send_duel_err(
                    &challenger,
                    client,
                    msg,
                    "Provide a positive point value.",
                )
                .await;
            }
            p if p > challenger_chatter.points => {
                return messaging::send_duel_err(
                    &challenger,
                    client,
                    msg,
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
                msg,
                "Provide a valid point value.",
            )
            .await;
        }
    };

    if cmd_iter.next().is_some() {
        return messaging::send_duel_err(&challenger, client, msg, "Too many arguments!").await;
    }

    let curr_duel = models::Duel::new(
        &challenger,
        &challenged,
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
            challenger, challenged, challenger
        ),
    )
    .await
}

pub async fn handle_answer_command(
    client: &mut tmi::Client,
    msg: tmi::Privmsg<'_>,
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
            let mut reply = String::new();
            if duel.challenger_guesses - 1 <= 0 {
                reply = format!("Incorrect! @{} you are out of guesses!", duel.challenger);
            } else {
                reply = format!(
              "Incorrect! @{} you have {} guesses remaining! type '!repeat' to repeat the question",
              duel.challenger, duel.challenger_guesses-1);
            };
            messaging::reply_to(client, &msg, reply.as_str()).await?;
        } else if responder == duel.challenged {
            if duel.challenged_guesses > 0 {
                duel.decrement_challenged_guesses()
            };
            let mut reply = String::new();
            if duel.challenged_guesses - 1 == 0 {
                reply = format!("Incorrect! @{} you are out of guesses!", duel.challenged);
            } else {
                reply = format!(
                  "Incorrect! @{} you have {} guesses remaining! type '!repeat' to repeat the question",
                  duel.challenged, duel.challenged_guesses-1
               );
            }
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
    msg: tmi::Privmsg<'_>,
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
