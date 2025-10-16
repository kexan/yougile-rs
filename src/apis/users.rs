use crate::{
    apis::{
        configuration::Configuration,
        parse_response, RequestBuilderExt,
    },
    models::{CreateUser, User, UserList, UpdateUser},
    YougileError,
};

const USERS_PATH: &str = "/api-v2/users";

pub async fn create_user(
    configuration: &Configuration,
    create_user: CreateUser,
) -> Result<crate::models::Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, USERS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_user)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn delete_user(
    configuration: &Configuration,
    id: &str,
) -> Result<crate::models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, USERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .delete(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_user(
    configuration: &Configuration,
    id: &str,
) -> Result<User, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, USERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_user(
    configuration: &Configuration,
    limit: Option<f64>,
    offset: Option<f64>,
    email: Option<&str>,
    project_id: Option<&str>,
) -> Result<UserList, YougileError> {
    let url = format!("{}{}", configuration.base_path, USERS_PATH);

    let mut query_params = vec![];
    if let Some(val) = limit {
        query_params.push(("limit", val.to_string()));
    }
    if let Some(val) = offset {
        query_params.push(("offset", val.to_string()));
    }
    if let Some(val) = email {
        query_params.push(("email", val.to_string()));
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

pub async fn update_user(
    configuration: &Configuration,
    id: &str,
    update_user: UpdateUser,
) -> Result<crate::models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, USERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_user)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
