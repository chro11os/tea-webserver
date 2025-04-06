use crate::models::broadcast::BroadcastRequest;
use std::collections::HashMap;
use std::env;
use reqwest::Client;

pub async fn send_to_pusher(payload: &BroadcastRequest) -> Result<(), String> {
    let key = env::var("PUSHER_KEY").map_err(|_| "Missing PUSHER_KEY")?;
    let secret = env::var("PUSHER_SECRET").map_err(|_| "Missing PUSHER_SECRET")?;
    let app_id = env::var("PUSHER_APP_ID").map_err(|_| "Missing PUSHER_APP_ID")?;
    let url = env::var("PUSHER_URL").unwrap_or("https://api.pusherapp.com".into());

    let endpoint = format!("{}/apps/{}/events", url.trim_end_matches('/'), app_id);

    let mut body = HashMap::new();
    body.insert("name", payload.event.clone());
    body.insert("channel", payload.channel.clone());
    body.insert("data", payload.data.to_string());

    let client = Client::new();
    let res = client
        .post(&endpoint)
        .basic_auth(key, Some(secret))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Pusher responded with {}", res.status()))
    }
}
