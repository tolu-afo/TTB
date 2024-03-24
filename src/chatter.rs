use std::str::FromStr;

use anyhow::{anyhow, Result};
use log::info;

use crate::db::{get_chatter, update_losses, update_points, update_wins};

// top 3 duelists

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
