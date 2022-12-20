use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PostBody {
    pub text: String,
    pub channel: String,
    pub thread_ts: Option<String>,
}
