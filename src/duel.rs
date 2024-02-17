use crate::content::question::Question;
use crate::{chatter::TwitchUserId, messaging::send_msg, state::State};
use std::num::NonZeroU32;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Duel {
    accepted: bool,
    pub points: NonZeroU32,
    pub challenger: TwitchUserId,
    pub challenged: TwitchUserId,
    pub winner: TwitchUserId,
}

impl Duel {
    pub fn new(challenger: &str, challenged: &str, points: NonZeroU32, state: &mut State) -> Duel {
        Duel {
            accepted: false,
            points,
            challenger: TwitchUserId::from_str(challenger).unwrap(),
            challenged: TwitchUserId::from_str(challenged).unwrap(),
            winner: TwitchUserId::from_str("").unwrap(),
        }
    }

    pub fn accept_duel(&mut self) {
        self.accepted = true;
    }

    pub async fn ask_question(&mut self, client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> () {
        let question = Question::randomized();
        send_msg(client, msg, question.q).await.unwrap();
    }

    fn award_winner(&mut self, username: TwitchUserId) -> () {}
}

// TODO
// duel flow
// generate question
// listen for answers
// determine winner
