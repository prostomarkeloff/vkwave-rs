//! Bot longpoll implementation.

use crate::api::api_context::APIContext;
use crate::api::client::Client;
use crate::errors::{Result, VkwaveError};
use crate::models::response::APIResponse;
use async_recursion::async_recursion;
use serde_json::{Map, Value};
use std::sync::Arc;
use vkwave_token::token::GetToken;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupsGetLongpollServer {
    pub key: String,
    pub server: String,
    pub ts: String,
}

/// The state of doing LP.
#[derive(Debug, Clone)]
pub struct BotLongpollInfo {
    /// Key
    key: String,
    /// Server
    server: String,
    /// TS
    ts: String,
    /// Url for making requests
    url: String,
    /// Wait
    wait: i64,

    /// Did we do first request?
    first_request: bool,
}

// own impl because `wait`
impl std::default::Default for BotLongpollInfo {
    fn default() -> Self {
        Self {
            first_request: false,
            key: String::new(),
            server: String::new(),
            ts: String::new(),
            url: String::new(),
            wait: 25,
        }
    }
}

impl BotLongpollInfo {
    /// Update data from response
    pub fn update_data(&mut self, data: GroupsGetLongpollServer) {
        self.key = data.key;
        self.server = data.server;
        self.ts = data.ts;
        self.update_url();
    }

    /// Update url
    pub fn update_url(&mut self) {
        self.url = format!(
            "{server}?act=a_check&key={key}&ts={ts}&wait={wait}",
            server = self.server,
            key = self.key,
            ts = self.ts,
            wait = self.wait.to_string()
        )
        .replace('\"', "");
    }
}

/// Bot longpoll implementation
pub struct BotLongpoll<T: GetToken + Debug> {
    /// API Client
    ctx: Arc<APIContext<T>>,
    /// HTTP client
    client: Arc<Client>,
    /// The state
    info: BotLongpollInfo,
    /// Group ID
    group_id: String,
}

impl<T: GetToken + Send + Sync + 'static + Debug> BotLongpoll<T> {
    /// Create new longpoll
    #[inline]
    pub fn new(ctx: Arc<APIContext<T>>, group_id: String) -> Self {
        Self {
            client: Arc::clone(&ctx.options().client),
            ctx,
            info: BotLongpollInfo::default(),
            group_id,
        }
    }

    /// Handle Longpoll's error
    #[async_recursion]
    pub async fn handle_error(
        &mut self,
        code: i64,
        data: Map<String, Value>,
    ) -> Result<Vec<Value>> {
        if code == 1 {
            self.info.ts = data["ts"].as_str().unwrap().to_owned();
            self.info.update_url();
            return Ok(self.get_updates().await?);
        }

        if code == 2 || code == 3 {
            self.update_data().await?;
            return Ok(self.get_updates().await?);
        }

        Err(VkwaveError::Other("Unknown code of fail".to_string()))
    }

    async fn update_data(&mut self) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("group_id", self.group_id.clone());

        let result: APIResponse<GroupsGetLongpollServer> = self
            .ctx
            .api_request("groups.getLongPollServer".into(), params)
            .await?;

        self.info.update_data(result.response);
        self.info.first_request = true;
        Ok(())
    }

    /// Get updates
    #[inline]
    pub async fn get_updates(&mut self) -> Result<Vec<Value>> {
        if !self.info.first_request {
            self.update_data().await?;
        }

        let data_v: Value = self
            .client
            .client
            .get(self.info.url.as_str())
            .send()
            .await
            .map_err(VkwaveError::from)?
            .json::<Value>()
            .await
            .map_err(VkwaveError::from)?;

        let data: &Map<String, Value> = data_v
            .as_object()
            .ok_or_else(|| VkwaveError::Other("Longpoll returned not an object".to_string()))?;

        match data.get("failed") {
            None => {
                self.info.ts = data["ts"].as_str().unwrap().to_owned();
                self.info.update_url();
                Ok(data["updates"]
                    .as_array()
                    .ok_or_else(|| {
                        VkwaveError::Other("Longpoll returned not a list of values".to_string())
                    })?
                    .to_vec())
            }

            Some(_) => {
                let code = data["failed"]
                    .as_i64()
                    .ok_or_else(|| VkwaveError::Other("Failed code is not a i64".to_string()))?;
                Ok(self.handle_error(code, data.clone()).await?)
            }
        }
    }
}
