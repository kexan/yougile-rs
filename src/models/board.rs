use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Board {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название доски
    #[serde(rename = "title")]
    pub title: String,
    /// ID проекта, в котором находится доска
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Стикеры доски
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Box<models::StickersDto>>,
}

impl Board {
    pub fn new(id: String, title: String, project_id: String) -> Board {
        Board {
            deleted: None,
            id,
            title,
            project_id,
            stickers: None,
        }
    }
}
