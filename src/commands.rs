use crate::duel;
use crate::state::State;
use std::num::NonZeroU32;

pub async fn handle_yo_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    crate::messaging::reply_to(client, msg, "yo").await
}

pub async fn handle_commands_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    crate::messaging::reply_to(client, msg, "!yo !duel !accept").await
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
            "@{} @{} the duel has been accepted! Prepare to battle in 3 seconds",
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

    let points = match cmd_iter.next() {
        Some(chal) => chal,
        None => {
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                msg,
                "You need to provide a point value.",
            )
            .await;
        }
    };

    let points: NonZeroU32 = match points.parse() {
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

    let curr_duel = duel::Duel::new(&challenger, &challenged, points, bot_state);

    bot_state.save_duel(&curr_duel);
    dbg!(curr_duel);

    crate::messaging::reply_to(
        client,
        &msg,
        &format!("@{} Challenge Announced", challenger),
    )
    .await
}
