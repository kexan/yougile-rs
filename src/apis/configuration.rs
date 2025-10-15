/// Configuration for the YouGile API client.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Base URL for API requests
    pub base_path: String,
    /// User agent string to use for requests
    pub user_agent: String,
    /// HTTP client to use for requests
    pub client: reqwest::Client,
    /// Bearer access token (required)
    pub bearer_access_token: String,
}

impl Configuration {
    /// Creates a new configuration with default values.
    /// `bearer_access_token` обязательный параметр.
    pub fn new(bearer_access_token: impl Into<String>) -> Self {
        Self {
            base_path: "https://yougile.com".to_owned(),
            user_agent: "yougile-client/2.0".to_owned(),
            client: reqwest::Client::new(),
            bearer_access_token: bearer_access_token.into(),
        }
    }

    /// Sets the base path for API requests
    pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
        self.base_path = base_path.into();
        self
    }

    /// Sets the bearer access token (returns self for chaining)
    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        self.bearer_access_token = token.into();
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        panic!("Bearer access token is required. Use `Configuration::new(token)` instead.");
    }
}
