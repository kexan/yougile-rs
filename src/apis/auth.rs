use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt},
    models::{
        AuthKey, AuthKeyWithDetails, Company, CompanyList, CredentialsWithCompany,
        CredentialsWithCompanyOptional, CredentialsWithName, Id, UpdateCompany,
    },
    YougileError,
};

const AUTH_KEYS_PATH: &str = "/api-v2/auth/keys";
const AUTH_COMPANIES_PATH: &str = "/api-v2/auth/companies";
const COMPANIES_PATH: &str = "/api-v2/companies*";

pub async fn create_auth_key(
    configuration: &Configuration,
    credentials_with_company: CredentialsWithCompany,
) -> Result<AuthKey, YougileError> {
    let url = format!("{}{}", configuration.base_path, AUTH_KEYS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&credentials_with_company)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn delete_auth_key(configuration: &Configuration, key: &str) -> Result<(), YougileError> {
    let encoded_key = crate::apis::urlencode(key);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, AUTH_KEYS_PATH, encoded_key
    );

    let resp = configuration
        .client
        .delete(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        parse_response(resp).await
    }
}

pub async fn search_auth_keys(
    configuration: &Configuration,
    credentials_with_company_optional: CredentialsWithCompanyOptional,
) -> Result<Vec<AuthKeyWithDetails>, YougileError> {
    let url = format!("{}/api-v2/auth/keys/get", configuration.base_path);

    let resp = configuration
        .client
        .post(&url)
        .json(&credentials_with_company_optional)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

/// Получить детали текущей компании
pub async fn get_company(configuration: &Configuration) -> Result<Company, YougileError> {
    let url = format!("{}{}", configuration.base_path, COMPANIES_PATH);

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

/// Изменить детали текущей компании
pub async fn update_company(
    configuration: &Configuration,
    update_company: UpdateCompany,
) -> Result<Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, COMPANIES_PATH);

    let resp = configuration
        .client
        .put(&url)
        .json(&update_company)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_companies(
    configuration: &Configuration,
    credentials_with_name: CredentialsWithName,
    limit: Option<f64>,
    offset: Option<f64>,
) -> Result<CompanyList, YougileError> {
    let url = format!("{}{}", configuration.base_path, AUTH_COMPANIES_PATH);

    let mut query_params = vec![];
    if let Some(val) = limit {
        query_params.push(("limit", val.to_string()));
    }
    if let Some(val) = offset {
        query_params.push(("offset", val.to_string()));
    }

    let resp = configuration
        .client
        .post(&url)
        .query(&query_params)
        .json(&credentials_with_name)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
