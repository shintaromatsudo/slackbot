use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub channel: Option<String>,
    pub user: String,
    pub text: Option<String>,
    pub thread_ts: Option<String>,
}

impl Event {
    pub fn from_bot(&self, bot_user: String) -> bool {
        self.user == bot_user
    }
}
