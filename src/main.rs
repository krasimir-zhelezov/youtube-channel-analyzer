use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .unwrap();

    println!("API_KEY: {}", api_key);
}
