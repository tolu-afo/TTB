use crate::chatter::add_points;
use crate::commands;
use crate::db;
use crate::state::State;

pub async fn send_duel_err(
    challenger: &str,
    client: &mut tmi::Client,
    msg: tmi::Privmsg<'_>,
    err: &str,
) -> anyhow::Result<()> {
    send_msg(client, &msg, &format!("@{} Error; {}", challenger, err)).await
}

pub async fn send_msg(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
    text: &str,
) -> anyhow::Result<(), anyhow::Error> {
    client.privmsg(msg.channel(), text).send().await?;
    Ok(())
}

pub async fn on_msg(
    client: &mut tmi::Client,
    msg: tmi::Privmsg<'_>,
    bot_state: &mut State,
) -> anyhow::Result<()> {
    println!("{}: {}", msg.sender().name(), msg.text());
    dbg!(&msg);
    db::record_user_presence(&msg.sender().id(), &msg.sender().name());
    add_points(&msg.sender().id(), 5);

    // TODO: !accept with no user, accepts if there is only one duel
    // TODO: make lurk end on next comment

    // TODO: add Moderator Commands
    // TODO: add rabbit hole command

    // TODO: POINT vs Wins ranking

    // Question Modifications
    // TODO: Change scramble every time

    // Custom Questions
    // TODO: Add categories command
    // TODO: add command for adding new questions
    // Duel Changes
    // TODO: add rematch command (double or nothing)
    // TODO: Add bot to Discord

    match msg.text().split_ascii_whitespace().next() {
        Some("!points") => commands::handle_points_command(client, &msg).await,
        Some("!commands") => commands::handle_commands_command(client, &msg).await,
        Some("!yo") => commands::handle_yo_command(client, &msg).await,
        Some("!lurk") => commands::handle_lurk_command(client, &msg).await,
        Some("!unlurk") => commands::handle_unlurk_command(client, &msg).await,
        Some("!lurkers") => commands::handle_lurkers_command(client, &msg).await,
        Some("!lurktime") => commands::handle_lurktime_command(client, &msg).await,
        Some("!accept") => commands::handle_accept_command(client, msg, bot_state).await,
        Some("!answer") => commands::handle_answer_command(client, msg).await,
        Some("!challenge") => commands::handle_duel_command(client, msg, bot_state).await,
        Some("!duel") => commands::handle_duel_command(client, msg, bot_state).await,
        Some("!kda") => commands::handle_kda_command(client, msg).await,
        Some("!repeat") => commands::handle_repeat_command(client, msg).await,
        Some("!topDuelists") => commands::handle_top_duelists_command(client, msg).await,
        Some("!ranking") => commands::handle_ranking_command(client, msg).await,
        _ => Ok(()),
    }
}

pub async fn reply_to(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
    reply: &str,
) -> anyhow::Result<(), anyhow::Error> {
    client
        .privmsg(msg.channel(), reply)
        .reply_to(msg.message_id())
        .send()
        .await?;
    Ok(())
}
