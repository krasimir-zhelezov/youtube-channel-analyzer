use dotenv::dotenv;
use std::{env, error::Error};
use reqwest;
use serde_json::Value;


#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .unwrap();

    println!("API_KEY: {}", api_key);

    let username = "kenforrest".to_string();
    let channel_id = get_channel_id(username, &api_key).await?;

    println!("Channel ID: {}", channel_id);

    Ok(())
}

async fn get_channel_id(username: String, api_key: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=channel&maxResults=1&key={}", username, api_key))
        .send()
        .await?;

    let json: Value = response.json().await?;

    // Properly extract the channel ID from YouTube's search response
    let channel_id = json["items"][0]["id"]["channelId"]
        .as_str()
        .ok_or("Channel ID not found in response")?
        .to_string();

    Ok(channel_id)
}
