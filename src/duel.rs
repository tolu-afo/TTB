use crate::content::question::Question;
use crate::db::create_duel;
use crate::models::{Duel, NewDuel};
use crate::{chatter::TwitchUserId, messaging::send_msg, state::State};
use std::num::NonZeroU32;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum DuelState {
    Challenged,
    Accepted,
    Completed,
}

impl Duel {
    pub fn new(challenger: &str, challenged: &str, points: i32, state: &mut State) -> Duel {
        create_duel(challenger, challenged, points)
    }

    pub fn accept_duel(&mut self) {
        self.accepted = true;
    }

    pub async fn ask_question(&mut self, client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> () {
        let question = Question::randomized();

        let category_announcement = format!(
            "@{} @{} the category is: {}",
            self.challenger,
            self.challenged,
            question.display_question_kind()
        );
        let question_msg = format!(
            "@{} @{} your question is: {}",
            self.challenger, self.challenged, question.q
        );
        send_msg(client, msg, &category_announcement).await.unwrap();
        send_msg(client, msg, &question_msg).await.unwrap();
    }

    pub fn is_winner(&self, answer: &str) -> bool {
        if Some(answer.to_string()) == self.answer {
            true
        } else {
            false
        }
    }

    fn award_winner(&mut self, username: TwitchUserId) -> () {}
}

// TODO
// duel flow
// generate question
// listen for answers
// determine winner
