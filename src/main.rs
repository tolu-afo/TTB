use std::hash::Hash;
use std::string;
use std::num::NonZeroU32;
use std::collections::HashMap;

use anyhow::Result;
use clap::error;
// use crate::state;
use tokio::select;
use tokio::signal::ctrl_c;
use dotenv::dotenv;

// use tokio::task::futures;
use twitch_api2::helix;
use futures::TryStreamExt;

use twitch_api2::{helix::channels::GetChannelInformationRequest, TwitchClient};
use twitch_api2::twitch_oauth2::{tokens::errors::AppAccessTokenError, AppAccessToken, Scope, TwitchToken};

use chrono::{Local, DateTime};

mod duel;
mod state;

// async fn get_chatter(client:&TwitchClient<'_, reqwest::Client>, token:&AppAccessToken){
//   let chatters: Vec<helix::chat::Chatter> = client.get_chatters("1234", "4321", 1000, &token).try_collect().await;

//   chatters
// }

#[derive(Debug)]
pub struct State {
    duel_cache: HashMap::<String, duel::duel::Duel> 
}

impl State {
    pub fn new() -> State {
        let cache:HashMap::<String, duel::duel::Duel> = HashMap::new();

        return State {
            duel_cache: cache,
        }
    }

    pub fn save_duel(&mut self, duel: &duel::duel::Duel) -> () {
        // saves duel to cache
        self.duel_cache.insert(format!("{}{}", duel.challenger.to_string(), duel.challenged.to_string()), duel.clone());
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  dotenv().ok();
  let broadcaster_id = std::env::var("TOLUAFO_BROADCASTER_ID").expect("TOLUAFO_BROADCASTER_ID must be set");
  let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set.").into();
  let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set.").into();

  let token = std::env::var("BONGO_OAUTH_TOKEN").expect("BONGO_OAUTH_TOKEN must be set.");
  let oauth = std::fmt::format(format_args!("oauth:{}", token));
  let user: String = std::env::var("BONGO_USER").expect("BONGO_USER must be set.");


  let twitch_client : TwitchClient<reqwest::Client> = TwitchClient::default();
  let twitch_token = AppAccessToken::get_app_access_token(&twitch_client, client_id, client_secret, Scope::all())
        .await?;

  let req = GetChannelInformationRequest::builder()
    .broadcaster_id(broadcaster_id)
    .build();
  
  println!(
      "{:?}",
      &twitch_client.helix.req_get(req, &twitch_token).await?.data.unwrap().title
  );

  tracing_subscriber::fmt::init();
  
  let credentials = tmi::client::Credentials::new(user, oauth);
  let channels = vec!["#ToluAfo".to_string()].into_iter().map(tmi::Channel::parse).collect::<Result<Vec<_>, _>>()?;

  println!("Connecting as {}", credentials.nick);
  let mut client = tmi::Client::builder()
    .credentials(credentials)
    .connect()
    .await?;

  client.join_all(&channels).await?;
  println!("Joined the following channels: {}", channels.join(", "));

  select! {
    _ = ctrl_c() => {
      Ok(())
    }
    res = tokio::spawn(run(client, channels)) => {
      res?
    }
  }
}

async fn run(mut client: tmi::Client, channels: Vec<tmi::Channel>) -> Result<()> {
  let mut bot_state = State::new(); 

  loop {
    let msg = client.recv().await?;
    match msg.as_typed()? {
      tmi::Message::Privmsg(msg) => on_msg(&mut client, dbg!(msg), dbg!(&mut bot_state)).await?,
      tmi::Message::Reconnect => {
        client.reconnect().await?;
        client.join_all(&channels).await?;
      }
      tmi::Message::Ping(ping) => client.pong(&ping).await?,
      _ => {}
    };
  }
}

async fn send_duel_err(challenger: &str, client: &mut tmi::Client, msg: tmi::Privmsg<'_>, err: &str) -> Result<()>{
  send_msg(client, &msg, &format!("@{} Error; {}", challenger, err )).await
}

async fn send_msg(client: &mut tmi::Client, msg: &tmi::Privmsg<'_>, text: &str) -> Result<(), anyhow::Error> {
    client
      .privmsg(msg.channel(), text)
      .send()
      .await?;
    Ok(())
}

async fn on_msg(client: &mut tmi::Client, msg: tmi::Privmsg<'_>, bot_state: &mut State) -> Result<()> {
  println!("{}: {}", msg.sender().name(), msg.text());

  match msg.text().split_ascii_whitespace().next() {
    Some("!commands") => handle_commands_command(client, &msg).await,
    Some("!yo") => handle_yo_command(client, &msg).await,
    Some("!accept") => handle_accept_command(client, msg, bot_state).await,
    Some("!duel") => handle_duel_command(client, msg, bot_state).await,
    _ => Ok(())
  }
}

async fn handle_yo_command(client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> Result<(), anyhow::Error> {
    reply_to(client, msg, "yo").await
}

async fn reply_to(client: &mut tmi::Client, msg: &tmi::Privmsg<'_>, reply:&str) -> Result<(), anyhow::Error> {
    client
      .privmsg(msg.channel(), reply)
      .reply_to(msg.message_id())
      .send()
      .await?;
    Ok(())
}

async fn handle_commands_command(client: &mut tmi::Client, msg: &tmi::Privmsg<'_>) -> Result<(), anyhow::Error> {
    reply_to(client, msg, "!yo !duel !accept").await
}

async fn handle_accept_command(client: &mut tmi::Client, msg: tmi::Privmsg<'_>,  bot_state: &mut State) -> Result<(), anyhow::Error> {
  // check that username of msg matches a challenged in a duel
  // !accept @<user>
  let mut cmd_iter = msg.text().split(' ');
  cmd_iter.next();
  let challenged = msg.sender().name();
  let challenger = match cmd_iter.next() {
      Some(chal) => {
        match chal.chars().nth(0) {
          Some('@') => &chal[1..],
          _ => chal
        }
      },
      None => {
        return send_duel_err(&challenged, client, msg, "You need to provide a username in the format @<user> or <user>").await;
      }
  };
  let key = format!("{}{}", challenger, challenged);
  let duel = match bot_state.duel_cache.get_mut(&key){
    Some(d) => {
      d.accept_duel();
      d
    },
    None => {
      return send_duel_err(&challenged, client, msg, "Wrong opponent!").await;
    }
  }; 

  // duel
  send_msg(client, &msg, &format!("@{} @{} the duel has been accepted! Prepare to battle in 3 seconds", challenger, challenged)).await;

  return Ok(())
}

// TODO: move to module
async fn handle_duel_command(client: &mut tmi::Client, msg: tmi::Privmsg<'_>, bot_state: &mut State) -> Result<(), anyhow::Error> {
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next();
    let challenger = dbg!(msg.sender().name());
    let challenged = match dbg!(cmd_iter.next()) {
      Some(chal) => {
        // filter @ symbol
        match chal.chars().nth(0) {
          Some('@') => &chal[1..],
          _ => chal
        }
      },
      None => {
        return send_duel_err(&challenger, client, msg, "You need to provide a username in the format @<user> or <user>").await;
      }
    };
    let points = match cmd_iter.next() {
      Some(chal) => chal,
      None => {
        return send_duel_err(&challenger, client, msg, "You need to provide a point value.").await;
      }
    };
    let points = match points.parse() {
      Ok(p) => p,
      Err(_) => {
        return send_duel_err(&challenger, client, msg, "Provide a valid point value.").await;
      }
    };

    if cmd_iter.next().is_some() {
      return send_duel_err(&challenger, client, msg, "Too many arguments!").await;
    }

    let curr_duel = duel::duel::Duel::new(
    &challenger, 
    &challenged, 
    points);

    bot_state.save_duel(&curr_duel);
    dbg!(curr_duel);

    reply_to(client, &msg, "its time to d-d-d-d-d-duel!!!").await
}