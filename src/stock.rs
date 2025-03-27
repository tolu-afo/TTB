// use rand::Rng;

// fn main() {
//     let old_price = 100.0;
//     let volatility = 0.05;

//     let rnd: f64 = rand::thread_rng().gen_range(0.0..1.0);
//     let mut change_percent = 2.0 * volatility * rnd;

//     if change_percent > volatility {
//         change_percent -= 2.0 * volatility;
//     }

//     let change_amount = old_price * change_percent;
//     let new_price = old_price + change_amount;

//     println!("Old price: {}", old_price);
//     println!("New price: {}", new_price);
// }
