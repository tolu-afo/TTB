use anyhow::{anyhow, Result};
use std::str::FromStr;
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

#[derive(Debug, Clone)]
pub struct Chatter{
    id: u32,
    username: TwitchUserId,
    points: u32,
    wins: u32,
    losses: u32
}

pub fn add_points(username: TwitchUserId, points:u32) -> () {
    // TODO: add points to specified user by updating record in database


}

pub fn subtract_points(username: TwitchUserId, points: u32) -> () {
    // TODO: add points to specified user by updating record in database
}

pub fn get_points(username: TwitchUserId) -> u32 {
    // TODO: returns a users points to display as a u32
    return 0;
}

pub fn add_win(username: TwitchUserId) -> () {
    // TODO: add points to specified user by updating record in database
}

pub fn subtract_win(username: TwitchUserId, points: u32) -> () {
    // TODO: subtract a win to specified user by updating record in database
}

pub fn add_loss(username: TwitchUserId) -> () {
    // TODO: add a loss to specified user by updating record in database
}

pub fn subtract_loss(username: TwitchUserId, points: u32) -> () {
    // TODO: subtract a loss to specified user by updating record in database
}
