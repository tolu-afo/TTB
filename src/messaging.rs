use duel_bot::*;
use crate::commands;
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

    record_user_presence(&msg.sender().id(), &msg.sender().name());

    // TODO: add answer command
    // TODO: add !points command
    match msg.text().split_ascii_whitespace().next() {
        Some("!commands") => commands::handle_commands_command(client, &msg).await,
        Some("!yo") => commands::handle_yo_command(client, &msg).await,
        Some("!accept") => commands::handle_accept_command(client, msg, bot_state).await,
        Some("!challenge") => commands::handle_duel_command(client, msg, bot_state).await,
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
