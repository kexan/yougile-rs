use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{CreateWebhook, Id, UpdateWebhook, Webhook},
};

/// API for working with webhooks
pub struct WebhooksAPI {
    client: Arc<YouGileClient>,
}

impl WebhooksAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Create a new webhook
    pub async fn create(&self, create_webhook: CreateWebhook) -> Result<Id, SDKError> {
        self.client
            .create_webhook(create_webhook)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing webhook
    pub async fn update(&self, id: &str, update_webhook: UpdateWebhook) -> Result<Id, SDKError> {
        self.client
            .update_webhook(id, update_webhook)
            .await
            .map_err(SDKError::from)
    }

    /// List all webhooks
    pub async fn list(&self, include_deleted: Option<bool>) -> Result<Vec<Webhook>, SDKError> {
        self.client
            .search_webhooks(include_deleted)
            .await
            .map_err(SDKError::from)
    }

    /// List all active webhooks
    pub async fn list_active(&self) -> Result<Vec<Webhook>, SDKError> {
        self.list(Some(false)).await
    }

    /// List all webhooks including deleted ones
    pub async fn list_all(&self) -> Result<Vec<Webhook>, SDKError> {
        self.list(Some(true)).await
    }
}
