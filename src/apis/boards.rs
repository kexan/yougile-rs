use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt, ResponseContent},
    models::{self, Board, BoardList, CreateBoard, Id, UpdateBoard},
    YougileError,
};

pub const BOARD_PATH: &str = "/api-v2/boards";

pub async fn create_board(
    configuration: &Configuration,
    create_board: CreateBoard,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, BOARD_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_board)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_board(configuration: &Configuration, id: &str) -> Result<Board, YougileError> {
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

pub async fn search_board(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    project_id: Option<&str>,
) -> Result<BoardList, YougileError> {
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

pub async fn update_board(
    configuration: &Configuration,
    id: &str,
    update_board: UpdateBoard,
) -> Result<Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!("{}{}/{}", configuration.base_path, BOARD_PATH, encoded_id);

    let resp = configuration
        .client
        .put(&url)
        .json(&update_board)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
