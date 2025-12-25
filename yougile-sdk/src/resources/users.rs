use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{CreateUser, Id, UpdateUser, User, UserList},
};

/// API for working with users
pub struct UsersAPI {
    client: Arc<YouGileClient>,
}

impl UsersAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific user by ID
    pub async fn get(&self, id: &str) -> Result<User, SDKError> {
        self.client.get_user(id).await.map_err(SDKError::from)
    }

    /// Create a new user
    pub async fn create(&self, create_user: CreateUser) -> Result<Id, SDKError> {
        self.client
            .create_user(create_user)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing user
    pub async fn update(&self, id: &str, update_user: UpdateUser) -> Result<Id, SDKError> {
        self.client
            .update_user(id, update_user)
            .await
            .map_err(SDKError::from)
    }

    /// Delete a user
    pub async fn delete(&self, id: &str) -> Result<Id, SDKError> {
        self.client.delete_user(id).await.map_err(SDKError::from)
    }

    /// Search for users with various filters using a fluent API
    pub fn search(&self) -> UserSearchBuilder {
        UserSearchBuilder::new(self.client.clone())
    }

    /// List all users (with default parameters)
    pub async fn list(&self) -> Result<UserList, SDKError> {
        self.search().execute().await
    }

    /// List all users for a specific project
    pub async fn list_by_project(&self, project_id: &str) -> Result<Vec<User>, SDKError> {
        self.search().project_id(project_id).all().await
    }
}

/// Search builder for users with fluent API
#[derive(Clone)]
pub struct UserSearchBuilder {
    client: Arc<YouGileClient>,
    limit: Option<f64>,
    offset: Option<f64>,
    email: Option<String>,
    project_id: Option<String>,
}

impl UserSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            limit: Some(100.0),
            offset: Some(0.0),
            email: None,
            project_id: None,
        }
    }

    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<UserList, SDKError> {
        self.client
            .search_users(
                self.limit,
                self.offset,
                self.email.as_deref(),
                self.project_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all users matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<User>, SDKError> {
        let mut all_users = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_users.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_users)
    }
}
