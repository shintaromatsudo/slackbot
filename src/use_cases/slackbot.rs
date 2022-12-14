use std::env;
use axum::{
    http::StatusCode,
    Json
};
use serde::{Serialize};

use crate::infrastructure::chat;
use crate::serializer::request::Request;



#[derive(Debug, Serialize)]
pub struct ResJson {
    ok: bool,
    challenge: Option<String>,
}

pub async fn slackbot(Json(req): Json<Request>) -> (StatusCode, Json<ResJson>) {
    tracing::info!("slackbot: {:?}", req);

    let verification_token = env::var("VERIFICATION_TOKEN").expect("VERIFICATION_TOKEN must be set");
    if req.token != verification_token {
        tracing::warn!("AuthenticationFailed, token: {}", req.token);
        return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None }));
    }

    match req.is_initialize() {
        false => {
            match req.event {
                Some(e) => {
                    let bot_user = env::var("BOT_USER").expect("BOT_USER must be set");
                    if e.from_bot(bot_user) {
                        return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None }));
                    }

                    let res = chat::post_message(e).await;
                    return (StatusCode::OK, Json(ResJson {ok: true, challenge: None }))
                },
                None => return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None })),
            };
        },
        true => {
            let res = ResJson {ok: true, challenge: req.challenge };
            tracing::debug!("{:?}", res);
            (StatusCode::OK, Json(res))
        }
    }
}
