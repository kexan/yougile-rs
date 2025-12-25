use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{CreateGroupChat, GroupChat, GroupChatList, Id, UpdateGroupChat},
};

/// API for working with group chats
pub struct GroupChatsAPI {
    client: Arc<YouGileClient>,
}

impl GroupChatsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific group chat by ID
    pub async fn get(&self, id: &str) -> Result<GroupChat, SDKError> {
        self.client.get_group_chat(id).await.map_err(SDKError::from)
    }

    /// Create a new group chat
    pub async fn create(&self, create_group_chat: CreateGroupChat) -> Result<Id, SDKError> {
        self.client
            .create_group_chat(create_group_chat)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing group chat
    pub async fn update(
        &self,
        id: &str,
        update_group_chat: UpdateGroupChat,
    ) -> Result<Id, SDKError> {
        self.client
            .update_group_chat(id, update_group_chat)
            .await
            .map_err(SDKError::from)
    }

    /// Search for group chats using a fluent API
    pub fn search(&self) -> GroupChatSearchBuilder {
        GroupChatSearchBuilder::new(self.client.clone())
    }

    /// List all group chats
    pub async fn list(&self) -> Result<GroupChatList, SDKError> {
        self.search().execute().await
    }

    /// List all group chats with automatic pagination
    pub async fn list_all(&self) -> Result<Vec<GroupChat>, SDKError> {
        self.search().all().await
    }
}

/// Search builder for group chats with fluent API
#[derive(Clone)]
pub struct GroupChatSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<String>,
}

impl GroupChatSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(100.0),
            offset: Some(0.0),
            title: None,
        }
    }

    pub fn include_deleted(mut self, include: bool) -> Self {
        self.include_deleted = Some(include);
        self
    }

    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<GroupChatList, SDKError> {
        self.client
            .search_group_chat(
                self.include_deleted,
                self.limit,
                self.offset,
                self.title.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all group chats matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<GroupChat>, SDKError> {
        let mut all_chats = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_chats.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_chats)
    }
}
