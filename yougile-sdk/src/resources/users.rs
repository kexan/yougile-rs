use crate::{SDKError, convenience::UserSearchBuilder};
use std::sync::Arc;
use yougile_client::YouGileClient;

/// API for working with users
pub struct UsersAPI {
    client: Arc<YouGileClient>,
}

impl UsersAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific user by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::User, SDKError> {
        self.client.get_user(id).await.map_err(SDKError::from)
    }

    /// Create a new user
    pub async fn create(
        &self,
        create_user: yougile_client::models::CreateUser,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .create_user(create_user)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing user
    pub async fn update(
        &self,
        id: &str,
        update_user: yougile_client::models::UpdateUser,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_user(id, update_user)
            .await
            .map_err(SDKError::from)
    }

    /// Delete a user
    pub async fn delete(&self, id: &str) -> Result<yougile_client::models::Id, SDKError> {
        self.client.delete_user(id).await.map_err(SDKError::from)
    }

    /// Search for users with various filters using a fluent API
    pub fn search(&self) -> UserSearchBuilder {
        UserSearchBuilder::new(&self.client)
    }

    /// List all users (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::UserList, SDKError> {
        self.search().execute().await
    }
}

