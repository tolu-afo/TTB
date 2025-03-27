use std::collections::HashMap;

use crate::models::Stock;
use bigdecimal::BigDecimal;

pub fn calculate_strike_price(stock: &Stock) -> BigDecimal {
    let future_value = &stock.future_value;
    let roi_percentage = &stock.roi_percentage;

    future_value - (future_value * roi_percentage)
}

// pub fn calculate_portfolio(orders: Vec<Order>) -> HashMap<str, i32> {}
