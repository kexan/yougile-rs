use crate::{
    apis::{
        configuration::Configuration,
        parse_response, RequestBuilderExt,
    },
    models::{
        CreateTask, Task, TaskList, UpdateTask, TaskChatSubscribers
    },
    YougileError,
};

const TASKS_PATH: &str = "/api-v2/tasks";

pub async fn create_task(
    configuration: &Configuration,
    create_task: CreateTask,
) -> Result<crate::models::Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, TASKS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_task)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_task(
    configuration: &Configuration,
    id: &str,
) -> Result<Task, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, TASKS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_task_chat_subscribers(
    configuration: &Configuration,
    id: &str,
) -> Result<Vec<String>, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}/api-v2/tasks/{}/chat-subscribers",
        configuration.base_path, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_task(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    column_id: Option<&str>,
    assigned_to: Option<&str>,
    sticker_id: Option<&str>,
    sticker_state_id: Option<&str>,
) -> Result<TaskList, YougileError> {
    let url = format!("{}/api-v2/task-list", configuration.base_path);

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
    if let Some(val) = column_id {
        query_params.push(("columnId", val.to_string()));
    }
    if let Some(val) = assigned_to {
        query_params.push(("assignedTo", val.to_string()));
    }
    if let Some(val) = sticker_id {
        query_params.push(("stickerId", val.to_string()));
    }
    if let Some(val) = sticker_state_id {
        query_params.push(("stickerStateId", val.to_string()));
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

/// Используйте /task-list вместо этого
pub async fn search_task_reversed(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    column_id: Option<&str>,
    assigned_to: Option<&str>,
    sticker_id: Option<&str>,
    sticker_state_id: Option<&str>,
) -> Result<TaskList, YougileError> {
    let url = format!("{}{}", configuration.base_path, TASKS_PATH);

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
    if let Some(val) = column_id {
        query_params.push(("columnId", val.to_string()));
    }
    if let Some(val) = assigned_to {
        query_params.push(("assignedTo", val.to_string()));
    }
    if let Some(val) = sticker_id {
        query_params.push(("stickerId", val.to_string()));
    }
    if let Some(val) = sticker_state_id {
        query_params.push(("stickerStateId", val.to_string()));
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

pub async fn update_task(
    configuration: &Configuration,
    id: &str,
    update_task: UpdateTask,
) -> Result<crate::models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, TASKS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_task)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_task_chat_subscribers(
    configuration: &Configuration,
    id: &str,
    task_chat_subscribers: TaskChatSubscribers,
) -> Result<crate::models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}/api-v2/tasks/{}/chat-subscribers",
        configuration.base_path, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&task_chat_subscribers)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
