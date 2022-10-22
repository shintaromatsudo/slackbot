use axum::{
    routing::{get, post},
    Router,
    http::StatusCode
};
use std::net::SocketAddr;

use dotenv::dotenv;

mod slackbot;
use crate::slackbot::slackbot;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    dotenv().ok();

    let app = Router::new()
        .route("/", get(root))
        .route("/slackbot", post(slackbot));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> (StatusCode, String) {
    (StatusCode::OK, "Hello world".to_string())
}
