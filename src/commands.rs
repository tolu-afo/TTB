use crate::db::{accept_duel, create_duel, get_chatter_by_username};

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
    crate::messaging::reply_to(client, msg, "!yo !duel !accept").await
}

pub async fn handle_accept_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    // check that username of msg matches a challenged in a duel
    // !accept @<user>
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let challenged = msg.sender().name();
    let challenged_id = msg.sender().id();
    let challenger = match cmd_iter.next() {
        Some(chal) => match chal.chars().nth(0) {
            Some('@') => &chal[1..],
            _ => chal,
        },
        None => {
            return crate::messaging::send_duel_err(
                &challenged,
                client,
                &msg,
                "You need to provide a username in the format @<user> or <user>",
            )
            .await;
        }
    };

    let challenger_id = match get_chatter_by_username(challenger) {
        Some(chatter) => chatter.twitch_id,
        None => {
            return crate::messaging::send_duel_err(
                &challenged,
                client,
                &msg,
                "Please @ the user who challenged you.",
            )
            .await;
        }
    };

    // let key = format!("{}{}", challenger, challenged);
    // TODO: replace below code
    accept_duel(&challenger_id, challenged_id);

    // duel
    let _ = crate::messaging::send_msg(
        client,
        &msg,
        &format!(
            "@{} @{} the duel has been accepted! Prepare to Battle!",
            challenger, challenged
        ),
    )
    .await;
    // duel.ask_question(client, &msg).await;

    // TODO: Run Duel, Get Winner, Give Points
    return Ok(());
}

pub async fn handle_duel_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();

    let challenger = msg.sender().id();
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
                &msg,
                "You need to provide a username in the format @<user> or <user>",
            )
            .await;
        }
    };

    let challenged_id = match get_chatter_by_username(challenged) {
        Some(chatter) => chatter.twitch_id,
        None => {
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                &msg,
                "You must provide a username for a user that has chatted before.",
            )
            .await;
        }
    };

    let points = match cmd_iter.next() {
        Some(p) => p,
        None => {
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                &msg,
                "You need to provide a point value.",
            )
            .await;
        }
    };

    let points: i32 = match points.parse() {
        Ok(p) => p,
        Err(_) => {
            return crate::messaging::send_duel_err(
                &challenger,
                client,
                &msg,
                "Provide a valid point value.",
            )
            .await;
        }
    };

    if cmd_iter.next().is_some() {
        return crate::messaging::send_duel_err(&challenger, client, msg, "Too many arguments!")
            .await;
    }

    let curr_duel = create_duel(&challenger, &challenged_id, points);

    crate::messaging::reply_to(
        client,
        &msg,
        &format!("@{} Challenge Announced", challenged),
    )
    .await
}
