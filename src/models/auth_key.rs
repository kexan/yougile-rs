use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthKeyDto {
    /// Ключ авторизации
    #[serde(rename = "key")]
    pub key: String,
}

impl AuthKeyDto {
    pub fn new(key: String) -> AuthKeyDto {
        AuthKeyDto { key }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthKeyWithDetails {
    /// Ключ авторизации
    #[serde(rename = "key")]
    pub key: String,
    /// ID компании, к которой относится ключ
    #[serde(rename = "companyId")]
    pub company_id: String,
    /// Время создания
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Ключ удален - да/нет
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl AuthKeyWithDetails {
    pub fn new(
        key: String,
        company_id: String,
        timestamp: f64,
        deleted: bool,
    ) -> AuthKeyWithDetails {
        AuthKeyWithDetails {
            key,
            company_id,
            timestamp,
            deleted,
        }
    }
}
