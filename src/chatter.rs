use std::str::FromStr;

use crate::helpers::overflow_add;
use anyhow::{anyhow, Result};
use chrono::TimeZone;
use log::info;

use crate::db::{
    self, get_chatter, get_lurker, update_losses, update_lurk_time, update_points, update_wins,
};
use crate::messaging;

#[derive(Debug, Clone)]
pub struct TwitchUserId(String);

impl FromStr for TwitchUserId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(s);
        if s.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Ok(TwitchUserId(String::from(s)))
        } else {
            Err(anyhow!("valid handles only contain characters 0-9 and a-f"))
        }
    }
}

impl std::fmt::Display for TwitchUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn unlurk(client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> () {
    let twitch_id = msg.sender().id();
    let lurker = match get_lurker(twitch_id.to_string()) {
        Some(lurker) => lurker,
        None => {
            info!("No Lurker with id: {} to update!", twitch_id);
            return;
        }
    };

    let now = chrono::Utc::now();
    let tz_created_at: chrono::DateTime<chrono::Utc> =
        chrono::Utc.from_utc_datetime(&lurker.created_at.unwrap());
    let time_lurked: i32 = now
        .signed_duration_since(tz_created_at)
        .num_seconds()
        .try_into()
        .unwrap();

    let chatter = match get_chatter(twitch_id) {
        Some(chatter) => chatter,
        None => {
            info!("No Chatter with id: {} to update!", twitch_id);
            return;
        }
    };

    let new_lurk_time = chatter.lurk_time + time_lurked;
    db::update_lurk_time(twitch_id, new_lurk_time);
    db::delete_lurker(twitch_id.to_owned());

    // welcome chatter back from lurk
    let _ = messaging::reply_to(
        client,
        msg,
        &format!(
            "Welcome back, {}! You were lurking for {} seconds.",
            chatter.username, time_lurked
        ),
    );
}

pub async fn on_new_chatter(client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> () {
    // greet new chatter and give 1000 points

    let twitch_id = msg.sender().id();
    let twitch_name = msg.sender().name();

    add_points(twitch_id, 1000);
    let _ = messaging::reply_to(
        client,
        msg,
        &format!("Welcome, {}! You have been given 1000 points, to gamble and duel with, type !commands to see what you can do.", twitch_name),
    ).await;
}
// TODO: Add a saturaton operation for negative overflows crates: ranged_integers, constrained_int, deranged (deranged might be the best one?)
// TODO: check out: checked_add ie. 5.checked_add(6)
pub fn add_points(twitch_id: &str, points: i64) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_points = if (overflow_add(chatter.points, points)) < -1000 {
                -1000
            } else {
                overflow_add(chatter.points, points)
            };
            update_points(twitch_id, new_points)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn subtract_points(twitch_id: &str, points: i64) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_points = if (chatter.points - points) < -1000 {
                -1000
            } else {
                chatter.points - points
            };
            update_points(twitch_id, new_points)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn get_points(twitch_id: &str) -> i64 {
    match get_chatter(twitch_id) {
        Some(chatter) => chatter.points,
        None => {
            info!("No Chatter with id: {}", twitch_id);
            0
        }
    }
}

pub fn add_win(twitch_id: &str) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_wins = chatter.wins + 1;
            update_wins(twitch_id, new_wins)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _subtract_win(twitch_id: &str) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_wins = chatter.wins - 1;
            update_wins(twitch_id, new_wins)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn add_loss(twitch_id: &str) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_losses = chatter.losses + 1;
            update_losses(twitch_id, new_losses)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _subtract_loss(twitch_id: &str) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_losses = chatter.losses - 1;
            update_losses(twitch_id, new_losses)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _add_lurk_time(twitch_id: &str, lurk_time: i32) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_lurk_time = dbg!(chatter.lurk_time) + lurk_time;
            update_lurk_time(twitch_id, dbg!(new_lurk_time))
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn get_challenge_to_accept(twitch_id: &str) -> Option<String> {
    let challenges = db::get_challenges(twitch_id);
    match challenges.len() {
        0 => None,
        1 => {
            let challenge = challenges[0].clone();
            Some(challenge.challenger)
        }
        _ => None,
    }
}
