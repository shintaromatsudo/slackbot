use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::env;

use crate::serializers::request::Request;
use crate::use_cases::slackbot;

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ResJson {
    ok: bool,
    challenge: Option<String>,
}

pub async fn post(state: State<AppState>, Json(req): Json<Request>) -> (StatusCode, Json<ResJson>) {
    tracing::info!("slackbot__post");

    // Validate
    let verification_token =
        env::var("VERIFICATION_TOKEN").expect("VERIFICATION_TOKEN must be set");
    if req.token != verification_token {
        tracing::warn!("AuthenticationFailed, token: {}", req.token);
        let res = Json(ResJson {
            ok: false,
            challenge: None,
        });
        return (StatusCode::BAD_REQUEST, res);
    }

    match &req.event {
        Some(event) => {
            let bot_user = env::var("BOT_USER").expect("BOT_USER must be set");
            if event.from_bot(bot_user) {
                tracing::debug!("From happy bot");
                let res = Json(ResJson {
                    ok: false,
                    challenge: None,
                });
                return (StatusCode::BAD_REQUEST, res);
            }
        }
        None => {
            let res = Json(ResJson {
                ok: false,
                challenge: None,
            });
            return (StatusCode::BAD_REQUEST, res);
        }
    }

    // Execute
    if req.is_initialize() {
        let res = Json(ResJson {
            ok: true,
            challenge: req.challenge,
        });
        return (StatusCode::OK, res);
    }

    slackbot::execute(state, req.event.unwrap());

    let res = Json(ResJson {
        ok: true,
        challenge: None,
    });
    return (StatusCode::OK, res);
}
