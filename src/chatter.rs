use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::TimeZone;
use log::info;

use crate::db::{
    self, get_chatter, get_lurker, update_losses, update_lurk_time, update_points, update_wins,
};
use crate::models::Duel;

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

// TODO: Add Points NewType Idiom https://doc.rust-lang.org/rust-by-example/generics/new_types.html

#[derive(Debug, Clone)]
pub struct Chatter {
    id: u32,
    username: TwitchUserId,
    points: u32,
    wins: u32,
    losses: u32,
    last_seen: String,
    lurk_time: u32,
}

pub fn unlurk(twitch_id: &str) -> () {
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

    // add to lurk_time on chatters table
}

pub fn add_points(twitch_id: &str, points: i32) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_points = chatter.points + points;
            update_points(twitch_id, new_points)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn subtract_points(twitch_id: &str, points: i32) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_points = chatter.points - points;
            update_points(twitch_id, new_points)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn get_points(twitch_id: &str) -> i32 {
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

pub fn subtract_win(twitch_id: &str) -> () {
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

pub fn subtract_loss(twitch_id: &str) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_losses = chatter.losses - 1;
            update_losses(twitch_id, new_losses)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn add_lurk_time(twitch_id: &str, lurk_time: i32) -> () {
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
