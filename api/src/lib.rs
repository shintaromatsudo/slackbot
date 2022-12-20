use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::net::SocketAddr;

use dotenv::dotenv;

pub use entity;

mod infrastructure;
mod serializers;
mod use_cases;
mod views;

use crate::views::slackbot;

#[tokio::main]
async fn start() {
    init().await;

    let state = connect_db().await;

    let app = Router::new()
        .route("/", get(health_check))
        .route("/slackbot", post(slackbot::post))
        .with_state(state);

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

async fn init() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok();
}

async fn connect_db() -> AppState {
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(db_uri)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    return state;
}

async fn health_check() -> (StatusCode, String) {
    (StatusCode::OK, "Hello world".to_string())
}

pub fn main() {
    let result = start();
}
