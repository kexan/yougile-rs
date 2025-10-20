use crate::{
    YougileError,
    apis::{RequestBuilderExt, configuration::Configuration, parse_response},
    models::{CreateWebhook, Id, UpdateWebhook, Webhook},
};

const WEBHOOKS_PATH: &str = "/api-v2/webhooks";

/// Создает подписку на события в компании
pub async fn create_webhook(
    configuration: &Configuration,
    create_webhook: CreateWebhook,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, WEBHOOKS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_webhook)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_webhook(
    configuration: &Configuration,
    id: &str,
    update_webhook: UpdateWebhook,
) -> Result<Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, WEBHOOKS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_webhook)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_webhooks(
    configuration: &Configuration,
    include_deleted: Option<bool>,
) -> Result<Vec<Webhook>, YougileError> {
    let url = format!("{}{}", configuration.base_path, WEBHOOKS_PATH);

    let mut query_params = vec![];
    if let Some(val) = include_deleted {
        query_params.push(("includeDeleted", val.to_string()));
    }

    let resp = configuration
        .client
        .get(&url)
        .query(&query_params)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
