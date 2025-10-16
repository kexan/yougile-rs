use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialsWithName {
    /// Логин пользователя
    #[serde(rename = "login")]
    pub login: String,
    /// Пароль пользователя
    #[serde(rename = "password")]
    pub password: String,
    /// Название компании
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CredentialsWithName {
    pub fn new(login: String, password: String) -> CredentialsWithName {
        CredentialsWithName {
            login,
            password,
            name: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialsWithCompany {
    /// Логин пользователя
    #[serde(rename = "login")]
    pub login: String,
    /// Пароль пользователя
    #[serde(rename = "password")]
    pub password: String,
    /// ID компании. Для получения ID компании можно использовать горячие клавиши в интерфейсе (Ctrl+Alt+Q / Ctrl+Option+Q для Mac) или воспользоваться [методом POST /auth/companies](../operations/getCompanies)
    #[serde(rename = "companyId")]
    pub company_id: String,
}

impl CredentialsWithCompany {
    pub fn new(login: String, password: String, company_id: String) -> CredentialsWithCompany {
        CredentialsWithCompany {
            login,
            password,
            company_id,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialsWithCompanyOptional {
    /// Логин пользователя
    #[serde(rename = "login")]
    pub login: String,
    /// Пароль пользователя
    #[serde(rename = "password")]
    pub password: String,
    /// ID компании. Для получения ID компании можно использовать горячие клавиши в интерфейсе (Ctrl+Alt+Q / Ctrl+Option+Q для Mac) или воспользоваться [методом POST /auth/companies](../operations/getCompanies)
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
}

impl CredentialsWithCompanyOptional {
    pub fn new(login: String, password: String) -> CredentialsWithCompanyOptional {
        CredentialsWithCompanyOptional {
            login,
            password,
            company_id: None,
        }
    }
}
