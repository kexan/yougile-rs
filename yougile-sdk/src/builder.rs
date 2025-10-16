use crate::{
    SDKError,
    resources::{BoardsAPI, ProjectsAPI, TasksAPI, UsersAPI},
};
use yougile_client::{YouGileClient, apis::configuration::Configuration};

/// Builder for creating a YouGileSDK instance
pub struct YouGileSDKBuilder {
    token: Option<String>,
    base_url: Option<String>,
}

impl YouGileSDKBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            base_url: None,
        }
    }

    /// Set the authentication token
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set the base URL for API requests
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Build the YouGileSDK client
    pub fn build(self) -> Result<YouGileSDK, SDKError> {
        let token = self
            .token
            .ok_or_else(|| SDKError::ConfigurationError("Token is required".into()))?;

        let mut config = Configuration::new(token);

        if let Some(base_url) = self.base_url {
            config = config.with_base_path(base_url);
        }

        let low_level_client = YouGileClient::new(config);

        Ok(YouGileSDK {
            inner: low_level_client,
        })
    }
}

impl Default for YouGileSDKBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The main YouGile SDK client with a high-level API
pub struct YouGileSDK {
    inner: YouGileClient,
}

impl YouGileSDK {
    /// Create a new builder for the YouGileSDK
    pub fn builder() -> YouGileSDKBuilder {
        YouGileSDKBuilder::new()
    }

    /// Get access to the tasks API
    pub fn tasks(&self) -> TasksAPI<'_> {
        TasksAPI::new(&self.inner)
    }

    /// Get access to the projects API
    pub fn projects(&self) -> ProjectsAPI<'_> {
        ProjectsAPI::new(&self.inner)
    }

    /// Get access to the users API
    pub fn users(&self) -> UsersAPI<'_> {
        UsersAPI::new(&self.inner)
    }

    /// Get access to the boards API
    pub fn boards(&self) -> BoardsAPI<'_> {
        BoardsAPI::new(&self.inner)
    }

    /// Get the company information
    pub async fn get_company(&self) -> Result<yougile_client::models::Company, SDKError> {
        self.inner.get_company().await.map_err(SDKError::from)
    }

    /// Get access to the low-level client if needed
    pub fn low_level_client(&self) -> &YouGileClient {
        &self.inner
    }
}

