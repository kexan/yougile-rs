/// Configuration for the YouGile API client.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Base URL for API requests
    pub base_path: String,
    /// User agent string to use for requests
    pub user_agent: String,
    /// HTTP client to use for requests
    pub client: reqwest::Client,
    /// Bearer access token
    pub token: String,
}

use std::time::Duration;

impl Configuration {
    pub fn new(token: String) -> Self {
        Self {
            base_path: "https://yougile.com".to_owned(),
            user_agent: "yougile-client/2.0".to_owned(),
            client: reqwest::Client::new(),
            token,
        }
    }

    /// Sets the base path for API requests
    pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
        self.base_path = base_path.into();
        self
    }

    /// Sets the timeout for all HTTP requests
    pub fn with_timeout(self, timeout: Duration) -> Self {
        self.with_timeout_builder(timeout)
    }

    pub(crate) fn with_timeout_builder(self, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build reqwest client with timeout");
        Self { client, ..self }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        panic!("Bearer access token is required. Use `Configuration::new(token)` instead.");
    }
}
