use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt},
    models::{
        CreateProject, CreateProjectRole, Id, Project, ProjectList, ProjectRole, ProjectRoleList,
        UpdateProject, UpdateProjectRole,
    },
    YougileError,
};

const PROJECTS_PATH: &str = "/api-v2/projects";

pub async fn create_project(
    configuration: &Configuration,
    create_project: CreateProject,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, PROJECTS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_project)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_project(configuration: &Configuration, id: &str) -> Result<Project, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, PROJECTS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_project(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<&str>,
) -> Result<ProjectList, YougileError> {
    let url = format!("{}{}", configuration.base_path, PROJECTS_PATH);

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

pub async fn update_project(
    configuration: &Configuration,
    id: &str,
    update_project: UpdateProject,
) -> Result<Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, PROJECTS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_project)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

const PROJECT_ROLES_PATH: &str = "/api-v2/projects/{}/roles";

pub async fn create_project_role(
    configuration: &Configuration,
    project_id: &str,
    create_project_role: CreateProjectRole,
) -> Result<Id, YougileError> {
    let encoded_project_id = crate::apis::urlencode(project_id);
    let url = format!(
        "{}{}",
        configuration.base_path,
        PROJECT_ROLES_PATH.replace("{}", &encoded_project_id)
    );

    let resp = configuration
        .client
        .post(&url)
        .json(&create_project_role)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn delete_project_role(
    configuration: &Configuration,
    project_id: &str,
    id: &str,
) -> Result<ProjectRole, YougileError> {
    let encoded_project_id = crate::apis::urlencode(project_id);
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}/api-v2/projects/{}/roles/{}",
        configuration.base_path, encoded_project_id, encoded_id
    );

    let resp = configuration
        .client
        .delete(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_project_role(
    configuration: &Configuration,
    project_id: &str,
    id: &str,
) -> Result<ProjectRole, YougileError> {
    let encoded_project_id = crate::apis::urlencode(project_id);
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}/api-v2/projects/{}/roles/{}",
        configuration.base_path, encoded_project_id, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_project_roles(
    configuration: &Configuration,
    project_id: &str,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<&str>,
) -> Result<ProjectRoleList, YougileError> {
    let encoded_project_id = crate::apis::urlencode(project_id);
    let url = format!(
        "{}/api-v2/projects/{}/roles",
        configuration.base_path, encoded_project_id
    );

    let mut query_params = vec![];
    if let Some(val) = limit {
        query_params.push(("limit", val.to_string()));
    }
    if let Some(val) = offset {
        query_params.push(("offset", val.to_string()));
    }
    if let Some(val) = name {
        query_params.push(("name", val.to_string()));
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

pub async fn update_project_role(
    configuration: &Configuration,
    project_id: &str,
    id: &str,
    update_project_role: UpdateProjectRole,
) -> Result<Id, YougileError> {
    let encoded_project_id = crate::apis::urlencode(project_id);
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}/api-v2/projects/{}/roles/{}",
        configuration.base_path, encoded_project_id, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_project_role)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
