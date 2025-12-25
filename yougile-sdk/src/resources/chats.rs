use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{ChatId, ChatMessage, ChatMessageList, CreateChatMessage, UpdateChatMessage},
};

/// API for working with chats
pub struct ChatsAPI {
    client: Arc<YouGileClient>,
}

impl ChatsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Send a message to a chat
    pub async fn send_message(
        &self,
        chat_id: &str,
        message: CreateChatMessage,
    ) -> Result<ChatId, SDKError> {
        self.client
            .send_chat_message(chat_id, message)
            .await
            .map_err(SDKError::from)
    }

    /// Get a specific chat message
    pub async fn get_message(&self, chat_id: &str, id: f64) -> Result<ChatMessage, SDKError> {
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
        update: UpdateChatMessage,
    ) -> Result<ChatId, SDKError> {
        self.client
            .update_chat_message(chat_id, id, update)
            .await
            .map_err(SDKError::from)
    }

    /// Search for chat messages using a fluent API
    pub fn search(&self, chat_id: &str) -> ChatMessageSearchBuilder {
        ChatMessageSearchBuilder::new(self.client.clone(), chat_id.to_string())
    }

    /// List all messages in a chat
    pub async fn list_messages(&self, chat_id: &str) -> Result<ChatMessageList, SDKError> {
        self.search(chat_id).execute().await
    }

    /// List all messages in a chat with automatic pagination
    pub async fn list_messages_all(&self, chat_id: &str) -> Result<Vec<ChatMessage>, SDKError> {
        self.search(chat_id).all().await
    }
}

/// Search builder for chat messages with fluent API
#[derive(Clone)]
pub struct ChatMessageSearchBuilder {
    client: Arc<YouGileClient>,
    chat_id: String,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    from_user_id: Option<String>,
    text: Option<String>,
    label: Option<String>,
    since: Option<f64>,
    include_system: Option<bool>,
}

impl ChatMessageSearchBuilder {
    pub fn new(client: Arc<YouGileClient>, chat_id: String) -> Self {
        Self {
            client,
            chat_id,
            include_deleted: None,
            limit: Some(100.0),
            offset: Some(0.0),
            from_user_id: None,
            text: None,
            label: None,
            since: None,
            include_system: None,
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

    pub fn from_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.from_user_id = Some(user_id.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn since(mut self, since: f64) -> Self {
        self.since = Some(since);
        self
    }

    pub fn include_system(mut self, include: bool) -> Self {
        self.include_system = Some(include);
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<ChatMessageList, SDKError> {
        self.client
            .search_chat_messages(
                &self.chat_id,
                self.include_deleted,
                self.limit,
                self.offset,
                self.from_user_id.as_deref(),
                self.text.as_deref(),
                self.label.as_deref(),
                self.since,
                self.include_system,
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all chat messages matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<ChatMessage>, SDKError> {
        let mut all_messages = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_messages.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_messages)
    }
}
