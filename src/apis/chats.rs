use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt, ResponseContent},
    models::{self, ChatId, ChatMessage, ChatMessageList, CreateChatMessage, UpdateChatMessage},
    YougileError,
};
const CHAT_MESSAGES_PATH: &str = "/api-v2/chats";

pub async fn get_chat_message(
    configuration: &Configuration,
    chat_id: &str,
    id: f64,
) -> Result<ChatMessage, YougileError> {
    let encoded_chat_id = crate::apis::urlencode(chat_id);
    let url = format!(
        "{}{}/{}/messages/{}",
        configuration.base_path, CHAT_MESSAGES_PATH, encoded_chat_id, id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_chat_messages(
    configuration: &Configuration,
    chat_id: &str,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    from_user_id: Option<&str>,
    text: Option<&str>,
    label: Option<&str>,
    since: Option<f64>,
    include_system: Option<bool>,
) -> Result<ChatMessageList, YougileError> {
    let encoded_chat_id = crate::apis::urlencode(chat_id);
    let url = format!(
        "{}/api-v2/chats/{}/messages",
        configuration.base_path, encoded_chat_id
    );

    let mut query_params = vec![];
    if let Some(val) = include_deleted {
        query_params.push(("includeDeleted", val.to_string()));
    }
    if let Some(val) = limit {
        query_params.push(("limit", val.to_string()));
    }
    if let Some(val) = offset {
        query_params.push(("offset", val.to_string()));
    }
    if let Some(val) = from_user_id {
        query_params.push(("fromUserId", val.to_string()));
    }
    if let Some(val) = text {
        query_params.push(("text", val.to_string()));
    }
    if let Some(val) = label {
        query_params.push(("label", val.to_string()));
    }
    if let Some(val) = since {
        query_params.push(("since", val.to_string()));
    }
    if let Some(val) = include_system {
        query_params.push(("includeSystem", val.to_string()));
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

pub async fn send_chat_message(
    configuration: &Configuration,
    chat_id: &str,
    create_chat_message_dto: CreateChatMessage,
) -> Result<ChatId, YougileError> {
    let encoded_chat_id = crate::apis::urlencode(chat_id);
    let url = format!(
        "{}/api-v2/chats/{}/messages",
        configuration.base_path, encoded_chat_id
    );

    let resp = configuration
        .client
        .post(&url)
        .json(&create_chat_message_dto)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_chat_message(
    configuration: &Configuration,
    chat_id: &str,
    id: f64,
    update_chat_message_dto: UpdateChatMessage,
) -> Result<ChatId, YougileError> {
    let encoded_chat_id = crate::apis::urlencode(chat_id);
    let url = format!(
        "{}/api-v2/chats/{}/messages/{}",
        configuration.base_path, encoded_chat_id, id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_chat_message_dto)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
