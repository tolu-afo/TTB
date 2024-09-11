use crate::chatter::{add_points, unlurk};
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

    unlurk(client, &msg);

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

    // TODO: !gamble command

    match msg.text().split_ascii_whitespace().next() {
        Some("!points") => commands::handle_points_command(client, &msg).await,
        Some("!commands") => commands::handle_commands_command(client, &msg).await,
        Some("!github") => commands::handle_github_command(client, &msg).await,
        Some("!botrepo") => commands::handle_botrepo_command(client, &msg).await,
        Some("!yo") => commands::handle_yo_command(client, &msg).await,
        Some("!addquestion") => commands::handle_addquestion_command(client, msg).await,
        Some("!lurk") => commands::handle_lurk_command(client, &msg).await,
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

const STAR_WITH_SPACE: &str = " ⭐ ";

pub enum ItemSeparator {
    Space,
    Comma,
    Dash,
    GoldStar,
}

#[must_use]
fn separator_str(separator: &ItemSeparator) -> String {
    match separator {
        ItemSeparator::Space => " ",
        ItemSeparator::Comma => ", ",
        ItemSeparator::Dash => " - ",
        ItemSeparator::GoldStar => STAR_WITH_SPACE,
    }
    .to_string()
}

/// This can be simplified a bit more once slice_concat_ext is stabilized
#[must_use]
fn format_list<S: AsRef<str>>(items: &Vec<S>, separator: ItemSeparator) -> String {
    let len = items.len();
    if len == 0 {
        return "None".to_string();
    }
    let separator_str = separator_str(&separator);
    let mut res = match separator {
        ItemSeparator::Space | ItemSeparator::Comma => "".to_string(),
        ItemSeparator::Dash | ItemSeparator::GoldStar => separator_str.to_string(),
    };

    for (i, item) in items.iter().enumerate() {
        res.push_str(item.as_ref());
        if i < len - 1 {
            res.push_str(&separator_str);
        }
    }

    res
}

#[must_use]
pub fn list_with_title<S: AsRef<str>>(
    title: &str,
    items: &Vec<S>,
    separator: ItemSeparator,
) -> String {
    format!("{} {}", title, format_list(items, separator))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_list() {
        assert_eq!("None", format_list::<String>(&vec![], ItemSeparator::Dash));
        assert_eq!(
            "a b c",
            format_list(&vec!["a", "b", "c"], ItemSeparator::Space)
        );
        assert_eq!(
            "a, b, c",
            format_list(&vec!["a", "b", "c"], ItemSeparator::Comma)
        );
        assert_eq!(
            " - a - b - c",
            format_list(&vec!["a", "b", "c"], ItemSeparator::Dash)
        );
    }

    #[test]
    fn test_list_with_title() {
        assert_eq!(
            "Entries: None",
            list_with_title::<String>("Entries:", &vec![], ItemSeparator::Space)
        );
        assert_eq!(
            "Entries: a b c",
            list_with_title("Entries:", &vec!["a", "b", "c"], ItemSeparator::Space)
        );
        assert_eq!(
            "Entries: a, b, c",
            list_with_title("Entries:", &vec!["a", "b", "c"], ItemSeparator::Comma)
        );
    }
}
