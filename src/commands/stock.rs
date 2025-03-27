use bigdecimal::BigDecimal;

use crate::db::stock;
use crate::messaging::{list_with_title, reply_to, ItemSeparator};

pub async fn handle_liststocks_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let stocks = dbg!(stock::get_stocks());

    let stock_msgs = stocks
        .into_iter()
        .map(|s| format!("{}: ${}", s.symbol, s.ticket_price).clone())
        .collect::<Vec<String>>();

    reply_to(
        client,
        &msg,
        &list_with_title("Current Stocks:", &stock_msgs, ItemSeparator::GoldStar),
    )
    .await
}

pub async fn handle_setstockprice_command(
    client: &mut tmi::Client,
    msg: &tmi::Privmsg<'_>,
) -> anyhow::Result<(), anyhow::Error> {
    let mut cmd_iter = msg.text().split(' ');
    cmd_iter.next(); // skipping command
    let stock = match cmd_iter.next() {
        Some(stock) => match stock::get_stock_by_symbol(&stock.to_uppercase()) {
            Some(stock) => stock,
            None => return reply_to(client, &msg, "That stock does not exist!").await,
        },
        None => return reply_to(client, &msg, "That stock does not exist!").await,
    };

    let point_value = match cmd_iter.next() {
        Some(w) => w,
        None => {
            return reply_to(
                client,
                msg,
                "You need to provide a point value! Format: '!setstockprice <symbol> <points>'",
            )
            .await;
        }
    };

    let points = match point_value.parse::<BigDecimal>() {
        Ok(w) => w,
        Err(_) => {
            return reply_to(
                client,
                msg,
                "Invalid point value! Format: '!settockprice <symbol> <points>'",
            )
            .await;
        }
    };
    stock::update_stock_price(&stock.symbol.to_uppercase(), points);
    return reply_to(client, msg, "New price set!").await;
}
