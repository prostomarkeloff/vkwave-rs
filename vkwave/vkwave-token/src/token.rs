//! Things related to tokens.
//!
//! This introduces the [`GetToken`] trait and its default implementation: [`DefaultToken`]

use futures::future::BoxFuture;
use std::sync::Arc;
pub trait GetToken {
    fn get_token(&self) -> BoxFuture<'static, Arc<RawToken>>;
}

/// Raw representation of a token.
pub type RawToken = String;

#[derive(Debug)]
pub struct DefaultToken(Arc<RawToken>);

impl DefaultToken {
    #[inline]
    pub fn new<T: Into<String>>(token: T) -> Self {
        Self(Arc::new(token.into()))
    }
}

impl GetToken for DefaultToken {
    #[inline]
    fn get_token(&self) -> BoxFuture<'static, Arc<String>> {
        let _token = Arc::clone(&self.0);
        Box::pin(async move { _token })
    }
}

impl<T: Into<String>> From<T> for DefaultToken {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}
