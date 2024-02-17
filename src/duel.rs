use chrono::{DateTime, Local};
use std::num::NonZeroU32;
use std::str::FromStr;

use crate::{chatter::TwitchUserId, messaging::send_msg, state::State};

#[derive(Copy, Clone)]
enum QuestionKind {
    ProgLang,
    Scramble,
    Riddle,
    Math,
}
#[derive(Copy, Clone)]
struct Question {
    q: &'static str,
    a: &'static str,
    k: QuestionKind,
}

#[rustfmt::skip]
const QUESTIONS: [Question; 10] = [
    Question {q: "ulot", a: "tolu", k: QuestionKind::Scramble },
    Question { q: "lopo", a: "pool", k: QuestionKind::Scramble },
    Question { q: "chooectal", a: "chocolate", k: QuestionKind::Scramble },
    Question { q: "ubritro", a: "burrito", k: QuestionKind::Scramble },
    Question { q: "algansa", a: "lasagna", k: QuestionKind::Scramble },
    Question { q: "This language can't have 'do for' loops", a: "python",k: QuestionKind::ProgLang },
    Question { q: "This language is full of parenthesis", a: "racket", k: QuestionKind::ProgLang },
    Question { q: "This langauge is much older than you.", a: "cobol",k: QuestionKind::ProgLang },
    Question { q: "Can go func yourself on accident", a: "go", k: QuestionKind::ProgLang },
    Question { q: "Only 7 people use this language", a: "haskell", k: QuestionKind::ProgLang },
];

#[derive(Debug, Clone)]
pub struct Duel {
    challenge_datetime: DateTime<Local>,

    pub challenger: TwitchUserId,

    pub challenged: TwitchUserId,

    points: NonZeroU32,

    pub winner: TwitchUserId,

    accepted: bool,
}

impl Duel {
    pub fn new(challenger: &str, challenged: &str, points: NonZeroU32, state: &mut State) -> Duel {
        let dt = Local::now();

        Duel {
            challenge_datetime: dt,
            challenger: TwitchUserId::from_str(challenger).unwrap(),
            challenged: TwitchUserId::from_str(challenged).unwrap(),
            points,
            winner: TwitchUserId::from_str("").unwrap(),
            accepted: false,
        }
    }

    pub fn accept_duel(&mut self) -> bool {
        self.accepted = true;
        self.accepted
    }

    pub async fn ask_question(&mut self, client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> () {
        //     TODO: ask question, listen for answer from duelists, award first to answer correct
        let question = self.generate_question();

        send_msg(client, msg, question.q).await.unwrap();
    }

    fn generate_question(&mut self) -> Question {
        //     return a question and answer
        //     unscramble a word
        //     quick maths
        //     riddle
        QUESTIONS[1]
    }
    fn award_winner(&mut self, username: TwitchUserId) -> () {}
}

// duel flow

// generate question

// listen for answers

// determine winner

//
