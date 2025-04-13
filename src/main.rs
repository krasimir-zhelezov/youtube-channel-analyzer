use dotenv::dotenv;
use std::{collections::HashMap, env, error::Error};
use reqwest;
use serde_json::Value;


#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .unwrap();

    println!("API_KEY: {}", api_key);

    let username = "kenforrest".to_string();
    let channel_id = get_channel_id(&username, &api_key).await?;

    println!("Channel ID: {}", channel_id);

    Ok(())
}

async fn make_request(url: &str, params: &HashMap<&str, &str>) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .query(&params)
        .send()
        .await?;

    let json: Value = response.json().await?;

    Ok(json)
}

async fn get_channel_id(username: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
    let url = "https://www.googleapis.com/youtube/v3/search";
    
    let mut params = HashMap::new();
    params.insert("part", "snippet");
    params.insert("q", &username);
    params.insert("type", "channel");
    params.insert("maxResults", "1");
    params.insert("key", api_key);
    
    let data = make_request(&url, &params).await?;

    let channel_id = data["items"][0]["id"]["channelId"]
        .as_str()
        .ok_or("Channel ID not found in response")?
        .to_string();

    Ok(channel_id)
}