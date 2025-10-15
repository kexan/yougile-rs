use super::{configuration, ContentType, Error};
use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt, ResponseContent},
    models::{self, Board, CreateBoard, UpdateBoard},
    YougileError,
};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

pub const BOARD_PATH: &str = "/api-v2/boards";

pub async fn board_controller_create(
    configuration: &Configuration,
    create_board_dto: CreateBoard,
) -> Result<models::Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, BOARD_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_board_dto)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn board_controller_get(
    configuration: &Configuration,
    id: &str,
) -> Result<Board, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!("{}{}/{}", configuration.base_path, BOARD_PATH, encoded_id);

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn board_controller_search(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    project_id: Option<&str>,
) -> Result<models::BoardListDto, YougileError> {
    let url = format!("{}{}", configuration.base_path, BOARD_PATH);

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
    if let Some(val) = project_id {
        query_params.push(("projectId", val.to_string()));
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

pub async fn board_controller_update(
    configuration: &Configuration,
    id: &str,
    update_board_dto: UpdateBoard,
) -> Result<models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!("{}{}/{}", configuration.base_path, BOARD_PATH, encoded_id);

    let resp = configuration
        .client
        .put(&url)
        .json(&update_board_dto)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
