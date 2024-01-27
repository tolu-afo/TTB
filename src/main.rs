use std::string;

use anyhow::Result;
use clap::error;
use tokio::select;
use tokio::signal::ctrl_c;
use dotenv::dotenv;

use chrono::{Local, DateTime};

mod duel;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  dotenv().ok();
  let token = std::env::var("BONGO_OAUTH_TOKEN").expect("BONGO_OAUTH_TOKEN must be set.");
  let oauth = std::fmt::format(format_args!("oauth:{}", token));
  let user: String = std::env::var("BONGO_USER").expect("BONGO_USER must be set.");

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
  loop {
    let msg = client.recv().await?;
    match msg.as_typed()? {
      tmi::Message::Privmsg(msg) => on_msg(&mut client, dbg!(msg)).await?,
      tmi::Message::Reconnect => {
        client.reconnect().await?;
        client.join_all(&channels).await?;
      }
      tmi::Message::Ping(ping) => client.pong(&ping).await?,
      _ => {}
    };
  }
}


async fn send_duel_use_help(challenger: &str, client: &mut tmi::Client, msg: tmi::Privmsg<'_>) -> Result<()>{
  client
  .privmsg(msg.channel(), 
  &format!(
    "@{} Error; Correct format is: !duel <challenged> <channel_point_wager>", challenger ))
  .send()
  .await?;
  return Ok(());
}

async fn on_msg(client: &mut tmi::Client, msg: tmi::Privmsg<'_>) -> Result<()> {
  println!("{}: {}", msg.sender().name(), msg.text());

  if client.credentials().is_anon() {
    return Ok(());
  }

  if msg.text().starts_with("!commands") {
    client
      .privmsg(msg.channel(), "!yo !duel")
      .reply_to(msg.message_id())
      .send()
      .await?;  
    return Ok(());
  }

  if msg.text().starts_with("!yo") {
    client
      .privmsg(msg.channel(), "yo")
      .reply_to(msg.message_id())
      .send()
      .await?;  
    return Ok(());
  }

  if msg.text().starts_with("!duel") {
    // "!duel <challenged> <channel_point_wager>"
    // challenge_datetime: datetime, challenger: user, challenged: user, points: i32, winner: user, accepted: bool}
    let mut cmd_iter = msg.text().split(' ');
    // make sure its split into the two strings
    // grab the challenger and point wager
    cmd_iter.next();

    let challenger = dbg!(msg.sender().name());

    let challenged = match cmd_iter.next() {
      Some(chal) => chal,
      None => {
        return send_duel_use_help(&challenger, client, msg).await;
      }
    };

    let points = match cmd_iter.next() {
      Some(chal) => chal,
      None => {
        return send_duel_use_help(&challenger, client, msg).await;
      }
    };

    let points = match points.parse() {
      Ok(p) => p,
      Err(_) => {
        return send_duel_use_help(&challenger, client, msg).await;
      }
    };

    if cmd_iter.next().is_some() {
      return send_duel_use_help(&challenger, client, msg).await;
    }
    
    let mut curr_duel = duel::duel::Duel::new(
    &challenger, 
    challenged, 
    points);
    dbg!(curr_duel);

    client
      .privmsg(msg.channel(), "its time to d-d-d-d-d-duel!!!")
      .reply_to(msg.message_id())
      .send()
      .await?;
    return Ok(());
  }

  Ok(())
}