use crate::errors::{Result, VkwaveError};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{from_value, Map, Value};

#[derive(Debug, Deserialize)]
pub struct APIResponse<T> {
    pub response: T,
}

/// Error came from API
#[derive(Debug, Deserialize)]
pub struct APIError {
    pub error: APIErrorInner,
}

#[derive(Debug, Deserialize)]
pub struct APIErrorInner {
    pub error_code: i32,
    pub error_msg: String,
    pub request_params: Vec<Value>,
}

#[inline]
pub(crate) fn parse_response<T: DeserializeOwned>(mut response: Value) -> Result<APIResponse<T>> {
    let as_map: &mut Map<String, Value> = response
        .as_object_mut()
        .ok_or_else(|| VkwaveError::Other("API has returned not an object".into()))?;

    match as_map.remove("response") {
        Some(ok) => {
            let result: T = from_value(ok)?;
            Ok(APIResponse::<T> { response: result })
        }
        None => {
            let error = as_map.remove("error").ok_or_else(|| {
                VkwaveError::Other("API has returned neither error nor response".into())
            })?;
            let inner: APIErrorInner = from_value(error)?;
            Err(VkwaveError::API(APIError { error: inner }))
        }
    }
}
