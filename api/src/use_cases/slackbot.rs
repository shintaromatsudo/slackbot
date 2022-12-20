use axum::extract::State;
use regex::Regex;

use crate::entity::plusplus::Model as PlusplusModel;
use crate::infrastructure::plusplus;
use crate::infrastructure::third_slack_chat;
use crate::serializers::event::Event;
use crate::serializers::post_body::PostBody;

use crate::AppState;

pub async fn execute(state: State<AppState>, event: Event) {
    tracing::info!("slackbot__execute");

    if let Some(text) = &event.text {
        if match_text(&text) {
            let slack_id = capture_text(&text);
            match (event.channel, event.thread_ts) {
                (Some(channel), None) => {
                    let plus_model = create_or_update(state, &slack_id).await;
                    post(channel, slack_id, plus_model.counter, None).await;
                }
                (Some(channel), Some(thread_ts)) => {
                    let plus_model = create_or_update(state, &slack_id).await;
                    post(channel, slack_id, plus_model.counter, Some(thread_ts)).await;
                }
                (None, None) => todo!(),
                (None, Some(_)) => todo!(),
            }
        }
    }
}

async fn create_or_update(state: State<AppState>, slack_id: &str) -> PlusplusModel {
    tracing::info!("slackbot__create_or_update");
    let some_plus = plusplus::get(&state, slack_id).await;

    let plus_model;

    match some_plus {
        None => {
            plus_model = plusplus::create(state, slack_id).await;
        }
        Some(plusplus) => {
            plus_model = plusplus::update(state, plusplus).await;
        }
    }

    plus_model
}

async fn post(channel: String, slack_id: &str, counter: i32, thread_ts: Option<String>) {
    tracing::info!("slackbot__post");
    let post_body = PostBody {
        text: String::from("Thank you ") + slack_id + " (counter:" + &counter.to_string() + ")",
        channel,
        thread_ts,
    };
    let res = third_slack_chat::post_message(post_body).await;
}

fn match_text(text: &str) -> bool {
    let re = Regex::new(r"^.*\+\+.*$").unwrap();
    if re.is_match(text) {
        return true;
    } else {
        return false;
    }
}

fn capture_text(text: &str) -> &str {
    let re = Regex::new(r"<@.*>").unwrap();
    let caps = re.captures(&text).unwrap();
    return caps.get(0).unwrap().as_str();
}
