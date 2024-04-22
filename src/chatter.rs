use std::str::FromStr;

use anyhow::{anyhow, Result};
use log::info;

use crate::db::{get_chatter, update_losses, update_points, update_wins};

// top 3 duelists

#[derive(Debug, Clone)]
pub struct TwitchUsername(String);

pub type TwitchId = i32;

impl FromStr for TwitchUsername {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(s);
        if s.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Ok(TwitchUsername(String::from(s)))
        } else {
            Err(anyhow!("valid handles only contain characters 0-9 and a-f"))
        }
    }
}

impl std::fmt::Display for TwitchUsername {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: Add Points NewType Idiom https://doc.rust-lang.org/rust-by-example/generics/new_types.html

pub fn add_points(twitch_id: TwitchId, points: i32) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_points = chatter.points + points;
            update_points(twitch_id, new_points)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _subtract_points(twitch_id: TwitchId, points: i32) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_points = chatter.points - points;
            update_points(twitch_id, new_points)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn get_points(twitch_id: TwitchId) -> i32 {
    match get_chatter(twitch_id) {
        Some(chatter) => chatter.points,
        None => {
            info!("No Chatter with id: {}", twitch_id);
            0
        }
    }
}

pub fn _add_win(twitch_id: TwitchId) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_wins = chatter.wins + 1;
            update_wins(twitch_id, new_wins)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _subtract_win(twitch_id: TwitchId) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_wins = chatter.wins - 1;
            update_wins(twitch_id, new_wins)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _add_loss(twitch_id: TwitchId) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_losses = chatter.losses + 1;
            update_losses(twitch_id, new_losses)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}

pub fn _subtract_loss(twitch_id: TwitchId) -> () {
    match get_chatter(twitch_id) {
        Some(chatter) => {
            let new_losses = chatter.losses - 1;
            update_losses(twitch_id, new_losses)
        }
        None => info!("No Chatter with id: {} to update!", twitch_id),
    }
}
