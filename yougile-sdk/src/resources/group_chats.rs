use yougile_client::YouGileClient;
use crate::SDKError;

/// API for working with group chats
pub struct GroupChatsAPI<'a> {
    client: &'a YouGileClient,
}

impl<'a> GroupChatsAPI<'a> {
    pub fn new(client: &'a YouGileClient) -> Self {
        Self { client }
    }

    /// Get a specific group chat by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::GroupChat, SDKError> {
        self.client.get_group_chat(id).await.map_err(SDKError::from)
    }

    /// Create a new group chat
    pub async fn create(
        &self, 
        create_group_chat: yougile_client::models::CreateGroupChat
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.create_group_chat(create_group_chat).await.map_err(SDKError::from)
    }

    /// Update an existing group chat
    pub async fn update(
        &self, 
        id: &str, 
        update_group_chat: yougile_client::models::UpdateGroupChat
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_group_chat(id, update_group_chat).await.map_err(SDKError::from)
    }

    /// Search for group chats with various filters
    pub async fn search(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
    ) -> Result<yougile_client::models::GroupChatList, SDKError> {
        self.client
            .search_group_chat(
                include_deleted,
                limit,
                offset,
                title,
            )
            .await
            .map_err(SDKError::from)
    }

    /// List all group chats (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::GroupChatList, SDKError> {
        self.search(None, Some(100.0), Some(0.0), None).await
    }
}