use yougile_client::YouGileClient;
use crate::SDKError;

/// API for authentication and company operations
pub struct AuthAPI<'a> {
    client: &'a YouGileClient,
}

impl<'a> AuthAPI<'a> {
    pub fn new(client: &'a YouGileClient) -> Self {
        Self { client }
    }

    /// Create an authentication key
    pub async fn create_auth_key(
        &self,
        credentials: yougile_client::models::CredentialsWithCompany,
    ) -> Result<yougile_client::models::AuthKey, SDKError> {
        self.client.create_auth_key(credentials).await.map_err(SDKError::from)
    }

    /// Delete an authentication key
    pub async fn delete_auth_key(&self, key: &str) -> Result<(), SDKError> {
        self.client.delete_auth_key(key).await.map_err(SDKError::from)
    }

    /// Search for authentication keys
    pub async fn search_auth_keys(
        &self,
        credentials: yougile_client::models::CredentialsWithCompanyOptional,
    ) -> Result<Vec<yougile_client::models::AuthKeyWithDetails>, SDKError> {
        self.client.search_auth_keys(credentials).await.map_err(SDKError::from)
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
        self.client.update_company(update).await.map_err(SDKError::from)
    }

    /// Get companies
    pub async fn get_companies(
        &self,
        credentials: yougile_client::models::CredentialsWithName,
        limit: Option<f64>,
        offset: Option<f64>,
    ) -> Result<yougile_client::models::CompanyList, SDKError> {
        self.client.get_companies(credentials, limit, offset).await.map_err(SDKError::from)
    }
}