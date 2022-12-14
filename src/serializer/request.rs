use serde::{Deserialize};
use crate::serializer::event::Event;

#[derive(Debug, Deserialize)]
pub struct Request {
    r#type: Option<String>,
    pub token: String,
    pub challenge: Option<String>,
    team_id: Option<String>,
    api_app_id: Option<String>,
    pub event: Option<Event>,
    authed_teams: Option<Vec<String>>,
    event_id: Option<String>,
    event_time: Option<i32>,
}

impl Request {
    pub fn is_initialize(&self) -> bool {
        self.challenge.is_some()
    }
}
