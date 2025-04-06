use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct BroadcastRequest {
    pub channel: String,
    pub event: String,
    pub data: Value,
}
