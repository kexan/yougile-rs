use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt},
    models::{
        column::{ColumnList, CreateColumn, UpdateColumn},
        Column, Id,
    },
    YougileError,
};

const COLUMNS_PATH: &str = "/api-v2/columns";

pub async fn create_column(
    configuration: &Configuration,
    create_column: CreateColumn,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, COLUMNS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_column)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_column(configuration: &Configuration, id: &str) -> Result<Column, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!("{}{}/{}", configuration.base_path, COLUMNS_PATH, encoded_id);

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_column(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    board_id: Option<&str>,
) -> Result<ColumnList, YougileError> {
    let url = format!("{}{}", configuration.base_path, COLUMNS_PATH);

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
    if let Some(val) = board_id {
        query_params.push(("board_id", val.to_string()));
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

pub async fn update_column(
    configuration: &Configuration,
    id: &str,
    update_column: UpdateColumn,
) -> Result<Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!("{}{}/{}", configuration.base_path, COLUMNS_PATH, encoded_id);

    let resp = configuration
        .client
        .put(&url)
        .json(&update_column)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
