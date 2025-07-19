use crate::db;
use crate::messaging::send_msg;
use crate::schema::categories;
use crate::schema::losers_pool;
use crate::schema::lurkers;
use crate::schema::questions;
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
    pub points: i64,
    pub wins: i32,
    pub losses: i32,
    pub last_seen: NaiveDateTime,
    pub lurk_time: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::duels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Duel {
    pub id: i32,
    pub accepted: bool, // TODO: depecrated field slated for removal
    pub points: i64,
    pub challenger: String,
    pub challenged: String,
    pub winner: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub challenger_id: Option<String>,
    pub challenged_id: Option<String>,
    pub challenger_guesses: i32,
    pub challenged_guesses: i32,
}

#[allow(dead_code)]
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
        points: i64,
    ) -> Duel {
        db::create_duel(challenger, challenged, challenger_id, challenged_id, points)
    }
    pub fn accept_duel(&mut self) {
        db::accept_duel(self.id);
        db::create_accepted_duel(self.id, &self.challenger, &self.challenged);
    }

    pub async fn ask_question(&mut self, client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> () {
        let question = match db::get_random_question() {
            Some(q) => q,
            None => {
                let _ = send_msg(client, msg, "No questions in the database yet!").await;
                return;
            }
        };

        let question_announcement = format!(
            "@{} @{} - format: '!a <answer>' - {}: {}",
            self.challenger,
            self.challenged,
            question.display_question_kind(),
            question.question
        );
        let _ = send_msg(client, msg, &question_announcement).await;
        db::set_question_duel(self.id, &question.question, &question.answer)
    }

    pub async fn repeat_question(
        &mut self,
        client: &mut tmi::Client,
        msg: &tmi::Privmsg<'_>,
    ) -> () {
        dbg!(&self);
        let question_msg = format!(
            "@{} @{} your question is: {}",
            self.challenger,
            self.challenged,
            self.question.as_ref().unwrap()
        );
        send_msg(client, msg, &question_msg).await.unwrap();
    }

    pub fn is_winner(&self, answer: &str) -> bool {
        let binding = answer.to_string().to_lowercase();
        let guess = binding.trim();
        let binding = self.answer.as_ref().unwrap().to_lowercase();
        let correct_answer = binding.trim();
        dbg!(&guess);
        dbg!(&correct_answer);
        if dbg!(guess == correct_answer) {
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

    pub fn decrement_challenger_guesses(&mut self) -> () {
        db::decrement_guesses(self.id, true);
    }

    pub fn decrement_challenged_guesses(&mut self) -> () {
        db::decrement_guesses(self.id, false);
    }

    pub fn complete_duel(&mut self, bot_state: &mut State) -> () {
        bot_state.clear_duel(self);
        db::complete_duel(self.id, "tie", "completed");
        db::destroy_accepted_duel(self.id);
    }
}

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
    pub points: i64,
}

use crate::schema::accepted_duels;

#[derive(Insertable)]
#[diesel(table_name = accepted_duels)]
pub struct NewAcceptedDuel<'a> {
    pub duel_id: i32,
    pub challenger_id: &'a str,
    pub challenged_id: &'a str,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Queryable, Selectable)]
pub struct AcceptedDuel {
    pub id: i32,
    pub duel_id: i32,
    pub challenger_id: String,
    pub challenged_id: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = lurkers)]
pub struct NewLurker<'a> {
    pub twitch_id: &'a str,
    pub username: &'a str,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Queryable, Selectable)]
pub struct Lurker {
    pub id: i32,
    pub username: String,
    pub twitch_id: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub submitter_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub submitter_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion<'a> {
    pub question: &'a str,
    pub answer: &'a str,
    pub category_id: i32,
    pub submitter_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Category))]
#[diesel(table_name = questions)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub category_id: i32,
    pub submitter_id: i32,
    pub times_asked: i32,
    pub times_not_answered: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Question {
    pub fn new(question: &str, answer: &str, category: &Category, submitter: &Chatter) -> Question {
        let twitch_id = submitter.twitch_id.parse().unwrap();
        db::create_question(question, answer, twitch_id, category.id)
    }

    pub fn display_question_kind(&self) -> String {
        let cat = match db::get_category(self.category_id) {
            Some(c) => c,
            None => unreachable!("Category must exist, because of foreign key constraint"),
        };
        cat.name
    }
    // TODO: Make these actually increment the values in the database
    pub fn increment_times_asked(&mut self) -> () {
        let new_times_asked = self.times_asked + 1;
        db::update_times_asked(self.id, new_times_asked);
    }

    pub fn increment_times_not_answered(&mut self) -> () {
        db::update_times_not_answered(self.id);
    }
}

#[derive(Insertable)]
#[diesel(table_name = losers_pool)]
pub struct NewPool {
    pub amount: i64,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = losers_pool)]
pub struct LosersPool {
    pub id: i32,
    pub amount: i64,
    pub winner: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
