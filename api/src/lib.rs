use axum::{
    routing::{get, post},
    Router,
    http::StatusCode
};
use std::net::SocketAddr;

use dotenv::dotenv;

mod infrastructure;
mod use_cases;
mod serializer;

use crate::use_cases::slackbot::slackbot;

#[tokio::main]
async fn start() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    dotenv().ok();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/slackbot", post(slackbot));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> (StatusCode, String) {
    (StatusCode::OK, "Hello world".to_string())
}

pub fn main() {
    let result = start();
}
