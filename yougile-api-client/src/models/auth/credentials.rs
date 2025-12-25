use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentials {
    #[serde(rename = "login")]
    pub login: String,

    #[serde(rename = "password")]
    pub password: String,

    /// Название компании
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,

    /// ID компании. Для получения ID компании можно использовать горячие клавиши в интерфейсе (Ctrl+Alt+Q / Ctrl+Option+Q для Mac) или воспользоваться [методом POST /auth/companies](../operations/getCompanies)
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
}

impl AuthCredentials {
    pub fn with_name(login: String, password: String, name: String) -> Self {
        Self {
            login,
            password,
            company_name: Some(name),
            company_id: None,
        }
    }

    pub fn with_company_id(login: String, password: String, company_id: String) -> Self {
        Self {
            login,
            password,
            company_name: None,
            company_id: Some(company_id),
        }
    }

    pub fn without_company(login: String, password: String) -> Self {
        Self {
            login,
            password,
            company_name: None,
            company_id: None,
        }
    }
}
