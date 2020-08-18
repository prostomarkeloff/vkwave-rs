use crate::api::client::Client;
use crate::api::token::GetToken;
use crate::errors::Result;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct APIOptions<T: GetToken + Debug> {
    token: T,
    pub(crate) client: Arc<Client>,
}

impl<T: GetToken + Debug> APIOptions<T> {
    pub fn new(token: T) -> Self {
        Self {
            token,
            client: Arc::new(Client::default()),
        }
    }

    pub fn new_with_client(token: T, client: Arc<Client>) -> Self {
        Self { token, client }
    }
}

#[derive(Debug)]
pub struct APIContext<T: GetToken + Debug> {
    options: APIOptions<T>,
}

impl<T: GetToken + Debug> APIContext<T> {
    #[inline]
    pub fn new(options: APIOptions<T>) -> Self {
        Self { options }
    }

    #[inline]
    pub fn options(&self) -> &APIOptions<T> {
        &self.options
    }

    #[inline]
    pub fn options_mut(&mut self) -> &mut APIOptions<T> {
        &mut self.options
    }

    #[inline]
    pub async fn api_request<R: DeserializeOwned>(
        &self,
        method: String,
        mut params: HashMap<&'static str, String>,
    ) -> Result<crate::models::response::APIResponse<R>> {
        params.insert(
            "access_token",
            self.options.token.get_token().await.to_string(),
        );
        params.insert("v", String::from("5.103"));
        let response = self.options.client.request(method, params).await?;
        crate::models::response::parse_response::<R>(response.json::<Value>().await?)
    }

    #[inline]
    pub async fn api_request_raw(
        &self,
        method: String,
        params: HashMap<&'static str, String>,
    ) -> Result<crate::models::response::APIResponse<Value>> {
        self.api_request(method, params).await
    }
}
