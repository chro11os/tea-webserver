use axum::{Json, response::IntoResponse};
use crate::models::broadcast::BroadcastRequest;
use crate::utils::pusher::send_to_pusher;
use tracing::info;

pub async fn broadcast(Json(payload): Json<BroadcastRequest>) -> impl IntoResponse {
    info!("🔔 Incoming event: {} -> {}", payload.channel, payload.event);

    if payload.channel.is_empty() || payload.event.is_empty() {
        return "❌ Invalid input.".to_string();
    }

    match send_to_pusher(&payload).await {
        Ok(_) => "✅ Broadcast sent.".into(),
        Err(e) => format!("❌ Error: {}", e),
    }
}
