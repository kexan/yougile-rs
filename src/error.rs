use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum YougileError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Unexpected content type: {0}")]
    UnsupportedContentType(String),

    #[error("Yougile API error: status {status}, content: {content}")]
    ApiError { status: StatusCode, content: String },

    #[error("Other error: {0}")]
    Other(String),
}

