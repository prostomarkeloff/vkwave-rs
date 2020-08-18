//! Things related to making API request.
//!
//! This introduces [`Client`].

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
/// API client.
///
/// It knows nothing about you, your token and anything else. It cares only about method and request parameters.
///
/// # Example
/// [`Client::request`]
pub struct Client {
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create new client.
    ///
    /// # Example
    /// ```
    /// # use vkwave::api::client::Client;
    /// let client = Client::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Send request to server.
    ///
    /// # Example
    /// ```no_run
    /// # use vkwave::api::client::Client;
    /// # use std::collections::HashMap;
    /// # #[tokio::test]
    /// # async fn test() {
    /// let client = Client::new();
    /// let mut data = HashMap::<String, String>::new();
    /// data.insert("v".into(), "5.103".into());
    /// data.insert("access_token".into(), "my-token".into());
    /// let response = client.request("status.get", data).await.unwrap();
    /// // ...
    /// # }
    /// ```
    #[inline]
    pub async fn request<T: Into<String>>(
        &self,
        method: T,
        data: HashMap<&'static str, String>,
    ) -> reqwest::Result<reqwest::Response> {
        self.client
            .post(format!("https://api.vk.com/method/{}", method.into()).as_str())
            .form(&data)
            .send()
            .await
    }
}

impl From<reqwest::Client> for Client {
    fn from(v: reqwest::Client) -> Self {
        Self { client: v }
    }
}
