use axum::{routing::post, Router};
use crate::handlers::broadcast_handler;

pub fn routes() -> Router {
    Router::new().route("/broadcast", post(broadcast_handler::broadcast))
}
