use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название компании
    #[serde(rename = "title")]
    pub title: String,
    /// Время создания компании
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Вспомогательные данные для разработки
    #[serde(rename = "apiData", skip_serializing_if = "Option::is_none")]
    pub api_data: Option<serde_json::Value>,
}

impl Company {
    pub fn new(id: String, title: String, timestamp: f64) -> Company {
        Company {
            deleted: None,
            id,
            title,
            timestamp,
            api_data: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список компаний пользователя
    #[serde(rename = "content")]
    pub content: Vec<CompanyListBase>,
}

impl CompanyList {
    pub fn new(paging: models::PagingMetadata, content: Vec<CompanyListBase>) -> CompanyList {
        CompanyList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyListBase {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название компании
    #[serde(rename = "name")]
    pub name: String,
    /// Права администратора в компании
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}

impl CompanyListBase {
    pub fn new(id: String, name: String, is_admin: bool) -> CompanyListBase {
        CompanyListBase { id, name, is_admin }
    }
}
