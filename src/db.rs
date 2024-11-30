use std::env;

use diesel::pg::PgConnection;
use diesel::{prelude::*, sql_function};
use dotenv::dotenv;
use log::info;

use crate::models::{
    AcceptedDuel, Category, Chatter, Duel, Lurker, NewAcceptedDuel, NewCategory, NewChatter,
    NewDuel, NewLurker, NewQuestion, Question,
};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_chatter(conn: &mut PgConnection, twitch_id: &str, username: &str) -> Chatter {
    use crate::schema::chatters;

    let new_chatter = NewChatter {
        username,
        twitch_id,
    };

    diesel::insert_into(chatters::table)
        .values(&new_chatter)
        .returning(Chatter::as_returning())
        .get_result(conn)
        .expect("Error saving new chatter")
}

pub fn db_get_chatter(conn: &mut PgConnection, chatter_id: &str) -> Option<Chatter> {
    use crate::schema::chatters::dsl::{chatters, twitch_id};

    let chatter = chatters
        .filter(twitch_id.eq(chatter_id))
        .select(Chatter::as_select())
        .first(conn)
        .optional();

    chatter.unwrap_or_else(|_| {
        println!("An error occurred while fetching chatter {}", chatter_id);
        None
    })
}

pub fn get_chatter(chatter_id: &str) -> Option<Chatter> {
    db_get_chatter(&mut establish_connection(), chatter_id)
}

fn db_get_chatter_by_username(conn: &mut PgConnection, username: &str) -> Option<Chatter> {
    use crate::schema::chatters::dsl::{chatters, username as chatter_name};

    let chatter = chatters
        .filter(chatter_name.eq(username))
        .select(Chatter::as_select())
        .first(conn)
        .optional();

    chatter.unwrap_or_else(|_| {
        println!("An error occurred while fetching chatter {}", username);
        None
    })
}

pub fn get_chatter_by_username(username: &str) -> Option<Chatter> {
    db_get_chatter_by_username(&mut establish_connection(), username)
}

fn update_last_seen(conn: &mut PgConnection, chatter_id: i32) {
    use crate::schema::chatters::dsl::{chatters, id, last_seen};
    use diesel::dsl;

    diesel::update(chatters)
        .filter(id.eq(chatter_id))
        .set(last_seen.eq(dsl::now))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

fn update_username(conn: &mut PgConnection, chatter_id: i32, new_username: &str) {
    use crate::schema::chatters::dsl::{chatters, id, username};

    diesel::update(chatters)
        .filter(id.eq(chatter_id))
        .set(username.eq(new_username))
        .returning(Chatter::as_returning())
        .execute(conn)
        .expect("Wrong Chatter ID");
}

pub fn record_user_presence(twitch_id: &str, username: &str) -> Chatter {
    let conn = &mut establish_connection();

    match db_get_chatter(conn, twitch_id) {
        Some(chatter) => {
            info!("Chatter found for {}", chatter.username);
            update_last_seen(conn, chatter.id);
            if chatter.username != username {
                update_username(conn, chatter.id, username);
            }
            chatter
        }
        None => {
            let chatter = create_chatter(conn, twitch_id, username);
            info!("Chatter created for twitch user {}", chatter.username);
            chatter
        }
    }
}

fn db_update_points(conn: &mut PgConnection, id: &str, new_points: i32) {
    use crate::schema::chatters::dsl::{chatters, points, twitch_id};

    diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(points.eq(new_points))
        .execute(conn)
        .expect("Points value should be i32");
}

pub fn update_points(id: &str, new_points: i32) {
    db_update_points(&mut establish_connection(), id, new_points)
}

fn db_update_wins(conn: &mut PgConnection, id: &str, new_wins: i32) {
    use crate::schema::chatters::dsl::{chatters, twitch_id, wins};
    diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(wins.eq(new_wins))
        .execute(conn)
        .expect("Wins value should be i32");
}

pub fn update_wins(id: &str, wins: i32) {
    db_update_wins(&mut establish_connection(), id, wins);
}

fn db_update_losses(conn: &mut PgConnection, id: &str, new_losses: i32) {
    use crate::schema::chatters::dsl::{chatters, losses, twitch_id};
    diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(losses.eq(new_losses))
        .execute(conn)
        .expect("Losses value should be i32");
}

pub fn update_losses(id: &str, losses: i32) {
    db_update_losses(&mut establish_connection(), id, losses);
}

fn db_update_lurk_time(conn: &mut PgConnection, id: &str, new_lurk_time: i32) {
    use crate::schema::chatters::dsl::{chatters, lurk_time, twitch_id};
    diesel::update(chatters.filter(twitch_id.eq(id)))
        .set(lurk_time.eq(new_lurk_time))
        .execute(conn)
        .expect("Lurk time value should be i32");
}

pub fn update_lurk_time(id: &str, new_lurk_time: i32) {
    db_update_lurk_time(&mut establish_connection(), id, new_lurk_time);
}

fn db_create_duel(
    conn: &mut PgConnection,
    challenger: &str,
    challenged: &str,
    challenger_id: &str,
    challenged_id: &str,
    points: i32,
) -> Duel {
    use crate::schema::duels;
    let new_duel = NewDuel {
        challenger,
        challenged,
        challenger_id: challenger_id,
        challenged_id: challenged_id,
        points,
    };

    diesel::insert_into(duels::table)
        .values(&new_duel)
        .returning(Duel::as_returning())
        .get_result(conn)
        .expect("Error saving new duel")
}

pub fn create_duel(
    challenger: &str,
    challenged: &str,
    challenger_id: &str,
    challenged_id: &str,
    points: i32,
) -> Duel {
    db_create_duel(
        &mut establish_connection(),
        challenger,
        challenged,
        challenger_id,
        challenged_id,
        points,
    )
}

fn db_get_duel(conn: &mut PgConnection, id: i32) -> Option<Duel> {
    use crate::schema::duels::dsl::duels;

    let duel = duels
        .find(id)
        .select(Duel::as_select())
        .first::<Duel>(conn)
        .optional();

    duel.unwrap_or_else(|_| {
        println!("An error occurred while fetching duel {}", id);
        None
    })
}

pub fn get_duel(duel_id: i32) -> Option<Duel> {
    db_get_duel(&mut establish_connection(), duel_id)
}

fn db_accept_duel(conn: &mut PgConnection, id: i32) {
    use crate::schema::duels::dsl::{duels, status as duel_status};

    diesel::update(duels.find(id))
        .set(duel_status.eq("accepted"))
        .execute(conn)
        .expect("Duel ID should be i32");
}

pub fn accept_duel(id: i32) {
    db_accept_duel(&mut establish_connection(), id);
}

fn db_get_accepted_duel(conn: &mut PgConnection, responder: &str) -> Option<AcceptedDuel> {
    use crate::schema::accepted_duels::dsl::{
        accepted_duels as duels, challenged_id as duel_challenged, challenger_id as duel_challenger,
    };
    let duel = duels
        .filter(
            duel_challenger
                .eq(responder)
                .or(duel_challenged.eq(responder)),
        )
        .select(AcceptedDuel::as_select())
        .first(conn)
        .optional();
    duel.unwrap_or_else(|_| {
        println!("An error occurred while fetching duel for {}", responder);
        None
    })
}

pub fn get_accepted_duel(responder: &str) -> Option<AcceptedDuel> {
    db_get_accepted_duel(&mut establish_connection(), responder)
}

fn db_set_question_duel(conn: &mut PgConnection, id: i32, question: &str, answer: &str) {
    use crate::schema::duels::dsl::{answer as duel_answer, duels, question as duel_question};

    diesel::update(duels.find(id))
        .set((duel_question.eq(question), duel_answer.eq(answer)))
        .execute(conn)
        .expect("Winner should be a valid twitch id");
}

pub fn set_question_duel(id: i32, question: &str, answer: &str) {
    db_set_question_duel(&mut establish_connection(), id, question, answer);
}

fn db_complete_duel(conn: &mut PgConnection, id: i32, winner: &str, status: &str) {
    use crate::schema::duels::dsl::{duels, status as duel_status, winner as winner_id};

    diesel::update(duels.find(id))
        .set((winner_id.eq(winner), duel_status.eq(status)))
        .execute(conn)
        .expect("Winner should be a valid twitch id");
}

pub fn complete_duel(id: i32, winner: &str, status: &str) {
    db_complete_duel(&mut establish_connection(), id, winner, status);
}

fn db_create_accepted_duel(
    conn: &mut PgConnection,
    duel_id: i32,
    challenger_id: &str,
    challenged_id: &str,
) -> AcceptedDuel {
    use crate::schema::accepted_duels;
    let new_accepted_duel = NewAcceptedDuel {
        duel_id,
        challenger_id,
        challenged_id,
    };

    diesel::insert_into(accepted_duels::table)
        .values(&new_accepted_duel)
        .returning(AcceptedDuel::as_returning())
        .get_result(conn)
        .expect("Error saving new duel")
}

pub fn create_accepted_duel(
    duel_id: i32,
    challenger_id: &str,
    challenged_id: &str,
) -> AcceptedDuel {
    db_create_accepted_duel(
        &mut establish_connection(),
        duel_id,
        challenger_id,
        challenged_id,
    )
}

fn db_destroy_accepted_duel(conn: &mut PgConnection, id: i32) {
    use crate::schema::accepted_duels::dsl::{accepted_duels, duel_id};

    diesel::delete(accepted_duels.filter(duel_id.eq(id)))
        .execute(conn)
        .expect("Duel ID should be i32");
}

pub fn destroy_accepted_duel(id: i32) {
    db_destroy_accepted_duel(&mut establish_connection(), id);
}

fn db_decrement_guesses(conn: &mut PgConnection, id: i32, is_challenger: bool) {
    use crate::schema::duels::dsl::{challenged_guesses, challenger_guesses, duels};
    if is_challenger {
        diesel::update(duels.find(id))
            .set(challenger_guesses.eq(challenger_guesses - 1))
            .execute(conn)
            .expect("Guesses should be i32");
    } else {
        diesel::update(duels.find(id))
            .set(challenged_guesses.eq(challenged_guesses - 1))
            .execute(conn)
            .expect("Guesses should be i32");
    }
}

pub fn decrement_guesses(id: i32, is_challenger: bool) {
    db_decrement_guesses(&mut establish_connection(), id, is_challenger);
}

fn db_get_top_duelists(conn: &mut PgConnection) -> Vec<Chatter> {
    use crate::schema::chatters::dsl::{chatters, wins};
    chatters
        .order(wins.desc())
        .limit(3)
        .load::<Chatter>(conn)
        .expect("Error loading top duelists")
}

pub fn get_top_duelists() -> Vec<Chatter> {
    db_get_top_duelists(&mut establish_connection())
}

fn db_get_ranking(conn: &mut PgConnection, id: &str) -> i64 {
    use crate::schema::chatters::dsl::{chatters, points};
    chatters
        .order_by(points.desc())
        .load::<Chatter>(conn)
        .unwrap()
        .iter()
        .position(|x| x.twitch_id == id)
        .unwrap() as i64
        + 1
}

pub fn get_ranking(id: &str) -> i64 {
    db_get_ranking(&mut establish_connection(), id)
}

fn db_create_lurker(conn: &mut PgConnection, username: &str, twitch_id: &str) {
    use crate::schema::lurkers;
    let new_lurker = NewLurker {
        twitch_id,
        username,
    };

    diesel::insert_into(lurkers::table)
        .values(&new_lurker)
        .returning(Lurker::as_returning())
        .execute(conn)
        .expect("Error saving new lurker");
}

pub fn create_lurker(username: &str, twitch_id: &str) {
    db_create_lurker(&mut establish_connection(), username, twitch_id);
}

fn db_get_lurker(conn: &mut PgConnection, id: String) -> Option<Lurker> {
    use crate::schema::lurkers::dsl::{lurkers, twitch_id as lurker_id};
    let lurker = lurkers
        .filter(lurker_id.eq(&id))
        .select(Lurker::as_select())
        .first(conn)
        .optional();
    lurker.unwrap_or_else(|_| {
        println!("An error occurred while fetching lurker {}", id);
        None
    })
}

pub fn get_lurker(id: String) -> Option<Lurker> {
    db_get_lurker(&mut establish_connection(), id)
}

fn db_get_lurkers(conn: &mut PgConnection) -> Vec<Lurker> {
    use crate::schema::lurkers::dsl::lurkers;
    lurkers
        .order_by(crate::schema::lurkers::dsl::created_at)
        .load::<Lurker>(conn)
        .expect("Error loading lurkers")
}

pub fn get_lurkers() -> Vec<Lurker> {
    db_get_lurkers(&mut establish_connection())
}

fn db_delete_lurker(conn: &mut PgConnection, id: String) {
    use crate::schema::lurkers::dsl::{lurkers, twitch_id as lurker_id};
    diesel::delete(lurkers.filter(lurker_id.eq(id)))
        .execute(conn)
        .expect("Lurker ID should be i32");
}

pub fn delete_lurker(id: String) {
    db_delete_lurker(&mut establish_connection(), id);
}

fn db_get_challenges(conn: &mut PgConnection, id: &str) -> Vec<Duel> {
    use crate::schema::duels::dsl::{challenged_id, duels, status};
    duels
        .filter(challenged_id.eq(id))
        .filter(status.eq("challenged"))
        .load::<Duel>(conn)
        .expect("Error loading challenges")
}

pub fn get_challenges(id: &str) -> Vec<Duel> {
    db_get_challenges(&mut establish_connection(), id)
}

fn db_create_question(
    conn: &mut PgConnection,
    question: &str,
    answer: &str,
    submitter_id: i32,
    category_id: i32,
) -> Question {
    let new_question = NewQuestion {
        question,
        answer,
        submitter_id,
        category_id,
    };

    use crate::schema::questions;
    diesel::insert_into(questions::table)
        .values(&new_question)
        .returning(Question::as_returning())
        .get_result(conn)
        .expect("Error saving new question")
}

pub fn create_question(
    question: &str,
    answer: &str,
    submitter_id: i32,
    category_id: i32,
) -> Question {
    db_create_question(
        &mut establish_connection(),
        question,
        answer,
        submitter_id,
        category_id,
    )
}

fn _db_get_question(conn: &mut PgConnection, id: i32) -> Option<Question> {
    use crate::schema::questions::dsl::questions;
    let question = questions
        .find(id)
        .select(Question::as_select())
        .first::<Question>(conn)
        .optional();
    question.unwrap_or_else(|_| {
        println!("An error occurred while fetching question {}", id);
        None
    })
}

pub fn _get_question(id: i32) -> Option<Question> {
    _db_get_question(&mut establish_connection(), id)
}

fn db_get_questions(conn: &mut PgConnection) -> Vec<Question> {
    use crate::schema::questions::dsl::questions;
    questions
        .load::<Question>(conn)
        .expect("Error loading questions")
}

pub fn get_questions() -> Vec<Question> {
    db_get_questions(&mut establish_connection())
}

fn db_update_times_asked(conn: &mut PgConnection, id: i32, new_times_asked: i32) {
    use crate::schema::questions::dsl::{questions, times_asked};
    diesel::update(questions.find(id))
        .set(times_asked.eq(new_times_asked))
        .execute(conn)
        .expect("Times asked should be i32");
}

pub fn update_times_asked(id: i32, new_times_asked: i32) {
    db_update_times_asked(&mut establish_connection(), id, new_times_asked);
}

fn db_update_times_not_answered(conn: &mut PgConnection, id: i32) {
    use crate::schema::questions::dsl::{questions, times_not_answered};
    diesel::update(questions.find(id))
        .set(times_not_answered.eq(times_not_answered + 1))
        .execute(conn)
        .expect("Times not answered should be i32");
}

pub fn update_times_not_answered(id: i32) {
    db_update_times_not_answered(&mut establish_connection(), id);
}

fn _db_create_category(conn: &mut PgConnection, name: &str, submitter_id: i32) -> Category {
    use crate::schema::categories;
    let new_category = NewCategory { name, submitter_id };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .returning(Category::as_returning())
        .get_result(conn)
        .expect("Error saving new category")
}

pub fn create_category(name: &str, submitter_id: i32) -> Category {
    _db_create_category(&mut establish_connection(), name, submitter_id)
}

fn _db_get_general_category(conn: &mut PgConnection) -> Category {
    use crate::schema::categories::dsl::{categories, name};
    categories
        .filter(name.eq("General"))
        .select(Category::as_select())
        .first::<Category>(conn)
        .expect("Error loading general category")
}

pub fn _get_general_category() -> Category {
    _db_get_general_category(&mut establish_connection())
}

fn db_get_category(conn: &mut PgConnection, id: i32) -> Option<Category> {
    use crate::schema::categories::dsl::categories;
    let category = categories
        .find(id)
        .select(Category::as_select())
        .first::<Category>(conn)
        .optional();
    category.unwrap_or_else(|_| {
        println!("An error occurred while fetching category {}", id);
        None
    })
}

pub fn get_category(id: i32) -> Option<Category> {
    db_get_category(&mut establish_connection(), id)
}

pub fn db_get_category_by_name(name: &str) -> Option<Category> {
    use crate::schema::categories::dsl::{categories, name as category_name};
    let category = categories
        .filter(category_name.eq(name))
        .select(Category::as_select())
        .first::<Category>(&mut establish_connection())
        .optional();
    category.unwrap_or_else(|_| {
        println!("An error occurred while fetching category {}", name);
        None
    })
}

pub fn get_category_by_name(name: &str) -> Option<Category> {
    db_get_category_by_name(name)
}

fn db_get_categories(conn: &mut PgConnection) -> Vec<Category> {
    use crate::schema::categories::dsl::{categories, created_at};
    categories
        .order(created_at)
        .load::<Category>(conn)
        .expect("Error loading categories")
}

pub fn get_categories() -> Vec<Category> {
    db_get_categories(&mut establish_connection())
}

fn db_get_random_question(conn: &mut PgConnection) -> Option<Question> {
    sql_function!(fn random() -> Integer);
    use crate::schema::questions::dsl::questions;
    let question = questions
        .order(random())
        .limit(1)
        .select(Question::as_select())
        .first::<Question>(conn)
        .optional();
    question.unwrap_or_else(|_| {
        println!("An error occurred while fetching random question");
        None
    })
}

pub fn get_random_question() -> Option<Question> {
    db_get_random_question(&mut establish_connection())
}

fn db_get_random_chatter(curr_chatter: &Chatter) -> Chatter {
    sql_function!(fn random() -> Integer);
    use crate::schema::chatters::dsl::{chatters, id as chatter_id, last_seen};
    let conn = &mut establish_connection();
    let chatter_list = chatters
        .filter(chatter_id.ne(curr_chatter.id))
        .filter(last_seen.gt(chrono::Utc::now().naive_utc() - chrono::Duration::minutes(30)))
        .order(random())
        .limit(1)
        .select(Chatter::as_select())
        .first::<Chatter>(conn)
        .expect("Error loading chatters");
    chatter_list
}

pub fn get_random_chatter(curr_chatter: &Chatter) -> Chatter {
    db_get_random_chatter(curr_chatter)
}
