use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDto {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название проекта
    #[serde(rename = "title")]
    pub title: String,
    /// Время создания проекта
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl ProjectDto {
    pub fn new(id: String, title: String, timestamp: f64) -> ProjectDto {
        ProjectDto {
            deleted: None,
            id,
            title,
            timestamp,
            users: None,
        }
    }
}
