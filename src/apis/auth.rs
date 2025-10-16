use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`auth_key_controller_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthKeyControllerCreateError {
    Status400(),
    Status401(),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_key_controller_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthKeyControllerDeleteError {
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_key_controller_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthKeyControllerSearchError {
    Status400(),
    Status401(),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_companies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompaniesError {
    Status401(),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`company_controller_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompanyControllerGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`company_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompanyControllerUpdateError {
    Status404(),
    UnknownValue(serde_json::Value),
}

pub async fn auth_key_controller_create(
    configuration: &configuration::Configuration,
    credentials_with_company_dto: models::CredentialsWithCompanyDto,
) -> Result<models::AuthKeyDto, Error<AuthKeyControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_credentials_with_company_dto = credentials_with_company_dto;

    let uri_str = format!("{}/api-v2/auth/keys", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_credentials_with_company_dto);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AuthKeyDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AuthKeyDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AuthKeyControllerCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn auth_key_controller_delete(
    configuration: &configuration::Configuration,
    key: &str,
) -> Result<(), Error<AuthKeyControllerDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_key = key;

    let uri_str = format!(
        "{}/api-v2/auth/keys/{key}",
        configuration.base_path,
        key = crate::apis::urlencode(p_path_key)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AuthKeyControllerDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn auth_key_controller_search(
    configuration: &configuration::Configuration,
    credentials_with_company_optional_dto: models::CredentialsWithCompanyOptionalDto,
) -> Result<Vec<models::AuthKeyWithDetailsDto>, Error<AuthKeyControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_credentials_with_company_optional_dto = credentials_with_company_optional_dto;

    let uri_str = format!("{}/api-v2/auth/keys/get", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_credentials_with_company_optional_dto);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec<models::AuthKeyWithDetailsDto>`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec<models::AuthKeyWithDetailsDto>`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AuthKeyControllerSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Получить детали текущей компании
pub async fn company_controller_get(
    configuration: &configuration::Configuration,
) -> Result<models::Company, Error<CompanyControllerGetError>> {
    let uri_str = format!("{}/api-v2/companies*", configuration.base_path);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CompanyDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CompanyDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CompanyControllerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Изменить детали текущей компании
pub async fn company_controller_update(
    configuration: &configuration::Configuration,
    update_company_dto: models::UpdateCompany,
) -> Result<models::Id, Error<CompanyControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_update_company_dto = update_company_dto;

    let uri_str = format!("{}/api-v2/companies*", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_company_dto);

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
        let entity: Option<CompanyControllerUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_companies(
    configuration: &configuration::Configuration,
    credentials_with_name_dto: models::CredentialsWithNameDto,
    limit: Option<f64>,
    offset: Option<f64>,
) -> Result<models::CompanyList, Error<GetCompaniesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_credentials_with_name_dto = credentials_with_name_dto;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!("{}/api-v2/auth/companies", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_credentials_with_name_dto);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CompanyListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CompanyListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCompaniesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
