use yougile_client::YouGileClient;
use crate::SDKError;

/// API for working with webhooks
pub struct WebhooksAPI<'a> {
    client: &'a YouGileClient,
}

impl<'a> WebhooksAPI<'a> {
    pub fn new(client: &'a YouGileClient) -> Self {
        Self { client }
    }

    /// Create a webhook
    pub async fn create(
        &self,
        create_webhook: yougile_client::models::CreateWebhook,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.create_webhook(create_webhook).await.map_err(SDKError::from)
    }

    /// Update a webhook
    pub async fn update(
        &self,
        id: &str,
        update_webhook: yougile_client::models::UpdateWebhook,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_webhook(id, update_webhook).await.map_err(SDKError::from)
    }

    /// Search for webhooks with various filters
    pub async fn search(
        &self,
        include_deleted: Option<bool>,
    ) -> Result<yougile_client::models::Webhook, SDKError> {
        self.client.search_webhooks(include_deleted).await.map_err(SDKError::from)
    }
}