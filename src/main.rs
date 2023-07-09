mod io;
mod logic;
mod model;

use logic::get_cheapest_hotels_for_input;

#[tokio::main]
async fn main() {
    let results = get_cheapest_hotels_for_input().await;
    println!("{:?}", results);
}