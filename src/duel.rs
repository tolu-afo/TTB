use std::num::NonZeroU32;
use chrono::{Local, DateTime};
use std::str::FromStr;

use crate::{chatter::TwitchUserId, state::{self, State}};

#[derive(Debug, Clone)]
pub struct Duel {
    challenge_datetime: DateTime<Local>,
    
    pub challenger: TwitchUserId, 
    
    pub challenged: TwitchUserId,
    
    points: NonZeroU32,

    pub winner: TwitchUserId, 

    accepted: bool
}

impl Duel {
    pub fn new(challenger: &str, challenged: &str, points: NonZeroU32, state: &mut State) -> Duel {

        let dt = Local::now();

        Duel {
            challenge_datetime: dt,
            challenger: TwitchUserId::from_str(challenger).unwrap(),
            challenged: TwitchUserId::from_str(challenged).unwrap(),
            points: points,
            winner: TwitchUserId::from_str("").unwrap(),
            accepted: false
        }
    }

    pub fn accept_duel(&mut self) -> bool {
        self.accepted = true;
        self.accepted
    }
}


// duel flow

    // generate question

    // listen for answers

    // determine winner

    // 
