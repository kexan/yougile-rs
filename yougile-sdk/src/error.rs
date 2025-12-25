use thiserror::Error;
use yougile_api_client::YougileError;

/// Errors that can occur in the YouGile SDK
#[derive(Debug, Error)]
pub enum SDKError {
    /// Error from the underlying API client
    #[error("API client error: {0}")]
    ClientError(#[from] YougileError),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Resource not found error
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Permission error
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Rate limit error
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

impl SDKError {
    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        SDKError::ValidationError(msg.into())
    }

    /// Create a not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        SDKError::NotFound(msg.into())
    }

    /// Create a permission denied error
    pub fn permission_denied(msg: impl Into<String>) -> Self {
        SDKError::PermissionDenied(msg.into())
    }

    /// Create a rate limit exceeded error
    pub fn rate_limit_exceeded(msg: impl Into<String>) -> Self {
        SDKError::RateLimitExceeded(msg.into())
    }
}
