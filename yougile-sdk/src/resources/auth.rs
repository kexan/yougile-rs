use crate::SDKError;
use std::sync::Arc;
use yougile_client::YouGileClient;

/// API for authentication and company operations
pub struct AuthAPI {
    client: Arc<YouGileClient>,
}

impl AuthAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Create an authentication key
    pub async fn create_auth_key(
        &self,
        credentials: yougile_client::models::AuthCredentials,
    ) -> Result<yougile_client::models::AuthKey, SDKError> {
        self.client
            .create_auth_key(credentials)
            .await
            .map_err(SDKError::from)
    }

    /// Delete an authentication key
    pub async fn delete_auth_key(&self, key: &str) -> Result<(), SDKError> {
        self.client
            .delete_auth_key(key)
            .await
            .map_err(SDKError::from)
    }

    /// Search for authentication keys
    pub async fn search_auth_keys(
        &self,
        credentials: yougile_client::models::AuthCredentials,
    ) -> Result<Vec<yougile_client::models::AuthKeyWithDetails>, SDKError> {
        self.client
            .search_auth_keys(credentials)
            .await
            .map_err(SDKError::from)
    }

    /// Get company information
    pub async fn get_company(&self) -> Result<yougile_client::models::Company, SDKError> {
        self.client.get_company().await.map_err(SDKError::from)
    }

    /// Update company information
    pub async fn update_company(
        &self,
        update: yougile_client::models::UpdateCompany,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_company(update)
            .await
            .map_err(SDKError::from)
    }

    /// Get companies
    pub async fn get_companies(
        &self,
        credentials: yougile_client::models::AuthCredentials,
        limit: Option<f64>,
        offset: Option<f64>,
    ) -> Result<yougile_client::models::CompanyList, SDKError> {
        self.client
            .get_companies(credentials, limit, offset)
            .await
            .map_err(SDKError::from)
    }
}
