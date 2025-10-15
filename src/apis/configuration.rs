/// Configuration for the YouGile API client.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Base URL for API requests
    pub base_path: String,
    /// User agent string to use for requests
    pub user_agent: Option<String>,
    /// HTTP client to use for requests
    pub client: reqwest::Client,
    /// Basic authentication credentials
    pub basic_auth: Option<BasicAuth>,
    /// OAuth access token
    pub oauth_access_token: Option<String>,
    /// Bearer access token
    pub bearer_access_token: Option<String>,
    /// API key for authentication
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    /// Creates a new configuration with default values
    pub fn new() -> Configuration {
        Configuration::default()
    }

    /// Sets the base path for API requests
    pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
        self.base_path = base_path.into();
        self
    }

    /// Sets the bearer access token for authentication
    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        self.bearer_access_token = Some(token.into());
        self
    }

    /// Sets the API key for authentication
    pub fn with_api_key(mut self, prefix: Option<String>, key: impl Into<String>) -> Self {
        self.api_key = Some(ApiKey {
            prefix,
            key: key.into(),
        });
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://yougile.com".to_owned(), // More appropriate default
            user_agent: Some("yougile-client/2.0".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
