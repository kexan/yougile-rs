use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateBoard {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название доски
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ID проекта, в котором находится доска
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Стикеры доски
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Box<models::StickersDto>>,
}

impl UpdateBoard {
    pub fn new() -> UpdateBoard {
        UpdateBoard {
            deleted: None,
            title: None,
            project_id: None,
            stickers: None,
        }
    }
}
