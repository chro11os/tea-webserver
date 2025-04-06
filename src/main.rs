mod routes;
mod utils;
mod handlers;
mod models;

use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new().merge(routes::broadcast::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
