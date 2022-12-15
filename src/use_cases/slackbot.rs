use std::env;
use axum::{
    http::StatusCode,
    Json
};
use serde::{Serialize};
use regex::Regex;

use crate::infrastructure::chat;
use crate::serializer::request::Request;
use crate::serializer::post_body::PostBody;


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
                Some(event) => {
                    let bot_user = env::var("BOT_USER").expect("BOT_USER must be set");
                    if event.from_bot(bot_user) {
                        return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None }));
                    }

                    match &event.text {
                        Some(text) => {
                            if match_text(&text) {
                                match (event.channel, event.text, event.thread_ts) {
                                    (Some(channel), Some(text), None) =>{
                                        let caps_text = capture_text(&text);
                                        let post_body = PostBody {
                                            text: String::from("Thank you ") + caps_text,
                                            channel,
                                            thread_ts: None,
                                        };
                                        let res = chat::post_message(post_body).await;
                                    },
                                    (Some(channel), Some(text), Some(thread_ts)) => {
                                        let caps_text = capture_text(&text);
                                        let post_body = PostBody {
                                            text: String::from("Thank you ") + caps_text,
                                            channel,
                                            thread_ts: Some(thread_ts),
                                        };
                                        let res = chat::post_message(post_body).await;
                                    },
                                    (None, None, None) => todo!(),
                                    (None, None, Some(_)) => todo!(),
                                    (None, Some(_), None) => todo!(),
                                    (None, Some(_), Some(_)) => todo!(),
                                    (Some(_), None, None) => todo!(),
                                    (Some(_), None, Some(_)) => todo!(),
                                }
                            }
                        }
                        None => todo!(),
                    }

                    return (StatusCode::OK, Json(ResJson {ok: true, challenge: None }))
                },
                None => return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None })),
            }
        },
        true => {
            let res = ResJson {ok: true, challenge: req.challenge };
            tracing::debug!("{:?}", res);
            (StatusCode::OK, Json(res))
        }
    }
}

fn match_text(text: &str) -> bool {
    let plusplus = Regex::new(r"^.*\+\+.*$").unwrap();
    if plusplus.is_match(text) {
        return true
    } else {
        return false
    }
}

fn capture_text(text: &str) -> &str {
    let re = Regex::new(r"<@.*>").unwrap();
    let caps = re.captures(&text).unwrap();
    return caps.get(0).unwrap().as_str();
}
