use crate::chatter::TwitchUserId;
use crate::content::question::Question;
use crate::db;
use crate::messaging::send_msg;
use crate::schema::duels::winner;
use crate::state::State;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chatters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatter {
    pub id: i32,
    pub twitch_id: String,
    pub username: String,
    pub points: i32,
    pub wins: i32,
    pub losses: i32,
    pub last_seen: NaiveDateTime,
}
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::duels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Duel {
    pub id: i32,
    pub accepted: bool,
    pub points: i32,
    pub challenger: String,
    pub challenged: String,
    pub challenger_id: Option<String>,
    pub challenged_id: Option<String>,
    pub winner: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub question: Option<String>,
    pub answer: Option<String>,
}

#[derive(Debug, Clone)]
pub enum DuelStatus {
    Challenged,
    Accepted,
    Completed,
}

impl Duel {
    pub fn new(
        challenger: &str,
        challenged: &str,
        challenger_id: &str,
        challenged_id: &str,
        points: i32,
    ) -> Duel {
        db::create_duel(challenger, challenged, challenger_id, challenged_id, points)
    }
    pub fn accept_duel(&mut self) {
        db::accept_duel(self.id);
        db::create_accepted_duel(self.id, &self.challenger, &self.challenged);
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

    pub fn award_winner(
        &mut self,
        duel_winner: &str,
        duel_winner_id: &str,
        duel_loser_id: &str,
    ) -> () {
        use crate::chatter;
        self.winner = Some(duel_winner.to_string());
        chatter::add_points(duel_winner_id, self.points);
        chatter::add_win(duel_winner_id);
        chatter::add_loss(duel_loser_id);
        chatter::subtract_points(duel_loser_id, self.points / 2);

        db::complete_duel(self.id, duel_winner, "completed");
        db::destroy_accepted_duel(self.id);
    }
}

// TODO
// duel flow
// generate question
// listen for answers
// determine winner

use crate::schema::chatters;

#[derive(Insertable)]
#[diesel(table_name = chatters)]
pub struct NewChatter<'a> {
    pub twitch_id: &'a str,
    pub username: &'a str,
}

use crate::schema::duels;

#[derive(Insertable)]
#[diesel(table_name = duels)]
pub struct NewDuel<'a> {
    pub challenger: &'a str,
    pub challenged: &'a str,
    pub challenger_id: &'a str,
    pub challenged_id: &'a str,
    pub points: i32,
}

use crate::schema::accepted_duels;

#[derive(Insertable)]
#[diesel(table_name = accepted_duels)]
pub struct NewAcceptedDuel<'a> {
    pub duel_id: i32,
    pub challenger_id: &'a str,
    pub challenged_id: &'a str,
}

#[derive(Debug, Clone, Queryable, Selectable)]
pub struct AcceptedDuel {
    pub id: i32,
    pub duel_id: i32,
    pub challenger_id: String,
    pub challenged_id: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
