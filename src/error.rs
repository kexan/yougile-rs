use std::fmt;

/// A type that represents errors that can occur when using the YouGile API.
#[derive(Debug)]
pub enum YouGileError {
    /// Error from the API client
    Api(crate::apis::Error<serde_json::Value>),
    /// Error from the HTTP client
    Http(reqwest::Error),
    /// Error from JSON serialization/deserialization
    Json(serde_json::Error),
    /// Error from IO operations
    Io(std::io::Error),
}

impl fmt::Display for YouGileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YouGileError::Api(e) => write!(f, "API error: {:?}", e),
            YouGileError::Http(e) => write!(f, "HTTP error: {}", e),
            YouGileError::Json(e) => write!(f, "JSON error: {}", e),
            YouGileError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for YouGileError {}

impl From<crate::apis::Error<serde_json::Value>> for YouGileError {
    fn from(error: crate::apis::Error<serde_json::Value>) -> Self {
        YouGileError::Api(error)
    }
}

impl From<reqwest::Error> for YouGileError {
    fn from(error: reqwest::Error) -> Self {
        YouGileError::Http(error)
    }
}

impl From<serde_json::Error> for YouGileError {
    fn from(error: serde_json::Error) -> Self {
        YouGileError::Json(error)
    }
}

impl From<std::io::Error> for YouGileError {
    fn from(error: std::io::Error) -> Self {
        YouGileError::Io(error)
    }
}

/// Result type that uses YouGileError
pub type Result<T> = std::result::Result<T, YouGileError>;