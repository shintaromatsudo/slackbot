use std::env;

use crate::serializers::post_body::PostBody;

pub async fn post_message(post_body: PostBody) -> reqwest::Result<()> {
    tracing::info!("third_slack_chat__post_message");

    let url = "https://slack.com/api/chat.postMessage";

    let res = post_request(url, post_body).await;

    Ok(())
}

async fn post_request(url: &str, post_body: PostBody) -> reqwest::Result<()> {
    tracing::info!("third_slack_chat__post_request");

    let bot_user_oauth_token =
        env::var("BOT_USER_OAUTH_TOKEN").expect("BOT_USER_OAUTH_TOKEN must be set");

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", bot_user_oauth_token),
        )
        .json(&post_body)
        .send()
        .await?;

    Ok(())
}
