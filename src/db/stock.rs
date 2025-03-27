use crate::db::establish_connection;
use bigdecimal::BigDecimal;
use diesel::{prelude::*, sql_function};

pub fn create_stock(
    stock_name: &str,
    stock_symbol: &str,
    future_value: BigDecimal,
    stock_price: BigDecimal,
) -> () {
    let connection = &mut establish_connection();
    let new_stock = crate::models::NewStock {
        name: stock_name,
        symbol: stock_symbol,
        ticket_price: stock_price,
        future_value: future_value,
    };

    diesel::insert_into(crate::schema::stocks::table)
        .values(&new_stock)
        .execute(connection)
        .expect("Error saving new stock");
}

pub fn get_stocks() -> Vec<crate::models::Stock> {
    let connection = &mut establish_connection();
    use crate::schema::stocks::dsl::stocks;
    stocks
        .load::<crate::models::Stock>(connection)
        .expect("Error loading stocks")
}
