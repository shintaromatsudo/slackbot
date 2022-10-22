use std::env;
use axum::{
    http::StatusCode,
    Json
};
use serde::{Deserialize, Serialize};
use serde_json::{json};

#[derive(Debug, Deserialize)]
pub struct ReqJson {
    r#type: Option<String>,
    token: String,
    challenge: Option<String>,
    team_id: Option<String>,
    api_app_id: Option<String>,
    event: Option<Event>,
    authed_teams: Option<Vec<String>>,
    event_id: Option<String>,
    event_time: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Event {
    r#type: Option<String>,
    channel: Option<String>,
    user: Option<String>,
    text: Option<String>,
    ts: Option<String>,
    event_ts: Option<String>,
    channel_type: Option<String>,
    thread_ts: Option<String>,
}

impl ReqJson {
    fn is_initialize(&self) -> bool {
        self.challenge.is_some()
    }
}

#[derive(Debug, Serialize)]
pub struct ResJson {
    ok: bool,
    challenge: Option<String>,
}

pub async fn slackbot(Json(req): Json<ReqJson>) -> (StatusCode, Json<ResJson>) {
    tracing::info!("slackbot: {:?}", req);

    let verification_token = env::var("VERIFICATION_TOKEN").expect("VERIFICATION_TOKEN must be set");
    let bot_user = env::var("BOT_USER").expect("BOT_USER must be set");

    if req.token != verification_token {
        tracing::warn!("AuthenticationFailed, token: {}", req.token);
        return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None }));
    }
    // TODO if req.event.user == bot_user

    match req.is_initialize() {
        false => {
            match req.event {
                Some(e) => {
                    let res = post_message(e).await;
                    return (StatusCode::OK, Json(ResJson {ok: true, challenge: None }))
                },
                None => return (StatusCode::BAD_REQUEST, Json(ResJson {ok: false, challenge: None })),
            };
        },
        true => {
            let res = ResJson {ok: false, challenge: req.challenge };
            tracing::debug!("{:?}", res);
            (StatusCode::OK, Json(res))
        }
    }
}

async fn post_message(event: Event) -> reqwest::Result<()> {
    tracing::info!("post_message, event: {:?}", event);
    let bot_user_oauth_token = env::var("BOT_USER_OAUTH_TOKEN").expect("BOT_USER_OAUTH_TOKEN must be set");

    let mut post_body = json!({
        "text": format!("happy {:?}", event.text.unwrap_or("".to_string())),
        "channel": event.channel.unwrap_or("".to_string()),
    });
    if let Some(thread_ts) = Some(event.thread_ts) {
        post_body.as_object_mut().unwrap().insert("thread_ts".to_string(), serde_json::to_value(thread_ts).unwrap());
    }
    tracing::debug!("post_message, post_body: {:?}", post_body);
    let client = reqwest::Client::new();
    let res = client.post("https://slack.com/api/chat.postMessage")
        .header(reqwest::header::AUTHORIZATION,format!("Bearer {}", bot_user_oauth_token))
        .json(&post_body)
        .send()
        .await?;

    tracing::debug!("{:?}", res.text().await?);

    Ok(())
}
