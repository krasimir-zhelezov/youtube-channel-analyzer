use dotenv::dotenv;
use std::{collections::HashMap, env, error::Error};
use reqwest;
use serde_json::Value;

pub struct Video {
    pub url: String,
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub view_count: String,
    pub tags: Option<Vec<String>>,
}

#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .unwrap();

    println!("API_KEY: {}", api_key);

    // let username = "kenforrest".to_string();
    // let channel_id = get_channel_id(&username, &api_key).await?;
    // println!("Channel ID: {}", channel_id);

    let video = get_video_by_id("Ff-f2tximdI", &api_key).await?;
    println!("Video: {}", video.title);

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

async fn get_video_by_id(video_id: &str, api_key: &str) -> Result<Video, Box<dyn Error>> {
    let url = "https://www.googleapis.com/youtube/v3/videos";

    let mut params = HashMap::new();
    params.insert("part", "snippet,statistics");
    params.insert("id", video_id);
    params.insert("key", api_key);

    let data = make_request(&url, &params).await?;

    let url = format!("https://youtu.be/{}", video_id);

    let title = data["items"][0]["snippet"]["title"]
        .as_str()
        .ok_or(format!("Title not found for {}", video_id))?
        .to_string();

    let description = data["items"][0]["snippet"]["description"]
        .as_str()
        .ok_or(format!("Description not found for {}", video_id))?
        .to_string();

    let published_at = data["items"][0]["snippet"]["publishedAt"]
        .as_str()
        .ok_or(format!("Published at not found for {}", video_id))?
        .to_string();

    let view_count = data["items"][0]["statistics"]["viewCount"]
        .as_str()
        .ok_or(format!("Published at not found for {}", video_id))?
        .to_string();
    
    let empty_vec = Vec::new();
    let tags = data["items"][0]["snippet"]["tags"]
        .as_array()
        .unwrap_or_else(|| {
            eprintln!("Tags not found for {}", video_id);
            &empty_vec
        })
        .iter()
        .map(|v| v.as_str().unwrap_or("").to_string())
        .collect::<Vec<String>>();

    Ok(Video {
        url,
        title,
        description,
        published_at,
        view_count,
        tags: Some(tags)
    })
}