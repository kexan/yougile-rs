use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`project_controller_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectControllerCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_controller_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectControllerGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_controller_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectControllerSearchError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectControllerUpdateError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_roles_controller_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectRolesControllerCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_roles_controller_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectRolesControllerDeleteError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_roles_controller_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectRolesControllerGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_roles_controller_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectRolesControllerSearchError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`project_roles_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectRolesControllerUpdateError {
    Status404(),
    UnknownValue(serde_json::Value),
}

pub async fn project_controller_create(
    configuration: &configuration::Configuration,
    create_project_dto: models::CreateProject,
) -> Result<models::Id, Error<ProjectControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_project_dto = create_project_dto;

    let uri_str = format!("{}/api-v2/projects", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_project_dto);

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
        let entity: Option<ProjectControllerCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_controller_get(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Project, Error<ProjectControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/projects/{id}",
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ProjectDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ProjectDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProjectControllerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_controller_search(
    configuration: &configuration::Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
) -> Result<models::ProjectList, Error<ProjectControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_include_deleted = include_deleted;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_title = title;

    let uri_str = format!("{}/api-v2/projects", configuration.base_path);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ProjectListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ProjectListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProjectControllerSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_controller_update(
    configuration: &configuration::Configuration,
    id: &str,
    update_project_dto: models::UpdateProject,
) -> Result<models::Id, Error<ProjectControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_project_dto = update_project_dto;

    let uri_str = format!(
        "{}/api-v2/projects/{id}",
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
    req_builder = req_builder.json(&p_body_update_project_dto);

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
        let entity: Option<ProjectControllerUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_roles_controller_create(
    configuration: &configuration::Configuration,
    project_id: &str,
    create_project_role_dto: models::CreateProjectRole,
) -> Result<models::Id, Error<ProjectRolesControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project_id = project_id;
    let p_body_create_project_role_dto = create_project_role_dto;

    let uri_str = format!(
        "{}/api-v2/projects/{projectId}/roles",
        configuration.base_path,
        projectId = crate::apis::urlencode(p_path_project_id)
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
    req_builder = req_builder.json(&p_body_create_project_role_dto);

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
        let entity: Option<ProjectRolesControllerCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_roles_controller_delete(
    configuration: &configuration::Configuration,
    project_id: &str,
    id: &str,
) -> Result<models::ProjectRole, Error<ProjectRolesControllerDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project_id = project_id;
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/projects/{projectId}/roles/{id}",
        configuration.base_path,
        projectId = crate::apis::urlencode(p_path_project_id),
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ProjectRoleDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ProjectRoleDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProjectRolesControllerDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_roles_controller_get(
    configuration: &configuration::Configuration,
    project_id: &str,
    id: &str,
) -> Result<models::ProjectRole, Error<ProjectRolesControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project_id = project_id;
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/projects/{projectId}/roles/{id}",
        configuration.base_path,
        projectId = crate::apis::urlencode(p_path_project_id),
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ProjectRoleDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ProjectRoleDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProjectRolesControllerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_roles_controller_search(
    configuration: &configuration::Configuration,
    project_id: &str,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<&str>,
) -> Result<models::ProjectRoleList, Error<ProjectRolesControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project_id = project_id;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_name = name;

    let uri_str = format!(
        "{}/api-v2/projects/{projectId}/roles",
        configuration.base_path,
        projectId = crate::apis::urlencode(p_path_project_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ProjectRoleListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ProjectRoleListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ProjectRolesControllerSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn project_roles_controller_update(
    configuration: &configuration::Configuration,
    project_id: &str,
    id: &str,
    update_project_role_dto: models::UpdateProjectRole,
) -> Result<models::Id, Error<ProjectRolesControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project_id = project_id;
    let p_path_id = id;
    let p_body_update_project_role_dto = update_project_role_dto;

    let uri_str = format!(
        "{}/api-v2/projects/{projectId}/roles/{id}",
        configuration.base_path,
        projectId = crate::apis::urlencode(p_path_project_id),
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_project_role_dto);

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
        let entity: Option<ProjectRolesControllerUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
