use std::collections::HashMap;
use vkwave::bots::event::{FromRawContext, RawContext};

#[derive(Clone)]
pub struct EasyContext {
    raw: RawContext,
}

impl EasyContext {
    #[inline]
    pub async fn answer_str<T: Into<String>>(&self, t: T) {
        let mut params = HashMap::<&'static str, String>::new();
        params.insert("peer_id", self.peer_id().to_string());
        params.insert("message", t.into());
        params.insert("random_id", "0".to_string());

        let _ = self
            .raw
            .1
            .api_request_raw("messages.send".into(), params)
            .await;
    }

    #[inline]
    pub fn peer_id(&self) -> i64 {
        self.raw.0["object"]["message"]["peer_id"].as_i64().unwrap()
    }
}

impl FromRawContext for EasyContext {
    #[inline]
    fn from_raw_context(raw: RawContext) -> Option<Self> {
        Some(Self { raw })
    }
}
