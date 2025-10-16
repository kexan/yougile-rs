use crate::{
    apis::{
        configuration::Configuration,
        parse_response, RequestBuilderExt,
    },
    models::{CreateGroupChat, GroupChat, GroupChatList, Id, UpdateGroupChat},
    YougileError,
};

const GROUP_CHATS_PATH: &str = "/api-v2/group-chats";

pub async fn create_group_chat(
    configuration: &Configuration,
    create_group_chat: CreateGroupChat,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, GROUP_CHATS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_group_chat)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_group_chat(
    configuration: &Configuration,
    id: &str,
) -> Result<GroupChat, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, GROUP_CHATS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_group_chat(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
) -> Result<GroupChatList, YougileError> {
    let url = format!("{}{}", configuration.base_path, GROUP_CHATS_PATH);

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
    if let Some(val) = title {
        query_params.push(("title", val.to_string()));
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

pub async fn update_group_chat(
    configuration: &Configuration,
    id: &str,
    update_group_chat: UpdateGroupChat,
) -> Result<Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, GROUP_CHATS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_group_chat)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
