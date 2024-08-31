use std::num::NonZeroU32;

use crate::db;
use crate::duel;
use crate::models;
use crate::state::State;

pub async fn handle_yo_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    crate::messaging::reply_to(client, msg, "yo").await
}

pub async fn handle_points_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    use crate::chatter::get_points;

    let points = get_points(msg.sender().id());

    let reply = format!("@{}, you have {} point(s)!", msg.sender().name(), points);
    crate::messaging::reply_to(client, msg, &reply).await
}

pub async fn handle_commands_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    crate::messaging::reply_to(client, msg, "!yo !points !challenge !accept").await
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
            return crate::messaging::send_duel_err(
                &challenged,
                client,
                msg,
                "You need to provide a username in the format @<user> or <user>",
            )
            .await;
        }
    };
    let key = format!("{}{}", challenger, challenged);
    let duel = match bot_state.duel_cache.get_mut(&key) {
        Some(d) => {
            d.accept_duel();
            d
        }
        None => {
            return crate::messaging::send_duel_err(&challenged, client, msg, "Wrong opponent!")
                .await;
        }
    };

    // duel
    let _ = crate::messaging::send_msg(
        client,
        &msg,
        &format!(
            "@{} @{} the duel has been accepted! Prepare to battle! Once you read the question; type '!answer <your answer>' to answer!",
            challenger, challenged
        ),
    )
    .await;
    duel.ask_question(client, &msg).await;

    // TODO: Run Duel, Get Winner, Give Points, remove duel from  cache.
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
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                msg,
                "You need to provide a username in the format @<user> or <user>",
            )
            .await;
        }
    };

    let points = match cmd_iter.next() {
        Some(chal) => chal,
        None => "100",
    };

    let points: i32 = match points.parse() {
        Ok(p) => p,
        Err(_) => {
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                msg,
                "Provide a valid point value.",
            )
            .await;
        }
    };

    if cmd_iter.next().is_some() {
        return crate::messaging::send_duel_err(&challenger, client, msg, "Too many arguments!")
            .await;
    }

    let curr_duel = models::Duel::new(&challenger, &challenged, points, bot_state);

    bot_state.save_duel(&curr_duel);
    db::create_duel(&challenger, &challenged, points as i32);

    crate::messaging::reply_to(
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
    bot_state: &mut State,
) -> anyhow::Result<(), anyhow::Error> {
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let challenger = dbg!(msg.sender().name());
    let answer = match dbg!(cmd_iter.next()) {
        Some(chal) => chal,
        None => {
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                msg,
                "You need to provide an answer.",
            )
            .await;
        }
    };

    let key = format!("{}{}", challenger, challenger);
    let duel = match bot_state.duel_cache.get_mut(&key) {
        Some(d) => d,
        None => {
            return crate::messaging::send_duel_err(&challenger, client, msg, "No duel found!")
                .await;
        }
    };

    if duel.is_winner(answer) {
        // determine which player owns the current messsage
        // compare twitch id of challenger to the twitch id of the message sender
        let winner = if challenger == duel.challenger {
            duel.challenger
        } else {
            duel.challenged
        };
        // award points to the winner
        duel.award_winner(&challenger);
    } else {
        // Deduct points for incorrect guess
        // send message to inform user of point deduction and incorrect guess.
        // max 5 guesses before duel is over, and challenger lose wagered points
    }

    crate::messaging::reply_to(client, &msg, "Answered!").await
}
