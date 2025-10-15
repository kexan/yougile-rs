use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`chat_message_controller_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageControllerGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`chat_message_controller_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageControllerSearchError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`chat_message_controller_send_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageControllerSendMessageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`chat_message_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageControllerUpdateError {
    Status404(),
    UnknownValue(serde_json::Value),
}

pub async fn chat_message_controller_get(
    configuration: &configuration::Configuration,
    chat_id: &str,
    id: f64,
) -> Result<models::ChatMessageDto, Error<ChatMessageControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_chat_id = chat_id;
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/chats/{chatId}/messages/{id}",
        configuration.base_path,
        chatId = crate::apis::urlencode(p_path_chat_id),
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChatMessageDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChatMessageDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChatMessageControllerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn chat_message_controller_search(
    configuration: &configuration::Configuration,
    chat_id: &str,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    from_user_id: Option<&str>,
    text: Option<&str>,
    label: Option<&str>,
    since: Option<f64>,
    include_system: Option<bool>,
) -> Result<models::ChatMessageListDto, Error<ChatMessageControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_chat_id = chat_id;
    let p_query_include_deleted = include_deleted;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_from_user_id = from_user_id;
    let p_query_text = text;
    let p_query_label = label;
    let p_query_since = since;
    let p_query_include_system = include_system;

    let uri_str = format!(
        "{}/api-v2/chats/{chatId}/messages",
        configuration.base_path,
        chatId = crate::apis::urlencode(p_path_chat_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_include_deleted {
        req_builder = req_builder.query(&[("includeDeleted", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_from_user_id {
        req_builder = req_builder.query(&[("fromUserId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_text {
        req_builder = req_builder.query(&[("text", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_label {
        req_builder = req_builder.query(&[("label", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_include_system {
        req_builder = req_builder.query(&[("includeSystem", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChatMessageListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChatMessageListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChatMessageControllerSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn chat_message_controller_send_message(
    configuration: &configuration::Configuration,
    chat_id: &str,
    create_chat_message_dto: models::CreateChatMessageDto,
) -> Result<models::ChatIdDto, Error<ChatMessageControllerSendMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_chat_id = chat_id;
    let p_body_create_chat_message_dto = create_chat_message_dto;

    let uri_str = format!(
        "{}/api-v2/chats/{chatId}/messages",
        configuration.base_path,
        chatId = crate::apis::urlencode(p_path_chat_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_chat_message_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChatIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChatIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChatMessageControllerSendMessageError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn chat_message_controller_update(
    configuration: &configuration::Configuration,
    chat_id: &str,
    id: f64,
    update_chat_message_dto: models::UpdateChatMessageDto,
) -> Result<models::ChatIdDto, Error<ChatMessageControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_chat_id = chat_id;
    let p_path_id = id;
    let p_body_update_chat_message_dto = update_chat_message_dto;

    let uri_str = format!(
        "{}/api-v2/chats/{chatId}/messages/{id}",
        configuration.base_path,
        chatId = crate::apis::urlencode(p_path_chat_id),
        id = p_path_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_chat_message_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChatIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChatIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChatMessageControllerUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

