use anyhow::Result;
use dotenv::dotenv;
use tmi::client::ConnectError;
use tmi::Client;
use tokio::select;
use tokio::signal::ctrl_c;
use twitch_api2::{helix::channels::GetChannelInformationRequest, TwitchClient};

use state::State;

mod chatter;
mod commands;
mod db;
mod duel;
mod messaging;
mod models;
mod schema;
mod state;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv().ok();
    // TODO: Add a database for chatters, duels, & bot state
    let broadcaster_id = std::env::var("BROADCASTER_ID").expect("BROADCASTER_ID must be set");
    let client_secret = std::env::var("TWITCH_CLIENT_SECRET")
        .expect("TWITCH_CLIENT_SECRET must be set.")
        .into();
    let client_id = std::env::var("TWITCH_CLIENT_ID")
        .expect("TWITCH_CLIENT_ID must be set.")
        .into();

    let token = std::env::var("BOT_OAUTH_TOKEN").expect("BOT_OAUTH_TOKEN must be set.");
    let oauth = std::fmt::format(format_args!("oauth:{}", token));
    let user: String = std::env::var("BOT_USERNAME").expect("BOT_USERNAME must be set.");

    let mut client =
        match get_client(broadcaster_id, client_secret, client_id, token, oauth, user).await {
            Ok(c) => c,
            Err(err) => panic!("Connection was not successful!"),
        };

    let channels = vec!["#ToluAfo".to_string()]
        .into_iter()
        .map(tmi::Channel::parse)
        .collect::<Result<Vec<_>, _>>()?;

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

async fn get_client(
    broadcaster_id: String,
    client_secret: String,
    client_id: String,
    token: String,
    oauth: String,
    user: String,
) -> Result<Client, ConnectError> {
    let twitch_client: TwitchClient<reqwest::Client> = TwitchClient::default();

    let req = GetChannelInformationRequest::builder()
        .broadcaster_id(broadcaster_id)
        .build();

    let credentials = tmi::client::Credentials::new(user, oauth);

    println!("Connecting as {}", credentials.nick);
    let mut client = tmi::Client::builder()
        .credentials(credentials)
        .connect()
        .await?;
    Ok(client)
}

async fn run(mut client: tmi::Client, channels: Vec<tmi::Channel>) -> Result<()> {
    let mut bot_state = State::new();

    loop {
        let msg = client.recv().await?;
        match msg.as_typed()? {
            tmi::Message::Privmsg(msg) => {
                messaging::on_msg(&mut client, msg, &mut bot_state).await?
            }
            tmi::Message::Reconnect => {
                client.reconnect().await?;
                client.join_all(&channels).await?;
            }
            tmi::Message::Ping(ping) => client.pong(&ping).await?,
            _ => {}
        };
    }
}
