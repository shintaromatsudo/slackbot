use std::env;
use serde::{Serialize};
use regex::Regex;

use crate::serializer::event::Event;

#[derive(Debug, Serialize)]
pub struct PostBody {
    text: String,
    channel: String,
    thread_ts: Option<String>,
}

pub async fn post_message(event: Event) -> reqwest::Result<()> {
    tracing::info!("post_message, event: {:?}", event);

    let url = "https://slack.com/api/chat.postMessage";

    match (event.channel, event.text, event.thread_ts) {
        (Some(channel), Some(text), None) =>{
            let caps_text = capture_text(&text);
            let post_body = PostBody {
                text: String::from("Thank you ") + caps_text,
                channel,
                thread_ts: None,
            };

            let res = post_request(url, post_body).await;
        },
        (Some(channel), Some(text), Some(thread_ts)) => {
            let caps_text = capture_text(&text);
            let post_body = PostBody {
                text: String::from("Thank you ") + caps_text,
                channel,
                thread_ts: Some(thread_ts),
            };

            let res = post_request(url, post_body).await;
        },
        (None, None, None) => todo!(),
        (None, None, Some(_)) => todo!(),
        (None, Some(_), None) => todo!(),
        (None, Some(_), Some(_)) => todo!(),
        (Some(_), None, None) => todo!(),
        (Some(_), None, Some(_)) => todo!(),
    }

    Ok(())
}

fn capture_text(text: &str) -> &str {
    let re = Regex::new(r"<@.*>").unwrap();
    let caps = re.captures(&text).unwrap();
    return caps.get(0).unwrap().as_str();
}

async fn post_request(url: &str, post_body: PostBody<>) -> reqwest::Result<()> {
    tracing::debug!("post_request, post_body: {:?}", post_body);

    let bot_user_oauth_token = env::var("BOT_USER_OAUTH_TOKEN").expect("BOT_USER_OAUTH_TOKEN must be set");

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(reqwest::header::AUTHORIZATION,format!("Bearer {}", bot_user_oauth_token))
        .json(&post_body)
        .send()
        .await?;

    tracing::debug!("{:?}", response.status());
    tracing::debug!("{:?}", response.text().await?);

    Ok(())
}
