use crate::api::api_context::APIContext;
use std::fmt::Debug;
use std::sync::Arc;
use vkwave_token::token::DefaultToken;

pub type Event = serde_json::Value; // TODO: replace it please with something meaningful

pub type ArcEvent = Arc<Event>;

#[derive(Debug, Clone)]
pub struct RawContext(pub ArcEvent, pub Arc<APIContext<DefaultToken>>);

pub trait FromRawContext {
    fn from_raw_context(raw_context: RawContext) -> Option<Self>
    where
        Self: Sized;
}

impl FromRawContext for RawContext {
    fn from_raw_context(raw_context: RawContext) -> Option<Self> {
        Some(raw_context)
    }
}

// only for `message_new` events
pub fn get_text_from_event(ev: &RawContext) -> &str {
    ev.0["object"]["message"]["text"].as_str().unwrap()
}
