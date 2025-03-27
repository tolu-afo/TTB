use crate::models::{NewOrder, Order, Stock};
use crate::{db::establish_connection, schema::orders::num_shares, schema::stocks::symbol};
use bigdecimal::BigDecimal;
use diesel::{prelude::*, sql_function};

pub fn create_stock(
    stock_name: &str,
    stock_symbol: &str,
    future_value: BigDecimal,
    stock_price: BigDecimal,
) -> () {
    let conn = &mut establish_connection();
    let new_stock = crate::models::NewStock {
        name: stock_name,
        symbol: stock_symbol,
        ticket_price: stock_price,
        future_value: future_value,
    };

    diesel::insert_into(crate::schema::stocks::table)
        .values(&new_stock)
        .execute(conn)
        .expect("Error saving new stock");
}

pub fn get_stocks() -> Vec<Stock> {
    let conn = &mut establish_connection();
    use crate::schema::stocks::dsl::stocks;
    stocks
        .order(symbol.asc())
        .load::<Stock>(conn)
        .expect("Error loading stocks")
}

pub fn get_stock_by_symbol(sym: &str) -> Option<Stock> {
    let conn = &mut establish_connection();
    use crate::schema::stocks::dsl::{stocks, symbol};
    let stock = stocks
        .filter(symbol.eq(dbg!(sym)))
        .select(Stock::as_select())
        .first(conn)
        .optional();

    stock.unwrap_or_else(|_| {
        println!(
            "Sonmething bad happened when fetching stock with symbol {}",
            sym
        );
        None
    })
}

pub fn update_stock_price(sym: &str, price: BigDecimal) {
    let conn = &mut establish_connection();
    use crate::schema::stocks::dsl::{stocks, symbol, ticket_price};

    let _ = diesel::update(stocks)
        .filter(symbol.eq(dbg!(sym)))
        .set(ticket_price.eq(dbg!(price)))
        .returning(Stock::as_returning())
        .execute(conn);
}

pub fn assign_share(stock_id: i32, chatter_id: i32, strike_price: BigDecimal, quantity: i32) {
    let conn = &mut establish_connection();
    // use crate::schema::shares::dsl::{stock_id, owner_id, shares, strike_price, quantity};

    let new_order = NewOrder {
        stock_id: stock_id,
        owner_id: chatter_id,
        strike_price: strike_price,
        num_shares: quantity,
    };

    let _ = diesel::insert_into(crate::schema::orders::table)
        .values(&new_order)
        .execute(conn)
        .expect("Error Saving Share Order");
}

pub fn get_shares(chatter_id: i32) -> Vec<Order> {
    let conn = &mut establish_connection();
    use crate::schema::orders::dsl::{orders, owner_id};

    orders
        .filter(owner_id.eq(chatter_id))
        .load::<Order>(conn)
        .expect("Some error occured while getting orders from db")
}
