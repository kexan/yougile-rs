use super::{configuration, ContentType, Error};
use crate::{
    apis::{configuration::Configuration, parse_response, ResponseContent},
    models::{self, CreateBoardDto},
    YougileError,
};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`board_controller_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoardControllerCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`board_controller_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoardControllerGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`board_controller_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoardControllerSearchError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`board_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoardControllerUpdateError {
    Status404(),
    UnknownValue(serde_json::Value),
}

pub async fn board_controller_create(
    configuration: &Configuration,
    create_board_dto: CreateBoardDto,
) -> Result<models::WithIdDto, YougileError> {
    let url = format!("{}/api-v2/boards", configuration.base_path);

    let mut req_builder = configuration.client.post(&url).json(&create_board_dto);

    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    }

    if let Some(ref ua) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, ua.clone());
    }

    let resp = req_builder.send().await?;
    parse_response(resp).await
}

pub async fn board_controller_get(
    configuration: &Configuration,
    id: &str,
) -> Result<models::BoardDto, Error<BoardControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/boards/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BoardDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BoardDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BoardControllerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn board_controller_search(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    project_id: Option<&str>,
) -> Result<models::BoardListDto, Error<BoardControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_include_deleted = include_deleted;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_title = title;
    let p_query_project_id = project_id;

    let uri_str = format!("{}/api-v2/boards", configuration.base_path);
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
    if let Some(ref param_value) = p_query_title {
        req_builder = req_builder.query(&[("title", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_project_id {
        req_builder = req_builder.query(&[("projectId", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BoardListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BoardListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BoardControllerSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn board_controller_update(
    configuration: &Configuration,
    id: &str,
    update_board_dto: models::UpdateBoardDto,
) -> Result<models::WithIdDto, Error<BoardControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_board_dto = update_board_dto;

    let uri_str = format!(
        "{}/api-v2/boards/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_board_dto);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BoardControllerUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
