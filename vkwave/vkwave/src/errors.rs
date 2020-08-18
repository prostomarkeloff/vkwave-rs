use crate::models::response::APIError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, VkwaveError>;

#[derive(Error, Debug)]
pub enum VkwaveError {
    #[error("Error during request")]
    Reqwest(#[from] reqwest::Error),
    #[error("Error during de/serialization")]
    Serde(#[from] serde_json::Error),
    #[error("[{}] {}", .0.error.error_code, .0.error.error_msg)]
    API(APIError),
    #[error("{0}")]
    Other(String),
}
