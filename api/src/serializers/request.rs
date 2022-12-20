use crate::serializers::event::Event;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub token: String,
    pub challenge: Option<String>,
    pub event: Option<Event>,
}

impl Request {
    pub fn is_initialize(&self) -> bool {
        self.challenge.is_some()
    }
}
