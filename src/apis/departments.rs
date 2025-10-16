use crate::{
    apis::{
        configuration::{self, Configuration},
        parse_response, RequestBuilderExt,
    },
    models::{
        department::{CreateDepartment, DepartmentList, UpdateDepartment},
        Department, Id,
    },
    YougileError,
};

const DEPARMENTS_PATH: &str = "/api-v2/departments";

pub async fn create_department(
    configuration: &Configuration,
    create_department: CreateDepartment,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, DEPARMENTS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_department)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_department(
    configuration: &Configuration,
    id: &str,
) -> Result<Department, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, DEPARMENTS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_department(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
    parent_id: Option<&str>,
) -> Result<DepartmentList, YougileError> {
    let url = format!("{}{}", configuration.base_path, DEPARMENTS_PATH);

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
    if let Some(val) = parent_id {
        query_params.push(("parent_id", val.to_string()));
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

pub async fn update_department(
    configuration: &Configuration,
    id: &str,
    update_department: UpdateDepartment,
) -> Result<Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, DEPARMENTS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_department)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
