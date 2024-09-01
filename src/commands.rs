use crate::db;
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

    // TODO: Run Duel, Get Winner, Give Points, remove duel from cache.
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

    // Find challenger and challenged in chatter table
    let challenger_chatter = db::get_chatter_by_username(&challenger).unwrap();
    let challenged_chatter = db::get_chatter_by_username(&challenged).unwrap();

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

    let curr_duel = models::Duel::new(
        &challenger,
        &challenged,
        &challenged_chatter.twitch_id,
        &challenged_chatter.twitch_id,
        points,
    );

    bot_state.save_duel(&curr_duel);

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
    let responder = dbg!(msg.sender().name());
    let answer = match dbg!(cmd_iter.next()) {
        Some(chal) => chal,
        None => {
            return crate::messaging::send_duel_err(
                &responder,
                client,
                msg,
                "You need to provide an answer.",
            )
            .await;
        }
    };

    // let key = format!("{}{}", challenger, challenger);
    // let duel = match bot_state.duel_cache.get_mut(&key) {
    //     Some(d) => d,
    //     None => {
    //         return crate::messaging::send_duel_err(&responder, client, msg, "No duel found!")
    //             .await;
    //     }
    // };

    let mut duel = match db::get_accepted_duel(&responder) {
        Some(d) => match db::get_duel(d.duel_id) {
            Some(duel) => duel,
            None => {
                return crate::messaging::send_duel_err(&responder, client, msg, "No duel found!")
                    .await;
            }
        },
        None => {
            return crate::messaging::send_duel_err(&responder, client, msg, "No duel found!")
                .await;
        }
    };

    if duel.is_winner(answer) {
        // determine which player owns the current messsage
        // compare twitch id of challenger to the twitch id of the message sender
        if responder == duel.challenger {
            duel.award_winner(
                &responder,
                duel.challenger_id.clone().unwrap().as_str(),
                duel.challenged_id.clone().unwrap().as_str(),
            );
            let reply_msg = format!("Correct! @{} wins the duel! @{} loses the duel! Therefore, @{} is awarded {} points, and @{} loses {} points", responder, duel.challenged, responder, duel.points, duel.challenged, duel.points / 2);
            crate::messaging::reply_to(client, &msg, &reply_msg).await?;
        } else if responder == duel.challenged {
            duel.award_winner(
                &responder,
                duel.challenged_id.clone().unwrap().as_str(),
                duel.challenger_id.clone().unwrap().as_str(),
            );
            let reply_msg = format!("Correct! @{} wins the duel! @{} loses the duel! Therefore, @{} is awarded {} points, and @{} loses {} points", responder, duel.challenger, responder, duel.points, duel.challenger, duel.points / 2);
            crate::messaging::reply_to(client, &msg, &reply_msg).await?;
        };
    } else {
        // Deduct points for incorrect guess
        // send message to inform user of point deduction and incorrect guess.
        // max 5 guesses before duel is over, and challenger lose wagered points
        // TODO: Decrement Guesses
        // TODO: End Duel if both players have guessed incorrectly 5 times i.e. guesses go to zero
        // TODO: Set Duel Status to Completed
        // TODO: Destroy AcceptedDuel record in accepted_duels table
    }

    crate::messaging::reply_to(client, &msg, "Answered!").await
}
