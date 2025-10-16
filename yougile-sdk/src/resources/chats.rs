use crate::SDKError;
use std::sync::Arc;
use yougile_client::YouGileClient;

/// API for working with chats
pub struct ChatsAPI {
    client: Arc<YouGileClient>,
}

impl ChatsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Send a chat message
    pub async fn send_message(
        &self,
        chat_id: &str,
        create_message: yougile_client::models::CreateChatMessage,
    ) -> Result<yougile_client::models::ChatId, SDKError> {
        self.client
            .send_chat_message(chat_id, create_message)
            .await
            .map_err(SDKError::from)
    }

    /// Get a specific chat message
    pub async fn get_message(
        &self,
        chat_id: &str,
        id: f64,
    ) -> Result<yougile_client::models::ChatMessage, SDKError> {
        self.client
            .get_chat_message(chat_id, id)
            .await
            .map_err(SDKError::from)
    }

    /// Update a chat message
    pub async fn update_message(
        &self,
        chat_id: &str,
        id: f64,
        update_message: yougile_client::models::UpdateChatMessage,
    ) -> Result<yougile_client::models::ChatId, SDKError> {
        self.client
            .update_chat_message(chat_id, id, update_message)
            .await
            .map_err(SDKError::from)
    }

    /// Search for chat messages with various filters
    #[allow(clippy::too_many_arguments)]
    pub async fn search_messages(
        &self,
        chat_id: &str,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        from_user_id: Option<&str>,
        text: Option<&str>,
        label: Option<&str>,
        since: Option<f64>,
        include_system: Option<bool>,
    ) -> Result<yougile_client::models::ChatMessageList, SDKError> {
        self.client
            .search_chat_messages(
                chat_id,
                include_deleted,
                limit,
                offset,
                from_user_id,
                text,
                label,
                since,
                include_system,
            )
            .await
            .map_err(SDKError::from)
    }
}
